# efm32gg11b-usb — DWC2 USB Device Driver for EFM32GG11B

Bare-metal USB device driver for the EFM32 Giant Gecko 11 (Cortex-M4F, EFM32GG11B820F2048GL192).
Uses the on-chip DWC2 OTG core. Ported from `efm32hg322_usb`.

## Features

| Feature | Default | Description |
|---|---|---|
| `dma` | **yes** | Buffer DMA mode — the DWC2 core moves data between RAM and FIFOs autonomously, freeing the CPU during transfers. Uses ~4 KB of static DMA buffers. |

Without `dma`, the driver operates in slave (FIFO) mode: the CPU reads/writes
the DWC2 FIFOs directly inside the ISR.  This saves the static buffer memory
at the cost of higher interrupt overhead.

```toml
# Slave (FIFO) mode — saves ~4 KB of static RAM
efm32gg11b-usb = { version = "0.0.1", default-features = false }
```

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
| DMA support | Yes — Buffer DMA mode via GAHBCFG.DMAEN |
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

```mermaid
sequenceDiagram
    participant FW as Firmware
    participant EMU as EMU (Power)
    participant CMU as CMU (Clocks)
    participant USB as USB / DWC2

    FW->>EMU: Wait R5VSYNC.OUTLEVELBUSY = 0
    FW->>EMU: R5VOUTLEVEL = 10 (5 V)
    FW->>CMU: Enable USHFRCO, wait USHFRCORDY
    FW->>CMU: USBCTRL.USBCLKSEL = USHFRCO, USBCLKEN
    FW->>CMU: USBCRCTRL.USBCREN (SOF clock recovery)
    FW->>CMU: HFBUSCLKEN0.USB = 1
    FW->>USB: CTRL: LEMOSCCTRL = GATE, LEMIDLEEN, LEMPHYCTRL
    FW->>USB: ROUTE = PHYPEN | VBUSENPEN
    FW->>USB: Clear PCGCCTL
    FW->>USB: GRSTCTL.CSFTRST (soft-reset)
    USB-->>FW: GRSTCTL.AHBIDLE
    FW->>USB: DATTRIM1.ENDLYPULLUP (GG11B D+ timing)
    FW->>USB: GUSBCFG.FORCEDEVMODE, USBTRDTIM = 9
    Note over FW: Wait 50 ms
    FW->>USB: DCFG.DEVSPD = FS, NZSTSOUTHSHK = 1
    FW->>USB: GAHBCFG: GLBLINTRMSK, DMAEN, HBSTLEN = INCR4
    FW->>USB: DCTL.IGNRFRMNUM = 1
    FW->>USB: Allocate FIFOs (RX, TX0–TX3), flush all
    FW->>USB: Clear EP interrupt masks and flags
    FW->>USB: Clear DCTL.SFTDISCON (connect)
    FW->>USB: IEN = VBUSDETH | VBUSDETL
    FW->>USB: Force-trigger VBUSDETH/VBUSDETL via IFS
```

Then in the VBUSDETH handler:

```mermaid
sequenceDiagram
    participant ISR as poll()
    participant EMU as EMU
    participant USB as USB / DWC2

    USB->>ISR: IF.VBUSDETH
    ISR->>EMU: R5VOUTLEVEL = 10 (5 V)
    ISR->>USB: GOTGCTL: BVALIDOV + VBVALIDOV overrides
    ISR->>USB: Toggle SFTDISCON (re-assert D+ pull-up)
    ISR->>USB: GINTMSK = USBRST + USBSUSP
```

## SETUP Packet Flow — FIFO (Slave) Mode

```mermaid
sequenceDiagram
    participant Host
    participant DWC2
    participant ISR as poll()
    participant Class as UsbClass

    Host->>DWC2: SETUP token + 8 data bytes
    DWC2->>ISR: GINTSTS.RXFLVL
    ISR->>DWC2: Read GRXSTSP (pktsts=0x6 SETUP_DATA)
    ISR->>DWC2: Read 2 words from RX FIFO
    Note over ISR: Parse SetupPacket

    DWC2->>ISR: GINTSTS.RXFLVL (pktsts=0x4 SETUP_COMPL)
    ISR->>DWC2: ep0_prepare_out()

    DWC2->>ISR: GINTSTS.OEPINT (DOEP0INT.SETUP)
    Note over ISR: Ignored (already handled in RXFLVL)

    ISR->>Class: handle_setup()

    alt Handled (e.g. SET_ADDRESS)
        ISR->>DWC2: ep0_write_packet(ZLP)
    else DataIn (e.g. GET_DESCRIPTOR)
        Class->>DWC2: ep0_write_packet(data)
    else DataOut (e.g. SET_LINE_CODING)
        ISR->>DWC2: ep0_prepare_out()
        Host->>DWC2: DATA OUT payload
        DWC2->>ISR: RXFLVL (pktsts=0x2 OUT_DATA)
        DWC2->>ISR: OEPINT.XFERCOMPL
        ISR->>Class: ep0_data_out(data)
        ISR->>DWC2: ep0_write_packet(ZLP)
    else Unhandled
        ISR->>DWC2: STALL EP0
    end
```

