//! End-to-end integration test: proves the happy path works end-to-end.
//!
//! 1. Forgemaster (agent-trait) receives a health.check Bottle → returns response
//! 2. Conservation audit on the response (trit sum preserved: input == output)
//! 3. InMemoryCocapn (cocapn) registers the Forgemaster as a ship
//! 4. Cocapn routes a fleet-level bottle
//! 5. Fleet audit confirms conservation holds at fleet level

use superinstance_agent_trait::{
    Agent, AgentReport, AgentRunner, AgentState,
    Bottle, Trit, audit, audit_strict,
};
use superinstance_agent_trait::forgemaster::Forgemaster;
use superinstance_cocapn::{
    Cocapn,
    FleetBottle, ShipId, ShipState, ShipHealth, ConservationState,
};
use superinstance_cocapn::cocapn::InMemoryCocapn;
use std::collections::HashMap;

/// Helper: build a healthy ShipState representing a Forgemaster.
fn forgemaster_ship_state(id: &str, gamma: f64, eta: f64, c: f64) -> ShipState {
    ShipState {
        id: ShipId(id.to_string()),
        conservation: ConservationState { gamma, eta, c },
        health: ShipHealth::Healthy,
        active_load: 0,
        capacity: 100,
        last_heartbeat: 0,
        metadata: HashMap::new(),
    }
}

// ---------------------------------------------------------------------------
// Part 1: Agent-level happy path — Forgemaster + Bottle conservation
// ---------------------------------------------------------------------------

#[test]
fn forgemaster_health_check_conserves_trit_sum() {
    // Step 1: Create a Forgemaster
    let fm = Forgemaster::new();
    let mut runner = AgentRunner::new(fm);

    // Step 2: Init the agent (system.init)
    let init_bottle = Bottle::new_empty("cocapn", "forgemaster-1", "system.init", vec![1, 0, -1], 300);
    let init_resp = runner.receive(init_bottle.clone()).expect("init should succeed");
    assert_eq!(init_resp.act, "system.init.ack");
    assert_eq!(runner.state(), AgentState::Active);

    // Step 3: Send a health.check bottle (non-system, unknown action → echo)
    let input_trits: Vec<Trit> = vec![1, 0, 1, -1, 0]; // sum = 1
    let request = Bottle::new(
        "cocapn",
        "forgemaster-1",
        "health.check",
        input_trits.clone(),
        &serde_json::json!({"check_type": "ping", "timeout_ms": 5000}),
        300,
    ).expect("bottle creation should succeed");

    // Step 4: Forgemaster receives the bottle → returns response bottle
    let response = runner.receive(request.clone()).expect("health.check should succeed");

    // Step 5: Verify conservation: trit sum(input) == trit sum(output)
    assert!(audit(&request, &response), 
        "conservation violated: input sum {} != output sum {}",
        request.trit_sum(), response.trit_sum());
    audit_strict(&request, &response).expect("strict audit should pass");

    // Verify response structure
    assert_eq!(response.src, "forgemaster");
    assert_eq!(response.tgt, "cocapn");
}

#[test]
fn forgemaster_cycle_request_conserves_trit_sum() {
    let fm = Forgemaster::new();
    let mut runner = AgentRunner::new(fm);

    // Init
    let _ = runner.receive(
        Bottle::new_empty("cocapn", "fm", "system.init", vec![1, 0, -1], 300)
    ).expect("init");

    // Run multiple cycles — conservation must hold for each
    let input_trits: Vec<Trit> = vec![1, 0, -1, 0, 1]; // sum = 1
    for i in 0..5 {
        let request = Bottle::new_empty("fleet-edge", "fm", "cycle.request", input_trits.clone(), 300);
        let response = runner.receive(request.clone())
            .unwrap_or_else(|e| panic!("cycle {} failed: {}", i, e));
        assert_eq!(response.act, "cycle.complete");
        assert!(audit(&request, &response),
            "conservation violated on cycle {}: input {} != output {}",
            i, request.trit_sum(), response.trit_sum());
    }

    // Verify agent state via inspect
    let report: AgentReport = runner.inspect();
    assert_eq!(report.cycle_count, 5);
    assert_eq!(report.state, AgentState::Active);
}

// ---------------------------------------------------------------------------
// Part 2: Fleet-level happy path — InMemoryCocapn + fleet conservation
// ---------------------------------------------------------------------------

