// Copyright (C) 2026 Jorge Andre Castro
//
// Ce programme est un logiciel libre : vous pouvez le redistribuer et/ou le modifier
// selon les termes de la Licence Publique Générale GNU telle que publiée par la
// Free Software Foundation, soit la version 2 de la licence, soit (à votre convention)
// n'importe quelle version ultérieure.
//
//! # embedded-f32-sqrt
//!
//! Racine carrée `f32` par Newton-Raphson pour systèmes embarqués `no_std`.
//!
//! Sans dépendance, sans `unsafe`, sans FPU requise.
//!
//! ```rust
//! use embedded_f32_sqrt::sqrt;
//!
//! assert!((sqrt(9.0).unwrap() - 3.0).abs() < 1e-5);
//! assert!((sqrt(2.0).unwrap() - 1.414_213_5).abs() < 1e-6);
//! assert!(sqrt(-1.0).is_err());
//! ```

#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

/// Erreur retournée pour une entrée négative.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NegativeInput;

/// Racine carrée `f32` par Newton-Raphson.
///
/// Estimation initiale via décalage de bits IEEE 754,
/// puis 5 itérations. Précision : erreur relative < 1 ULP f32.
///
/// Retourne `Err(NegativeInput)` si `x < 0.0`.
pub fn sqrt(x: f32) -> Result<f32, NegativeInput> {
    if x < 0.0 {
        return Err(NegativeInput);
    }
    if x == 0.0 {
        return Ok(0.0);
    }

    // Estimation initiale par manipulation d'exposant IEEE 754
    let mut r = f32::from_bits(((x.to_bits() >> 1) + 0x1FBB_4F2E) & 0x7FFF_FFFF);

    // Newton-Raphson : r = (r + x/r) / 2
    for _ in 0..5 {
        r = 0.5 * (r + x / r);
    }

    Ok(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perfect_squares() {
        for n in [0u32, 1, 4, 9, 16, 25, 100, 10_000] {
            let expected = (n as f32).sqrt();
            assert!((sqrt(n as f32).unwrap() - expected).abs() < 1e-4);
        }
    }

    #[test]
    fn irrational() {
        assert!((sqrt(2.0).unwrap() - 1.414_213_5_f32).abs() < 1e-5);
    }

    #[test]
    fn zero() {
        assert_eq!(sqrt(0.0), Ok(0.0));
    }

    #[test]
    fn negative() {
        assert_eq!(sqrt(-1.0), Err(NegativeInput));
    }
}