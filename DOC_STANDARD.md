# DOC_STANDARD.md — The Fleet Documentation Standard

*Standard for all SuperInstance crates. Documentation is and always will be the "mortar" that binds our ternary bricks.*

---

## 📖 The Core Philosophy

A crate is not "finished" when the tests pass; it is finished when it is **contextualized**. Every crate must explain its place in the ternary ecosystem.

---

## 📋 Mandatory Sections

Every `README.md` must contain the following four sections, in this order:

### 1. The a la mode Essence
A 2-3 sentence distillation of the crate's purpose. Avoid corporate speak. Use the "Sovereign" tone: precise, a bit eccentric, and technically honest.

### 2. The Ternary Substrate
Explicitly state how the crate uses $\{-1, 0, +1\}$.
- What does `+1` represent?
- What does `0` (Neutral) represent?
- What does `-1` represent?
- Is there a conservation law? (e.g., "The sum of all states must remain constant across transformations")

### 3. The la-Links (The Anti-Silo Section)
**Crucial.** A list of at least 3 references to other crates in the SuperInstance fleet.
- **Siblings**: Crates that do similar things.
- **Ancestors**: Crates this one depends on or evolved from.
- **Descendants**: Crates that use this one as a primitive.
- **The Bridge**: "If you like [Crate X], you'll find the [Feature Y] in this crate useful for [Z]."

### 4. CORTEX Alignment
A brief declaration of the crate's `CORTEX.json` manifestation.
- Which `construct_tier` (ESP32, Pi, Workstation, etc.)?
- Which `capabilities` does it provide?

---

## 🛠️ Documentation Checklist

- [ ] Does the README avoid "aspirational fiction"? (No mentions of features not yet in `src/`)
- [ la-Sovereign ] Does the tone match the fleet's "Truth-Sayer" persona?
- [ la-Link ] Are there at least 3 valid links to other SuperInstanceH github repos?
- [ Ternary ] Is the mapping of $\{-1, 0, +1\}$ explicit?

---

*"Precision in documentation is the only way to prevent a fleet of 211 crates from becoming a noise floor."*
