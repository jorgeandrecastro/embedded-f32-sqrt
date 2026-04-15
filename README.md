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
embedded-f32-sqrt = "0.1.3"
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

# Exemple Clé en main Oled , Hcsr505 , et Blink pico 2.
```rust 
#![no_std]
#![no_main]

use cortex_m_rt as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_rp::i2c::{Config as I2cConfig, I2c, Async};
use embassy_time::{Delay, Duration, Timer};
use hd44780_i2c_nostd::LcdI2c;
use {panic_halt as _, embassy_rp as _};
use heapless::String;
use core::fmt::Write;

//  Mes CRATES 
use embassy_hcsr505::Hcsr505;
use embassy_hcsr505::signals::MOTION_SIGNAL;
use embedded_f32_sqrt::sqrt; // Ta nouvelle crate mathématique

use rp2350_linker as _;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::{I2C0, PIN_20}; 
use embassy_rp::Peri; 

bind_interrupts!(struct Irqs {
    I2C0_IRQ => embassy_rp::i2c::InterruptHandler<I2C0>;
});

// TASK : DETECTION PIR VIA CRATE 
#[embassy_executor::task]
async fn pir_task(pin_p: Peri<'static, PIN_20>) {
    let pin = Input::new(pin_p, Pull::Down);
    let mut pir = Hcsr505::new(pin);

    loop {
        pir.wait_for_motion().await;
        // Optionnel: on signale manuellement si ta crate ne le fait pas déjà en interne
        MOTION_SIGNAL.signal(true);
        
        pir.wait_for_idle().await;
        MOTION_SIGNAL.signal(false);
    }
}

// TASK : DISPLAY JC-OS SECURITY 
#[embassy_executor::task]
async fn display_task(mut lcd: LcdI2c<I2c<'static, I2C0, Async>>) {
    let mut delay = Delay;
    let mut count = 0u32; 

    Timer::after(Duration::from_millis(500)).await;
    
    if lcd.init(&mut delay).await.is_ok() {
        let _ = lcd.set_backlight(true);
        let _ = lcd.clear(&mut delay).await;
        let _ = lcd.write_str("   JC-OS KERNEL", &mut delay).await;
        let _ = lcd.set_cursor(1, 0, &mut delay).await;
        let _ = lcd.write_str("   SECURE MODE", &mut delay).await;
    }

    Timer::after(Duration::from_secs(2)).await;

    loop {
        let detected = MOTION_SIGNAL.wait().await;
        
        let _ = lcd.clear(&mut delay).await;
        let _ = lcd.set_cursor(0, 0, &mut delay).await;

        if detected {
            count += 1;
            
            //  TEST DE LA CRATE SQRT 
            // Calcul de la racine du compteur pour valider le f32 sur RP2350
            let s_val = sqrt(count as f32).unwrap_or(0.0);

            let mut s: String<16> = String::new();
            let _ = write!(s, "ID:{} RT:{:.4}", count, s_val);
            
            let _ = lcd.write_str(s.as_str(), &mut delay).await;
            let _ = lcd.set_cursor(1, 0, &mut delay).await;
            let _ = lcd.write_str(" EAGLE ALERT :)", &mut delay).await;
        } else {
            let _ = lcd.write_str("  SYSTEM READY", &mut delay).await;
            let _ = lcd.set_cursor(1, 0, &mut delay).await;
            let _ = lcd.write_str("   ALL CLEAR", &mut delay).await;
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(embassy_rp::config::Config::default());

    MOTION_SIGNAL.signal(false); 

    let mut i2c_config = I2cConfig::default();
    i2c_config.frequency = 100_000;
    let i2c = I2c::new_async(p.I2C0, p.PIN_5, p.PIN_4, Irqs, i2c_config);
    let lcd = LcdI2c::new(i2c, 0x3F); 

    // Lancement des tâches en gardant ta syntaxe
    spawner.spawn(pir_task(p.PIN_20)).unwrap();
    spawner.spawn(display_task(lcd)).unwrap();

    let mut led = Output::new(p.PIN_25, Level::Low);
    loop {
        led.toggle();
        Timer::after_millis(500).await; 
    }
}

```




Licence
GPL-2.0-or-later

Copyright (C) 2026 Jorge Andre Castro.