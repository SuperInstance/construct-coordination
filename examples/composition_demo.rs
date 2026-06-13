//! # SEED Crate Composition Demo
//!
//! Demonstrates that SEED crates compose together into real coordination patterns.
//! Each example is a self-contained function showing a distinct distributed-systems
//! pattern built from multiple SEED crates working in concert.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

// ── Crate imports ─────────────────────────────────────────────────────────────

use circuit_breaker::{CircuitBreaker, CircuitBreakerConfig, CircuitError};
use config_center::{ConfigCenter, ConfigValue};
use feature_flag::{
    EvaluationContext, FeatureFlag, FeatureFlagService, Rule, RuleOperator,
};
use health_check::{
    HealthCheckResult, HealthRegistry, HealthStatus, LambdaHealthCheck,
};
use load_balancer::{
    LoadBalancer, RequestContext, RoundRobin, Target,
};
use rate_limiter::TokenBucketRateLimiter;
use retry_backoff::{retry, ExponentialBackoff, RetryPolicy};
use service_discovery::{
    HealthStatus as SdHealthStatus, ServiceInstance, ServiceRegistry,
};

// ── Main ──────────────────────────────────────────────────────────────────────

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║         SEED Crate Composition Demos                        ║");
    println!("║   Proving that independently developed crates compose       ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    demo_1_resilience_pattern();
    println!();

    demo_2_traffic_control_pattern();
    println!();

    demo_3_discovery_pattern();
    println!();

    println!("════════════════════════════════════════════════════════════════");
    println!("  All three composition demos completed successfully. ✅");
    println!("  SEED crates compose. Each pattern used 2-3 crates together.");
    println!("════════════════════════════════════════════════════════════════");
}

// ╔═════════════════════════════════════════════════════════════════════════════╗
// ║  DEMO 1: Resilience Pattern                                               ║
// ║  health-check + retry-backoff + circuit-breaker                            ║
// ╚═════════════════════════════════════════════════════════════════════════════╝

