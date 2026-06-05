//! MUD World ↔ Ternary Fleet Bridge Experiment
//!
//! Tests whether mud-arena's MUD concepts (rooms, agents, items, NPCs, movement)
//! can be expressed using ONLY the ternary crate fleet:
//!   - ternary-room: Room graph with doors and agent tracking
//!   - ternary-cell: Agent state as TernaryCells with tick lifecycle
//!   - ternary-current: Information flow / event tracking as currents
//!   - ternary-ecosystem: NPC species with ecological dynamics
//!
//! Simulates 5 agents navigating a room graph for 500 ticks.

use std::collections::HashMap;

use ternary_cell::{TernaryCell, TernaryMessenger};
use ternary_current::{
    Current, CurrentMap, CurrentStrength, DownstreamConsumer, FlowDirection, RoomId, UpstreamSource,
};
use ternary_ecosystem::{CarryingCapacity, Ecosystem, FoodWeb, Niche, Species, Ternary};
use ternary_room::{Door, DoorAccess, Room, RoomBuilder, RoomCoordinator};

// ── MUD Items stored as room environment keys ──────────────────────────────

/// Helper: store items in a room's environment map as "item_{name}" = "count".
fn place_item(room: &mut Room, item_name: &str) {
    let key = format!("item_{}", item_name);
    let current: u32 = room.get_env(&key).and_then(|v| v.parse().ok()).unwrap_or(0);
    room.set_env(&key, &(current + 1).to_string());
}

/// Helper: take an item from a room's environment. Returns true if successful.
fn take_item(room: &mut Room, item_name: &str) -> bool {
    let key = format!("item_{}", item_name);
    if let Some(val) = room.get_env(&key).and_then(|v| v.parse::<u32>().ok()) {
        if val > 0 {
            room.set_env(&key, &(val - 1).to_string());
            return true;
        }
    }
    false
}

/// Helper: count items in a room.
fn count_items(room: &Room) -> usize {
    room.snapshot()
        .environment
        .iter()
        .filter(|(k, v)| k.starts_with("item_") && *v != "0")
        .map(|(_, v)| v.parse::<usize>().unwrap_or(0))
        .sum()
}

/// Helper: list item names in a room.
fn list_items(room: &Room) -> Vec<String> {
    room.snapshot()
        .environment
        .iter()
        .filter(|(k, v)| k.starts_with("item_") && *v != "0")
        .map(|(k, v)| {
            let count: usize = v.parse().unwrap_or(0);
            format!("{}x{}", k.strip_prefix("item_").unwrap_or(k), count)
        })
        .collect()
}

// ── Agent State ────────────────────────────────────────────────────────────

/// A MUD agent backed by a TernaryCell for decision-making.
struct MudAgent {
    /// Agent ID (maps to ternary_cell id).
    id: u64,
    /// Display name.
    name: String,
    /// Current room ID.
    current_room: u64,
    /// Internal ternary cell for perception/decision lifecycle.
    cell: TernaryCell,
    /// Personal inventory: item_name → count.
    inventory: HashMap<String, u32>,
    /// Rooms visited.
    rooms_visited: std::collections::HashSet<u64>,
    /// Total items collected.
    items_collected: u64,
}

impl MudAgent {
    fn new(id: u64, name: &str, start_room: u64) -> Self {
        Self {
            id,
            name: name.to_string(),
            current_room: start_room,
            cell: TernaryCell::with_value(id, 0),
            inventory: HashMap::new(),
            rooms_visited: std::collections::HashSet::from([start_room]),
            items_collected: 0,
        }
    }

    /// Items held count.
    fn items_held(&self) -> u32 {
        self.inventory.values().sum()
    }
}

// ── NPC System ─────────────────────────────────────────────────────────────

/// An NPC backed by an ecosystem Species with fixed strategy.
struct Npc {
    /// NPC name.
    name: String,
    /// Which room this NPC inhabits.
    room_id: u64,
    /// Species index in the ecosystem.
    species_idx: usize,
}

// ── Action types ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    Move,
    PickUp,
    Interact,
    Wait,
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Move => write!(f, "move"),
            Action::PickUp => write!(f, "pickup"),
            Action::Interact => write!(f, "interact"),
            Action::Wait => write!(f, "wait"),
        }
    }
}

// ── World Setup ────────────────────────────────────────────────────────────