#[test]
fn cocapn_registers_ship_and_fleet_audit_confirms_conservation() {
    // Step 5: Create an InMemoryCocapn
    let mut cocapn = InMemoryCocapn::new();

    // Step 6: Register a Forgemaster as a ship with conservation state
    // γ (gamma) = 0.3, η (eta) = 0.7, C = 1.0  →  γ + η = C ✓
    let ship = forgemaster_ship_state("forgemaster-1", 0.3, 0.7, 1.0);
    cocapn.register_ship(ship.id.clone(), ship);

    // Register a second ship: γ = 0.5, η = 0.5, C = 1.0
    let ship2 = forgemaster_ship_state("forgemaster-2", 0.5, 0.5, 1.0);
    cocapn.register_ship(ship2.id.clone(), ship2);

    // Step 7: Cocapn routes a bottle (health.check via FleetBottle)
    let route_response = cocapn.handle_bottle(FleetBottle::RouteRequest {
        payload: b"health.check".to_vec(),
        required_capability: None,
    });
    match route_response {
        FleetBottle::RouteResponse(decision) => {
            assert!(decision.target.is_some(), "should route to a ship");
            assert!(decision.target_utilization.is_some());
        }
        other => panic!("expected RouteResponse, got {:?}", other),
    }

    // Step 8: Fleet audit confirms conservation holds
    let audit_response = cocapn.handle_bottle(FleetBottle::AuditRequest { tolerance: 0.01 });
    match audit_response {
        FleetBottle::AuditResponse(fc) => {
            // total γ + total η should equal total C
            // ship1: 0.3 + 0.7 = 1.0, ship2: 0.5 + 0.5 = 1.0
            // fleet: γ=0.8, η=1.2, C=2.0 → 0.8+1.2=2.0 ✓
            assert_eq!(fc.ship_count, 2);
            assert!(fc.balanced, 
                "fleet conservation violated: γ({}) + η({}) ≠ C({}), deficit={}",
                fc.total_gamma, fc.total_eta, fc.fleet_c, fc.deficit());
            assert!((fc.total_gamma + fc.total_eta - fc.fleet_c).abs() < 0.01);
        }
        other => panic!("expected AuditResponse, got {:?}", other),
    }
}

#[test]
fn cocapn_rebalance_returns_no_decisions_for_balanced_fleet() {
    let mut cocapn = InMemoryCocapn::new();
    
    let ship = forgemaster_ship_state("ship-1", 0.4, 0.6, 1.0);
    cocapn.register_ship(ship.id.clone(), ship);

    let response = cocapn.handle_bottle(FleetBottle::RebalanceCommand);
    match response {
        FleetBottle::RebalanceResponse(decisions) => {
            assert!(decisions.is_empty(), "single ship fleet needs no rebalance");
        }
        other => panic!("expected RebalanceResponse, got {:?}", other),
    }
}

// ---------------------------------------------------------------------------
// Part 3: Full pipeline — wire bottle through agent → register in fleet → audit
// ---------------------------------------------------------------------------

#[test]
fn full_pipeline_agent_to_fleet_conservation() {
    // --- Agent layer ---
    let fm = Forgemaster::new();
    let mut runner = AgentRunner::new(fm);

    // Init
    let init = Bottle::new_empty("cocapn", "pipeline-ship", "system.init", vec![1, 0, -1], 300);
    let _ = runner.receive(init).expect("init");

    // Agent processes a cycle
    let input_trits: Vec<Trit> = vec![1, 0, -1, 1]; // sum = 1
    let request = Bottle::new_empty("cocapn", "pipeline-ship", "cycle.request", input_trits.clone(), 300);
    let response = runner.receive(request.clone()).expect("cycle should succeed");

    // Agent-level conservation
    assert!(audit(&request, &response));
    assert_eq!(response.act, "cycle.complete");

    // Derive fleet conservation state from the agent's response trits
    let trit_sum = response.trit_sum() as f64;
    let gamma = trit_sum * 0.3;
    let eta = trit_sum * 0.7;
    let c = trit_sum; // γ + η = C

    // --- Fleet layer ---
    let mut cocapn = InMemoryCocapn::new();
    let ship = forgemaster_ship_state("pipeline-ship", gamma, eta, c);
    cocapn.register_ship(ship.id.clone(), ship);

    // Fleet audit
    let fc = match cocapn.handle_bottle(FleetBottle::AuditRequest { tolerance: 0.01 }) {
        FleetBottle::AuditResponse(fc) => fc,
        other => panic!("expected AuditResponse, got {:?}", other),
    };

    assert!(fc.balanced,
        "fleet-level conservation violated: γ({}) + η({}) != C({})",
        fc.total_gamma, fc.total_eta, fc.fleet_c);

    // Route to the ship
    let route = cocapn.handle_bottle(FleetBottle::RouteRequest {
        payload: b"next_cycle".to_vec(),
        required_capability: None,
    });
    match route {
        FleetBottle::RouteResponse(decision) => {
            assert_eq!(decision.target, Some(ShipId("pipeline-ship".to_string())));
        }
        other => panic!("expected RouteResponse, got {:?}", other),
    }
}
