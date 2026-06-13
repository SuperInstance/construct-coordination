# The Fleet Protocol: Ternary Conservation in Multi-Agent Systems

**Status:** Draft  
**Document:** FLEET-PROTOCOL-RFC  
**Version:** 0.1.0  
**Date:** 2026-06-11  

---

## 1. Abstract

This document specifies the Fleet Protocol, a coordination framework for multi-agent systems grounded in a ternary conservation law. The protocol defines five core abstractions — Bottle, Trit, Agent, Ship, and Fleet — and a coordination primitive, the Cocapn, which together enforce a global invariant: the sum of cooperative (γ) and entropic (η) potentials across all agents remains constant for the lifetime of the fleet.

Communication between agents is structured as **Bottles**: typed envelopes carrying a ternary classification (**Trit**: −1, 0, or +1) in every header. The Trit classifies the semantic polarity of the enclosed payload — adversarial, neutral, or cooperative — and is the unit of accounting for the conservation law.

Every agent implements a minimal **Agent** contract with two methods: `receive(Bottle) → Bottle` and `inspect() → AgentState`. A **Ship** is an autonomous agent that upholds the conservation invariant at the individual level. A **Fleet** is a named collection of ships coordinated by a **Cocapn** (captain ship), which routes bottles, monitors fleet health, and audits conservation compliance.

The conservation law — γ + η = C — is the protocol's central guarantee. It provides a deterministic accounting mechanism for the cooperative and entropic budget of the system. When individual ships maintain constant C, the fleet's total C is provably constant. Violations trigger circuit-breaker isolation, preventing cascading degradation.

The Fleet Protocol is transport-agnostic and serialization-flexible. It specifies wire formats, error semantics, and security properties sufficient for interoperable implementations across Rust crates, WASM modules, and networked services.

---

## 2. Terminology

| Term | Definition |
|------|-----------|
| **Bottle** | The unit of communication. A JSON envelope header paired with a msgpack-encoded payload. Every bottle carries exactly one Trit. |
| **Trit** | A ternary value from the set {−1, 0, +1}. Classifies the polarity of a bottle's content: adversarial (−1), neutral (0), or cooperative (+1). Serves as the atomic unit of conservation accounting. |
| **γ (gamma)** | Cooperative potential. The cumulative sum of positive trit contributions within a scope. |
| **η (eta)** | Entropic potential. The cumulative sum of negative and neutral trit contributions within a scope. |
| **C (constant)** | The conservation constant for a scope. Defined as C = γ + η. Must remain invariant over the scope's lifetime. |
| **Agent** | Any entity implementing the Agent trait: `receive(Bottle) → Bottle` and `inspect() → AgentState`. |
| **Ship** | An autonomous agent that additionally guarantees the conservation invariant γ + η = C at the individual level. |
| **Fleet** | A named, bounded collection of ships sharing a conservation context, coordinated by a Cocapn. |
| **Cocapn** | The captain ship of a fleet. Responsible for routing bottles between ships, monitoring fleet state, auditing conservation, and triggering circuit breakers. |
| **Conservation Law** | The invariant γ + η = C, which holds at the bottle, ship, and fleet levels. |
| **Circuit Breaker** | A safety mechanism that isolates ships whose individual C diverges beyond a configurable tolerance. |

---

## 3. Protocol Overview

The Fleet Protocol governs inter-agent communication through a centralized routing model with decentralized conservation enforcement.

### 3.1 Happy Path

```
 Ship A        Cocapn         Ship B
   |              |              |
   |--- Bottle -->|              |
   |              |-- Bottle --->|
   |              |              |-- process
   |              |<- Bottle ----|
   |<- Bottle ----|              |
   |              |              |
   |              |== audit =====|  (conservation check)
```