fn build_world() -> (RoomCoordinator, Vec<MudAgent>, Vec<Npc>, Ecosystem) {
    // ── Rooms ──────────────────────────────────────────────────────────────
    // Create a 6-room MUD map:
    //  [Tavern] --north-- [Market] --east-- [Armory]
    //     |                  |                  |
    //   south             south              south
    //     |                  |                  |
    //  [Cellar] --east-- [Alley]  --east-- [Tunnel]

    let mut coord = RoomCoordinator::new();

    let rooms = vec![
        RoomBuilder::new(0, "Tavern").env("description", "A smoky tavern with wooden benches.").build(),
        RoomBuilder::new(1, "Market").env("description", "A bustling market square.").build(),
        RoomBuilder::new(2, "Armory").env("description", "Racks of weapons line the walls.").build(),
        RoomBuilder::new(3, "Cellar").env("description", "A damp cellar beneath the tavern.").build(),
        RoomBuilder::new(4, "Alley").env("description", "A dark alley between buildings.").build(),
        RoomBuilder::new(5, "Tunnel").env("description", "An underground tunnel with dripping water.").build(),
    ];

    for room in rooms {
        coord.add_room(room);
    }

    // Two-way doors
    let doors = vec![
        (0, 1, DoorAccess::Open), // Tavern ↔ Market
        (1, 2, DoorAccess::Open), // Market ↔ Armory
        (0, 3, DoorAccess::Open), // Tavern ↔ Cellar
        (1, 4, DoorAccess::Open), // Market ↔ Alley
        (2, 5, DoorAccess::Open), // Armory ↔ Tunnel
        (3, 4, DoorAccess::Open), // Cellar ↔ Alley
        (4, 5, DoorAccess::Open), // Alley ↔ Tunnel
    ];

    for (i, (a, b, access)) in doors.into_iter().enumerate() {
        coord.add_door(Door::new(i as u64 + 1, a, b, access));
    }

    // Place items in rooms
    if let Some(r) = coord.room_mut(0) { place_item(r, "ale"); place_item(r, "bread"); }
    if let Some(r) = coord.room_mut(1) { place_item(r, "apple"); place_item(r, "map"); place_item(r, "rope"); }
    if let Some(r) = coord.room_mut(2) { place_item(r, "sword"); place_item(r, "shield"); }
    if let Some(r) = coord.room_mut(3) { place_item(r, "key"); }
    if let Some(r) = coord.room_mut(4) { place_item(r, "dagger"); }
    if let Some(r) = coord.room_mut(5) { place_item(r, "torch"); place_item(r, "gem"); }

    // ── Agents ─────────────────────────────────────────────────────────────
    let agents = vec![
        MudAgent::new(1, "Explorer", 0),
        MudAgent::new(2, "Scavenger", 1),
        MudAgent::new(3, "Warrior", 2),
        MudAgent::new(4, "Sneak", 4),
        MudAgent::new(5, "Wanderer", 3),
    ];

    // Place agents in rooms
    for agent in &agents {
        if let Some(room) = coord.room_mut(agent.current_room) {
            room.add_agent(agent.id);
        }
    }

    // ── NPCs (backed by ecosystem species) ─────────────────────────────────
    let npcs = vec![
        Npc { name: "Innkeeper".into(), room_id: 0, species_idx: 0 },
        Npc { name: "Merchant".into(), room_id: 1, species_idx: 1 },
        Npc { name: "Blacksmith".into(), room_id: 2, species_idx: 2 },
        Npc { name: "Rat".into(), room_id: 3, species_idx: 3 },
        Npc { name: "Thief".into(), room_id: 4, species_idx: 4 },
        Npc { name: "Ghost".into(), room_id: 5, species_idx: 5 },
    ];

    // Store NPC presence in room environments
    for npc in &npcs {
        if let Some(room) = coord.room_mut(npc.room_id) {
            let npcs_key = "npcs";
            let existing = room.get_env(npcs_key).unwrap_or("").to_string();
            let updated = if existing.is_empty() {
                npc.name.clone()
            } else {
                format!("{},{}", existing, npc.name)
            };
            room.set_env(npcs_key, &updated);
        }
    }

    // ── Ecosystem ──────────────────────────────────────────────────────────
    let species = vec![
        Species::new("Innkeeper", 5, 5, 0, Ternary::Zero),    // neutral, stable
        Species::new("Merchant", 10, 8, 1, Ternary::Pos),     // grows, aggressive trade
        Species::new("Blacksmith", 3, 3, 1, Ternary::Pos),    // moderate
        Species::new("Rat", 50, 15, 0, Ternary::Neg),         // conservative, fast breed
        Species::new("Thief", 8, 6, 2, Ternary::Neg),         // predator (steals), conservative
        Species::new("Ghost", 2, 1, 2, Ternary::Zero),        // rare, neutral
    ];

    let mut food_web = FoodWeb::new();
    food_web.add_link(4, 1, 30); // Thief preys on Merchant's resources
    food_web.add_link(3, 0, 10); // Rat competes with Innkeeper

    let niches = vec![
        Niche::new("tavern", vec![Ternary::Zero, Ternary::Neg], 20),
        Niche::new("market", vec![Ternary::Pos, Ternary::Zero], 50),
        Niche::new("forge", vec![Ternary::Pos, Ternary::Pos], 10),
        Niche::new("cellar", vec![Ternary::Neg, Ternary::Neg], 100),
        Niche::new("alley", vec![Ternary::Neg, Ternary::Zero], 30),
        Niche::new("tunnel", vec![Ternary::Zero, Ternary::Neg], 10),
    ];

    let cc = CarryingCapacity::new(500, vec![20, 50, 10, 100, 30, 10]);
    let ecosystem = Ecosystem::new(species, food_web, niches, cc);

    (coord, agents, npcs, ecosystem)
}

