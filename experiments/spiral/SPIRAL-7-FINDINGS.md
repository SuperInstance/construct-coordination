# Spiral 7: Compressibility + Z₃ Cryptography

## 1. Compressibility of Ternary Streams

| Stream | Entropy | RLE Size | 3-gram Dict | Autocorr | Compress Ratio |
|--------|---------|----------|-------------|----------|----------------|
| Random | 1.5849 | 13346 | 27 | 0.33 | 1.33 |
| Periodic (-1,0,1 repeat) | 1.5850 | 20000 | 3 | 0.00 | 2.00 |
| Fibonacci | 1.5613 | 15000 | 8 | 0.25 | 1.50 |
| Majority rule | 1.3932 | 62 | 13 | 0.70 | **0.62** |
| RPS wave | 0.7112 | 6796 | 7 | 0.66 | 0.68 |

**Key Finding**: 
- **Majority rule is the most compressible** (0.62 ratio, only 62 RLE entries for 10K symbols)
- **RPS waves are very compressible** (0.68 ratio, low entropy 0.71)
- **Periodic is incompressible by RLE** because it alternates every value (20000 runs)
- **Fibonacci is moderate** (1.50 ratio, 8 unique 3-grams = period-8 structure)
- **Random is nearly incompressible** (1.33 ratio, 27/27 possible 3-grams)

**Implication**: The most interesting dynamics (RPS waves, majority domains) are the most compressible.
They have STRUCTURE. Random is boring because it has no structure.
The ternary systems we've been building exist in the sweet spot between order and chaos.

## 2. Z₃ as Cipher

The Z₃ group operation (ternary addition) is:
```
  | -1  0  +1
--+----------
-1| -1  0  +1
 0|  0  +1 -1
+1| +1 -1  0
```

This is NOT binary XOR. Key differences:
- Identity is 0 (same as XOR)
- a ⊕ a = -1, not 0 (self-XOR doesn't give 0)
- The operation is NOT self-inverse (need subtraction to decrypt)
- Encryption needs the INVERSE key, not the same key

This means ternary encryption requires: encrypt(a, k) = a ⊕ k, decrypt(c, k) = c ⊖ k (subtract)
Unlike binary XOR where encrypt and decrypt use the same operation.

**Implication for PLATO**: Room-to-room communication can be encrypted with Z₃ addition.
The key is the room's identity. Decryption requires subtraction. This is a natural 
one-time pad for ternary messages — each room has a ternary identity key.

## Cross-Spiral Convergence

The compressibility results connect to everything:
- **Period 8 (Fibonacci)** → 8 unique 3-grams → moderate compressibility
- **Majority domains** → extreme compressibility → STRUCTURE exists
- **RPS waves** → 7 unique 3-grams → wave structure IS compressible
- **Random** → 27/27 3-grams → no structure → maximum entropy

The interesting systems are the compressible ones. Random is boring.
Structure = compressibility = the system has found a pattern worth extracting.
