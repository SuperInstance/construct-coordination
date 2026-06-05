# FINDINGS: MUD World ↔ Ternary Fleet Bridge

## Experiment Summary

Built a working MUD world simulation (5 agents, 6 rooms, 6 NPCs, items) using **only** the ternary crate fleet:
- `ternary-room` — room graph, doors, agent tracking, event history
- `ternary-cell` — agent decision state via TernaryCell tick lifecycle
- `ternary-current` — information flow tracking (movement currents, interaction currents)
- `ternary-ecosystem` — NPC species with food web, carrying capacity, succession

Ran 500 ticks. All agents remained active, collected items, navigated rooms, and interacted.

## 1. Can ternary crates fully express MUD world mechanics?

**Mostly yes, with important gaps.**

### What maps cleanly:

| mud-arena concept | ternary crate mapping | Notes |
|---|---|---|
| Room (id, name, description) | `ternary_room::Room` + `environment` map | Room env map stores items, NPC lists as string values. Clunky but works. |
| Room exits | `ternary_room::Door` with `DoorAccess` | **Stronger** than mud-arena: supports locked, one-way, open. mud-arena only has dict-based exits. |
| Agent position | Agent tracked via `Room::add_agent/remove_agent` | `RoomCoordinator::transfer()` atomically moves agents. Clean. |
| Agent perception → decide → act | `TernaryCell::tick()` (predict→perceive→surprise→vibe→gc→conservation) | Interesting structural match. See §3 below. |
| Event tracking | `ternary_room::RoomHistory` | Records enter/leave events automatically. `EventBus` equivalent but per-room. |
| NPC populations | `ternary_ecosystem::Species` | Species have population, growth, trophic levels, strategies. Richer than static NPC lists. |
| NPC interactions | `ternary_ecosystem::FoodWeb` | Predator-prey links. Thief preys on Merchant, Rats compete with Innkeeper. |
| Item tracking | Room `environment` map with `"item_{name}" = "count"` | **Workaround**, not a clean mapping. No dedicated item system in ternary crates. |
| Information flow | `ternary_current::CurrentMap`, `UpstreamSource`, `DownstreamConsumer` | New capability: tracks who-sent-what-where. mud-arena has no equivalent. |

### What doesn't map cleanly:

1. **Items/Inventory**: No ternary crate has an inventory system. We stored items as string-encoded environment variables in rooms and `HashMap<String, u32>` in agents. This is the biggest gap. mud-arena has a proper `Inventory` class with capacity, tags, use tracking.

2. **Command parsing**: mud-arena has a full verb parser (`go north`, `take key`, `use key with door`). The ternary fleet has nothing for text commands. We mapped actions to ternary values (Signal→aggressive, Silence→balanced, Suppress→defensive) which is elegant but loses the command richness.

3. **Agent inventory as personal state**: In mud-arena, agents have an `Inventory` object. In ternary-cell, cells have `energy`, `ternary_value`, `prediction`, `surprise` — cellular biology concepts that don't naturally represent "items carried".

4. **Room descriptions/flavor**: mud-arena rooms have `description` text. ternary-room rooms have a `name` and generic `environment` map. We used environment for this too but it's not purpose-built.

5. **Event bus pub/sub**: mud-arena has a global `EventBus` with typed events (ROOM_ENTER, ITEM_PICKED_UP, etc.) and subscriber callbacks. ternary-room has `RoomHistory` (append-only log) but no pub/sub dispatch. `ternary-current` provides directional flow tracking but not event-type subscriptions.

## 2. What's missing?

For a full MUD integration, the ternary fleet would need:

1. **An inventory/collection crate** (or trait) — items with names, descriptions, tags, uses, capacity limits
2. **A command/action module** — verb parsing or at least structured action types beyond ternary values
3. **Event dispatch** — pub/sub on top of RoomHistory, or a separate event crate
4. **Agent identity** — cells have numeric IDs and energy but no concept of name, persona, or narrative state
5. **World state persistence** — snapshot/restore exists in Room but not at world level

## 3. How does the ternary tick compare to mud-arena's perceive→decide→act?

This is the most interesting finding.

**mud-arena loop** (per `Agent::step`):
```
perceive(graph) → perception dict
decide(perception) → Command
act(command, graph, bus) → result string
```
Linear. State flows forward. No feedback.