// ── Adjacency map for navigation ───────────────────────────────────────────

fn build_adjacency(coord: &RoomCoordinator) -> HashMap<u64, Vec<u64>> {
    let mut adj: HashMap<u64, Vec<u64>> = HashMap::new();
    // All room IDs
    for id in 0..6u64 {
        adj.entry(id).or_default();
    }
    // We know the door structure. In a real system we'd iterate doors.
    // For this experiment, hard-code based on our build_world().
    for id in 0..6u64 {
        if let Some(room) = coord.room(id) {
            // Check all other rooms for connectivity via doors
            for other in 0..6u64 {
                if other == id { continue; }
                // Try to find if there's a path (we know they're all Open)
                // Use a simple heuristic: check if coordinator has rooms
                let _ = room; // we just need the room to exist
                adj.entry(id).or_default().push(other);
            }
        }
    }
    // Actually, let's be smarter: only connect rooms that have doors between them.
    // We built these connections: 0-1, 1-2, 0-3, 1-4, 2-5, 3-4, 4-5
    adj.clear();
    let connections: Vec<(u64, u64)> = vec![
        (0, 1), (1, 0),
        (1, 2), (2, 1),
        (0, 3), (3, 0),
        (1, 4), (4, 1),
        (2, 5), (5, 2),
        (3, 4), (4, 3),
        (4, 5), (5, 4),
    ];
    for (a, b) in connections {
        adj.entry(a).or_default().push(b);
    }
    adj
}

// ── Simulation ─────────────────────────────────────────────────────────────