## SETUP Packet Flow — DMA Mode

EP0 OUT is always armed with **SNAK** so that only SETUP packets (which
bypass NAK on the DWC2) can arrive.  DATA OUT is gated by an explicit
`ep0_clear_out_nak()` call after the SETUP has been read and processed,
preventing a host DATA OUT from overwriting the SETUP data in the shared
DMA buffer before the ISR can parse it.

```mermaid
sequenceDiagram
    participant Host
    participant DWC2
    participant ISR as poll()
    participant Class as UsbClass

    Note over DWC2: EP0 OUT armed with SNAK<br/>(SETUP bypasses NAK)
    Host->>DWC2: SETUP token + 8 data bytes
    Note over DWC2: DMA writes 8 bytes to EP0 OUT buffer

    DWC2->>ISR: OEPINT (setup + xfercompl both set)
    ISR->>DWC2: Read SETUP from DMA buffer
    Note over ISR: Parse SetupPacket
    ISR->>Class: handle_setup()

    Note over ISR: xfercompl is from SETUP DMA,<br/>not a DATA OUT payload

    alt Handled
        ISR->>DWC2: ep0_write_packet(ZLP)
    else DataIn
        Class->>DWC2: ep0_write_packet(data)
    else DataOut
        Note over ISR: setup bit set → skip xfercompl
        ISR->>DWC2: ep0_prepare_out() [SNAK]
        ISR->>DWC2: ep0_clear_out_nak() [CNAK]
        Note over DWC2: NAK cleared — DATA OUT can arrive
        Host->>DWC2: DATA OUT payload
        Note over DWC2: DMA writes payload to EP0 OUT buffer
        DWC2->>ISR: OEPINT.XFERCOMPL (setup NOT set)
        ISR->>DWC2: Read actual byte count from DOEP0TSIZ
        ISR->>Class: ep0_data_out(data)
        ISR->>DWC2: ep0_write_packet(ZLP)
    else Unhandled
        ISR->>DWC2: STALL EP0
    end
```

## Multi-Packet EP0 IN Transfer

`DIEP0TSIZ.xfersize` is only 7 bits wide (max 127), so descriptors
larger than 64 bytes must be sent one packet at a time in both modes.

```mermaid
sequenceDiagram
    participant FW as ep0_start_in
    participant Bus as UsbBus
    participant Host

    FW->>Bus: ep0_write_packet(data[0..64])
    Note over FW: Save pointer + remaining

    loop While ep0_in_remaining > 0
        Bus-->>Host: 64-byte DATA packet
        Host-->>Bus: ACK
        Bus->>FW: IEPINT.XFERCOMPL
        FW->>Bus: ep0_continue_in → ep0_write_packet(next chunk)
    end

    Bus-->>Host: Final packet (may be short / ZLP)
    Host-->>Bus: ACK
    Bus->>FW: IEPINT.XFERCOMPL

    alt FIFO mode
        FW->>Bus: ep0_prepare_out() [CNAK]
    else DMA mode
        FW->>Bus: ep0_prepare_out() [SNAK]
        FW->>Bus: ep0_clear_out_nak() [CNAK]
        Note over FW: CNAK after prepare so status<br/>ZLP / next SETUP can arrive
    end
```

## IN Transfer Flow (EPn)

Isochronous IN endpoints are activated with `usbactep` deferred: the bit
is **not** set in `activate_endpoints()` so the DWC2 NAKs host IN tokens
until the first `ep_write()` supplies real data.  This prevents the
controller from transmitting stale/zero data (which appears as a green
flash in YUY2 video).

```mermaid
sequenceDiagram
    participant Class as UsbClass
    participant Bus as UsbBus
    participant DWC2
    participant Host

    Note over Bus: Iso IN: usbactep deferred<br/>until first ep_write

    Class->>Bus: ep_write(ep, data)

    alt DMA mode
        Note over Bus: Copy data to DMA buffer
        Note over Bus: DSB (flush write buffer)
        Bus->>DWC2: DIEPnDMAADDR
    else FIFO mode
        Note over Bus: (data written to TX FIFO after EPENA)
    end

    Bus->>DWC2: DIEPnTSIZ + DIEPnCTL (USBACTEP, EPENA, CNAK)
    Host->>DWC2: IN token
    DWC2-->>Host: Data packet
    DWC2->>Class: IEPINT.XFERCOMPL → in_complete(ep)
```