fn demo_1_resilience_pattern() {
    println!("━━━ Demo 1: Resilience Pattern ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  Crates: health-check + retry-backoff + circuit-breaker");
    println!();

    // Simulate a service that fails a configurable number of times then recovers.
    let fail_count = Arc::new(Mutex::new(0u32));
    let should_fail_until = Arc::new(Mutex::new(3u32)); // fail first 3 calls

    // ── Health check: monitor the simulated service ───────────────────────
    let fail_count_hc = fail_count.clone();
    let should_fail_hc = should_fail_until.clone();
    let mut registry = HealthRegistry::new();
    registry.register(Box::new(LambdaHealthCheck::new("payment-service", move || {
        let fc = *fail_count_hc.lock().unwrap();
        let threshold = *should_fail_hc.lock().unwrap();
        let status = if fc < threshold {
            HealthStatus::Unhealthy
        } else {
            HealthStatus::Healthy
        };
        HealthCheckResult {
            status,
            message: Some(format!("fail_count={fc}, threshold={threshold}")),
            duration: Duration::from_micros(50),
        }
    })));

    // ── Circuit breaker: protect against cascading failures ──────────────
    let mut cb = CircuitBreaker::new(CircuitBreakerConfig {
        failure_threshold: 3,
        success_threshold: 2,
        timeout: Duration::from_millis(50),
    });

    // ── Simulated operation ──────────────────────────────────────────────
    let fail_count_op = fail_count.clone();
    let should_fail_op = should_fail_until.clone();
    let mut call_service = || -> Result<String, String> {
        let fc = *fail_count_op.lock().unwrap();
        let threshold = *should_fail_op.lock().unwrap();
        if fc < threshold {
            // Don't increment here — retry does that
            return Err(format!("service unavailable (attempt {})", fc + 1));
        }
        Ok("payment processed".to_string())
    };

    // ── Phase 1: Service is down, retries exhaust, circuit trips ─────────
    println!("  Phase 1: Service failing → retries exhaust → circuit trips");
    for attempt in 1..=4 {
        let report = registry.check_all();
        println!("    [attempt {attempt}] health={}", report.status);

        // Retry with exponential backoff (very short for demo)
        let mut policy = RetryPolicy::new(
            ExponentialBackoff::new(
                Duration::from_millis(1),
                Duration::from_millis(10),
                2.0,
                2, // 2 retries
            ),
            |_| true, // all errors retryable
        );

        let result = cb.call(|| retry(&mut policy, &mut call_service));
        match result {
            Err(CircuitError::Open) => {
                println!("    [attempt {attempt}] ⛔ circuit OPEN — call rejected");
            }
            Err(CircuitError::Inner(retry_backoff::RetryError::Exhausted { last_error, attempts })) => {
                // Increment fail count to simulate service eventually recovering
                let mut fc = fail_count.lock().unwrap();
                *fc += 1;
                println!("    [attempt {attempt}] ❌ retry exhausted after {attempts} tries: {last_error}");
                println!("             circuit state: {:?}", cb.state());
            }
            Err(CircuitError::Inner(retry_backoff::RetryError::Inner(e))) => {
                println!("    [attempt {attempt}] ❌ non-retryable: {e}");
            }
            Ok(val) => {
                println!("    [attempt {attempt}] ✅ {val}");
            }
        }
    }

    println!("    → Circuit state: {:?}", cb.state());

    // ── Phase 2: Service recovers, wait for half-open, then close ────────
    println!();
    println!("  Phase 2: Service recovers → circuit half-opens → closes");
    std::thread::sleep(Duration::from_millis(60)); // wait for CB timeout

    // Service is now healthy (fail_count >= threshold)
    let report = registry.check_all();
    println!("    health={}", report.status);

    for probe in 1..=3 {
        let mut policy = RetryPolicy::new(
            ExponentialBackoff::new(Duration::from_millis(1), Duration::from_millis(10), 2.0, 1),
            |_| true,
        );
        match cb.call(|| retry(&mut policy, &mut call_service)) {
            Ok(val) => println!("    [probe {probe}] ✅ {val}  (state: {:?})", cb.state()),
            Err(e) => println!("    [probe {probe}] ❌ {e}  (state: {:?})", cb.state()),
        }
    }

    println!("    → Final circuit state: {:?}", cb.state());
    println!("  ✅ Resilience pattern: health-check detects, retry-backoff retries, circuit-breaker protects");
}

// ╔═════════════════════════════════════════════════════════════════════════════╗
// ║  DEMO 2: Traffic Control Pattern                                           ║
// ║  rate-limiter + load-balancer                                              ║
// ╚═════════════════════════════════════════════════════════════════════════════╝