fn main() {
    println!("=== MUD World ↔ Ternary Fleet Bridge Experiment ===\n");

    let (mut coord, mut agents, npcs, mut ecosystem) = build_world();
    let adj = build_adjacency(&coord);

    // ── Current tracking for information flow ──────────────────────────────
    let mut current_map = CurrentMap::new();
    let mut upstream_sources: Vec<UpstreamSource> = Vec::new();
    let mut downstream_consumers: Vec<DownstreamConsumer> = Vec::new();

    // Each agent is an upstream source of action currents
    for agent in &agents {
        let src = UpstreamSource::new(RoomId::new(agent.current_room))
            .with_strength(CurrentStrength::new(50));
        upstream_sources.push(src);
        downstream_consumers.push(DownstreamConsumer::new(RoomId::new(agent.current_room), 100));
    }

    // ── CSV output ─────────────────────────────────────────────────────────
    println!("tick,agent_id,room_id,action,items_held,agents_in_room");

    let total_ticks = 500;
    let mut rng_state: u64 = 42; // simple LCG RNG

    for tick in 0..total_ticks {
        // Tick the ecosystem (NPC world dynamics)
        ecosystem.tick();

        // ── Per-agent perceive → decide → act ──────────────────────────────
        for agent_idx in 0..agents.len() {
            let agent = &mut agents[agent_idx];

            // ── PERCEIVE ────────────────────────────────────────────────────
            // Build perception from room state
            let room_snapshot = coord.room(agent.current_room).map(|r| r.snapshot());
            let items_in_room = room_snapshot.as_ref().map(|s| {
                s.environment.iter()
                    .filter(|(k, v)| k.starts_with("item_") && *v != "0")
                    .count()
            }).unwrap_or(0);

            let agents_in_room = coord.room(agent.current_room)
                .map(|r| r.agents().len())
                .unwrap_or(0);

            // Use TernaryCell tick lifecycle for decision:
            // The cell receives signals from its environment and updates its ternary value.
            // We feed it perception-derived signals:
            //   - items present → Signal (positive, attractive)
            //   - many agents → Suppress (negative, crowded)
            //   - NPC present → based on species strategy
            if items_in_room > 0 {
                agent.cell.receive(TernaryMessenger::Signal);
            }
            if agents_in_room > 2 {
                agent.cell.receive(TernaryMessenger::Suppress);
            } else if agents_in_room == 1 {
                agent.cell.receive(TernaryMessenger::Signal); // alone, explore more
            }

            // NPC influence based on ecosystem species strategy
            for npc in &npcs {
                if npc.room_id == agent.current_room {
                    let species = &ecosystem.species[npc.species_idx];
                    let msg = match species.strategy {
                        Ternary::Pos => TernaryMessenger::Signal,
                        Ternary::Neg => TernaryMessenger::Suppress,
                        Ternary::Zero => TernaryMessenger::Silence,
                    };
                    agent.cell.receive(msg);
                }
            }

            // Run cell tick: predict → perceive → surprise → vibe → gc → conservation
            let surprise = agent.cell.tick();

            // ── DECIDE ──────────────────────────────────────────────────────
            // Ternary value drives action:
            //   +1 (Signal)  → aggressive action: move or pick up
            //    0 (Silence) → moderate: interact or wait
            //   -1 (Suppress) → defensive: wait or move away
            let action = decide_action(
                agent,
                &adj,
                items_in_room,
                agents_in_room,
                surprise,
                &mut rng_state,
            );

            // ── ACT ─────────────────────────────────────────────────────────
            let prev_room = agent.current_room;

            match action {
                Action::Move => {
                    // Pick a random adjacent room
                    if let Some(neighbors) = adj.get(&agent.current_room) {
                        if !neighbors.is_empty() {
                            // Use ternary value to bias direction
                            let idx = if agent.cell.ternary_value > 0 {
                                // Aggressive: pick first neighbor (deterministic push)
                                0
                            } else if agent.cell.ternary_value < 0 {
                                // Defensive: pick last neighbor
                                neighbors.len() - 1
                            } else {
                                // Neutral: pseudo-random
                                (next_rand(&mut rng_state) as usize) % neighbors.len()
                            };
                            let dest = neighbors[idx];

                            // Use RoomCoordinator to transfer
                            if coord.transfer(agent.id, agent.current_room, dest).is_ok() {
                                agent.current_room = dest;
                                agent.rooms_visited.insert(dest);

                                // Update current map: agent moved, creating flow
                                current_map.set(
                                    RoomId::new(prev_room),
                                    Current::new(
                                        FlowDirection::With,
                                        CurrentStrength::new(30),
                                    ).with_label(format!("agent_{}", agent.id)),
                                );
                            }
                        }
                    }
                }
                Action::PickUp => {
                    // Try to take an item from the room
                    if let Some(room) = coord.room_mut(agent.current_room) {
                        // Find an item to pick up
                        let item_name = room.snapshot()
                            .environment
                            .iter()
                            .find(|(k, v)| k.starts_with("item_") && *v != "0")
                            .map(|(k, _)| k.strip_prefix("item_").unwrap_or(k).to_string());

                        if let Some(name) = item_name {
                            if take_item(room, &name) {
                                *agent.inventory.entry(name.clone()).or_insert(0) += 1;
                                agent.items_collected += 1;

                                // Signal that item was taken (information flow)
                                current_map.set(
                                    RoomId::new(agent.current_room),
                                    Current::new(
                                        FlowDirection::Against, // resource removal = against flow
                                        CurrentStrength::new(20),
                                    ).with_label(format!("pickup_{}", agent.id)),
                                );
                            }
                        }
                    }
                }
                Action::Interact => {
                    // Interact with an NPC in the room
                    // Use ecosystem to determine interaction outcome
                    for npc in &npcs {
                        if npc.room_id == agent.current_room {
                            let species = &ecosystem.species[npc.species_idx];
                            if !species.extinct {
                                // Interaction creates information current
                                current_map.set(
                                    RoomId::new(agent.current_room),
                                    Current::new(
                                        FlowDirection::With,
                                        CurrentStrength::new(40),
                                    ).with_label(format!("interact_{}_{}", agent.id, npc.name)),
                                );
                                break; // interact with one NPC per tick
                            }
                        }
                    }
                }
                Action::Wait => {
                    // No-op; energy recovery via cell conservation
                    // Waiting creates still current
                    current_map.set(
                        RoomId::new(agent.current_room),
                        Current::new(FlowDirection::Still, CurrentStrength::zero()),
                    );
                }
            }

            // ── Track downstream consumption ────────────────────────────────
            // Agents in the same room "consume" each other's information
            if let Some(room) = coord.room(agent.current_room) {
                for &other_id in room.agents() {
                    if other_id != agent.id {
                        let other_idx = (other_id - 1) as usize;
                        if other_idx < downstream_consumers.len() {
                            let consumer = &mut downstream_consumers[other_idx];
                            consumer.receive(Current::new(
                                FlowDirection::With,
                                CurrentStrength::new(10),
                            ).with_label(format!("proximity_{}", agent.id)));
                        }
                    }
                }
            }

            // Update upstream source position
            upstream_sources[agent_idx] = UpstreamSource::new(RoomId::new(agent.current_room))
                .with_strength(CurrentStrength::new(50 + (agent.items_held() as u8).min(100)));

            // ── CSV Output ──────────────────────────────────────────────────
            let agents_in_room_now = coord.room(agent.current_room)
                .map(|r| r.agents().len())
                .unwrap_or(0);

            println!("{},{},{},{},{},{}",
                tick,
                agent.id,
                agent.current_room,
                action,
                agent.items_held(),
                agents_in_room_now,
            );
        }

        // Periodically replenish items (world regeneration)
        if tick % 50 == 0 && tick > 0 {
            for room_id in 0..6u64 {
                if let Some(room) = coord.room_mut(room_id) {
                    if count_items(room) < 2 {
                        let items = [" ale", "bread", "apple", "coin", "herb"];
                        let idx = (next_rand(&mut rng_state) as usize) % items.len();
                        place_item(room, items[idx].trim());
                    }
                }
            }
        }
    }

    // ── Summary ────────────────────────────────────────────────────────────
    eprintln!("\n=== SIMULATION SUMMARY ===");
    for agent in &agents {
        eprintln!("Agent {} ({}):", agent.id, agent.name);
        eprintln!("  Rooms visited: {}", agent.rooms_visited.len());
        eprintln!("  Items collected: {}", agent.items_collected);
        eprintln!("  Items held: {}", agent.items_held());
        eprintln!("  Cell energy: {}, ternary: {}", agent.cell.energy, agent.cell.ternary_value);
        eprintln!("  Cell state: {:?}", agent.cell.state);
    }

    eprintln!("\n=== ECOSYSTEM STATE ===");
    for sp in &ecosystem.species {
        eprintln!("  {} ({}): pop={}, strategy={:?}, extinct={}",
            sp.name, sp.trophic_level, sp.population, sp.strategy, sp.extinct);
    }
    eprintln!("  Total population: {}", ecosystem.total_population());
    eprintln!("  Succession stage: {:?}", ecosystem.succession.current_stage);

    eprintln!("\n=== CURRENT MAP ===");
    eprintln!("  Active rooms: {}", current_map.active_rooms().len());
    if let Some(strongest) = current_map.strongest() {
        eprintln!("  Strongest current at room: {}", strongest.value());
    }

    eprintln!("\n=== ROOM HISTORIES ===");
    for id in 0..6u64 {
        if let Some(room) = coord.room(id) {
            eprintln!("  Room {} ({}): {} events", id, room.name, room.history().len());
        }
    }
}