## OUT Transfer Flow (EPn)

Each endpoint's `EpConfig.out_max_xfer` controls how the OUT side is armed:

| `out_max_xfer` | Behaviour | Use case |
|---|---|---|
| `0` | Single-packet: `pktcnt=1, xfersize=MPS` | CDC-ACM, MSC, MIDI — one packet per XFERCOMPL |
| `> 0` | Multi-packet: `pktcnt=max_xfer/MPS, xfersize=max_xfer` | CDC-ECM — receive a full Ethernet frame (up to 1536 B) in one DMA transfer |

Multi-packet mode eliminates per-packet NAK/ISR round-trips that otherwise
add ~1 ms latency per 64-byte packet at Full Speed (one USB frame per NAK).
The transfer completes when the host sends a short packet or ZLP.

```mermaid
sequenceDiagram
    participant Class as UsbClass
    participant Bus as UsbBus
    participant DWC2
    participant Host

    Bus->>DWC2: DOEPnTSIZ + DOEPnCTL (EPENA, CNAK)
    Note over DWC2: EP armed for reception

    Host->>DWC2: OUT token + data

    alt FIFO mode (always single-packet)
        DWC2->>Bus: RXFLVL (pktsts=0x2)
        Bus->>Bus: Read data from RX FIFO
        Bus->>Class: data_out(ep, data)
        DWC2->>Bus: OEPINT.XFERCOMPL
        Bus->>DWC2: Re-arm EP (ep_prepare_out)
    else DMA mode — single-packet (out_max_xfer = 0)
        Note over DWC2: pktcnt=1, xfersize=MPS
        Note over DWC2: DMA writes to EPn OUT buffer
        DWC2->>Bus: OEPINT.XFERCOMPL
        Bus->>Bus: len = MPS − remaining
        Bus->>Bus: DSB (flush DMA writes)
        Bus->>Class: data_out(ep, data)
        Bus->>DWC2: Re-arm EP (ep_prepare_out)
    else DMA mode — multi-packet (out_max_xfer > 0)
        Note over DWC2: pktcnt=N, xfersize=out_max_xfer
        loop N packets (MPS each)
            Host->>DWC2: OUT token + 64 B
            Note over DWC2: DMA appends to buffer, pktcnt−−
        end
        Host->>DWC2: Short packet or ZLP (terminates transfer)
        DWC2->>Bus: OEPINT.XFERCOMPL
        Bus->>Bus: len = out_max_xfer − remaining
        Bus->>Bus: DSB (flush DMA writes)
        Bus->>Class: data_out(ep, data)
        Bus->>DWC2: Re-arm EP (ep_prepare_out)
    end
```

## Architecture

```mermaid
graph TD
    subgraph "USB ISR (poll)"
        VBUS["VBUSDETH / VBUSDETL<br/><i>wrapper interrupts</i>"]
        GINTSTS["GINTSTS dispatch"]
        RXFLVL["RXFLVL<br/><i>FIFO mode only</i>"]
        OEPINT["OEPINT<br/>SETUP + OUT completion"]
        IEPINT["IEPINT<br/>IN completion"]
        RST["USBRST / ENUMDONE"]
        SUSP["USBSUSP"]
    end

    subgraph "UsbDevice&lt;C: UsbClass&gt;"
        STATE["EP0 state<br/>(pending_data_out,<br/>ep0_in_ptr, ep0_in_remaining)"]
        SETUP["handle_setup()"]
    end

    subgraph UsbBus
        direction LR
        EP0W["ep0_write_packet"]
        EP0P["ep0_prepare_out"]
        EPW["ep_write"]
        EPP["ep_prepare_out"]
    end

    subgraph "UsbClass (trait)"
        CDC["CdcAcmClass"]
        CDCECM["CdcEcmClass"]
        HID["HidKeyboardClass"]
        VIDEO["VideoClass"]
        MIDI["MidiClass"]
        AUDIO["AudioClass"]
        MSC["MscClass"]
    end

    VBUS --> GINTSTS
    GINTSTS --> RXFLVL
    GINTSTS --> OEPINT
    GINTSTS --> IEPINT
    GINTSTS --> RST
    GINTSTS --> SUSP

    OEPINT --> SETUP
    SETUP --> EP0W
    SETUP --> EP0P

    OEPINT -->|"data_out()"| CDC
    IEPINT -->|"in_complete()"| CDC

    EP0W -->|"FIFO mode"| FIFO["write_fifo()"]
    EP0W -->|"DMA mode"| DMA["DMA buffer + DSB"]
    EPW -->|"FIFO mode"| FIFO
    EPW -->|"DMA mode"| DMA
```
