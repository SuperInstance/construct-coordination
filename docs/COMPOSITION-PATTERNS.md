# SEED Composition Patterns

Higher-order patterns formed by composing SEED crates.
Each pattern preserves the ternary conservation law: **γ + η = C** (sum of trits in = sum of trits out).

---

## 1. Resilience Pattern

**Circuit-Breaker + Retry-Backoff + Health-Check → Self-Healing**

Operations are wrapped in a retry loop with backoff. The circuit breaker gates retries based on cumulative failure. Health checks drive circuit state transitions and feed back into retry eligibility.

### Rust Type Signatures

```rust
/// A self-healing operation wrapper.
pub struct ResilienceLayer<F, B, H> {
    breaker: CircuitBreaker,
    backoff: B,
    health: H,
    _marker: std::marker::PhantomData<F>,
}

impl<F, B, H> ResilienceLayer<F, B, H>
where
    F: Fn() -> Result<Trit, Error>,
    B: BackoffStrategy,
    H: HealthChecker,
{
    /// Execute `f` with retry + circuit-breaker + health feedback.
    /// Returns the trit result or a terminal error.
    pub fn execute(&mut self, f: F) -> Result<Trit, ResilienceError> {
        if self.breaker.state() == State::Open {
            // Check health before deciding
            if self.health.check().status == HealthStatus::Unhealthy {
                return Err(ResilienceError::CircuitOpenUnhealthy);
            }
            return Err(ResilienceError::CircuitOpen);
        }

        self.backoff.reset();
        while let Some(_delay) = self.backoff.next_delay() {
            match self.breaker.call(|| f()) {
                Ok(trit) => return Ok(trit),
                Err(_) => continue,
            }
        }
        Err(ResilienceError::RetriesExhausted)
    }
}

/// Compose the three SEEDs into a resilience layer.
pub fn resilience_layer(
    breaker_config: CircuitBreakerConfig,
    backoff: impl BackoffStrategy,
    health: impl HealthChecker,
) -> ResilienceLayer<fn() -> Result<Trit, Error>, impl BackoffStrategy, impl HealthChecker> {
    ResilienceLayer {
        breaker: CircuitBreaker::new(breaker_config),
        backoff,
        health,
        _marker: std::marker::PhantomData,
    }
}
```

### ASCII Data Flow

```
                    ┌─────────────────┐
                    │  Health Check   │
                    │  (periodic)     │
                    └────────┬────────┘
                             │ HealthStatus
                             ▼
  Request ──► [Retry-Backoff]──►[Circuit-Breaker]──► Target
                   ▲                    │
                   │   failure/success  │
                   └────────────────────┘
                             │
                    ┌────────▼────────┐
                    │  Health Check   │◄── feedback loop
                    │  (on failure)   │
                    └─────────────────┘
```

### Conservation Analysis

Trits flow: request carries `trits: Vec<i8>` with sum C.
- Retry does not modify trits — it re-dispatches the same bottle.
- Circuit-breaker returns the response bottle unchanged or an error bottle with the same trit sum.
- Health check is a side-channel; its trit sum is independent.

**Conservation scope: per-bottle, end-to-end.** The resilience layer is transparent to the conservation law.

| Component | γ (in) | η (out) | C preserved? |
|-----------|--------|---------|--------------|
| Retry-Backoff | C | C | ✅ (same bottle re-sent) |
| Circuit-Breaker | C | C or error(C) | ✅ (error bottles carry same sum) |
| Health-Check | independent | independent | ✅ (side-channel, not in data path) |
| **Composed** | **C** | **C** | **✅** |

---

## 2. Traffic Control Pattern

**Rate-Limiter + Load-Balancer + Feature-Flag → Canary System**

Requests pass through rate limiting, are evaluated against feature flags (for canary routing), then dispatched via load balancing to appropriate backends.

### Rust Type Signatures