// ── Decision logic ─────────────────────────────────────────────────────────

fn decide_action(
    agent: &MudAgent,
    adj: &HashMap<u64, Vec<u64>>,
    items_in_room: usize,
    agents_in_room: usize,
    surprise: i32,
    rng: &mut u64,
) -> Action {
    let tv = agent.cell.ternary_value;

    // Decision matrix based on ternary value and perception
    match tv {
        1 => {
            // Aggressive/Signal: prefer to act on the world
            if items_in_room > 0 && next_rand(rng) % 3 != 0 {
                Action::PickUp
            } else if !adj.get(&agent.current_room).map_or(true, |n| n.is_empty()) {
                Action::Move
            } else {
                Action::Interact
            }
        }
        0 => {
            // Neutral: balanced choices
            let roll = next_rand(rng) % 4;
            match roll {
                0 => Action::Move,
                1 => if items_in_room > 0 { Action::PickUp } else { Action::Wait },
                2 => Action::Interact,
                _ => Action::Wait,
            }
        }
        -1 => {
            // Defensive/Suppress: prefer safety
            if surprise > 1 {
                // High surprise → move away
                Action::Move
            } else if agents_in_room > 1 {
                Action::Interact // interact when others present
            } else {
                Action::Wait
            }
        }
        _ => Action::Wait,
    }
}

// ── Simple LCG PRNG ────────────────────────────────────────────────────────

fn next_rand(state: &mut u64) -> u64 {
    *state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    *state
}