fn demo_2_traffic_control_pattern() {
    println!("━━━ Demo 2: Traffic Control Pattern ━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  Crates: rate-limiter + load-balancer");
    println!();

    // ── Rate limiter: cap at 5 requests per burst ────────────────────────
    let mut limiter = TokenBucketRateLimiter::new(5, 100.0); // 5 tokens, fast refill

    // ── Load balancer: 3 backends with round-robin ───────────────────────
    let backends = vec![
        Target {
            id: "backend-alpha".into(),
            weight: 1.0,
            healthy: true,
            active_connections: 0,
        },
        Target {
            id: "backend-beta".into(),
            weight: 1.0,
            healthy: true,
            active_connections: 0,
        },
        Target {
            id: "backend-gamma".into(),
            weight: 1.0,
            healthy: true,
            active_connections: 0,
        },
    ];
    let lb = LoadBalancer::new(backends, Box::new(RoundRobin::new()));

    // ── Send 10 requests through both crates ─────────────────────────────
    println!("  Sending 10 requests through rate-limiter → load-balancer:");
    println!();

    let mut accepted = 0u32;
    let mut rejected = 0u32;
    let mut distribution: HashMap<String, u32> = HashMap::new();

    for req_id in 1..=10 {
        // Gate 1: rate limiter
        if !limiter.try_acquire() {
            println!("    request {req_id:>2}: ⛔ rate-limited (no tokens)");
            rejected += 1;
            continue;
        }

        // Gate 2: load balancer picks a backend
        let ctx = RequestContext {
            source: format!("client-{req_id}"),
            action: "process".into(),
        };
        match lb.select(&ctx) {
            Some(idx) => {
                let backend_id = &lb.targets[idx].id;
                *distribution.entry(backend_id.clone()).or_default() += 1;
                accepted += 1;
                println!("    request {req_id:>2}: ✅ → {backend_id}");
            }
            None => {
                rejected += 1;
                println!("    request {req_id:>2}: ⚠️  no healthy backend");
            }
        }
    }

    println!();
    println!("  Results: {accepted} accepted, {rejected} rejected");
    println!("  Backend distribution:");
    for (backend, count) in &distribution {
        println!("    {backend}: {count} requests");
    }

    // ── Phase 2: tokens refill, more requests flow ───────────────────────
    println!();
    println!("  After token refill, sending 5 more requests:");
    std::thread::sleep(Duration::from_millis(20)); // let tokens refill a bit

    for req_id in 11..=15 {
        if !limiter.try_acquire() {
            println!("    request {req_id:>2}: ⛔ rate-limited");
            continue;
        }
        let ctx = RequestContext {
            source: format!("client-{req_id}"),
            action: "process".into(),
        };
        if let Some(idx) = lb.select(&ctx) {
            println!("    request {req_id:>2}: ✅ → {}", lb.targets[idx].id);
        }
    }

    println!("  ✅ Traffic control: rate-limiter caps throughput, load-balancer distributes evenly");
}

// ╔═════════════════════════════════════════════════════════════════════════════╗
// ║  DEMO 3: Discovery Pattern                                                ║
// ║  service-discovery + feature-flag + config-center                          ║
// ╚═════════════════════════════════════════════════════════════════════════════╝