```rust
pub struct TrafficControlLayer {
    limiter: TokenBucketRateLimiter,
    balancer: Box<dyn LoadBalancingStrategy>,
    flags: FeatureFlagService,
    targets: Vec<Target>,
}

pub struct RoutingDecision {
    pub target: Target,
    pub variant: Option<Variant>,
    pub rate_limit_remaining: f64,
}

impl TrafficControlLayer {
    /// Route a request through rate-limit → flag eval → load-balance.
    pub fn route(
        &mut self,
        context: &RequestContext,
        flag_context: &EvaluationContext,
    ) -> Result<RoutingDecision, TrafficError> {
        // 1. Rate limit
        if !self.limiter.try_acquire() {
            return Err(TrafficError::RateLimited);
        }

        // 2. Feature flag evaluation
        let variant = self.flags.evaluate("canary", flag_context);

        // 3. Filter targets by flag variant
        let eligible: Vec<&Target> = match variant {
            Some(Variant::Trit(1)) => self.targets.iter().filter(|t| t.metadata.get("tier") == Some(&"canary".into())).collect(),
            Some(Variant::Trit(0)) => self.targets.iter().filter(|t| t.metadata.get("tier") == Some(&"stable".into())).collect(),
            _ => self.targets.iter().collect(),
        };

        // 4. Load balance among eligible targets
        let idx = self.balancer.select(&self.targets, context)
            .ok_or(TrafficError::NoTargets)?;

        Ok(RoutingDecision {
            target: self.targets[idx].clone(),
            variant,
            rate_limit_remaining: self.limiter.remaining(),
        })
    }
}
```

### ASCII Data Flow

```
  Request ──►[Rate Limiter]──►[Feature Flag]──►[Load Balancer]──► Target
                  │               │                    │
                  │          ┌────┴────┐          ┌────┴────┐
                  │          │ canary? │          │ targets │
                  │          │ trit=+1 │          │ pool    │
                  │          │ → canary│          │         │
                  │          │ trit= 0 │          │         │
                  │          │ → stable│          │         │
                  │          │ trit=-1 │          │         │
                  │          │ → kill  │          │         │
                  │          └─────────┘          └─────────┘
                  ▼
           429 if empty
```

### Conservation Analysis

The canary system uses trits as *routing signals*:
- `trit = +1` → route to canary backend
- `trit = 0` → route to stable backend
- `trit = -1` → kill switch (reject)

Rate limiting is a gate — rejected requests don't consume trits. Load balancing preserves the incoming trit sum.

| Component | γ (in) | η (out) | C preserved? |
|-----------|--------|---------|--------------|
| Rate-Limiter | C or reject | C or 0 | ✅ (reject = no trits consumed) |
| Feature-Flag | C | C (routing hint) | ✅ (trits read, not mutated) |
| Load-Balancer | C | C | ✅ (transparent dispatch) |
| **Composed** | **C** | **C** | **✅** |

---

## 3. Service Mesh Pattern

**Service-Discovery + Config-Center + Health-Check + Load-Balancer → Full Mesh**

Services register with discovery, are health-checked continuously, and load-balanced using config-driven weights. Config changes propagate via watchers to rebalance in real-time.

### Rust Type Signatures