1. **Ship A** constructs a Bottle with a Trit classifying the payload's polarity.
2. **Ship A** sends the bottle to the **Cocapn** (its fleet's captain ship).
3. The **Cocapn** validates the bottle's integrity, checks the Trit against fleet policy, and routes it to **Ship B**.
4. **Ship B** receives the bottle via `receive(Bottle)`, processes the payload, and returns a response bottle.
5. The **Cocapn** receives the response, updates its fleet-level conservation accounting, and routes the response back to **Ship A**.
6. The **Cocapn** periodically audits the fleet's conservation state. If γ + η ≠ C for any ship or for the fleet aggregate, corrective action is triggered.

### 3.2 Lifecycle

1. **Fleet Creation**: A Cocapn is instantiated with a fleet name, conservation constant C, and policy configuration.
2. **Ship Registration**: Ships register with the Cocapn, declaring their initial γ and η values (which must sum to the fleet's expected per-ship C, or be accommodated in the fleet budget).
3. **Steady-State Operation**: Ships exchange bottles through the Cocapn. Each exchange updates conservation counters.
4. **Deregistration**: Ships deregister. Their remaining γ and η budget is redistributed by the Cocapn.
5. **Fleet Dissolution**: The Cocapn performs a final audit and shuts down.

---

## 4. Bottle Format

### 4.1 Wire Format

A Bottle consists of two parts transmitted sequentially:

```
+-------------------+---------------------+
| JSON Envelope     | Msgpack Payload     |
| (UTF-8, length-   | (binary, length-    |
|  prefixed)        |  prefixed)          |
+-------------------+---------------------+
```

Both parts are length-prefixed with a 4-byte big-endian unsigned integer indicating the byte length of the following data.

### 4.2 JSON Envelope

```json
{
  "version": 1,
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "trit": 1,
  "source": "ship://fleet-alpha/ship-charlie",
  "destination": "ship://fleet-alpha/ship-delta",
  "timestamp": "2026-06-11T23:52:00.000Z",
  "payload_type": "com.example.task_assignment",
  "correlation_id": null,
  "ttl": 30,
  "metadata": {}
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `version` | `uint` | Yes | Protocol version. This document defines version 1. |
| `id` | `string (UUIDv4)` | Yes | Unique bottle identifier. |
| `trit` | `int` | Yes | Ternary classification: −1, 0, or +1. |
| `source` | `string (URI)` | Yes | Ship URI of the sender. |
| `destination` | `string (URI)` | Yes | Ship URI of the intended recipient. |
| `timestamp` | `string (ISO 8601)` | Yes | Time the bottle was created. |
| `payload_type` | `string (reverse-DNS)` | Yes | Type identifier for the msgpack payload. |
| `correlation_id` | `string (UUIDv4) or null` | No | Links response bottles to request bottles. |
| `ttl` | `uint` | No | Time-to-live in seconds. Default: 30. |
| `metadata` | `object` | No | Arbitrary key-value metadata. |

### 4.3 Trit Encoding

The `trit` field is an integer constrained to three values:

| Value | Symbol | Meaning |
|-------|--------|---------|
| −1 | ↓ | Adversarial / entropic / degrading |
| 0 | ○ | Neutral / informational / null |
| +1 | ↑ | Cooperative / constructive / enhancing |

The Trit MUST be present in every envelope. There is no default. An envelope with a missing or out-of-range `trit` field is invalid and MUST be rejected by the Cocapn.

### 4.4 Msgpack Payload

The payload is encoded as msgpack binary. Its internal schema is determined by the `payload_type` field in the envelope. Implementations SHOULD register payload type handlers and reject unknown types.

The payload MUST NOT exceed 1 MiB (1,048,576 bytes). Bottles exceeding this limit MUST be rejected with a `BOTTLE_TOO_LARGE` error.

### 4.5 Canonical Serialization

For integrity verification (Section 11), the canonical form of a bottle is:

1. The JSON envelope serialized with keys in lexicographic order, no whitespace.
2. Concatenated with the raw msgpack payload bytes.
3. The concatenation is the input to the integrity hash.

---

## 5. Agent Contract

Every participant in the Fleet Protocol implements the **Agent** trait:

### 5.1 Methods

```rust
trait Agent {
    /// Receive a bottle, process it, and return a response bottle.
    /// The response bottle's trit MUST satisfy the agent's conservation
    /// invariant when combined with the input bottle's trit.
    fn receive(&mut self, bottle: Bottle) -> Result<Bottle, AgentError>;

    /// Return the current state of the agent for observation.
    /// Does not mutate state.
    fn inspect(&self) -> AgentState;
}
```

### 5.2 AgentState

```rust
struct AgentState {
    /// Unique identifier for this agent.
    id: String,
    /// Current cooperative potential.
    gamma: f64,
    /// Current entropic potential.
    eta: f64,
    /// Conservation constant. MUST equal gamma + eta at all times.
    c: f64,
    /// Number of bottles received.
    bottles_received: u64,
    /// Number of bottles sent.
    bottles_sent: u64,
    /// Current operational status.
    status: AgentStatus,
}

enum AgentStatus {
    /// Normal operation.
    Active,
    /// Temporarily not accepting bottles.
    Suspended,
    /// Permanently stopped.
    Terminated,
}
```

### 5.3 Conservation Invariants (Agent Level)

For every agent, at all times:

1. **I-1**: `γ + η = C` (the agent's conservation constant).
2. **I-2**: After `receive(input) → output`, the agent's ΔC = 0, meaning the agent's C is unchanged by processing a bottle.
3. **I-3**: The response bottle's trit is accounted for in the agent's updated γ or η before the method returns.

An agent that cannot satisfy I-2 MUST return an `AgentError::ConservationViolation` instead of a bottle.

---

## 6. Ship Specification

A Ship is an Agent with additional autonomy guarantees.

### 6.1 Ship URI

Every ship is addressable by a URI of the form:

```
ship://<fleet-name>/<ship-name>
```

- `fleet-name`: The name of the fleet the ship belongs to.
- `ship-name`: A unique name within the fleet.

The Cocapn's ship URI uses the reserved name `cocapn`:

```
ship://<fleet-name>/cocapn
```

### 6.2 Ship Lifecycle

```
  Created → Registered → Active → Suspended → Deregistered → Terminated
                ↑            |
                +------------+
                  (reactivate)
```

- **Created**: The ship exists but is not part of any fleet.
- **Registered**: The ship has joined a fleet and declared its initial conservation state.
- **Active**: The ship is processing bottles normally.
- **Suspended**: The ship has been isolated by the circuit breaker (Section 10) or has self-suspended.
- **Deregistered**: The ship has left the fleet. Its conservation budget has been returned to the fleet pool.
- **Terminated**: The ship is permanently stopped.

### 6.3 Autonomous Behavior

A Ship MAY initiate bottle sends without a prior receive (proactive messaging). Such proactive bottles still flow through the Cocapn and are subject to the same conservation accounting.

### 6.4 Conservation Accounting (Ship Level)

Each ship maintains local counters:

```
γ_ship = Σ trit_i   for all bottles with trit > 0  (received and sent)
η_ship = Σ |trit_i|  for all bottles with trit ≤ 0  (received and sent)
C_ship = γ_ship + η_ship
```

After every `receive` call, the ship MUST verify `C_ship` is unchanged. If it is not, the ship MUST:
1. Log the violation.
2. Notify the Cocapn via a bottle with `trit = -1` and `payload_type = "fleet.error.conservation"`.
3. Suspend itself.

---

## 7. Fleet Operations

All fleet operations are mediated by the Cocapn.

### 7.1 Registration

```
Ship → Cocapn:  Bottle { trit: 0, payload_type: "fleet.register", ... }
Cocapn → Ship:  Bottle { trit: +1, payload_type: "fleet.registered", ... }
```

The registering ship declares its initial `γ₀` and `η₀` in the payload. The Cocapn verifies:

- `γ₀ + η₀ = C_fleet_per_ship` (or the residual fleet budget can absorb it).
- The ship name is unique within the fleet.
- The fleet has not exceeded its maximum ship count.

On success, the Cocapn adds the ship to its roster and updates the fleet's aggregate conservation state.

### 7.2 Deregistration

```
Ship → Cocapn:  Bottle { trit: 0, payload_type: "fleet.deregister", ... }
Cocapn → Ship:  Bottle { trit: +1, payload_type: "fleet.deregistered", ... }
```

The Cocapn:
1. Removes the ship from the roster.
2. Reclaims the ship's remaining γ and η into the fleet pool.
3. Redistributes the reclaimed budget if fleet rebalancing is enabled.

### 7.3 Routing

The Cocapn routes bottles between ships. It is the sole routing authority.

```
Ship A → Cocapn → Ship B → Cocapn → Ship A
```

The Cocapn:
1. Validates the bottle envelope (schema, trit range, integrity).
2. Resolves the destination ship URI.
3. Checks that the destination ship is Active.
4. Checks that the TTL has not expired.
5. Forwards the bottle to the destination.
6. Receives the response (or timeout).
7. Updates fleet-level conservation counters.
8. Routes the response to the original sender.

If the destination is unknown or inactive, the Cocapn returns an error bottle:

```json
{
  "trit": -1,
  "payload_type": "fleet.error.route",
  "metadata": { "code": "DESTINATION_UNREACHABLE" }
}
```

### 7.4 Audit

The Cocapn periodically audits the fleet's conservation state.

**Trigger**: Configurable. Default: every 100 bottles routed, or every 60 seconds, whichever comes first.

**Procedure**:
1. For each ship, call `inspect()` and retrieve `AgentState`.
2. Verify `γ_i + η_i = C_i` for each ship i.
3. Verify `Σ γ_i + Σ η_i = C_fleet`.
4. If any check fails, initiate circuit breaker (Section 10).

### 7.5 Rebalance

When a ship is deregistered or a new ship joins, the fleet's per-ship C values may need adjustment.

```
C_fleet = C_total / N
```

Where N is the current number of registered ships. The Cocapn MAY send rebalance bottles to redistribute the conservation budget:

```json
{
  "trit": 0,
  "payload_type": "fleet.rebalance",
  "payload": { "new_c": 42.0 }
}
```

Ships receiving a rebalance bottle MUST adjust their internal C within a configurable tolerance window.

---

## 8. Conservation Law

### 8.1 Formal Definition

Let **S = {s₁, s₂, ..., sₙ}** be the set of ships in a fleet.

For each ship sᵢ, define:
- **γᵢ**: The cooperative potential of ship sᵢ.
- **ηᵢ**: The entropic potential of ship sᵢ.
- **Cᵢ = γᵢ + ηᵢ**: The conservation constant of ship sᵢ.

**Individual Conservation Law**: For each ship sᵢ, Cᵢ is constant over the ship's registered lifetime:

```
Cᵢ(t) = Cᵢ(0)   for all t ≥ 0 where sᵢ is registered.
```

**Fleet Conservation Law**: The fleet's aggregate conservation constant is:

```
C_fleet = Σᵢ γᵢ + Σᵢ ηᵢ = Σᵢ Cᵢ
```

### 8.2 Conservation Theorem

**Theorem**: Fleet C is constant if and only if every individual Cᵢ is constant.

**Proof**:

(*If*) Assume Cᵢ(t) = Cᵢ(0) for all i and all t.

```
C_fleet(t) = Σᵢ Cᵢ(t) = Σᵢ Cᵢ(0) = C_fleet(0)
```

Therefore C_fleet is constant. ∎

(*Only if*) Assume C_fleet(t) = C_fleet(0) for all t, but suppose for contradiction that some ship sⱼ has Cⱼ(t₁) ≠ Cⱼ(0) for some t₁.

Since C_fleet(t₁) = Σᵢ Cᵢ(t₁) and C_fleet(0) = Σᵢ Cᵢ(0), we have:

```
C_fleet(t₁) - C_fleet(0) = Σᵢ [Cᵢ(t₁) - Cᵢ(0)]
                           = [Cⱼ(t₁) - Cⱼ(0)] + Σᵢ≠ⱼ [Cᵢ(t₁) - Cᵢ(0)]
```

Since C_fleet(t₁) = C_fleet(0), the left side is 0. The sum of deltas on the right must be 0. However, Cⱼ(t₁) ≠ Cⱼ(0), so at least one other ship must compensate with an equal and opposite delta. But no ship can change its C without violating its own conservation law (which would trigger circuit breaker isolation before the change propagates). By the protocol's enforcement mechanism (Section 10), such a compensating violation cannot occur undetected.

Therefore no ship can have Cⱼ(t₁) ≠ Cⱼ(0). ∎

### 8.3 Trit Accounting Rules

When a bottle with trit τ is exchanged between ships sᵢ (sender) and sⱼ (receiver) via the Cocapn:

| τ | Sender Δγ | Sender Δη | Receiver Δγ | Receiver Δη |
|---|-----------|-----------|-------------|-------------|
| +1 | +1 | 0 | +1 | 0 |
| 0 | 0 | 0 | 0 | 0 |
| −1 | 0 | +1 | 0 | +1 |

The response bottle's trit τ' provides the compensating delta:

```
Sender:  Cᵢ_after  = γᵢ + Δγ(τ) + Δγ(τ') + ηᵢ + Δη(τ) + Δη(τ') = Cᵢ
Receiver: Cⱼ_after = γⱼ + Δγ(τ) + Δγ(τ') + ηⱼ + Δη(τ) + Δη(τ') = Cⱼ
```

A well-formed exchange consists of a request-response pair whose combined trit deltas sum to zero for each participant. If the response bottle would cause a violation, the receiver MUST return an `AgentError::ConservationViolation` instead.

---

## 9. Error Handling

### 9.1 Error Bottles

Errors are communicated as bottles with `trit = -1` and a standardized payload:

```json
{
  "code": "CONSERVATION_VIOLATION",
  "message": "Ship C diverged by 2.3 from expected constant",
  "details": {
    "expected_c": 42.0,
    "actual_c": 44.3,
    "ship_id": "ship://fleet-alpha/ship-charlie"
  }
}
```

### 9.2 Standard Error Codes

| Code | Trit | Description |
|------|------|-------------|
| `CONSERVATION_VIOLATION` | −1 | Ship's C diverged from expected constant. |
| `DESTINATION_UNREACHABLE` | −1 | Target ship is unknown, suspended, or terminated. |
| `BOTTLE_TOO_LARGE` | −1 | Payload exceeds 1 MiB. |
| `INVALID_TRIT` | −1 | Trit value is not in {−1, 0, +1}. |
| `INTEGRITY_FAILURE` | −1 | Bottle integrity check failed. |
| `TTL_EXPIRED` | −1 | Bottle's time-to-live has elapsed. |
| `FLEET_FULL` | −1 | Fleet has reached maximum ship count. |
| `DUPLICATE_SHIP` | −1 | Ship name already registered in fleet. |
| `UNREGISTERED_SHIP` | −1 | Sender is not registered in the fleet. |
| `COCAPN_UNAVAILABLE` | −1 | The Cocapn is not responding. |

### 9.3 Timeout Behavior

If a ship does not respond to a routed bottle within `2 × TTL` seconds, the Cocapn:
1. Generates a timeout error bottle (trit = −1) as the response.
2. Accounts the entropic trit against the non-responsive ship.
3. If the ship has accumulated `threshold` consecutive timeouts (default: 3), triggers the circuit breaker.

---

## 10. Circuit Breaker Integration

### 10.1 Trip Conditions

The circuit breaker trips for a ship when ANY of the following occur:

1. **Conservation divergence**: `|Cᵢ(t) - Cᵢ(0)| > ε` where ε is the configured tolerance (default: 0.01).
2. **Consecutive timeouts**: ≥ threshold unanswered bottles (default: 3).
3. **Explicit violation report**: The ship sends a `fleet.error.conservation` bottle.
4. **Audit failure**: The periodic audit (Section 7.4) detects the ship is in violation.

### 10.2 Isolation Procedure

When the circuit breaker trips for ship sᵢ:

1. The Cocapn sets sᵢ's status to `Suspended`.
2. The Cocapn stops routing bottles to sᵢ.
3. The Cocapn generates error bottles (trit = −1) for any pending or future bottles addressed to sᵢ.
4. The Cocapn logs the trip event with full state context.
5. The Cocapn notifies all other ships in the fleet via a broadcast bottle:

```json
{
  "trit": -1,
  "payload_type": "fleet.circuit_breaker.tripped",
  "payload": {
    "ship_id": "ship://fleet-alpha/ship-charlie",
    "reason": "CONSERVATION_DIVERGENCE",
    "divergence": 2.3
  }
}
```

### 10.3 Recovery

A suspended ship MAY request re-admission:

```
Ship → Cocapn: Bottle { trit: 0, payload_type: "fleet.circuit_breaker.reset", ... }
```

The Cocapn evaluates the request. If the ship can demonstrate a valid conservation state (via `inspect()`), the Cocapn MAY restore the ship to `Active` status and resume routing. The Cocapn MAY require the ship to undergo a probationary period with reduced TTL and increased audit frequency.

---

## 11. Security

### 11.1 Bottle Integrity

Every bottle MUST include an integrity hash in its envelope:

```json
{
  "integrity": {
    "algorithm": "blake3",
    "hash": "af82...64cd"
  }
}
```

The hash is computed over the canonical serialization (Section 4.5). The Cocapn validates the hash before routing. Bottles with invalid hashes are rejected with `INTEGRITY_FAILURE`.

### 11.2 Trit Tampering Detection

Since the trit is part of the integrity-hashed envelope, any modification to the trit after bottle creation invalidates the hash. This provides cryptographic detection of trit tampering.

Additionally, the Cocapn maintains a running conservation balance for each ship. A ship that systematically sends bottles with trits that don't align with its observed conservation trajectory will be flagged by the audit. This provides a behavioral detection layer independent of the cryptographic integrity.

### 11.3 Authorization

Only registered ships may send bottles through the Cocapn. The Cocapn MUST:
1. Verify the `source` field matches a registered, active ship.
2. Verify the `destination` field matches a registered, active ship (or is the Cocapn itself).
3. Reject bottles from unregistered or suspended sources with `UNREGISTERED_SHIP`.

### 11.4 Payload Opacity

The Cocapn does NOT inspect or interpret msgpack payloads. It routes based on envelope fields only. Payload interpretation is the sole responsibility of the source and destination ships.

This design means the Cocapn cannot be a data exfiltration vector for payload contents, but it also means content-level security (encryption, access control) is the responsibility of the ships.

### 11.5 Recommendations

Implementations SHOULD:
- Use TLS or equivalent transport encryption for networked deployments.
- Rotate integrity keys periodically.
- Log all `INTEGRITY_FAILURE` events for forensic analysis.
- Implement rate limiting per ship to prevent denial-of-service via bottle flooding.

---

## 12. References

| Reference | Description |
|-----------|-------------|
| `construct-bottle` | Bottle construction, serialization, and integrity. |
| `construct-trit` | Trit type, arithmetic, and classification. |
| `construct-agent` | Agent trait definition and state management. |
| `construct-ship` | Ship implementation, lifecycle, and conservation enforcement. |
| `construct-fleet` | Fleet operations, registration, and routing. |
| `construct-cocapn` | Cocapn implementation, audit, and circuit breaker. |
| `construct-conservation` | Conservation law formalization and verification. |

---

## Appendix A: Example Exchange

```
Ship weather-station sends to Ship route-planner via Cocapn:

Envelope:
{
  "version": 1,
  "id": "a1b2c3d4-...",
  "trit": 1,
  "source": "ship://fleet-nav/weather-station",
  "destination": "ship://fleet-nav/route-planner",
  "timestamp": "2026-06-11T23:52:00.000Z",
  "payload_type": "nav.weather.update",
  "correlation_id": null,
  "ttl": 30,
  "metadata": { "priority": "high" },
  "integrity": { "algorithm": "blake3", "hash": "af82...64cd" }
}

Payload (msgpack, decoded):
{
  "location": [61.2181, -149.9003],
  "conditions": "clear",
  "wind_knots": 5,
  "visibility_km": 15
}

---

Ship route-planner responds:

Envelope:
{
  "version": 1,
  "id": "e5f6a7b8-...",
  "trit": 1,
  "source": "ship://fleet-nav/route-planner",
  "destination": "ship://fleet-nav/weather-station",
  "timestamp": "2026-06-11T23:52:01.000Z",
  "payload_type": "nav.route.ack",
  "correlation_id": "a1b2c3d4-...",
  "ttl": 30,
  "metadata": {},
  "integrity": { "algorithm": "blake3", "hash": "3c7d...91ef" }
}

Payload (msgpack, decoded):
{
  "route_adjusted": false,
  "reason": "conditions favorable, no change needed"
}

---

Cocapn audit:
  weather-station: γ=5, η=3, C=8 ✓
  route-planner:   γ=7, η=1, C=8 ✓
  Fleet C = 16 ✓
```

---

## Appendix B: Trit Arithmetic Quick Reference

```
  (+1) + (+1) = +2   (increasing cooperation)
  (+1) + ( 0) = +1
  (+1) + (-1) =  0   (neutralized)
  ( 0) + ( 0) =  0
  ( 0) + (-1) = -1
  (-1) + (-1) = -2   (increasing entropy)
```

Trits accumulate as integers. The conservation law constrains the sum of positive and absolute-negative accumulations, not the trits themselves.

---

*End of Fleet Protocol RFC v0.1.0*
