# embedded-f32-sqrt
Racine carrée f32 par Newton-Raphson optimisée pour les systèmes embarqués no_std.

Points forts
Zéro dépendance  Pas de libm, pas de micromath, pas de bibliothèque C.

Sécurité maximale  #![forbid(unsafe_code)] et gestion robuste via Result.

Portabilité totale :

Cibles sans FPU (Cortex-M0+, RISC-V) : Implémentation logicielle déterministe.

Cibles avec FPU (Cortex-M4F, M33, M7) : Exploite nativement les unités de calcul flottant.

Précision : Erreur relative < 1 ULP (Unit in the Last Place) après 5 itérations.

Utilisation
Ini, TOML
[dependencies]
embedded-f32-sqrt = "0.1.2"
````rust
use embedded_f32_sqrt::sqrt;

fn main() {
    let val = 9.0;
    match sqrt(val) {
        Ok(res) => println!("sqrt({}) = {}", val, res),
        Err(_)  => println!("Erreur : valeur négative !"),
    }
}

````
Algorithme
L'implémentation repose sur la méthode de Newton-Raphson, une approche itérative qui double le nombre de bits de précision à chaque étape.

Estimation initiale : Manipulation directe des bits de l'exposant IEEE 754 pour obtenir une première approximation à ~3% de la valeur réelle.

Raffinement : 5 itérations de la formule r = 0.5 * (r + x / r) pour saturer la précision du type f32.

Sur les architectures récentes, LLVM optimise cette boucle ou émet directement l'instruction machine VSQRT si le matériel le permet.

Licence
GPL-2.0-or-later

Copyright (C) 2026 Jorge Andre Castro.