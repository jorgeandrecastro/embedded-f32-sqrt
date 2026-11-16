# embedded-f32-sqrt
An optimized Newton–Raphson square root for `f32` on `no_std` embedded systems.

[![Crates.io](https://img.shields.io/crates/v/embedded-f32-sqrt.svg)](https://crates.io/crates/embedded-f32-sqrt)
[![License](https://img.shields.io/crates/l/embedded-f32-sqrt.svg)](https://github.com/JorgeAndresCastro/embedded-f32-sqrt/blob/main/LICENSE)
[![Docs.rs](https://docs.rs/embedded-f32-sqrt/badge.svg)](https://docs.rs/embedded-f32-sqrt/)

# Update Version 0.2.1
Documentation translated into English; no changes to the logic or implementation.

# Update Version 0.2.0
NaN and infinity handling for improved robustness.


# Update Version 0.1.4
README update. No changes were made to the logic or implementation in version 0.1.4. `#![forbid(unsafe_code)]` was chosen for greater reliability, and `opt-level = 3` for maximum speed.


**Highlights**
Zero dependencies: no `libm`, no `micromath`, and no C library.

Maximum safety: `#![forbid(unsafe_code)]` and robust error handling with `Result`.

**Full portability:**

Targets without an FPU (Cortex-M0+, RISC-V): deterministic software implementation.

Targets with an FPU (Cortex-M4F, M33, M7): uses the floating-point units natively.

Precision: relative error < 1 ULP (Unit in the Last Place) after 5 iterations.

**Usage**
````
[dependencies]
embedded-f32-sqrt = "0.2.0"
````

----

## Basic usage example
````rust
use embedded_f32_sqrt::sqrt;

if let Ok(result) = sqrt(9.0) {
    // Use result (e.g. for logging or further calculations).
}

// Handle the result with a match expression.
match sqrt(-1.0) {
    Ok(val) => {
        // Use val.
    },
    Err(e) => {
        // Handle or log the error.
    },
}
````
----

**Algorithm**
The implementation is based on the Newton–Raphson method, an iterative approach that doubles the number of bits of precision at each step.

Initial estimate: direct manipulation of the IEEE 754 exponent bits provides a first approximation within ~3% of the actual value.

Refinement: 5 iterations of the formula `r = 0.5 * (r + x / r)` bring the result to the precision limit of the `f32` type.

On recent architectures, LLVM optimizes this loop or emits the `VSQRT` machine instruction directly when the hardware supports it.

----

# Complete example: OLED, HCSR505, and Pico 2 LED blink
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

// My crates
use embassy_hcsr505::Hcsr505;
use embassy_hcsr505::signals::MOTION_SIGNAL;
use embedded_f32_sqrt::sqrt; // The square-root crate

use rp2350_linker as _;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::{I2C0, PIN_20}; 
use embassy_rp::Peri; 

bind_interrupts!(struct Irqs {
    I2C0_IRQ => embassy_rp::i2c::InterruptHandler<I2C0>;
});

// Task: PIR detection using the crate
#[embassy_executor::task]
async fn pir_task(pin_p: Peri<'static, PIN_20>) {
    let pin = Input::new(pin_p, Pull::Down);
    let mut pir = Hcsr505::new(pin);

    loop {
        pir.wait_for_motion().await;
        // Optional: signal manually if the crate does not already do this internally.
        MOTION_SIGNAL.signal(true);
        
        pir.wait_for_idle().await;
        MOTION_SIGNAL.signal(false);
    }
}

// Task: JC-OS security display
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
            
            // Test the square-root crate.
            // Calculate the square root of the counter to validate f32 on the RP2350.
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

    // Start the tasks.
    spawner.spawn(pir_task(p.PIN_20)).unwrap();
    spawner.spawn(display_task(lcd)).unwrap();

    let mut led = Output::new(p.PIN_25, Level::Low);
    loop {
        led.toggle();
        Timer::after_millis(500).await; 
    }
}

```
----

# Licence
GPL-2.0-or-later

Copyright (C) 2026 Jorge Andre Castro.