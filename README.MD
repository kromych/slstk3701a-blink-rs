# Example applications for SLSTK3701A

This example shows how to log via RTT and blink an LED on
[the SLSTK3701A starter kit board](https://www.silabs.com/development-tools/mcu/32-bit/efm32gg11-starter-kit?tab=overview)

## Prepare

One-time tool installation:

```sh
cargo install flip-link
# Wait for the updated version with support for EFM32 Happy Gecko to be released
# or use the main branch of https://github.com/probe-rs/probe-rs.
cargo install cargo-embed
cargo install cargo-flash
cargo install probe-rs-cli
cargo install probe-rs-debugger
cargo install probe-run
```

## SVD packs

https://www.keil.arm.com/packs/

* [EFM32GG11B](https://www.silabs.com/documents/public/cmsis-packs/SiliconLabs.GeckoPlatform_EFM32GG11B_DFP.4.3.0.pack)

## Thank you

It is specualted that if one wants to build something from scratch, they ought to start from creating the Universe itself :)
That said, this project would not have been possible without many great folks on whose efforts this repo has relied.

1. The infrastructure parts of `support/efm32hg322_hal` borrow heavily from [`efm32hg-hal`](https://github.com/fudanchii/efm32hg-hal).
   Despite its name, it supports just one MCU of the EFM32HG family (EFM32HG309F64), and the means of extending weren't obvious to me.
   As the cleanest approach, I copied as much code as needed, compiled with the licensing terms, and added support for EFM32322F64's GPIO.

2. The idea of generating the register description comes from [`efm32hg-pac`](https://github.com/em32-rs/efm32hg-pac). Again,
   the project supports just one MCU (EFM32HG309F64) so I procured the SVD file, patched it, and generated the definitions for
   EFM32HG322F64 myself.
