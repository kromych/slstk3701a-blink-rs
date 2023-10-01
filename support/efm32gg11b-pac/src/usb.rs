#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - System Status Register"]
    pub status: STATUS,
    #[doc = "0x08 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x0c - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x10 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x14 - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x18 - I/O Routing Register"]
    pub route: ROUTE,
    _reserved7: [u8; 0x10],
    #[doc = "0x2c - Charger Detect Configuration Register"]
    pub cdconf: CDCONF,
    #[doc = "0x30 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x34 - Data TRIM 1 Values for USB DP and DM"]
    pub dattrim1: DATTRIM1,
    _reserved10: [u8; 0x04],
    #[doc = "0x3c - USB LEM Control Register"]
    pub lemctrl: LEMCTRL,
    _reserved11: [u8; 0x000d_dfc0],
    #[doc = "0xde000 - OTG Control and Status Register"]
    pub gotgctl: GOTGCTL,
    #[doc = "0xde004 - OTG Interrupt Register"]
    pub gotgint: GOTGINT,
    #[doc = "0xde008 - AHB Configuration Register"]
    pub gahbcfg: GAHBCFG,
    #[doc = "0xde00c - USB Configuration Register"]
    pub gusbcfg: GUSBCFG,
    #[doc = "0xde010 - Reset Register"]
    pub grstctl: GRSTCTL,
    #[doc = "0xde014 - Interrupt Register"]
    pub gintsts: GINTSTS,
    #[doc = "0xde018 - Interrupt Mask Register"]
    pub gintmsk: GINTMSK,
    #[doc = "0xde01c - Receive Status Debug Read Register"]
    pub grxstsr: GRXSTSR,
    #[doc = "0xde020 - Receive Status Read /Pop Register"]
    pub grxstsp: GRXSTSP,
    #[doc = "0xde024 - Receive FIFO Size Register"]
    pub grxfsiz: GRXFSIZ,
    #[doc = "0xde028 - Non-periodic Transmit FIFO Size Register"]
    pub gnptxfsiz: GNPTXFSIZ,
    #[doc = "0xde02c - Non-periodic Transmit FIFO/Queue Status Register"]
    pub gnptxsts: GNPTXSTS,
    _reserved23: [u8; 0x10],
    #[doc = "0xde040 - Synopsys ID Register"]
    pub gsnpsid: GSNPSID,
    _reserved24: [u8; 0x18],
    #[doc = "0xde05c - Global DFIFO Configuration Register"]
    pub gdfifocfg: GDFIFOCFG,
    _reserved25: [u8; 0xa0],
    #[doc = "0xde100 - Host Periodic Transmit FIFO Size Register"]
    pub hptxfsiz: HPTXFSIZ,
    #[doc = "0xde104 - Device IN Endpoint Transmit FIFO Size Register 1"]
    pub dieptxf1: DIEPTXF1,
    #[doc = "0xde108 - Device IN Endpoint Transmit FIFO Size Register 2"]
    pub dieptxf2: DIEPTXF2,
    #[doc = "0xde10c - Device IN Endpoint Transmit FIFO Size Register 3"]
    pub dieptxf3: DIEPTXF3,
    #[doc = "0xde110 - Device IN Endpoint Transmit FIFO Size Register 4"]
    pub dieptxf4: DIEPTXF4,
    #[doc = "0xde114 - Device IN Endpoint Transmit FIFO Size Register 5"]
    pub dieptxf5: DIEPTXF5,
    #[doc = "0xde118 - Device IN Endpoint Transmit FIFO Size Register 6"]
    pub dieptxf6: DIEPTXF6,
    _reserved32: [u8; 0x02e4],
    #[doc = "0xde400 - Host Configuration Register"]
    pub hcfg: HCFG,
    #[doc = "0xde404 - Host Frame Interval Register"]
    pub hfir: HFIR,
    #[doc = "0xde408 - Host Frame Number/Frame Time Remaining Register"]
    pub hfnum: HFNUM,
    _reserved35: [u8; 0x04],
    #[doc = "0xde410 - Host Periodic Transmit FIFO/Queue Status Register"]
    pub hptxsts: HPTXSTS,
    #[doc = "0xde414 - Host All Channels Interrupt Register"]
    pub haint: HAINT,
    #[doc = "0xde418 - Host All Channels Interrupt Mask Register"]
    pub haintmsk: HAINTMSK,
    _reserved38: [u8; 0x24],
    #[doc = "0xde440 - Host Port Control and Status Register"]
    pub hprt: HPRT,
    _reserved39: [u8; 0xbc],
    #[doc = "0xde500 - Host Channel x Characteristics Register"]
    pub hc0_char: HC0_CHAR,
    #[doc = "0xde504 - Host Channel x Split Control Register"]
    pub hc0_splt: HC0_SPLT,
    #[doc = "0xde508 - Host Channel x Interrupt Register"]
    pub hc0_int: HC0_INT,
    #[doc = "0xde50c - Host Channel x Interrupt Mask Register"]
    pub hc0_intmsk: HC0_INTMSK,
    #[doc = "0xde510 - Host Channel x Transfer Size Register"]
    pub hc0_tsiz: HC0_TSIZ,
    #[doc = "0xde514 - Host Channel x DMA Address Register"]
    pub hc0_dmaaddr: HC0_DMAADDR,
    _reserved45: [u8; 0x08],
    #[doc = "0xde520 - Host Channel x Characteristics Register"]
    pub hc1_char: HC1_CHAR,
    #[doc = "0xde524 - Host Channel x Split Control Register"]
    pub hc1_splt: HC1_SPLT,
    #[doc = "0xde528 - Host Channel x Interrupt Register"]
    pub hc1_int: HC1_INT,
    #[doc = "0xde52c - Host Channel x Interrupt Mask Register"]
    pub hc1_intmsk: HC1_INTMSK,
    #[doc = "0xde530 - Host Channel x Transfer Size Register"]
    pub hc1_tsiz: HC1_TSIZ,
    #[doc = "0xde534 - Host Channel x DMA Address Register"]
    pub hc1_dmaaddr: HC1_DMAADDR,
    _reserved51: [u8; 0x08],
    #[doc = "0xde540 - Host Channel x Characteristics Register"]
    pub hc2_char: HC2_CHAR,
    #[doc = "0xde544 - Host Channel x Split Control Register"]
    pub hc2_splt: HC2_SPLT,
    #[doc = "0xde548 - Host Channel x Interrupt Register"]
    pub hc2_int: HC2_INT,
    #[doc = "0xde54c - Host Channel x Interrupt Mask Register"]
    pub hc2_intmsk: HC2_INTMSK,
    #[doc = "0xde550 - Host Channel x Transfer Size Register"]
    pub hc2_tsiz: HC2_TSIZ,
    #[doc = "0xde554 - Host Channel x DMA Address Register"]
    pub hc2_dmaaddr: HC2_DMAADDR,
    _reserved57: [u8; 0x08],
    #[doc = "0xde560 - Host Channel x Characteristics Register"]
    pub hc3_char: HC3_CHAR,
    #[doc = "0xde564 - Host Channel x Split Control Register"]
    pub hc3_splt: HC3_SPLT,
    #[doc = "0xde568 - Host Channel x Interrupt Register"]
    pub hc3_int: HC3_INT,
    #[doc = "0xde56c - Host Channel x Interrupt Mask Register"]
    pub hc3_intmsk: HC3_INTMSK,
    #[doc = "0xde570 - Host Channel x Transfer Size Register"]
    pub hc3_tsiz: HC3_TSIZ,
    #[doc = "0xde574 - Host Channel x DMA Address Register"]
    pub hc3_dmaaddr: HC3_DMAADDR,
    _reserved63: [u8; 0x08],
    #[doc = "0xde580 - Host Channel x Characteristics Register"]
    pub hc4_char: HC4_CHAR,
    #[doc = "0xde584 - Host Channel x Split Control Register"]
    pub hc4_splt: HC4_SPLT,
    #[doc = "0xde588 - Host Channel x Interrupt Register"]
    pub hc4_int: HC4_INT,
    #[doc = "0xde58c - Host Channel x Interrupt Mask Register"]
    pub hc4_intmsk: HC4_INTMSK,
    #[doc = "0xde590 - Host Channel x Transfer Size Register"]
    pub hc4_tsiz: HC4_TSIZ,
    #[doc = "0xde594 - Host Channel x DMA Address Register"]
    pub hc4_dmaaddr: HC4_DMAADDR,
    _reserved69: [u8; 0x08],
    #[doc = "0xde5a0 - Host Channel x Characteristics Register"]
    pub hc5_char: HC5_CHAR,
    #[doc = "0xde5a4 - Host Channel x Split Control Register"]
    pub hc5_splt: HC5_SPLT,
    #[doc = "0xde5a8 - Host Channel x Interrupt Register"]
    pub hc5_int: HC5_INT,
    #[doc = "0xde5ac - Host Channel x Interrupt Mask Register"]
    pub hc5_intmsk: HC5_INTMSK,
    #[doc = "0xde5b0 - Host Channel x Transfer Size Register"]
    pub hc5_tsiz: HC5_TSIZ,
    #[doc = "0xde5b4 - Host Channel x DMA Address Register"]
    pub hc5_dmaaddr: HC5_DMAADDR,
    _reserved75: [u8; 0x08],
    #[doc = "0xde5c0 - Host Channel x Characteristics Register"]
    pub hc6_char: HC6_CHAR,
    #[doc = "0xde5c4 - Host Channel x Split Control Register"]
    pub hc6_splt: HC6_SPLT,
    #[doc = "0xde5c8 - Host Channel x Interrupt Register"]
    pub hc6_int: HC6_INT,
    #[doc = "0xde5cc - Host Channel x Interrupt Mask Register"]
    pub hc6_intmsk: HC6_INTMSK,
    #[doc = "0xde5d0 - Host Channel x Transfer Size Register"]
    pub hc6_tsiz: HC6_TSIZ,
    #[doc = "0xde5d4 - Host Channel x DMA Address Register"]
    pub hc6_dmaaddr: HC6_DMAADDR,
    _reserved81: [u8; 0x08],
    #[doc = "0xde5e0 - Host Channel x Characteristics Register"]
    pub hc7_char: HC7_CHAR,
    #[doc = "0xde5e4 - Host Channel x Split Control Register"]
    pub hc7_splt: HC7_SPLT,
    #[doc = "0xde5e8 - Host Channel x Interrupt Register"]
    pub hc7_int: HC7_INT,
    #[doc = "0xde5ec - Host Channel x Interrupt Mask Register"]
    pub hc7_intmsk: HC7_INTMSK,
    #[doc = "0xde5f0 - Host Channel x Transfer Size Register"]
    pub hc7_tsiz: HC7_TSIZ,
    #[doc = "0xde5f4 - Host Channel x DMA Address Register"]
    pub hc7_dmaaddr: HC7_DMAADDR,
    _reserved87: [u8; 0x08],
    #[doc = "0xde600 - Host Channel x Characteristics Register"]
    pub hc8_char: HC8_CHAR,
    #[doc = "0xde604 - Host Channel x Split Control Register"]
    pub hc8_splt: HC8_SPLT,
    #[doc = "0xde608 - Host Channel x Interrupt Register"]
    pub hc8_int: HC8_INT,
    #[doc = "0xde60c - Host Channel x Interrupt Mask Register"]
    pub hc8_intmsk: HC8_INTMSK,
    #[doc = "0xde610 - Host Channel x Transfer Size Register"]
    pub hc8_tsiz: HC8_TSIZ,
    #[doc = "0xde614 - Host Channel x DMA Address Register"]
    pub hc8_dmaaddr: HC8_DMAADDR,
    _reserved93: [u8; 0x08],
    #[doc = "0xde620 - Host Channel x Characteristics Register"]
    pub hc9_char: HC9_CHAR,
    #[doc = "0xde624 - Host Channel x Split Control Register"]
    pub hc9_splt: HC9_SPLT,
    #[doc = "0xde628 - Host Channel x Interrupt Register"]
    pub hc9_int: HC9_INT,
    #[doc = "0xde62c - Host Channel x Interrupt Mask Register"]
    pub hc9_intmsk: HC9_INTMSK,
    #[doc = "0xde630 - Host Channel x Transfer Size Register"]
    pub hc9_tsiz: HC9_TSIZ,
    #[doc = "0xde634 - Host Channel x DMA Address Register"]
    pub hc9_dmaaddr: HC9_DMAADDR,
    _reserved99: [u8; 0x08],
    #[doc = "0xde640 - Host Channel x Characteristics Register"]
    pub hc10_char: HC10_CHAR,
    #[doc = "0xde644 - Host Channel x Split Control Register"]
    pub hc10_splt: HC10_SPLT,
    #[doc = "0xde648 - Host Channel x Interrupt Register"]
    pub hc10_int: HC10_INT,
    #[doc = "0xde64c - Host Channel x Interrupt Mask Register"]
    pub hc10_intmsk: HC10_INTMSK,
    #[doc = "0xde650 - Host Channel x Transfer Size Register"]
    pub hc10_tsiz: HC10_TSIZ,
    #[doc = "0xde654 - Host Channel x DMA Address Register"]
    pub hc10_dmaaddr: HC10_DMAADDR,
    _reserved105: [u8; 0x08],
    #[doc = "0xde660 - Host Channel x Characteristics Register"]
    pub hc11_char: HC11_CHAR,
    #[doc = "0xde664 - Host Channel x Split Control Register"]
    pub hc11_splt: HC11_SPLT,
    #[doc = "0xde668 - Host Channel x Interrupt Register"]
    pub hc11_int: HC11_INT,
    #[doc = "0xde66c - Host Channel x Interrupt Mask Register"]
    pub hc11_intmsk: HC11_INTMSK,
    #[doc = "0xde670 - Host Channel x Transfer Size Register"]
    pub hc11_tsiz: HC11_TSIZ,
    #[doc = "0xde674 - Host Channel x DMA Address Register"]
    pub hc11_dmaaddr: HC11_DMAADDR,
    _reserved111: [u8; 0x08],
    #[doc = "0xde680 - Host Channel x Characteristics Register"]
    pub hc12_char: HC12_CHAR,
    #[doc = "0xde684 - Host Channel x Split Control Register"]
    pub hc12_splt: HC12_SPLT,
    #[doc = "0xde688 - Host Channel x Interrupt Register"]
    pub hc12_int: HC12_INT,
    #[doc = "0xde68c - Host Channel x Interrupt Mask Register"]
    pub hc12_intmsk: HC12_INTMSK,
    #[doc = "0xde690 - Host Channel x Transfer Size Register"]
    pub hc12_tsiz: HC12_TSIZ,
    #[doc = "0xde694 - Host Channel x DMA Address Register"]
    pub hc12_dmaaddr: HC12_DMAADDR,
    _reserved117: [u8; 0x08],
    #[doc = "0xde6a0 - Host Channel x Characteristics Register"]
    pub hc13_char: HC13_CHAR,
    #[doc = "0xde6a4 - Host Channel x Split Control Register"]
    pub hc13_splt: HC13_SPLT,
    #[doc = "0xde6a8 - Host Channel x Interrupt Register"]
    pub hc13_int: HC13_INT,
    #[doc = "0xde6ac - Host Channel x Interrupt Mask Register"]
    pub hc13_intmsk: HC13_INTMSK,
    #[doc = "0xde6b0 - Host Channel x Transfer Size Register"]
    pub hc13_tsiz: HC13_TSIZ,
    #[doc = "0xde6b4 - Host Channel x DMA Address Register"]
    pub hc13_dmaaddr: HC13_DMAADDR,
    _reserved123: [u8; 0x0148],
    #[doc = "0xde800 - Device Configuration Register"]
    pub dcfg: DCFG,
    #[doc = "0xde804 - Device Control Register"]
    pub dctl: DCTL,
    #[doc = "0xde808 - Device Status Register"]
    pub dsts: DSTS,
    _reserved126: [u8; 0x04],
    #[doc = "0xde810 - Device IN Endpoint Common Interrupt Mask Register"]
    pub diepmsk: DIEPMSK,
    #[doc = "0xde814 - Device OUT Endpoint Common Interrupt Mask Register"]
    pub doepmsk: DOEPMSK,
    #[doc = "0xde818 - Device All Endpoints Interrupt Register"]
    pub daint: DAINT,
    #[doc = "0xde81c - Device All Endpoints Interrupt Mask Register"]
    pub daintmsk: DAINTMSK,
    _reserved130: [u8; 0x08],
    #[doc = "0xde828 - Device VBUS Discharge Time Register"]
    pub dvbusdis: DVBUSDIS,
    #[doc = "0xde82c - Device VBUS Pulsing Time Register"]
    pub dvbuspulse: DVBUSPULSE,
    #[doc = "0xde830 - Device Threshold Control Register"]
    pub dthrctl: DTHRCTL,
    #[doc = "0xde834 - Device IN Endpoint FIFO Empty Interrupt Mask Register"]
    pub diepempmsk: DIEPEMPMSK,
    _reserved134: [u8; 0xc8],
    #[doc = "0xde900 - Device Control IN Endpoint 0 Control Register"]
    pub diep0ctl: DIEP0CTL,
    _reserved135: [u8; 0x04],
    #[doc = "0xde908 - Device IN Endpoint 0 Interrupt Register"]
    pub diep0int: DIEP0INT,
    _reserved136: [u8; 0x04],
    #[doc = "0xde910 - Device IN Endpoint 0 Transfer Size Register"]
    pub diep0tsiz: DIEP0TSIZ,
    #[doc = "0xde914 - Device IN Endpoint 0 DMA Address Register"]
    pub diep0dmaaddr: DIEP0DMAADDR,
    #[doc = "0xde918 - Device IN Endpoint Transmit FIFO Status Register 0"]
    pub diep0txfsts: DIEP0TXFSTS,
    _reserved139: [u8; 0x04],
    #[doc = "0xde920 - Device Control IN Endpoint x+1 Control Register"]
    pub diep0_ctl: DIEP0_CTL,
    _reserved140: [u8; 0x04],
    #[doc = "0xde928 - Device IN Endpoint x+1 Interrupt Register"]
    pub diep0_int: DIEP0_INT,
    _reserved141: [u8; 0x04],
    #[doc = "0xde930 - Device IN Endpoint x+1 Transfer Size Register"]
    pub diep0_tsiz: DIEP0_TSIZ,
    #[doc = "0xde934 - Device IN Endpoint x+1 DMA Address Register"]
    pub diep0_dmaaddr: DIEP0_DMAADDR,
    #[doc = "0xde938 - Device IN Endpoint Transmit FIFO Status Register 1"]
    pub diep0_dtxfsts: DIEP0_DTXFSTS,
    _reserved144: [u8; 0x04],
    #[doc = "0xde940 - Device Control IN Endpoint x+1 Control Register"]
    pub diep1_ctl: DIEP1_CTL,
    _reserved145: [u8; 0x04],
    #[doc = "0xde948 - Device IN Endpoint x+1 Interrupt Register"]
    pub diep1_int: DIEP1_INT,
    _reserved146: [u8; 0x04],
    #[doc = "0xde950 - Device IN Endpoint x+1 Transfer Size Register"]
    pub diep1_tsiz: DIEP1_TSIZ,
    #[doc = "0xde954 - Device IN Endpoint x+1 DMA Address Register"]
    pub diep1_dmaaddr: DIEP1_DMAADDR,
    #[doc = "0xde958 - Device IN Endpoint Transmit FIFO Status Register 1"]
    pub diep1_dtxfsts: DIEP1_DTXFSTS,
    _reserved149: [u8; 0x04],
    #[doc = "0xde960 - Device Control IN Endpoint x+1 Control Register"]
    pub diep2_ctl: DIEP2_CTL,
    _reserved150: [u8; 0x04],
    #[doc = "0xde968 - Device IN Endpoint x+1 Interrupt Register"]
    pub diep2_int: DIEP2_INT,
    _reserved151: [u8; 0x04],
    #[doc = "0xde970 - Device IN Endpoint x+1 Transfer Size Register"]
    pub diep2_tsiz: DIEP2_TSIZ,
    #[doc = "0xde974 - Device IN Endpoint x+1 DMA Address Register"]
    pub diep2_dmaaddr: DIEP2_DMAADDR,
    #[doc = "0xde978 - Device IN Endpoint Transmit FIFO Status Register 1"]
    pub diep2_dtxfsts: DIEP2_DTXFSTS,
    _reserved154: [u8; 0x04],
    #[doc = "0xde980 - Device Control IN Endpoint x+1 Control Register"]
    pub diep3_ctl: DIEP3_CTL,
    _reserved155: [u8; 0x04],
    #[doc = "0xde988 - Device IN Endpoint x+1 Interrupt Register"]
    pub diep3_int: DIEP3_INT,
    _reserved156: [u8; 0x04],
    #[doc = "0xde990 - Device IN Endpoint x+1 Transfer Size Register"]
    pub diep3_tsiz: DIEP3_TSIZ,
    #[doc = "0xde994 - Device IN Endpoint x+1 DMA Address Register"]
    pub diep3_dmaaddr: DIEP3_DMAADDR,
    #[doc = "0xde998 - Device IN Endpoint Transmit FIFO Status Register 1"]
    pub diep3_dtxfsts: DIEP3_DTXFSTS,
    _reserved159: [u8; 0x04],
    #[doc = "0xde9a0 - Device Control IN Endpoint x+1 Control Register"]
    pub diep4_ctl: DIEP4_CTL,
    _reserved160: [u8; 0x04],
    #[doc = "0xde9a8 - Device IN Endpoint x+1 Interrupt Register"]
    pub diep4_int: DIEP4_INT,
    _reserved161: [u8; 0x04],
    #[doc = "0xde9b0 - Device IN Endpoint x+1 Transfer Size Register"]
    pub diep4_tsiz: DIEP4_TSIZ,
    #[doc = "0xde9b4 - Device IN Endpoint x+1 DMA Address Register"]
    pub diep4_dmaaddr: DIEP4_DMAADDR,
    #[doc = "0xde9b8 - Device IN Endpoint Transmit FIFO Status Register 1"]
    pub diep4_dtxfsts: DIEP4_DTXFSTS,
    _reserved164: [u8; 0x04],
    #[doc = "0xde9c0 - Device Control IN Endpoint x+1 Control Register"]
    pub diep5_ctl: DIEP5_CTL,
    _reserved165: [u8; 0x04],
    #[doc = "0xde9c8 - Device IN Endpoint x+1 Interrupt Register"]
    pub diep5_int: DIEP5_INT,
    _reserved166: [u8; 0x04],
    #[doc = "0xde9d0 - Device IN Endpoint x+1 Transfer Size Register"]
    pub diep5_tsiz: DIEP5_TSIZ,
    #[doc = "0xde9d4 - Device IN Endpoint x+1 DMA Address Register"]
    pub diep5_dmaaddr: DIEP5_DMAADDR,
    #[doc = "0xde9d8 - Device IN Endpoint Transmit FIFO Status Register 1"]
    pub diep5_dtxfsts: DIEP5_DTXFSTS,
    _reserved169: [u8; 0x0124],
    #[doc = "0xdeb00 - Device Control OUT Endpoint 0 Control Register"]
    pub doep0ctl: DOEP0CTL,
    _reserved170: [u8; 0x04],
    #[doc = "0xdeb08 - Device OUT Endpoint 0 Interrupt Register"]
    pub doep0int: DOEP0INT,
    _reserved171: [u8; 0x04],
    #[doc = "0xdeb10 - Device OUT Endpoint 0 Transfer Size Register"]
    pub doep0tsiz: DOEP0TSIZ,
    #[doc = "0xdeb14 - Device OUT Endpoint 0 DMA Address Register"]
    pub doep0dmaaddr: DOEP0DMAADDR,
    _reserved173: [u8; 0x08],
    #[doc = "0xdeb20 - Device Control OUT Endpoint x+1 Control Register"]
    pub doep0_ctl: DOEP0_CTL,
    _reserved174: [u8; 0x04],
    #[doc = "0xdeb28 - Device OUT Endpoint x+1 Interrupt Register"]
    pub doep0_int: DOEP0_INT,
    _reserved175: [u8; 0x04],
    #[doc = "0xdeb30 - Device OUT Endpoint x+1 Transfer Size Register"]
    pub doep0_tsiz: DOEP0_TSIZ,
    #[doc = "0xdeb34 - Device OUT Endpoint x+1 DMA Address Register"]
    pub doep0_dmaaddr: DOEP0_DMAADDR,
    _reserved177: [u8; 0x08],
    #[doc = "0xdeb40 - Device Control OUT Endpoint x+1 Control Register"]
    pub doep1_ctl: DOEP1_CTL,
    _reserved178: [u8; 0x04],
    #[doc = "0xdeb48 - Device OUT Endpoint x+1 Interrupt Register"]
    pub doep1_int: DOEP1_INT,
    _reserved179: [u8; 0x04],
    #[doc = "0xdeb50 - Device OUT Endpoint x+1 Transfer Size Register"]
    pub doep1_tsiz: DOEP1_TSIZ,
    #[doc = "0xdeb54 - Device OUT Endpoint x+1 DMA Address Register"]
    pub doep1_dmaaddr: DOEP1_DMAADDR,
    _reserved181: [u8; 0x08],
    #[doc = "0xdeb60 - Device Control OUT Endpoint x+1 Control Register"]
    pub doep2_ctl: DOEP2_CTL,
    _reserved182: [u8; 0x04],
    #[doc = "0xdeb68 - Device OUT Endpoint x+1 Interrupt Register"]
    pub doep2_int: DOEP2_INT,
    _reserved183: [u8; 0x04],
    #[doc = "0xdeb70 - Device OUT Endpoint x+1 Transfer Size Register"]
    pub doep2_tsiz: DOEP2_TSIZ,
    #[doc = "0xdeb74 - Device OUT Endpoint x+1 DMA Address Register"]
    pub doep2_dmaaddr: DOEP2_DMAADDR,
    _reserved185: [u8; 0x08],
    #[doc = "0xdeb80 - Device Control OUT Endpoint x+1 Control Register"]
    pub doep3_ctl: DOEP3_CTL,
    _reserved186: [u8; 0x04],
    #[doc = "0xdeb88 - Device OUT Endpoint x+1 Interrupt Register"]
    pub doep3_int: DOEP3_INT,
    _reserved187: [u8; 0x04],
    #[doc = "0xdeb90 - Device OUT Endpoint x+1 Transfer Size Register"]
    pub doep3_tsiz: DOEP3_TSIZ,
    #[doc = "0xdeb94 - Device OUT Endpoint x+1 DMA Address Register"]
    pub doep3_dmaaddr: DOEP3_DMAADDR,
    _reserved189: [u8; 0x08],
    #[doc = "0xdeba0 - Device Control OUT Endpoint x+1 Control Register"]
    pub doep4_ctl: DOEP4_CTL,
    _reserved190: [u8; 0x04],
    #[doc = "0xdeba8 - Device OUT Endpoint x+1 Interrupt Register"]
    pub doep4_int: DOEP4_INT,
    _reserved191: [u8; 0x04],
    #[doc = "0xdebb0 - Device OUT Endpoint x+1 Transfer Size Register"]
    pub doep4_tsiz: DOEP4_TSIZ,
    #[doc = "0xdebb4 - Device OUT Endpoint x+1 DMA Address Register"]
    pub doep4_dmaaddr: DOEP4_DMAADDR,
    _reserved193: [u8; 0x08],
    #[doc = "0xdebc0 - Device Control OUT Endpoint x+1 Control Register"]
    pub doep5_ctl: DOEP5_CTL,
    _reserved194: [u8; 0x04],
    #[doc = "0xdebc8 - Device OUT Endpoint x+1 Interrupt Register"]
    pub doep5_int: DOEP5_INT,
    _reserved195: [u8; 0x04],
    #[doc = "0xdebd0 - Device OUT Endpoint x+1 Transfer Size Register"]
    pub doep5_tsiz: DOEP5_TSIZ,
    #[doc = "0xdebd4 - Device OUT Endpoint x+1 DMA Address Register"]
    pub doep5_dmaaddr: DOEP5_DMAADDR,
    _reserved197: [u8; 0x0228],
    #[doc = "0xdee00 - Power and Clock Gating Control Register"]
    pub pcgcctl: PCGCCTL,
}
#[doc = "CTRL (rw) register accessor: System Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "System Control Register"]
pub mod ctrl;
#[doc = "STATUS (r) register accessor: System Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`status`] module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "System Status Register"]
pub mod status;
#[doc = "IF (r) register accessor: Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`if_`] module"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "Interrupt Flag Register"]
pub mod if_;
#[doc = "IFS (w) register accessor: Interrupt Flag Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifs::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ifs`] module"]
pub type IFS = crate::Reg<ifs::IFS_SPEC>;
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "IFC (w) register accessor: Interrupt Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ifc`] module"]
pub type IFC = crate::Reg<ifc::IFC_SPEC>;
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "IEN (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ien::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ien::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ien`] module"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ien;
#[doc = "ROUTE (rw) register accessor: I/O Routing Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`route::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`route::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`route`] module"]
pub type ROUTE = crate::Reg<route::ROUTE_SPEC>;
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "CDCONF (rw) register accessor: Charger Detect Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdconf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdconf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cdconf`] module"]
pub type CDCONF = crate::Reg<cdconf::CDCONF_SPEC>;
#[doc = "Charger Detect Configuration Register"]
pub mod cdconf;
#[doc = "CMD (w) register accessor: Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmd`] module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "DATTRIM1 (rw) register accessor: Data TRIM 1 Values for USB DP and DM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dattrim1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dattrim1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dattrim1`] module"]
pub type DATTRIM1 = crate::Reg<dattrim1::DATTRIM1_SPEC>;
#[doc = "Data TRIM 1 Values for USB DP and DM"]
pub mod dattrim1;
#[doc = "LEMCTRL (rw) register accessor: USB LEM Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lemctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lemctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lemctrl`] module"]
pub type LEMCTRL = crate::Reg<lemctrl::LEMCTRL_SPEC>;
#[doc = "USB LEM Control Register"]
pub mod lemctrl;
#[doc = "GOTGCTL (rw) register accessor: OTG Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gotgctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gotgctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gotgctl`] module"]
pub type GOTGCTL = crate::Reg<gotgctl::GOTGCTL_SPEC>;
#[doc = "OTG Control and Status Register"]
pub mod gotgctl;
#[doc = "GOTGINT (rw) register accessor: OTG Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gotgint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gotgint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gotgint`] module"]
pub type GOTGINT = crate::Reg<gotgint::GOTGINT_SPEC>;
#[doc = "OTG Interrupt Register"]
pub mod gotgint;
#[doc = "GAHBCFG (rw) register accessor: AHB Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gahbcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gahbcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gahbcfg`] module"]
pub type GAHBCFG = crate::Reg<gahbcfg::GAHBCFG_SPEC>;
#[doc = "AHB Configuration Register"]
pub mod gahbcfg;
#[doc = "GUSBCFG (rw) register accessor: USB Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gusbcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gusbcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gusbcfg`] module"]
pub type GUSBCFG = crate::Reg<gusbcfg::GUSBCFG_SPEC>;
#[doc = "USB Configuration Register"]
pub mod gusbcfg;
#[doc = "GRSTCTL (rw) register accessor: Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grstctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grstctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`grstctl`] module"]
pub type GRSTCTL = crate::Reg<grstctl::GRSTCTL_SPEC>;
#[doc = "Reset Register"]
pub mod grstctl;
#[doc = "GINTSTS (rw) register accessor: Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gintsts`] module"]
pub type GINTSTS = crate::Reg<gintsts::GINTSTS_SPEC>;
#[doc = "Interrupt Register"]
pub mod gintsts;
#[doc = "GINTMSK (rw) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gintmsk`] module"]
pub type GINTMSK = crate::Reg<gintmsk::GINTMSK_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod gintmsk;
#[doc = "GRXSTSR (r) register accessor: Receive Status Debug Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`grxstsr`] module"]
pub type GRXSTSR = crate::Reg<grxstsr::GRXSTSR_SPEC>;
#[doc = "Receive Status Debug Read Register"]
pub mod grxstsr;
#[doc = "GRXSTSP (r) register accessor: Receive Status Read /Pop Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`grxstsp`] module"]
pub type GRXSTSP = crate::Reg<grxstsp::GRXSTSP_SPEC>;
#[doc = "Receive Status Read /Pop Register"]
pub mod grxstsp;
#[doc = "GRXFSIZ (rw) register accessor: Receive FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`grxfsiz`] module"]
pub type GRXFSIZ = crate::Reg<grxfsiz::GRXFSIZ_SPEC>;
#[doc = "Receive FIFO Size Register"]
pub mod grxfsiz;
#[doc = "GNPTXFSIZ (rw) register accessor: Non-periodic Transmit FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gnptxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gnptxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gnptxfsiz`] module"]
pub type GNPTXFSIZ = crate::Reg<gnptxfsiz::GNPTXFSIZ_SPEC>;
#[doc = "Non-periodic Transmit FIFO Size Register"]
pub mod gnptxfsiz;
#[doc = "GNPTXSTS (r) register accessor: Non-periodic Transmit FIFO/Queue Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gnptxsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gnptxsts`] module"]
pub type GNPTXSTS = crate::Reg<gnptxsts::GNPTXSTS_SPEC>;
#[doc = "Non-periodic Transmit FIFO/Queue Status Register"]
pub mod gnptxsts;
#[doc = "GSNPSID (r) register accessor: Synopsys ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gsnpsid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gsnpsid`] module"]
pub type GSNPSID = crate::Reg<gsnpsid::GSNPSID_SPEC>;
#[doc = "Synopsys ID Register"]
pub mod gsnpsid;
#[doc = "GDFIFOCFG (rw) register accessor: Global DFIFO Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gdfifocfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gdfifocfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gdfifocfg`] module"]
pub type GDFIFOCFG = crate::Reg<gdfifocfg::GDFIFOCFG_SPEC>;
#[doc = "Global DFIFO Configuration Register"]
pub mod gdfifocfg;
#[doc = "HPTXFSIZ (rw) register accessor: Host Periodic Transmit FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hptxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hptxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hptxfsiz`] module"]
pub type HPTXFSIZ = crate::Reg<hptxfsiz::HPTXFSIZ_SPEC>;
#[doc = "Host Periodic Transmit FIFO Size Register"]
pub mod hptxfsiz;
#[doc = "DIEPTXF1 (rw) register accessor: Device IN Endpoint Transmit FIFO Size Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dieptxf1`] module"]
pub type DIEPTXF1 = crate::Reg<dieptxf1::DIEPTXF1_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register 1"]
pub mod dieptxf1;
#[doc = "DIEPTXF2 (rw) register accessor: Device IN Endpoint Transmit FIFO Size Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dieptxf2`] module"]
pub type DIEPTXF2 = crate::Reg<dieptxf2::DIEPTXF2_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register 2"]
pub mod dieptxf2;
#[doc = "DIEPTXF3 (rw) register accessor: Device IN Endpoint Transmit FIFO Size Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dieptxf3`] module"]
pub type DIEPTXF3 = crate::Reg<dieptxf3::DIEPTXF3_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register 3"]
pub mod dieptxf3;
#[doc = "DIEPTXF4 (rw) register accessor: Device IN Endpoint Transmit FIFO Size Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dieptxf4`] module"]
pub type DIEPTXF4 = crate::Reg<dieptxf4::DIEPTXF4_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register 4"]
pub mod dieptxf4;
#[doc = "DIEPTXF5 (rw) register accessor: Device IN Endpoint Transmit FIFO Size Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dieptxf5`] module"]
pub type DIEPTXF5 = crate::Reg<dieptxf5::DIEPTXF5_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register 5"]
pub mod dieptxf5;
#[doc = "DIEPTXF6 (rw) register accessor: Device IN Endpoint Transmit FIFO Size Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dieptxf6`] module"]
pub type DIEPTXF6 = crate::Reg<dieptxf6::DIEPTXF6_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register 6"]
pub mod dieptxf6;
#[doc = "HCFG (rw) register accessor: Host Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hcfg`] module"]
pub type HCFG = crate::Reg<hcfg::HCFG_SPEC>;
#[doc = "Host Configuration Register"]
pub mod hcfg;
#[doc = "HFIR (rw) register accessor: Host Frame Interval Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hfir`] module"]
pub type HFIR = crate::Reg<hfir::HFIR_SPEC>;
#[doc = "Host Frame Interval Register"]
pub mod hfir;
#[doc = "HFNUM (r) register accessor: Host Frame Number/Frame Time Remaining Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfnum::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hfnum`] module"]
pub type HFNUM = crate::Reg<hfnum::HFNUM_SPEC>;
#[doc = "Host Frame Number/Frame Time Remaining Register"]
pub mod hfnum;
#[doc = "HPTXSTS (r) register accessor: Host Periodic Transmit FIFO/Queue Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hptxsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hptxsts`] module"]
pub type HPTXSTS = crate::Reg<hptxsts::HPTXSTS_SPEC>;
#[doc = "Host Periodic Transmit FIFO/Queue Status Register"]
pub mod hptxsts;
#[doc = "HAINT (r) register accessor: Host All Channels Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`haint::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`haint`] module"]
pub type HAINT = crate::Reg<haint::HAINT_SPEC>;
#[doc = "Host All Channels Interrupt Register"]
pub mod haint;
#[doc = "HAINTMSK (rw) register accessor: Host All Channels Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`haintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`haintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`haintmsk`] module"]
pub type HAINTMSK = crate::Reg<haintmsk::HAINTMSK_SPEC>;
#[doc = "Host All Channels Interrupt Mask Register"]
pub mod haintmsk;
#[doc = "HPRT (rw) register accessor: Host Port Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hprt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hprt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hprt`] module"]
pub type HPRT = crate::Reg<hprt::HPRT_SPEC>;
#[doc = "Host Port Control and Status Register"]
pub mod hprt;
#[doc = "HC0_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc0_char::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc0_char::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc0_char`] module"]
pub type HC0_CHAR = crate::Reg<hc0_char::HC0_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc0_char;
#[doc = "HC0_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc0_splt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc0_splt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc0_splt`] module"]
pub type HC0_SPLT = crate::Reg<hc0_splt::HC0_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc0_splt;
#[doc = "HC0_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc0_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc0_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc0_int`] module"]
pub type HC0_INT = crate::Reg<hc0_int::HC0_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc0_int;
#[doc = "HC0_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc0_intmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc0_intmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc0_intmsk`] module"]
pub type HC0_INTMSK = crate::Reg<hc0_intmsk::HC0_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc0_intmsk;
#[doc = "HC0_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc0_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc0_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc0_tsiz`] module"]
pub type HC0_TSIZ = crate::Reg<hc0_tsiz::HC0_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc0_tsiz;
#[doc = "HC0_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc0_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc0_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc0_dmaaddr`] module"]
pub type HC0_DMAADDR = crate::Reg<hc0_dmaaddr::HC0_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc0_dmaaddr;
#[doc = "HC1_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc1_char::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc1_char::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc1_char`] module"]
pub type HC1_CHAR = crate::Reg<hc1_char::HC1_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc1_char;
#[doc = "HC1_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc1_splt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc1_splt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc1_splt`] module"]
pub type HC1_SPLT = crate::Reg<hc1_splt::HC1_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc1_splt;
#[doc = "HC1_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc1_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc1_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc1_int`] module"]
pub type HC1_INT = crate::Reg<hc1_int::HC1_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc1_int;
#[doc = "HC1_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc1_intmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc1_intmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc1_intmsk`] module"]
pub type HC1_INTMSK = crate::Reg<hc1_intmsk::HC1_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc1_intmsk;
#[doc = "HC1_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc1_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc1_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc1_tsiz`] module"]
pub type HC1_TSIZ = crate::Reg<hc1_tsiz::HC1_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc1_tsiz;
#[doc = "HC1_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc1_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc1_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc1_dmaaddr`] module"]
pub type HC1_DMAADDR = crate::Reg<hc1_dmaaddr::HC1_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc1_dmaaddr;
#[doc = "HC2_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc2_char::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc2_char::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc2_char`] module"]
pub type HC2_CHAR = crate::Reg<hc2_char::HC2_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc2_char;
#[doc = "HC2_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc2_splt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc2_splt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc2_splt`] module"]
pub type HC2_SPLT = crate::Reg<hc2_splt::HC2_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc2_splt;
#[doc = "HC2_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc2_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc2_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc2_int`] module"]
pub type HC2_INT = crate::Reg<hc2_int::HC2_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc2_int;
#[doc = "HC2_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc2_intmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc2_intmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc2_intmsk`] module"]
pub type HC2_INTMSK = crate::Reg<hc2_intmsk::HC2_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc2_intmsk;
#[doc = "HC2_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc2_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc2_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc2_tsiz`] module"]
pub type HC2_TSIZ = crate::Reg<hc2_tsiz::HC2_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc2_tsiz;
#[doc = "HC2_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc2_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc2_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc2_dmaaddr`] module"]
pub type HC2_DMAADDR = crate::Reg<hc2_dmaaddr::HC2_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc2_dmaaddr;
#[doc = "HC3_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc3_char::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc3_char::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc3_char`] module"]
pub type HC3_CHAR = crate::Reg<hc3_char::HC3_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc3_char;
#[doc = "HC3_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc3_splt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc3_splt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc3_splt`] module"]
pub type HC3_SPLT = crate::Reg<hc3_splt::HC3_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc3_splt;
#[doc = "HC3_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc3_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc3_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc3_int`] module"]
pub type HC3_INT = crate::Reg<hc3_int::HC3_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc3_int;
#[doc = "HC3_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc3_intmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc3_intmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc3_intmsk`] module"]
pub type HC3_INTMSK = crate::Reg<hc3_intmsk::HC3_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc3_intmsk;
#[doc = "HC3_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc3_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc3_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc3_tsiz`] module"]
pub type HC3_TSIZ = crate::Reg<hc3_tsiz::HC3_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc3_tsiz;
#[doc = "HC3_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc3_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc3_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc3_dmaaddr`] module"]
pub type HC3_DMAADDR = crate::Reg<hc3_dmaaddr::HC3_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc3_dmaaddr;
#[doc = "HC4_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc4_char::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc4_char::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc4_char`] module"]
pub type HC4_CHAR = crate::Reg<hc4_char::HC4_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc4_char;
#[doc = "HC4_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc4_splt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc4_splt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc4_splt`] module"]
pub type HC4_SPLT = crate::Reg<hc4_splt::HC4_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc4_splt;
#[doc = "HC4_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc4_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc4_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc4_int`] module"]
pub type HC4_INT = crate::Reg<hc4_int::HC4_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc4_int;
#[doc = "HC4_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc4_intmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc4_intmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc4_intmsk`] module"]
pub type HC4_INTMSK = crate::Reg<hc4_intmsk::HC4_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc4_intmsk;
#[doc = "HC4_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc4_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc4_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc4_tsiz`] module"]
pub type HC4_TSIZ = crate::Reg<hc4_tsiz::HC4_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc4_tsiz;
#[doc = "HC4_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc4_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc4_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc4_dmaaddr`] module"]
pub type HC4_DMAADDR = crate::Reg<hc4_dmaaddr::HC4_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc4_dmaaddr;
#[doc = "HC5_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc5_char::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc5_char::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc5_char`] module"]
pub type HC5_CHAR = crate::Reg<hc5_char::HC5_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc5_char;
#[doc = "HC5_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc5_splt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc5_splt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc5_splt`] module"]
pub type HC5_SPLT = crate::Reg<hc5_splt::HC5_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc5_splt;
#[doc = "HC5_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc5_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc5_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc5_int`] module"]
pub type HC5_INT = crate::Reg<hc5_int::HC5_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc5_int;
#[doc = "HC5_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc5_intmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc5_intmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc5_intmsk`] module"]
pub type HC5_INTMSK = crate::Reg<hc5_intmsk::HC5_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc5_intmsk;
#[doc = "HC5_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc5_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc5_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc5_tsiz`] module"]
pub type HC5_TSIZ = crate::Reg<hc5_tsiz::HC5_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc5_tsiz;
#[doc = "HC5_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc5_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc5_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc5_dmaaddr`] module"]
pub type HC5_DMAADDR = crate::Reg<hc5_dmaaddr::HC5_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc5_dmaaddr;
#[doc = "HC6_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc6_char::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc6_char::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc6_char`] module"]
pub type HC6_CHAR = crate::Reg<hc6_char::HC6_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc6_char;
#[doc = "HC6_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc6_splt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc6_splt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc6_splt`] module"]
pub type HC6_SPLT = crate::Reg<hc6_splt::HC6_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc6_splt;
#[doc = "HC6_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc6_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc6_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc6_int`] module"]
pub type HC6_INT = crate::Reg<hc6_int::HC6_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc6_int;
#[doc = "HC6_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc6_intmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc6_intmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc6_intmsk`] module"]
pub type HC6_INTMSK = crate::Reg<hc6_intmsk::HC6_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc6_intmsk;
#[doc = "HC6_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc6_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc6_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc6_tsiz`] module"]
pub type HC6_TSIZ = crate::Reg<hc6_tsiz::HC6_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc6_tsiz;
#[doc = "HC6_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc6_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc6_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc6_dmaaddr`] module"]
pub type HC6_DMAADDR = crate::Reg<hc6_dmaaddr::HC6_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc6_dmaaddr;
#[doc = "HC7_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc7_char::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc7_char::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc7_char`] module"]
pub type HC7_CHAR = crate::Reg<hc7_char::HC7_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc7_char;
#[doc = "HC7_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc7_splt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc7_splt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc7_splt`] module"]
pub type HC7_SPLT = crate::Reg<hc7_splt::HC7_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc7_splt;
#[doc = "HC7_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc7_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc7_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc7_int`] module"]
pub type HC7_INT = crate::Reg<hc7_int::HC7_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc7_int;
#[doc = "HC7_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc7_intmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc7_intmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc7_intmsk`] module"]
pub type HC7_INTMSK = crate::Reg<hc7_intmsk::HC7_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc7_intmsk;
#[doc = "HC7_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc7_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc7_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc7_tsiz`] module"]
pub type HC7_TSIZ = crate::Reg<hc7_tsiz::HC7_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc7_tsiz;
#[doc = "HC7_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc7_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc7_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc7_dmaaddr`] module"]
pub type HC7_DMAADDR = crate::Reg<hc7_dmaaddr::HC7_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc7_dmaaddr;
#[doc = "HC8_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc8_char::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc8_char::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc8_char`] module"]
pub type HC8_CHAR = crate::Reg<hc8_char::HC8_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc8_char;
#[doc = "HC8_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc8_splt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc8_splt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc8_splt`] module"]
pub type HC8_SPLT = crate::Reg<hc8_splt::HC8_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc8_splt;
#[doc = "HC8_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc8_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc8_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc8_int`] module"]
pub type HC8_INT = crate::Reg<hc8_int::HC8_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc8_int;
#[doc = "HC8_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc8_intmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc8_intmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc8_intmsk`] module"]
pub type HC8_INTMSK = crate::Reg<hc8_intmsk::HC8_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc8_intmsk;
#[doc = "HC8_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc8_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc8_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc8_tsiz`] module"]
pub type HC8_TSIZ = crate::Reg<hc8_tsiz::HC8_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc8_tsiz;
#[doc = "HC8_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc8_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc8_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc8_dmaaddr`] module"]
pub type HC8_DMAADDR = crate::Reg<hc8_dmaaddr::HC8_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc8_dmaaddr;
#[doc = "HC9_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc9_char::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc9_char::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc9_char`] module"]
pub type HC9_CHAR = crate::Reg<hc9_char::HC9_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc9_char;
#[doc = "HC9_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc9_splt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc9_splt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc9_splt`] module"]
pub type HC9_SPLT = crate::Reg<hc9_splt::HC9_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc9_splt;
#[doc = "HC9_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc9_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc9_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc9_int`] module"]
pub type HC9_INT = crate::Reg<hc9_int::HC9_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc9_int;
#[doc = "HC9_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc9_intmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc9_intmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc9_intmsk`] module"]
pub type HC9_INTMSK = crate::Reg<hc9_intmsk::HC9_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc9_intmsk;
#[doc = "HC9_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc9_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc9_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc9_tsiz`] module"]
pub type HC9_TSIZ = crate::Reg<hc9_tsiz::HC9_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc9_tsiz;
#[doc = "HC9_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc9_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc9_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc9_dmaaddr`] module"]
pub type HC9_DMAADDR = crate::Reg<hc9_dmaaddr::HC9_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc9_dmaaddr;
#[doc = "HC10_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc10_char::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc10_char::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc10_char`] module"]
pub type HC10_CHAR = crate::Reg<hc10_char::HC10_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc10_char;
#[doc = "HC10_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc10_splt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc10_splt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc10_splt`] module"]
pub type HC10_SPLT = crate::Reg<hc10_splt::HC10_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc10_splt;
#[doc = "HC10_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc10_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc10_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc10_int`] module"]
pub type HC10_INT = crate::Reg<hc10_int::HC10_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc10_int;
#[doc = "HC10_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc10_intmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc10_intmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc10_intmsk`] module"]
pub type HC10_INTMSK = crate::Reg<hc10_intmsk::HC10_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc10_intmsk;
#[doc = "HC10_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc10_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc10_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc10_tsiz`] module"]
pub type HC10_TSIZ = crate::Reg<hc10_tsiz::HC10_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc10_tsiz;
#[doc = "HC10_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc10_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc10_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc10_dmaaddr`] module"]
pub type HC10_DMAADDR = crate::Reg<hc10_dmaaddr::HC10_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc10_dmaaddr;
#[doc = "HC11_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc11_char::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc11_char::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc11_char`] module"]
pub type HC11_CHAR = crate::Reg<hc11_char::HC11_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc11_char;
#[doc = "HC11_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc11_splt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc11_splt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc11_splt`] module"]
pub type HC11_SPLT = crate::Reg<hc11_splt::HC11_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc11_splt;
#[doc = "HC11_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc11_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc11_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc11_int`] module"]
pub type HC11_INT = crate::Reg<hc11_int::HC11_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc11_int;
#[doc = "HC11_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc11_intmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc11_intmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc11_intmsk`] module"]
pub type HC11_INTMSK = crate::Reg<hc11_intmsk::HC11_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc11_intmsk;
#[doc = "HC11_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc11_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc11_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc11_tsiz`] module"]
pub type HC11_TSIZ = crate::Reg<hc11_tsiz::HC11_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc11_tsiz;
#[doc = "HC11_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc11_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc11_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc11_dmaaddr`] module"]
pub type HC11_DMAADDR = crate::Reg<hc11_dmaaddr::HC11_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc11_dmaaddr;
#[doc = "HC12_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc12_char::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc12_char::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc12_char`] module"]
pub type HC12_CHAR = crate::Reg<hc12_char::HC12_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc12_char;
#[doc = "HC12_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc12_splt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc12_splt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc12_splt`] module"]
pub type HC12_SPLT = crate::Reg<hc12_splt::HC12_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc12_splt;
#[doc = "HC12_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc12_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc12_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc12_int`] module"]
pub type HC12_INT = crate::Reg<hc12_int::HC12_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc12_int;
#[doc = "HC12_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc12_intmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc12_intmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc12_intmsk`] module"]
pub type HC12_INTMSK = crate::Reg<hc12_intmsk::HC12_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc12_intmsk;
#[doc = "HC12_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc12_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc12_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc12_tsiz`] module"]
pub type HC12_TSIZ = crate::Reg<hc12_tsiz::HC12_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc12_tsiz;
#[doc = "HC12_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc12_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc12_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc12_dmaaddr`] module"]
pub type HC12_DMAADDR = crate::Reg<hc12_dmaaddr::HC12_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc12_dmaaddr;
#[doc = "HC13_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc13_char::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc13_char::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc13_char`] module"]
pub type HC13_CHAR = crate::Reg<hc13_char::HC13_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc13_char;
#[doc = "HC13_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc13_splt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc13_splt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc13_splt`] module"]
pub type HC13_SPLT = crate::Reg<hc13_splt::HC13_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc13_splt;
#[doc = "HC13_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc13_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc13_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc13_int`] module"]
pub type HC13_INT = crate::Reg<hc13_int::HC13_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc13_int;
#[doc = "HC13_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc13_intmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc13_intmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc13_intmsk`] module"]
pub type HC13_INTMSK = crate::Reg<hc13_intmsk::HC13_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc13_intmsk;
#[doc = "HC13_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc13_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc13_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc13_tsiz`] module"]
pub type HC13_TSIZ = crate::Reg<hc13_tsiz::HC13_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc13_tsiz;
#[doc = "HC13_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc13_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc13_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hc13_dmaaddr`] module"]
pub type HC13_DMAADDR = crate::Reg<hc13_dmaaddr::HC13_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc13_dmaaddr;
#[doc = "DCFG (rw) register accessor: Device Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dcfg`] module"]
pub type DCFG = crate::Reg<dcfg::DCFG_SPEC>;
#[doc = "Device Configuration Register"]
pub mod dcfg;
#[doc = "DCTL (rw) register accessor: Device Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dctl`] module"]
pub type DCTL = crate::Reg<dctl::DCTL_SPEC>;
#[doc = "Device Control Register"]
pub mod dctl;
#[doc = "DSTS (r) register accessor: Device Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsts`] module"]
pub type DSTS = crate::Reg<dsts::DSTS_SPEC>;
#[doc = "Device Status Register"]
pub mod dsts;
#[doc = "DIEPMSK (rw) register accessor: Device IN Endpoint Common Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepmsk`] module"]
pub type DIEPMSK = crate::Reg<diepmsk::DIEPMSK_SPEC>;
#[doc = "Device IN Endpoint Common Interrupt Mask Register"]
pub mod diepmsk;
#[doc = "DOEPMSK (rw) register accessor: Device OUT Endpoint Common Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepmsk`] module"]
pub type DOEPMSK = crate::Reg<doepmsk::DOEPMSK_SPEC>;
#[doc = "Device OUT Endpoint Common Interrupt Mask Register"]
pub mod doepmsk;
#[doc = "DAINT (r) register accessor: Device All Endpoints Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daint::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`daint`] module"]
pub type DAINT = crate::Reg<daint::DAINT_SPEC>;
#[doc = "Device All Endpoints Interrupt Register"]
pub mod daint;
#[doc = "DAINTMSK (rw) register accessor: Device All Endpoints Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`daintmsk`] module"]
pub type DAINTMSK = crate::Reg<daintmsk::DAINTMSK_SPEC>;
#[doc = "Device All Endpoints Interrupt Mask Register"]
pub mod daintmsk;
#[doc = "DVBUSDIS (rw) register accessor: Device VBUS Discharge Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dvbusdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dvbusdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dvbusdis`] module"]
pub type DVBUSDIS = crate::Reg<dvbusdis::DVBUSDIS_SPEC>;
#[doc = "Device VBUS Discharge Time Register"]
pub mod dvbusdis;
#[doc = "DVBUSPULSE (rw) register accessor: Device VBUS Pulsing Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dvbuspulse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dvbuspulse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dvbuspulse`] module"]
pub type DVBUSPULSE = crate::Reg<dvbuspulse::DVBUSPULSE_SPEC>;
#[doc = "Device VBUS Pulsing Time Register"]
pub mod dvbuspulse;
#[doc = "DTHRCTL (rw) register accessor: Device Threshold Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dthrctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dthrctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dthrctl`] module"]
pub type DTHRCTL = crate::Reg<dthrctl::DTHRCTL_SPEC>;
#[doc = "Device Threshold Control Register"]
pub mod dthrctl;
#[doc = "DIEPEMPMSK (rw) register accessor: Device IN Endpoint FIFO Empty Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepempmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepempmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepempmsk`] module"]
pub type DIEPEMPMSK = crate::Reg<diepempmsk::DIEPEMPMSK_SPEC>;
#[doc = "Device IN Endpoint FIFO Empty Interrupt Mask Register"]
pub mod diepempmsk;
#[doc = "DIEP0CTL (rw) register accessor: Device Control IN Endpoint 0 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep0ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep0ctl`] module"]
pub type DIEP0CTL = crate::Reg<diep0ctl::DIEP0CTL_SPEC>;
#[doc = "Device Control IN Endpoint 0 Control Register"]
pub mod diep0ctl;
#[doc = "DIEP0INT (rw) register accessor: Device IN Endpoint 0 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep0int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep0int`] module"]
pub type DIEP0INT = crate::Reg<diep0int::DIEP0INT_SPEC>;
#[doc = "Device IN Endpoint 0 Interrupt Register"]
pub mod diep0int;
#[doc = "DIEP0TSIZ (rw) register accessor: Device IN Endpoint 0 Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep0tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep0tsiz`] module"]
pub type DIEP0TSIZ = crate::Reg<diep0tsiz::DIEP0TSIZ_SPEC>;
#[doc = "Device IN Endpoint 0 Transfer Size Register"]
pub mod diep0tsiz;
#[doc = "DIEP0DMAADDR (rw) register accessor: Device IN Endpoint 0 DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep0dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep0dmaaddr`] module"]
pub type DIEP0DMAADDR = crate::Reg<diep0dmaaddr::DIEP0DMAADDR_SPEC>;
#[doc = "Device IN Endpoint 0 DMA Address Register"]
pub mod diep0dmaaddr;
#[doc = "DIEP0TXFSTS (r) register accessor: Device IN Endpoint Transmit FIFO Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0txfsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep0txfsts`] module"]
pub type DIEP0TXFSTS = crate::Reg<diep0txfsts::DIEP0TXFSTS_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Status Register 0"]
pub mod diep0txfsts;
#[doc = "DIEP0_CTL (rw) register accessor: Device Control IN Endpoint x+1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep0_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep0_ctl`] module"]
pub type DIEP0_CTL = crate::Reg<diep0_ctl::DIEP0_CTL_SPEC>;
#[doc = "Device Control IN Endpoint x+1 Control Register"]
pub mod diep0_ctl;
#[doc = "DIEP0_INT (rw) register accessor: Device IN Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep0_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep0_int`] module"]
pub type DIEP0_INT = crate::Reg<diep0_int::DIEP0_INT_SPEC>;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep0_int;
#[doc = "DIEP0_TSIZ (rw) register accessor: Device IN Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep0_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep0_tsiz`] module"]
pub type DIEP0_TSIZ = crate::Reg<diep0_tsiz::DIEP0_TSIZ_SPEC>;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep0_tsiz;
#[doc = "DIEP0_DMAADDR (rw) register accessor: Device IN Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep0_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep0_dmaaddr`] module"]
pub type DIEP0_DMAADDR = crate::Reg<diep0_dmaaddr::DIEP0_DMAADDR_SPEC>;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep0_dmaaddr;
#[doc = "DIEP0_DTXFSTS (r) register accessor: Device IN Endpoint Transmit FIFO Status Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0_dtxfsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep0_dtxfsts`] module"]
pub type DIEP0_DTXFSTS = crate::Reg<diep0_dtxfsts::DIEP0_DTXFSTS_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1"]
pub mod diep0_dtxfsts;
#[doc = "DIEP1_CTL (rw) register accessor: Device Control IN Endpoint x+1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep1_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep1_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep1_ctl`] module"]
pub type DIEP1_CTL = crate::Reg<diep1_ctl::DIEP1_CTL_SPEC>;
#[doc = "Device Control IN Endpoint x+1 Control Register"]
pub mod diep1_ctl;
#[doc = "DIEP1_INT (rw) register accessor: Device IN Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep1_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep1_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep1_int`] module"]
pub type DIEP1_INT = crate::Reg<diep1_int::DIEP1_INT_SPEC>;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep1_int;
#[doc = "DIEP1_TSIZ (rw) register accessor: Device IN Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep1_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep1_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep1_tsiz`] module"]
pub type DIEP1_TSIZ = crate::Reg<diep1_tsiz::DIEP1_TSIZ_SPEC>;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep1_tsiz;
#[doc = "DIEP1_DMAADDR (rw) register accessor: Device IN Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep1_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep1_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep1_dmaaddr`] module"]
pub type DIEP1_DMAADDR = crate::Reg<diep1_dmaaddr::DIEP1_DMAADDR_SPEC>;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep1_dmaaddr;
#[doc = "DIEP1_DTXFSTS (r) register accessor: Device IN Endpoint Transmit FIFO Status Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep1_dtxfsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep1_dtxfsts`] module"]
pub type DIEP1_DTXFSTS = crate::Reg<diep1_dtxfsts::DIEP1_DTXFSTS_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1"]
pub mod diep1_dtxfsts;
#[doc = "DIEP2_CTL (rw) register accessor: Device Control IN Endpoint x+1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep2_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep2_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep2_ctl`] module"]
pub type DIEP2_CTL = crate::Reg<diep2_ctl::DIEP2_CTL_SPEC>;
#[doc = "Device Control IN Endpoint x+1 Control Register"]
pub mod diep2_ctl;
#[doc = "DIEP2_INT (rw) register accessor: Device IN Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep2_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep2_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep2_int`] module"]
pub type DIEP2_INT = crate::Reg<diep2_int::DIEP2_INT_SPEC>;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep2_int;
#[doc = "DIEP2_TSIZ (rw) register accessor: Device IN Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep2_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep2_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep2_tsiz`] module"]
pub type DIEP2_TSIZ = crate::Reg<diep2_tsiz::DIEP2_TSIZ_SPEC>;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep2_tsiz;
#[doc = "DIEP2_DMAADDR (rw) register accessor: Device IN Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep2_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep2_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep2_dmaaddr`] module"]
pub type DIEP2_DMAADDR = crate::Reg<diep2_dmaaddr::DIEP2_DMAADDR_SPEC>;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep2_dmaaddr;
#[doc = "DIEP2_DTXFSTS (r) register accessor: Device IN Endpoint Transmit FIFO Status Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep2_dtxfsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep2_dtxfsts`] module"]
pub type DIEP2_DTXFSTS = crate::Reg<diep2_dtxfsts::DIEP2_DTXFSTS_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1"]
pub mod diep2_dtxfsts;
#[doc = "DIEP3_CTL (rw) register accessor: Device Control IN Endpoint x+1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep3_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep3_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep3_ctl`] module"]
pub type DIEP3_CTL = crate::Reg<diep3_ctl::DIEP3_CTL_SPEC>;
#[doc = "Device Control IN Endpoint x+1 Control Register"]
pub mod diep3_ctl;
#[doc = "DIEP3_INT (rw) register accessor: Device IN Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep3_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep3_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep3_int`] module"]
pub type DIEP3_INT = crate::Reg<diep3_int::DIEP3_INT_SPEC>;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep3_int;
#[doc = "DIEP3_TSIZ (rw) register accessor: Device IN Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep3_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep3_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep3_tsiz`] module"]
pub type DIEP3_TSIZ = crate::Reg<diep3_tsiz::DIEP3_TSIZ_SPEC>;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep3_tsiz;
#[doc = "DIEP3_DMAADDR (rw) register accessor: Device IN Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep3_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep3_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep3_dmaaddr`] module"]
pub type DIEP3_DMAADDR = crate::Reg<diep3_dmaaddr::DIEP3_DMAADDR_SPEC>;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep3_dmaaddr;
#[doc = "DIEP3_DTXFSTS (r) register accessor: Device IN Endpoint Transmit FIFO Status Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep3_dtxfsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep3_dtxfsts`] module"]
pub type DIEP3_DTXFSTS = crate::Reg<diep3_dtxfsts::DIEP3_DTXFSTS_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1"]
pub mod diep3_dtxfsts;
#[doc = "DIEP4_CTL (rw) register accessor: Device Control IN Endpoint x+1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep4_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep4_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep4_ctl`] module"]
pub type DIEP4_CTL = crate::Reg<diep4_ctl::DIEP4_CTL_SPEC>;
#[doc = "Device Control IN Endpoint x+1 Control Register"]
pub mod diep4_ctl;
#[doc = "DIEP4_INT (rw) register accessor: Device IN Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep4_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep4_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep4_int`] module"]
pub type DIEP4_INT = crate::Reg<diep4_int::DIEP4_INT_SPEC>;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep4_int;
#[doc = "DIEP4_TSIZ (rw) register accessor: Device IN Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep4_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep4_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep4_tsiz`] module"]
pub type DIEP4_TSIZ = crate::Reg<diep4_tsiz::DIEP4_TSIZ_SPEC>;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep4_tsiz;
#[doc = "DIEP4_DMAADDR (rw) register accessor: Device IN Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep4_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep4_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep4_dmaaddr`] module"]
pub type DIEP4_DMAADDR = crate::Reg<diep4_dmaaddr::DIEP4_DMAADDR_SPEC>;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep4_dmaaddr;
#[doc = "DIEP4_DTXFSTS (r) register accessor: Device IN Endpoint Transmit FIFO Status Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep4_dtxfsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep4_dtxfsts`] module"]
pub type DIEP4_DTXFSTS = crate::Reg<diep4_dtxfsts::DIEP4_DTXFSTS_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1"]
pub mod diep4_dtxfsts;
#[doc = "DIEP5_CTL (rw) register accessor: Device Control IN Endpoint x+1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep5_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep5_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep5_ctl`] module"]
pub type DIEP5_CTL = crate::Reg<diep5_ctl::DIEP5_CTL_SPEC>;
#[doc = "Device Control IN Endpoint x+1 Control Register"]
pub mod diep5_ctl;
#[doc = "DIEP5_INT (rw) register accessor: Device IN Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep5_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep5_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep5_int`] module"]
pub type DIEP5_INT = crate::Reg<diep5_int::DIEP5_INT_SPEC>;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep5_int;
#[doc = "DIEP5_TSIZ (rw) register accessor: Device IN Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep5_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep5_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep5_tsiz`] module"]
pub type DIEP5_TSIZ = crate::Reg<diep5_tsiz::DIEP5_TSIZ_SPEC>;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep5_tsiz;
#[doc = "DIEP5_DMAADDR (rw) register accessor: Device IN Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep5_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep5_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep5_dmaaddr`] module"]
pub type DIEP5_DMAADDR = crate::Reg<diep5_dmaaddr::DIEP5_DMAADDR_SPEC>;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep5_dmaaddr;
#[doc = "DIEP5_DTXFSTS (r) register accessor: Device IN Endpoint Transmit FIFO Status Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep5_dtxfsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep5_dtxfsts`] module"]
pub type DIEP5_DTXFSTS = crate::Reg<diep5_dtxfsts::DIEP5_DTXFSTS_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1"]
pub mod diep5_dtxfsts;
#[doc = "DOEP0CTL (rw) register accessor: Device Control OUT Endpoint 0 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep0ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep0ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep0ctl`] module"]
pub type DOEP0CTL = crate::Reg<doep0ctl::DOEP0CTL_SPEC>;
#[doc = "Device Control OUT Endpoint 0 Control Register"]
pub mod doep0ctl;
#[doc = "DOEP0INT (rw) register accessor: Device OUT Endpoint 0 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep0int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep0int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep0int`] module"]
pub type DOEP0INT = crate::Reg<doep0int::DOEP0INT_SPEC>;
#[doc = "Device OUT Endpoint 0 Interrupt Register"]
pub mod doep0int;
#[doc = "DOEP0TSIZ (rw) register accessor: Device OUT Endpoint 0 Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep0tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep0tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep0tsiz`] module"]
pub type DOEP0TSIZ = crate::Reg<doep0tsiz::DOEP0TSIZ_SPEC>;
#[doc = "Device OUT Endpoint 0 Transfer Size Register"]
pub mod doep0tsiz;
#[doc = "DOEP0DMAADDR (rw) register accessor: Device OUT Endpoint 0 DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep0dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep0dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep0dmaaddr`] module"]
pub type DOEP0DMAADDR = crate::Reg<doep0dmaaddr::DOEP0DMAADDR_SPEC>;
#[doc = "Device OUT Endpoint 0 DMA Address Register"]
pub mod doep0dmaaddr;
#[doc = "DOEP0_CTL (rw) register accessor: Device Control OUT Endpoint x+1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep0_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep0_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep0_ctl`] module"]
pub type DOEP0_CTL = crate::Reg<doep0_ctl::DOEP0_CTL_SPEC>;
#[doc = "Device Control OUT Endpoint x+1 Control Register"]
pub mod doep0_ctl;
#[doc = "DOEP0_INT (rw) register accessor: Device OUT Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep0_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep0_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep0_int`] module"]
pub type DOEP0_INT = crate::Reg<doep0_int::DOEP0_INT_SPEC>;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep0_int;
#[doc = "DOEP0_TSIZ (rw) register accessor: Device OUT Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep0_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep0_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep0_tsiz`] module"]
pub type DOEP0_TSIZ = crate::Reg<doep0_tsiz::DOEP0_TSIZ_SPEC>;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep0_tsiz;
#[doc = "DOEP0_DMAADDR (rw) register accessor: Device OUT Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep0_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep0_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep0_dmaaddr`] module"]
pub type DOEP0_DMAADDR = crate::Reg<doep0_dmaaddr::DOEP0_DMAADDR_SPEC>;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep0_dmaaddr;
#[doc = "DOEP1_CTL (rw) register accessor: Device Control OUT Endpoint x+1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep1_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep1_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep1_ctl`] module"]
pub type DOEP1_CTL = crate::Reg<doep1_ctl::DOEP1_CTL_SPEC>;
#[doc = "Device Control OUT Endpoint x+1 Control Register"]
pub mod doep1_ctl;
#[doc = "DOEP1_INT (rw) register accessor: Device OUT Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep1_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep1_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep1_int`] module"]
pub type DOEP1_INT = crate::Reg<doep1_int::DOEP1_INT_SPEC>;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep1_int;
#[doc = "DOEP1_TSIZ (rw) register accessor: Device OUT Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep1_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep1_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep1_tsiz`] module"]
pub type DOEP1_TSIZ = crate::Reg<doep1_tsiz::DOEP1_TSIZ_SPEC>;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep1_tsiz;
#[doc = "DOEP1_DMAADDR (rw) register accessor: Device OUT Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep1_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep1_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep1_dmaaddr`] module"]
pub type DOEP1_DMAADDR = crate::Reg<doep1_dmaaddr::DOEP1_DMAADDR_SPEC>;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep1_dmaaddr;
#[doc = "DOEP2_CTL (rw) register accessor: Device Control OUT Endpoint x+1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep2_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep2_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep2_ctl`] module"]
pub type DOEP2_CTL = crate::Reg<doep2_ctl::DOEP2_CTL_SPEC>;
#[doc = "Device Control OUT Endpoint x+1 Control Register"]
pub mod doep2_ctl;
#[doc = "DOEP2_INT (rw) register accessor: Device OUT Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep2_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep2_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep2_int`] module"]
pub type DOEP2_INT = crate::Reg<doep2_int::DOEP2_INT_SPEC>;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep2_int;
#[doc = "DOEP2_TSIZ (rw) register accessor: Device OUT Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep2_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep2_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep2_tsiz`] module"]
pub type DOEP2_TSIZ = crate::Reg<doep2_tsiz::DOEP2_TSIZ_SPEC>;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep2_tsiz;
#[doc = "DOEP2_DMAADDR (rw) register accessor: Device OUT Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep2_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep2_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep2_dmaaddr`] module"]
pub type DOEP2_DMAADDR = crate::Reg<doep2_dmaaddr::DOEP2_DMAADDR_SPEC>;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep2_dmaaddr;
#[doc = "DOEP3_CTL (rw) register accessor: Device Control OUT Endpoint x+1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep3_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep3_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep3_ctl`] module"]
pub type DOEP3_CTL = crate::Reg<doep3_ctl::DOEP3_CTL_SPEC>;
#[doc = "Device Control OUT Endpoint x+1 Control Register"]
pub mod doep3_ctl;
#[doc = "DOEP3_INT (rw) register accessor: Device OUT Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep3_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep3_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep3_int`] module"]
pub type DOEP3_INT = crate::Reg<doep3_int::DOEP3_INT_SPEC>;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep3_int;
#[doc = "DOEP3_TSIZ (rw) register accessor: Device OUT Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep3_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep3_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep3_tsiz`] module"]
pub type DOEP3_TSIZ = crate::Reg<doep3_tsiz::DOEP3_TSIZ_SPEC>;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep3_tsiz;
#[doc = "DOEP3_DMAADDR (rw) register accessor: Device OUT Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep3_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep3_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep3_dmaaddr`] module"]
pub type DOEP3_DMAADDR = crate::Reg<doep3_dmaaddr::DOEP3_DMAADDR_SPEC>;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep3_dmaaddr;
#[doc = "DOEP4_CTL (rw) register accessor: Device Control OUT Endpoint x+1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep4_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep4_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep4_ctl`] module"]
pub type DOEP4_CTL = crate::Reg<doep4_ctl::DOEP4_CTL_SPEC>;
#[doc = "Device Control OUT Endpoint x+1 Control Register"]
pub mod doep4_ctl;
#[doc = "DOEP4_INT (rw) register accessor: Device OUT Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep4_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep4_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep4_int`] module"]
pub type DOEP4_INT = crate::Reg<doep4_int::DOEP4_INT_SPEC>;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep4_int;
#[doc = "DOEP4_TSIZ (rw) register accessor: Device OUT Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep4_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep4_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep4_tsiz`] module"]
pub type DOEP4_TSIZ = crate::Reg<doep4_tsiz::DOEP4_TSIZ_SPEC>;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep4_tsiz;
#[doc = "DOEP4_DMAADDR (rw) register accessor: Device OUT Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep4_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep4_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep4_dmaaddr`] module"]
pub type DOEP4_DMAADDR = crate::Reg<doep4_dmaaddr::DOEP4_DMAADDR_SPEC>;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep4_dmaaddr;
#[doc = "DOEP5_CTL (rw) register accessor: Device Control OUT Endpoint x+1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep5_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep5_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep5_ctl`] module"]
pub type DOEP5_CTL = crate::Reg<doep5_ctl::DOEP5_CTL_SPEC>;
#[doc = "Device Control OUT Endpoint x+1 Control Register"]
pub mod doep5_ctl;
#[doc = "DOEP5_INT (rw) register accessor: Device OUT Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep5_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep5_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep5_int`] module"]
pub type DOEP5_INT = crate::Reg<doep5_int::DOEP5_INT_SPEC>;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep5_int;
#[doc = "DOEP5_TSIZ (rw) register accessor: Device OUT Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep5_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep5_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep5_tsiz`] module"]
pub type DOEP5_TSIZ = crate::Reg<doep5_tsiz::DOEP5_TSIZ_SPEC>;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep5_tsiz;
#[doc = "DOEP5_DMAADDR (rw) register accessor: Device OUT Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep5_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep5_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep5_dmaaddr`] module"]
pub type DOEP5_DMAADDR = crate::Reg<doep5_dmaaddr::DOEP5_DMAADDR_SPEC>;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep5_dmaaddr;
#[doc = "PCGCCTL (rw) register accessor: Power and Clock Gating Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcgcctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcgcctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcgcctl`] module"]
pub type PCGCCTL = crate::Reg<pcgcctl::PCGCCTL_SPEC>;
#[doc = "Power and Clock Gating Control Register"]
pub mod pcgcctl;