```rust
pub struct ServiceMesh {
    discovery: ServiceRegistry,
    config: ConfigCenter,
    health: HealthRegistry,
    balancer: Box<dyn LoadBalancingStrategy>,
}

pub struct MeshEndpoint {
    pub instance: ServiceInstance,
    pub config_version: u64,
    pub health_status: HealthStatus,
}

impl ServiceMesh {
    /// Register a service instance with health check + config watch.
    pub fn register_service(
        &mut self,
        instance: ServiceInstance,
        health_check: Box<dyn HealthChecker>,
    ) {
        let service_name = instance.service_name.clone();
        let instance_id = instance.id.clone();

        // Register in discovery
        self.discovery.register(instance).expect("register");

        // Register health check
        self.health.register(health_check);

        // Watch config for weight changes
        let config_key = format!("mesh.weights.{}", service_name);
        self.config.watch(
            &config_key,
            Box::new(move |val| {
                // Weight update triggers rebalance
                if let ConfigValue::Float(w) = val {
                    log::info!("weight update for {}: {}", service_name, w);
                }
            }),
        );
    }

    /// Resolve a service request through the full mesh pipeline.
    pub fn resolve(
        &self,
        service_name: &str,
        context: &RequestContext,
    ) -> Result<MeshEndpoint, MeshError> {
        // 1. Discover instances
        let instances = self.discovery.discover(service_name);
        if instances.is_empty() {
            return Err(MeshError::NoInstances(service_name.into()));
        }

        // 2. Check health — filter unhealthy
        let health_report = self.health.check_all();
        let healthy: Vec<&ServiceInstance> = instances
            .into_iter()
            .filter(|inst| inst.health == HealthStatus::Healthy)
            .collect();

        // 3. Build targets from healthy instances + config weights
        let targets: Vec<Target> = healthy
            .iter()
            .map(|inst| {
                let weight_key = format!("mesh.weights.{}", inst.service_name);
                let weight = self.config.get(&weight_key)
                    .and_then(|v| if let ConfigValue::Float(f) = v { Some(*f) } else { None })
                    .unwrap_or(1.0);
                Target {
                    id: inst.id.clone(),
                    weight,
                    healthy: true,
                    active_connections: 0,
                }
            })
            .collect();

        // 4. Load balance
        let idx = self.balancer.select(&targets, context)
            .ok_or(MeshError::NoTargets)?;

        let inst = healthy[idx];
        Ok(MeshEndpoint {
            instance: inst.clone(),
            config_version: self.config.version(),
            health_status: inst.health.clone(),
        })
    }
}
```

### ASCII Data Flow

```
  ┌─────────────────────────────────────────────────────────────┐
  │                     Service Mesh                            │
  │                                                             │
  │  ┌──────────────┐     ┌──────────────┐                     │
  │  │   Service     │     │   Config     │                     │
  │  │   Discovery   │     │   Center     │                     │
  │  │               │     │              │                     │
  │  │  register()   │     │  watch()     │                     │
  │  │  discover()   │◄────│  get()/set() │                     │
  │  │  heartbeat()  │     │  version     │                     │
  │  └──────┬───────┘     └──────┬───────┘                     │
  │         │                    │                              │
  │         │  instances +       │  weights                     │
  │         │  metadata          │                              │
  │         ▼                    ▼                              │
  │  ┌──────────────────────────────────────┐                  │
  │  │         Health Check Filter          │                  │
  │  │   Healthy / Degraded / Unhealthy     │                  │
  │  └──────────────┬───────────────────────┘                  │
  │                  │ healthy targets                           │
  │                  ▼                                          │
  │          ┌──────────────┐                                   │
  │          │ Load Balancer│──────► resolved endpoint           │
  │          │ (weighted)   │                                   │
  │          └──────────────┘                                   │
  │                                                             │
  └─────────────────────────────────────────────────────────────┘

  Request ──► ServiceDiscovery.discover()
                  │
                  ▼
          HealthCheck.filter(healthy_only)
                  │
                  ▼
          ConfigCenter.get_weights()
                  │
                  ▼
          LoadBalancer.select(targets, ctx)
                  │
                  ▼
            MeshEndpoint
```

### Conservation Analysis

The mesh introduces **per-instance trit state** from `ServiceInstance.trit_state`. Conservation flows through the mesh as a whole:

- Discovery trit sums are preserved (instances registered = instances discoverable).
- Config values include `ConfigValue::Trit(i8)` — config mutations must preserve aggregate trit sums.
- Health checks don't modify trits — they filter.
- Load balancing is transparent.

**Scope: global mesh conservation.** Total trit sum across all registered instances is invariant.

| Component | γ (in) | η (out) | C preserved? |
|-----------|--------|---------|--------------|
| Service-Discovery | Σ(inst.trit_state) | Σ(inst.trit_state) | ✅ (additive) |
| Config-Center | Σ(config_trits) | Σ(config_trits) | ✅ (versioned) |
| Health-Check | C | C (filtered subset) | ✅ (subset sum ≤ total) |
| Load-Balancer | C | C | ✅ (transparent) |
| **Composed** | **Σ(all_trits)** | **Σ(all_trits)** | **✅** |

