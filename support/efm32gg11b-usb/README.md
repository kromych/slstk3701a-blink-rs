# efm32gg11b-usb — DWC2 USB Device Driver for EFM32GG11B

Bare-metal USB device driver for the EFM32 Giant Gecko 11 (Cortex-M4F, EFM32GG11B820F2048GL192).
Uses the on-chip DWC2 OTG core in slave (FIFO) mode. Ported from `efm32hg322_usb`.

## DWC2 on the EFM32GG11B

| Property | Value |
|---|---|
| Core | Cortex-M4F |
| USB peripheral base | `0x4002_2000` (EFM32 wrapper registers) |
| DWC2 core offset | `+0xDE000` (core at `0x4010_0000`) |
| FIFO base | `USB_BASE + 0xDF000 + ep * 0x1000` |
| Device endpoints | EP0–EP6 (7 total) |
| FIFO RAM | 512 words (2 KB) shared |
| Max packet size (EP0) | 64 bytes |
| PHY | Integrated FS-only |
| USB clock | USHFRCO 48 MHz with SOF clock recovery |
| DMA support | Yes (GAHBCFG.DMAEN) — not used by this driver |
| VBUS detection | `USB->STATUS.VBUSDETH` + `EMU->R5VSTATUS.VBUSDET` |
| OTG | Yes — OTG registers present (GOTGCTL), device-only use |
| D+ pull-up | `DCTL.SFTDISCON` (clear to connect, set to disconnect) |
| Power regulator | EMU R5V (5 V LDO with VBUS/VREGI input, VREGO output) |

## Key Differences from EFM32HG (efm32hg322_usb)

| Feature | EFM32HG | EFM32GG11B |
|---|---|---|
| **Register layout** | USB base = DWC2 base (`0x400C4000`) | USB wrapper at `0x40022000`, DWC2 at `+0xDE000` |
| **FIFO offset** | `base + 0x3D000` | `base + 0xDF000` |
| **Endpoints** | 3 (EP0–EP2) | 7 (EP0–EP6) |
| **FIFO RAM** | 256 words (1 KB) | 512 words (2 KB) |
| **D+ pull-up** | `USB->ROUTE.DMPUPEN` (explicit bit) | `DCTL.SFTDISCON` (standard DWC2 soft-disconnect) |
| **VBUS detect** | `USB->CTRL.VREGOSEN` + `STATUS.VREGOS` | `USB->STATUS.VBUSDETH` + `EMU->R5VSTATUS` |
| **VBUS flow** | Always-on (bus-powered) | Requires VBUSDETH interrupt + GOTGCTL overrides |
| **OTG registers** | Not present | Present; GOTGCTL BVALIDOV needed for D+ pullup |
| **Extra wrapper regs** | `IF`, `IEN`, `IFC`, `IFS` (basic) | `IF`, `IEN`, `IFC`, `IFS` with VBUSDETH/VBUSDETL bits |
| **EMU R5V** | Not present | Must set `R5VOUTLEVEL = 10` (5 V), wait `R5VSYNC` |
| **DATTRIM1** | Not present | Must set `ENDLYPULLUP` after every reset |
| **Power-on handshake** | Toggle `DCTL.PWRONPRGDONE` | Not used (Silicon Labs EMLIB omits it on GG11B) |
| **Clock tree** | `HFCORECLKEN0.USBC+USB+LE`, `CMD.USBCCLKSEL` | `CMU.USBCTRL.USBCLKSEL+USBCLKEN`, `HFBUSCLKEN0.USB` |
| **LEM** | `CTRL.LEMOSCCTRL = NONE` during init | `CTRL.LEMOSCCTRL = GATE`, `LEMIDLEEN`, `LEMPHYCTRL` |

### Why GOTGCTL overrides are required on GG11B

The EFM32GG11B has OTG-capable DWC2 silicon. The DWC2 core uses internal VBUS comparators to
set `GOTGCTL.BSESVLD` (B-session valid). On the GG11B, these comparators are not connected to
the actual VBUS pin — Silicon Labs routes VBUS sensing through the EMU R5V subsystem and the
USB wrapper's `STATUS.VBUSDETH` bit instead.

Without `GOTGCTL.BSESVLD = 1`, the DWC2 core does not apply the D+ pull-up even when
`DCTL.SFTDISCON = 0`. The driver must set GOTGCTL overrides (`BVALIDOVEN + BVALIDOVVAL +
VBVALIDOVEN + VBVALIDOVVAL`) when `VBUSDETH` fires to force the core into active device mode.

### SLSTK3701A board setup

The SLSTK3701A starter kit has a 3-position power selector switch (BAT / USB / AEM):

