# Example applications for SLSTK3701A

Here are some examples for the
[SLSTK3701A starter kit board](https://www.silabs.com/development-tools/mcu/32-bit/efm32gg11-starter-kit?tab=overview)

Demo | Description
-----|--------------------------
[`adc-temp`](./demos/adc-temp) | internal temperature sensor
[`blink`](./demos/blink) | blink
[`blink-pb`](./demos/blink-pb) | blink with push buttons
[`blink-systick`](./demos/blink-systick) | blink with SysTick
[`eth-http`](./demos/eth-http) | Ethernet DHCP + HTTP demo
[`lcd-counter`](./demos/lcd-counter) | RTC counter on JDI Color LCD
[`lcd-hello`](./demos/lcd-hello) | JDI Color LCD hello demo
[`space-invaders`](./demos/space-invaders) | Space Invaders on JDI Color LCD
[`stopwatch`](./demos/stopwatch) | stopwatch with JDI Color LCD
[`usb-audio`](./demos/usb-audio) | USB Audio synth demo
[`usb-cdc`](./demos/usb-cdc) | USB CDC ACM serial port demo
[`usb-hid`](./demos/usb-hid) | USB HID keyboard demo
[`usb-midi`](./demos/usb-midi) | USB MIDI demo
[`usb-msc`](./demos/usb-msc) | USB Mass Storage demo
[`usb-video`](./demos/usb-video) | USB Video demo

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