fn demo_3_discovery_pattern() {
    println!("━━━ Demo 3: Discovery Pattern ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  Crates: service-discovery + feature-flag + config-center");
    println!();

    // ── Config center: store service endpoints ───────────────────────────
    let mut config = ConfigCenter::new();
    config.set("api-v2.endpoint", ConfigValue::String("10.0.1.50:8443".into()));
    config.set("api-v2.timeout_ms", ConfigValue::Int(5000));
    config.set("api-v2.max_retries", ConfigValue::Int(3));

    let mut api_metadata = HashMap::new();
    api_metadata.insert("version".to_string(), ConfigValue::String("2.1.0".into()));
    api_metadata.insert("tier".to_string(), ConfigValue::String("premium".into()));
    config.set(
        "api-v2.metadata",
        ConfigValue::Nested(api_metadata.clone()),
    );
    println!("  Config center loaded: api-v2 endpoint, timeout, retries");

    // ── Service discovery: register instances ────────────────────────────
    let mut discovery = ServiceRegistry::new(Duration::from_secs(30));

    let inst1 = ServiceInstance {
        id: "api-v2-us-east".into(),
        service_name: "api-v2".into(),
        address: "10.0.1.50".into(),
        port: 8443,
        metadata: {
            let mut m = HashMap::new();
            m.insert("region".into(), "us-east-1".into());
            m.insert("tier".into(), "premium".into());
            m
        },
        health: SdHealthStatus::Healthy,
        registered_at: std::time::Instant::now(),
        last_heartbeat: std::time::Instant::now(),
        trit_state: 1,
    };

    let inst2 = ServiceInstance {
        id: "api-v2-eu-west".into(),
        service_name: "api-v2".into(),
        address: "10.0.2.80".into(),
        port: 8443,
        metadata: {
            let mut m = HashMap::new();
            m.insert("region".into(), "eu-west-1".into());
            m.insert("tier".into(), "standard".into());
            m
        },
        health: SdHealthStatus::Healthy,
        registered_at: std::time::Instant::now(),
        last_heartbeat: std::time::Instant::now(),
        trit_state: 0,
    };

    let inst3 = ServiceInstance {
        id: "api-v2-ap-south".into(),
        service_name: "api-v2".into(),
        address: "10.0.3.120".into(),
        port: 8443,
        metadata: {
            let mut m = HashMap::new();
            m.insert("region".into(), "ap-south-1".into());
            m.insert("tier".into(), "premium".into());
            m
        },
        health: SdHealthStatus::Healthy,
        registered_at: std::time::Instant::now(),
        last_heartbeat: std::time::Instant::now(),
        trit_state: -1,
    };

    discovery.register(inst1).unwrap();
    discovery.register(inst2).unwrap();
    discovery.register(inst3).unwrap();
    println!("  Service discovery: registered 3 api-v2 instances");

    // ── Feature flag: gate access by tier ────────────────────────────────
    let mut flags = FeatureFlagService::new();

    // Premium users see all regions
    flags.register(FeatureFlag {
        key: "api-v2-access".into(),
        enabled: true,
        variants: HashMap::new(),
        rules: vec![Rule {
            attribute: "tier".into(),
            operator: RuleOperator::Equals,
            value: "premium".into(),
        }],
        default_variant: None,
    });

    // Standard users see only standard-tier services
    flags.register(FeatureFlag {
        key: "api-v2-standard".into(),
        enabled: true,
        variants: HashMap::new(),
        rules: vec![Rule {
            attribute: "tier".into(),
            operator: RuleOperator::Equals,
            value: "standard".into(),
        }],
        default_variant: None,
    });

    println!("  Feature flags: api-v2-access (premium), api-v2-standard (standard)");
    println!();

    // ── Resolve: discovery + feature-flag + config ───────────────────────
    println!("  ── Premium user resolves services ──");
    let mut premium_ctx = EvaluationContext::new();
    premium_ctx.attributes.insert("tier".into(), "premium".into());

    let premium_access = flags.is_enabled("api-v2-access", &premium_ctx);
    println!("    Feature flag 'api-v2-access' enabled: {premium_access}");

    if premium_access {
        let instances = discovery.discover_healthy("api-v2");
        let endpoint = config.get("api-v2.endpoint");
        let timeout = config.get("api-v2.timeout_ms");

        println!("    Discovered {} healthy instances:", instances.len());
        for inst in &instances {
            let region = inst.metadata.get("region").map(|s| s.as_str()).unwrap_or("?");
            println!("      {} → {}:{} [region={}]", inst.id, inst.address, inst.port, region);
        }
        if let Some(ConfigValue::String(ep)) = endpoint {
            println!("    Config endpoint: {ep}");
        }
        if let Some(ConfigValue::Int(t)) = timeout {
            println!("    Config timeout: {t}ms");
        }
    }

    println!();
    println!("  ── Standard user resolves services ──");
    let mut standard_ctx = EvaluationContext::new();
    standard_ctx.attributes.insert("tier".into(), "standard".into());

    let standard_access = flags.is_enabled("api-v2-standard", &standard_ctx);
    println!("    Feature flag 'api-v2-standard' enabled: {standard_access}");

    if standard_access {
        // Standard users: filter to standard-tier instances only
        let all_instances = discovery.discover_healthy("api-v2");
        let standard_instances: Vec<_> = all_instances
            .into_iter()
            .filter(|i| i.metadata.get("tier").map_or(false, |t| t == "standard"))
            .collect();

        println!("    Visible to standard tier: {} instances", standard_instances.len());
        for inst in &standard_instances {
            let region = inst.metadata.get("region").map(|s| s.as_str()).unwrap_or("?");
            println!("      {} → {}:{} [region={}]", inst.id, inst.address, inst.port, region);
        }
    }

    // ── Toggle: disable flag, show access revoked ────────────────────────
    println!();
    println!("  ── Toggle: disable premium access ──");
    flags.toggle("api-v2-access", false);
    let blocked = flags.is_enabled("api-v2-access", &premium_ctx);
    println!("    Feature flag 'api-v2-access' now: {blocked}");
    println!("    Premium user access: {}", if blocked { "GRANTED" } else { "DENIED" });

    println!("  ✅ Discovery pattern: config stores endpoints, discovery finds instances, feature-flags gate access");
}
