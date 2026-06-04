# Documentation Quality Standard — SuperInstance

*Documentation often outlasts code. Write accordingly.*

## What We Want: Service Manuals

NOT marketing copy. NOT AI-generated filler. A service manual for each crate that a new engineer can pick up on day one and understand:

1. **What this thing IS** — concrete, specific, no buzzwords
2. **Why it exists** — what problem drove someone to write it
3. **How to use it** — working code, not hand-waving
4. **What can go wrong** — edge cases, limitations, known issues
5. **How it works internally** — enough to debug and extend
6. **How it fits the ecosystem** — what depends on it, what it depends on

## Anti-Patterns (DO NOT DO)

- ❌ "A powerful library for..." — every library claims to be powerful
- ❌ "Leverages cutting-edge..." — no it doesn't, and even if it did, say what it DOES
- ❌ Listing features without explaining when you'd use each one
- ❌ Code examples that don't compile because they reference types not shown
- ❌ "See the documentation for more details" when YOU ARE the documentation
- ❌ Copy-paste between READMEs with just the name changed
- ❌ Omitting edge cases or failure modes
- ❌ Using jargon without defining it on first use

## Required Sections

### 1. Title + One-Liner
Plain English. "ternary-kalman: Kalman filter for {-1, 0, +1} state spaces"

### 2. Why This Exists
2-3 sentences. What problem? Why ternary? What couldn't you do before?

### 3. Core Concepts
Define EVERY term a newcomer wouldn't know. "Balanced ternary" → explain it.
"Kalman filter" → one sentence about what it predicts.
"GF(3)" → "the finite field with 3 elements, where -1 + 1 = 0"

### 4. Quick Start
```toml
# Cargo.toml
[dependencies]
crate-name = "0.1"
```

```rust
// 10-20 lines that actually compile and do something useful
// Include all imports, all type definitions shown
// Prefer showing ONE complete example over three partial ones
```

### 5. API Overview
Table of main types with ONE-LINE descriptions. Not marketing. What each type IS.

### 6. How It Works
2-3 paragraphs. Enough to debug. Mention the algorithm by name.
Explain design decisions — why THIS approach over alternatives?

### 7. Known Limitations
What doesn't work. What's untested. What degrades at scale.
Honesty builds trust. Every crate has limitations.

### 8. Use Cases
3+ real scenarios. Not "AI" or "ML" — specific.
"Robot arm controller choosing between three motor states"
"Spam filter classifying email as spam/unsure/ham"
"Trading signal: buy/hold/sell"

### 9. Ecosystem Context
Which other ternary crates does this relate to? Is it a dependency of others?
Where does it sit in the architecture?

### 10. License
MIT

## Review Checklist

Before submitting a README, verify:
- [ ] Every code example compiles (mentally trace it)
- [ ] Every technical term is defined on first use
- [ ] No marketing language ("powerful", "cutting-edge", "leverages")
- [ ] At least one limitation is honestly stated
- [ ] A new engineer with NO context about our project could understand it
- [ ] The "Why This Exists" section answers a real question, not a straw man