**ternary-cell loop** (per `TernaryCell::tick`):
```
predict() → set prediction from inbox signals
perceive() → update ternary value from combined signals
compute_surprise() → |actual - predicted| (prediction error)
vibe() → adjust energy based on surprise
gc() → clear inbox
conservation() → clamp energy, check apoptosis
```
Cyclic with feedback. The predict→surprise→energy loop creates **self-regulating behavior**: cells that consistently mispredict their environment lose energy and eventually die (apoptosis). Cells in sync gain energy.

**Bridge insight**: We can use the cell tick as the decision engine. Feed room perception as TernaryMessenger signals into the cell's inbox. The resulting ternary value (+1/0/-1) becomes the action selection input. The surprise metric tells us how predictable the agent's environment is. Low surprise = stable room, high surprise = chaotic or novel room.

This actually **improves** on mud-arena's decision model by adding:
- **Energy budget**: agents can't act forever; low energy limits behavior
- **Adaptation**: cell ternary value adjusts to match local signals (NPC strategy, crowd density)
- **Death**: agents that can't adapt to their environment undergo apoptosis
- **Surprise tracking**: quantifies how predictable each agent's world is

In our 500-tick run, all agents converged to ternary value -1 (Suppress) because they ended up crowded in room 0 (5 agents in one room = constant Suppress signals). The cell lifecycle correctly detected this as the stable state.

## 4. Full integration architecture

```
┌─────────────────────────────────────────────────────┐
│                    MUD World Layer                   │
│  (wrapper types providing MUD semantics)            │
├──────────┬──────────┬──────────┬────────────────────┤
│ MudRoom  │ MudAgent │ MudNPC   │ MudEventBus        │
│ wraps:   │ wraps:   │ wraps:   │ wraps:             │
│ Room +   │ Ternary  │ Species  │ CurrentMap +       │
│ Door +   │ Cell +   │ + Niche  │ RoomHistory        │
│ History  │ Inventory│ + FoodWeb│                    │
├──────────┴──────────┴──────────┴────────────────────┤
│              Ternary Fleet (foundation)              │
│  ternary-room  ternary-cell  ternary-current         │
│  ternary-ecosystem                                   │
├─────────────────────────────────────────────────────┤
│           NEW: ternary-inventory (needed)            │
│  Item, Inventory, capacity, tags, uses               │
└─────────────────────────────────────────────────────┘
```

The wrapper layer translates between MUD concepts and ternary primitives:
- `MudRoom` holds a `Room`, stores items in environment, queries doors
- `MudAgent` holds a `TernaryCell` for decisions, a separate `HashMap` for inventory, tracks room position
- `MudNPC` wraps a `Species` from the ecosystem, anchored to a room
- `MudEventBus` combines `CurrentMap` (flow tracking) with `RoomHistory` (event log)

The `ternary-inventory` crate would fill the biggest gap: a proper item system with the richness of mud-arena's `Inventory` class.

## 5. Observations from the run

- **Agents converged to room 0**: By tick ~50, all 5 agents ended up in the Tavern and mostly stayed. This is because items were depleted elsewhere and the cell lifecycle gave Suppress signals in crowded rooms, making agents wait rather than explore.

- **Ecosystem collapsed to rats**: By tick 500, only Rat survived (pop=500, hit carrying capacity). All other species went extinct. The pioneer-stage growth modifier of 15% was too aggressive for small populations combined with predation. The ecological dynamics are working but need tuning for MUD use.

- **Current map shows information flow**: 5 active rooms had currents at end of simulation, strongest at room 0. This is a genuinely new capability — mud-arena can't tell you where the most "activity" is happening.

- **Room histories work**: Room 0 (Tavern) had 7 events, Room 1 (Market) had 8. These track agent enter/leave. Useful for MUD logging.

## Conclusion

The ternary fleet can serve as a **computational foundation** for MUD worlds, but needs a **semantic layer** on top. The cell tick lifecycle is genuinely better than mud-arena's flat perceive→decide→act for creating adaptive, self-regulating agents. The ecosystem adds dynamic NPC populations that respond to predation and carrying capacity. The current system adds information flow tracking that no MUD framework has.

The missing piece is an inventory system. Everything else can be bridged with wrapper types.