- **AEM** (default): MCU powered from debug USB via LDO. VBUS from Micro-AB does NOT reach MCU.
- **USB**: MCU powered from Micro-AB via R5V regulator. Required for USB device mode.
- **BAT**: MCU powered from CR2032 coin cell.

For USB device mode: set switch to **USB**, connect debug USB (Type-C) for flashing,
and connect a second cable to the **USB Micro-AB** connector.

## EFM32GG11B-specific Init Sequence

1. Wait for `EMU->R5VSYNC.OUTLEVELBUSY = 0`, set `R5VOUTLEVEL = 10` (5 V)
2. Enable USHFRCO, select as USB clock via `CMU.USBCTRL`, enable clock recovery
3. Enable USB on HFBUS: `HFBUSCLKEN0.USB = 1`
4. `USB->CTRL`: `LEMOSCCTRL = GATE`, `LEMIDLEEN`, `LEMPHYCTRL`
5. `USB->ROUTE`: `PHYPEN + VBUSENPEN`
6. Clear `PCGCCTL` power/clock gating bits
7. Soft-reset: `GRSTCTL.CSFTRST`, wait `AHBIDLE`
8. Set `DATTRIM1.ENDLYPULLUP` (GG11B-specific D+ timing)
9. Force device mode: `GUSBCFG.FORCEDEVMODE`, `USBTRDTIM = 9`, wait 50 ms
10. `DCFG.DEVSPD = FS`, `NZSTSOUTHSHK = 1`
11. `GAHBCFG.GLBLINTRMSK = 1` (slave mode, no DMA)
12. `DCTL.IGNRFRMNUM = 1`
13. Allocate FIFOs (RX, TX0, TX1, TX2), flush all
14. Clear EP interrupt masks and flags
15. Connect: clear `DCTL.SFTDISCON`
16. Enable VBUS detect: `USB->IEN = VBUSDETH | VBUSDETL`, force-trigger via `IFS`

Then in the VBUSDETH handler:
- Set `R5VOUTLEVEL = 10`
- Set GOTGCTL overrides (BVALIDOV + VBVALIDOV)
- Toggle SFTDISCON to re-assert D+ pull-up
- Enable `GINTMSK`: USBRST + USBSUSP

## SETUP Packet Flow (Slave Mode)

Identical to EFM32HG — the DWC2 core operates the same way in slave mode:

```
Host sends SETUP token + 8 data bytes
  |
  v
GINTSTS.RXFLVL fires
  +-- Read GRXSTSP: pktsts = SETUP_DATA_RECVD (0x6), bcnt = 8
  +-- Read 2 words from RX FIFO -> parse SetupPacket
  v
GINTSTS.RXFLVL fires again
  +-- Read GRXSTSP: pktsts = SETUP_COMPL (0x4)
  v
GINTSTS.OEPINT fires -> DOEP0INT.SETUP set
  +-- Dispatch to UsbClass::handle_setup()
  +-- Handled  -> ZLP status     DataIn  -> class wrote response
      DataOut  -> prepare OUT     Unhandled -> STALL
```

## IN Transfer Flow

```
Set DIEPnTSIZ (xfersize, pktcnt=1)
Set DIEPnCTL: EPENA + CNAK [+ frame parity for ISO]
Write data words to FIFO[ep]
  --> DWC2 sends on next IN token
  --> GINTSTS.IEPINT -> DIEPnINT.XFERCOMPL
  --> For EP0: send next chunk if multi-packet
  --> For EPn: call UsbClass::in_complete(ep)
```

## OUT Transfer Flow

```
Set DOEPnTSIZ (xfersize=MPS, pktcnt=1)
Set DOEPnCTL: EPENA + CNAK
  --> Host sends OUT token + data
  --> GINTSTS.RXFLVL -> read GRXSTSP (pktsts=0x2), read FIFO
  --> GINTSTS.OEPINT -> DOEPnINT.XFERCOMPL
  --> Call UsbClass::data_out(ep, data)
```

## Architecture

```
UsbDevice<C: UsbClass>
  +-- UsbBus           (FIFO read/write, EP control)
  +-- C                (class driver: CDC, HID, MIDI, ...)
  +-- ep0 state        (setup buffer, multi-packet IN pointer)

USB ISR -> poll():
  1. Check USB->IF for VBUSDETH/VBUSDETL (wrapper interrupts)
  2. Check GINTSTS for DWC2 core interrupts:
     RXFLVL  -> read GRXSTSP, read FIFO, buffer SETUP/OUT data
     OEPINT  -> process buffered SETUP, dispatch to class
     IEPINT  -> handle IN completions, multi-packet EP0
     USBRST  -> reset state, DATTRIM1, re-enumerate
     ENUMDONE -> configure EP0, activate endpoints, set full GINTMSK
     USBSUSP -> notify class
```