---

## 4. Conservation: γ + η = C Flow Analysis

### The Law

For any SEED composition, the ternary conservation invariant holds:

```
γ (input trit sum) + η (internal trit mutation) = C (constant)
```

More precisely: `Σ(input_trits) = Σ(output_trits)` for any closed transformation.

### Per-SEED vs Global

| Scope | Rule | Rationale |
|-------|------|-----------|
| **Per-bottle** | `bottle_in.trit_sum() == bottle_out.trit_sum()` | Agent trait enforces this via `Runner` |
| **Per-SEED** | SEEDs that don't modify trits are transparent | Rate-limiter, load-balancer, health-check |
| **Per-composition** | Composed SEEDs form a pipeline; trits pass through | Each stage preserves, so composition preserves |
| **Global (mesh)** | `Σ(all_registered_trits)` is constant | Registration/deregistration is balanced |

### Conservation Through Composition

```
  Bottle (trits = [+1, 0, -1, +1], sum = +1)
      │
      ▼
  ┌─────────────────────────────────────────────┐
  │          Resilience Layer                    │
  │  Retry (transparent) → CB (transparent)      │
  │  Output: trits = [+1, 0, -1, +1], sum = +1  │
  └─────────────────────────────────────────────┘
      │
      ▼
  ┌─────────────────────────────────────────────┐
  │          Traffic Control Layer               │
  │  RateLimit (gate) → Flag (read) → LB (pass) │
  │  Output: trits = [+1, 0, -1, +1], sum = +1  │
  └─────────────────────────────────────────────┘
      │
      ▼
  ┌─────────────────────────────────────────────┐
  │          Service Mesh Layer                  │
  │  Discovery → Health → Config → LB            │
  │  Output: trits = [+1, 0, -1, +1], sum = +1  │
  └─────────────────────────────────────────────┘
      │
      ▼
  Bottle (trits = [+1, 0, -1, +1], sum = +1)  ← CONSERVED
```

### Where η (mutation) Can Occur

Only SEEDs with explicit trit mutation may change internal state:

1. **Feature-Flag** — `Variant::Trit(i8)` can inject routing trits, but these are *added context*, not payload mutations. Conservation holds because the flag evaluation doesn't modify the bottle's `trits` field.
2. **Config-Center** — `ConfigValue::Trit(i8)` stores trits but doesn't mutate passing bottles.
3. **Service-Discovery** — `ServiceInstance.trit_state` is metadata, not a transformation.

**Conclusion: In all compositions, the `Runner` enforcement at agent boundaries is sufficient. SEEDs are conservation-transparent by design.**

---

## 5. Bottle Protocol Integration

### How Composed Patterns Fit Into Bottles

The Bottle protocol defines the wire format. Compositions are *middleware layers* that bottles pass through. Each layer is a pure function `Bottle → Result<Bottle, Error>`.

### Type Signature

```rust
/// A middleware layer that bottles pass through.
pub trait BottleMiddleware: Send + Sync {
    /// Process a bottle. Returns the (possibly transformed) bottle or an error.
    /// Conservation invariant: output.trit_sum() == input.trit_sum()
    fn process(&self, bottle: Bottle) -> Result<Bottle, BottleError>;
}

/// Compose multiple middleware layers into a pipeline.
pub struct BottlePipeline {
    layers: Vec<Box<dyn BottleMiddleware>>,
}

impl BottlePipeline {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    /// Add a middleware layer.
    pub fn layer(mut self, mw: Box<dyn BottleMiddleware>) -> Self {
        self.layers.push(mw);
        self
    }

    /// Process a bottle through all layers.
    /// Conservation is checked after each layer.
    pub fn process(&self, bottle: Bottle) -> Result<Bottle, BottleError> {
        let mut current = bottle;
        let expected_sum = current.trit_sum();

        for layer in &self.layers {
            current = layer.process(current)?;
            // Audit conservation at each layer boundary
            debug_assert_eq!(
                current.trit_sum(), expected_sum,
                "Conservation violated at layer boundary"
            );
        }

        Ok(current)
    }
}
```

