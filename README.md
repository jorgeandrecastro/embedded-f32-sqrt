# embedded-f32-sqrt

[![Crates.io](https://img.shields.io/crates/v/embedded-f32-sqrt)](https://crates.io/crates/embedded-f32-sqrt)
[![Docs.rs](https://docs.rs/embedded-f32-sqrt/badge.svg)](https://docs.rs/embedded-f32-sqrt)
[![License: GPL-2.0-or-later](https://img.shields.io/badge/license-GPL--2.0--or--later-blue)](LICENSE)
![no_std](https://img.shields.io/badge/no__std-✓-green)

Racine carrée `f32` par **Newton-Raphson** pour systèmes embarqués `no_std`.

- Zéro dépendance — pas de `libm`, pas de `micromath`
- `#![forbid(unsafe_code)]`
- Fonctionne sans FPU (Cortex-M0+, RISC-V)
- Si FPU présente, LLVM émet `VSQRT` automatiquement

## Utilisation

```toml
[dependencies]
embedded-f32-sqrt = "0.1"
```

```rust
use embedded_f32_sqrt::sqrt;

sqrt(9.0).unwrap();   // Ok(3.0)
sqrt(2.0).unwrap();   // Ok(1.4142135)
sqrt(-1.0);           // Err(NegativeInput)
```

## Algorithme

Estimation initiale par décalage de bits sur l'exposant IEEE 754, puis 5 itérations Newton-Raphson `r = (r + x/r) / 2`. Précision : erreur relative < 1 ULP f32.

## Licence

GPL-2.0-or-later

Copyright (C) 2026 Jorge Andre Castro.