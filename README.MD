# Example applications for SLSTK3701A

This example shows how to log via RTT and blink an LED on
[the SLSTK3701A starter kit board](https://www.silabs.com/development-tools/mcu/32-bit/efm32gg11-starter-kit?tab=overview)

## Prepare

One-time tool installation:

```sh
cargo install flip-link
cargo install probe-rs --features="cli"
```

## SVD packs

https://www.keil.arm.com/packs/

* [EFM32GG11B](https://www.silabs.com/documents/public/cmsis-packs/SiliconLabs.GeckoPlatform_EFM32GG11B_DFP.4.3.0.pack)

To create the Peripheral Access Crate (a.k.a PAC) from the SVD, you'll need these tools:

```sh
cargo install svd2rust
cargo install svdtools
cargo install form
```

and then run [`./support/efm32gg11b-pac/svd/gen.sh`](./support/efm32gg11b-pac/svd/gen.sh) from its directory.
Finally, `cargo fmt` will format the generated desciptions.

## Thank you

It is speculated that if one wants to build something from scratch, they ought to start from creating the Universe itself :)
That said, this project would not have been possible without many great folks on whose efforts this repo has relied.

1. The idea of generating the register description comes from [`efm32hg-pac`](https://github.com/em32-rs/efm32hg-pac).
   The project supports just one MCU (EFM32HG309F64) so I procured the SVD file, patched it, and generated the definitions for
   EFM32GG11B820F2048GL192 myself.