### Bottle Flow Through Compositions

```
  ┌──────────────────────────────────────────────────────────────┐
  │                     Bottle Pipeline                          │
  │                                                              │
  │  Bottle {                                                    │
  │    id: uuidv7,                                               │
  │    src: "client",                                            │
  │    tgt: "service",                                           │
  │    act: "request",                                           │
  │    trits: [+1, 0, -1],   // sum = 0                         │
  │    payload: msgpack(...)                                     │
  │  }                                                           │
  │      │                                                       │
  │      ▼                                                       │
  │  ┌──────────────────┐                                        │
  │  │ Resilience Layer │  CB + Retry + Health                   │
  │  │ (middleware)     │  trit_sum check: 0 == 0 ✓              │
  │  └────────┬─────────┘                                        │
  │           ▼                                                  │
  │  ┌──────────────────┐                                        │
  │  │ Traffic Control  │  RateLimit + Flag + LB                 │
  │  │ (middleware)     │  trit_sum check: 0 == 0 ✓              │
  │  └────────┬─────────┘                                        │
  │           ▼                                                  │
  │  ┌──────────────────┐                                        │
  │  │ Service Mesh     │  Discovery + Config + Health + LB      │
  │  │ (middleware)     │  trit_sum check: 0 == 0 ✓              │
  │  └────────┬─────────┘                                        │
  │           ▼                                                  │
  │  Bottle {                                                    │
  │    id: uuidv7,                                               │
  │    src: "client",                                            │
  │    tgt: "resolved-endpoint",                                 │
  │    act: "request",                                           │
  │    trits: [+1, 0, -1],   // sum = 0 ← CONSERVED             │
  │    payload: msgpack(...)                                     │
  │  }                                                           │
  └──────────────────────────────────────────────────────────────┘
```

### Mapping Compositions to Bottle Actions

Each composition maps to a namespace in the bottle `act` field:

| Composition | Bottle `act` namespace | Example |
|-------------|----------------------|---------|
| Resilience | `resilience.*` | `resilience.execute`, `resilience.probe` |
| Traffic Control | `traffic.*` | `traffic.route`, `traffic.canary` |
| Service Mesh | `mesh.*` | `mesh.register`, `mesh.resolve`, `mesh.heartbeat` |
| Config Center | `config.*` | `config.get`, `config.set`, `config.watch` |
| Health Check | `health.*` | `health.check`, `health.report` |

### Error Bottles

When a layer rejects a bottle, it produces an error bottle with conserved trits:

```rust
impl BottleError {
    /// Create an error bottle that preserves the input trit sum.
    pub fn into_bottle(self, input: &Bottle) -> Bottle {
        Bottle::new(
            input.tgt.clone(),  // flip src/tgt
            input.src.clone(),
            format!("error.{}", input.act),
        )
        .with_trits(input.trits.clone())  // ← conservation preserved
        .with_payload(msgpack::to_vec(&self).unwrap())
    }
}
```

---

## Summary

| Pattern | SEEDs Composed | Conservation | Bottle Integration |
|---------|---------------|-------------|-------------------|
| **Resilience** | CB + Retry + Health | Per-bottle, transparent | `resilience.*` middleware |
| **Traffic Control** | RateLimit + Flag + LB | Per-bottle, gate+route | `traffic.*` middleware |
| **Service Mesh** | Discovery + Config + Health + LB | Global (all instances) | `mesh.*` middleware |
| **Full Stack** | All SEEDs | End-to-end, audited per-layer | `BottlePipeline` |

**Core invariant: every composition is a pure function `Bottle → Bottle` with `trit_sum` preserved at every layer boundary. The `Runner` enforces this at agent boundaries; the pipeline audits it at middleware boundaries.**
