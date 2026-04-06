#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    timctrl: Timctrl,
    perctrl: Perctrl,
    decctrl: Decctrl,
    biasctrl: Biasctrl,
    evalctrl: Evalctrl,
    prsctrl: Prsctrl,
    cmd: Cmd,
    chen: Chen,
    scanres: Scanres,
    status: Status,
    ptr: Ptr,
    bufdata: Bufdata,
    curch: Curch,
    decstate: Decstate,
    sensorstate: Sensorstate,
    idleconf: Idleconf,
    altexconf: Altexconf,
    _reserved18: [u8; 0x08],
    if_: If,
    ifs: Ifs,
    ifc: Ifc,
    ien: Ien,
    syncbusy: Syncbusy,
    routepen: Routepen,
    _reserved24: [u8; 0x98],
    st0_tconfa: St0Tconfa,
    st0_tconfb: St0Tconfb,
    st1_tconfa: St1Tconfa,
    st1_tconfb: St1Tconfb,
    st2_tconfa: St2Tconfa,
    st2_tconfb: St2Tconfb,
    st3_tconfa: St3Tconfa,
    st3_tconfb: St3Tconfb,
    st4_tconfa: St4Tconfa,
    st4_tconfb: St4Tconfb,
    st5_tconfa: St5Tconfa,
    st5_tconfb: St5Tconfb,
    st6_tconfa: St6Tconfa,
    st6_tconfb: St6Tconfb,
    st7_tconfa: St7Tconfa,
    st7_tconfb: St7Tconfb,
    st8_tconfa: St8Tconfa,
    st8_tconfb: St8Tconfb,
    st9_tconfa: St9Tconfa,
    st9_tconfb: St9Tconfb,
    st10_tconfa: St10Tconfa,
    st10_tconfb: St10Tconfb,
    st11_tconfa: St11Tconfa,
    st11_tconfb: St11Tconfb,
    st12_tconfa: St12Tconfa,
    st12_tconfb: St12Tconfb,
    st13_tconfa: St13Tconfa,
    st13_tconfb: St13Tconfb,
    st14_tconfa: St14Tconfa,
    st14_tconfb: St14Tconfb,
    st15_tconfa: St15Tconfa,
    st15_tconfb: St15Tconfb,
    st16_tconfa: St16Tconfa,
    st16_tconfb: St16Tconfb,
    st17_tconfa: St17Tconfa,
    st17_tconfb: St17Tconfb,
    st18_tconfa: St18Tconfa,
    st18_tconfb: St18Tconfb,
    st19_tconfa: St19Tconfa,
    st19_tconfb: St19Tconfb,
    st20_tconfa: St20Tconfa,
    st20_tconfb: St20Tconfb,
    st21_tconfa: St21Tconfa,
    st21_tconfb: St21Tconfb,
    st22_tconfa: St22Tconfa,
    st22_tconfb: St22Tconfb,
    st23_tconfa: St23Tconfa,
    st23_tconfb: St23Tconfb,
    st24_tconfa: St24Tconfa,
    st24_tconfb: St24Tconfb,
    st25_tconfa: St25Tconfa,
    st25_tconfb: St25Tconfb,
    st26_tconfa: St26Tconfa,
    st26_tconfb: St26Tconfb,
    st27_tconfa: St27Tconfa,
    st27_tconfb: St27Tconfb,
    st28_tconfa: St28Tconfa,
    st28_tconfb: St28Tconfb,
    st29_tconfa: St29Tconfa,
    st29_tconfb: St29Tconfb,
    st30_tconfa: St30Tconfa,
    st30_tconfb: St30Tconfb,
    st31_tconfa: St31Tconfa,
    st31_tconfb: St31Tconfb,
    buf0_data: Buf0Data,
    buf1_data: Buf1Data,
    buf2_data: Buf2Data,
    buf3_data: Buf3Data,
    buf4_data: Buf4Data,
    buf5_data: Buf5Data,
    buf6_data: Buf6Data,
    buf7_data: Buf7Data,
    buf8_data: Buf8Data,
    buf9_data: Buf9Data,
    buf10_data: Buf10Data,
    buf11_data: Buf11Data,
    buf12_data: Buf12Data,
    buf13_data: Buf13Data,
    buf14_data: Buf14Data,
    buf15_data: Buf15Data,
    ch0_timing: Ch0Timing,
    ch0_interact: Ch0Interact,
    ch0_eval: Ch0Eval,
    _reserved107: [u8; 0x04],
    ch1_timing: Ch1Timing,
    ch1_interact: Ch1Interact,
    ch1_eval: Ch1Eval,
    _reserved110: [u8; 0x04],
    ch2_timing: Ch2Timing,
    ch2_interact: Ch2Interact,
    ch2_eval: Ch2Eval,
    _reserved113: [u8; 0x04],
    ch3_timing: Ch3Timing,
    ch3_interact: Ch3Interact,
    ch3_eval: Ch3Eval,
    _reserved116: [u8; 0x04],
    ch4_timing: Ch4Timing,
    ch4_interact: Ch4Interact,
    ch4_eval: Ch4Eval,
    _reserved119: [u8; 0x04],
    ch5_timing: Ch5Timing,
    ch5_interact: Ch5Interact,
    ch5_eval: Ch5Eval,
    _reserved122: [u8; 0x04],
    ch6_timing: Ch6Timing,
    ch6_interact: Ch6Interact,
    ch6_eval: Ch6Eval,
    _reserved125: [u8; 0x04],
    ch7_timing: Ch7Timing,
    ch7_interact: Ch7Interact,
    ch7_eval: Ch7Eval,
    _reserved128: [u8; 0x04],
    ch8_timing: Ch8Timing,
    ch8_interact: Ch8Interact,
    ch8_eval: Ch8Eval,
    _reserved131: [u8; 0x04],
    ch9_timing: Ch9Timing,
    ch9_interact: Ch9Interact,
    ch9_eval: Ch9Eval,
    _reserved134: [u8; 0x04],
    ch10_timing: Ch10Timing,
    ch10_interact: Ch10Interact,
    ch10_eval: Ch10Eval,
    _reserved137: [u8; 0x04],
    ch11_timing: Ch11Timing,
    ch11_interact: Ch11Interact,
    ch11_eval: Ch11Eval,
    _reserved140: [u8; 0x04],
    ch12_timing: Ch12Timing,
    ch12_interact: Ch12Interact,
    ch12_eval: Ch12Eval,
    _reserved143: [u8; 0x04],
    ch13_timing: Ch13Timing,
    ch13_interact: Ch13Interact,
    ch13_eval: Ch13Eval,
    _reserved146: [u8; 0x04],
    ch14_timing: Ch14Timing,
    ch14_interact: Ch14Interact,
    ch14_eval: Ch14Eval,
    _reserved149: [u8; 0x04],
    ch15_timing: Ch15Timing,
    ch15_interact: Ch15Interact,
    ch15_eval: Ch15Eval,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Timing Control Register"]
    #[inline(always)]
    pub const fn timctrl(&self) -> &Timctrl {
        &self.timctrl
    }
    #[doc = "0x08 - Peripheral Control Register"]
    #[inline(always)]
    pub const fn perctrl(&self) -> &Perctrl {
        &self.perctrl
    }
    #[doc = "0x0c - Decoder Control Register"]
    #[inline(always)]
    pub const fn decctrl(&self) -> &Decctrl {
        &self.decctrl
    }
    #[doc = "0x10 - Bias Control Register"]
    #[inline(always)]
    pub const fn biasctrl(&self) -> &Biasctrl {
        &self.biasctrl
    }
    #[doc = "0x14 - LESENSE Evaluation Control"]
    #[inline(always)]
    pub const fn evalctrl(&self) -> &Evalctrl {
        &self.evalctrl
    }
    #[doc = "0x18 - PRS Control Register"]
    #[inline(always)]
    pub const fn prsctrl(&self) -> &Prsctrl {
        &self.prsctrl
    }
    #[doc = "0x1c - Command Register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x20 - Channel Enable Register"]
    #[inline(always)]
    pub const fn chen(&self) -> &Chen {
        &self.chen
    }
    #[doc = "0x24 - Scan Result Register"]
    #[inline(always)]
    pub const fn scanres(&self) -> &Scanres {
        &self.scanres
    }
    #[doc = "0x28 - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x2c - Result Buffer Pointers"]
    #[inline(always)]
    pub const fn ptr(&self) -> &Ptr {
        &self.ptr
    }
    #[doc = "0x30 - Result Buffer Data Register"]
    #[inline(always)]
    pub const fn bufdata(&self) -> &Bufdata {
        &self.bufdata
    }
    #[doc = "0x34 - Current Channel Index"]
    #[inline(always)]
    pub const fn curch(&self) -> &Curch {
        &self.curch
    }
    #[doc = "0x38 - Current Decoder State"]
    #[inline(always)]
    pub const fn decstate(&self) -> &Decstate {
        &self.decstate
    }
    #[doc = "0x3c - Decoder Input Register"]
    #[inline(always)]
    pub const fn sensorstate(&self) -> &Sensorstate {
        &self.sensorstate
    }
    #[doc = "0x40 - GPIO Idle Phase Configuration"]
    #[inline(always)]
    pub const fn idleconf(&self) -> &Idleconf {
        &self.idleconf
    }
    #[doc = "0x44 - Alternative Excite Pin Configuration"]
    #[inline(always)]
    pub const fn altexconf(&self) -> &Altexconf {
        &self.altexconf
    }
    #[doc = "0x50 - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x54 - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &Ifs {
        &self.ifs
    }
    #[doc = "0x58 - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &Ifc {
        &self.ifc
    }
    #[doc = "0x5c - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x60 - Synchronization Busy Register"]
    #[inline(always)]
    pub const fn syncbusy(&self) -> &Syncbusy {
        &self.syncbusy
    }
    #[doc = "0x64 - I/O Routing Register"]
    #[inline(always)]
    pub const fn routepen(&self) -> &Routepen {
        &self.routepen
    }
    #[doc = "0x100 - State Transition Configuration a"]
    #[inline(always)]
    pub const fn st0_tconfa(&self) -> &St0Tconfa {
        &self.st0_tconfa
    }
    #[doc = "0x104 - State Transition Configuration B"]
    #[inline(always)]
    pub const fn st0_tconfb(&self) -> &St0Tconfb {
        &self.st0_tconfb
    }
    #[doc = "0x108 - State Transition Configuration a"]
    #[inline(always)]
    pub const fn st1_tconfa(&self) -> &St1Tconfa {
        &self.st1_tconfa
    }
    #[doc = "0x10c - State Transition Configuration B"]
    #[inline(always)]
    pub const fn st1_tconfb(&self) -> &St1Tconfb {
        &self.st1_tconfb
    }
    #[doc = "0x110 - State Transition Configuration a"]
    #[inline(always)]
    pub const fn st2_tconfa(&self) -> &St2Tconfa {
        &self.st2_tconfa
    }
    #[doc = "0x114 - State Transition Configuration B"]
    #[inline(always)]
    pub const fn st2_tconfb(&self) -> &St2Tconfb {
        &self.st2_tconfb
    }
    #[doc = "0x118 - State Transition Configuration a"]
    #[inline(always)]
    pub const fn st3_tconfa(&self) -> &St3Tconfa {
        &self.st3_tconfa
    }
    #[doc = "0x11c - State Transition Configuration B"]
    #[inline(always)]
    pub const fn st3_tconfb(&self) -> &St3Tconfb {
        &self.st3_tconfb
    }
    #[doc = "0x120 - State Transition Configuration a"]
    #[inline(always)]
    pub const fn st4_tconfa(&self) -> &St4Tconfa {
        &self.st4_tconfa
    }
    #[doc = "0x124 - State Transition Configuration B"]
    #[inline(always)]
    pub const fn st4_tconfb(&self) -> &St4Tconfb {
        &self.st4_tconfb
    }
    #[doc = "0x128 - State Transition Configuration a"]
    #[inline(always)]
    pub const fn st5_tconfa(&self) -> &St5Tconfa {
        &self.st5_tconfa
    }
    #[doc = "0x12c - State Transition Configuration B"]
    #[inline(always)]
    pub const fn st5_tconfb(&self) -> &St5Tconfb {
        &self.st5_tconfb
    }
    #[doc = "0x130 - State Transition Configuration a"]
    #[inline(always)]
    pub const fn st6_tconfa(&self) -> &St6Tconfa {
        &self.st6_tconfa
    }
    #[doc = "0x134 - State Transition Configuration B"]
    #[inline(always)]
    pub const fn st6_tconfb(&self) -> &St6Tconfb {
        &self.st6_tconfb
    }
    #[doc = "0x138 - State Transition Configuration a"]
    #[inline(always)]
    pub const fn st7_tconfa(&self) -> &St7Tconfa {
        &self.st7_tconfa
    }
    #[doc = "0x13c - State Transition Configuration B"]
    #[inline(always)]
    pub const fn st7_tconfb(&self) -> &St7Tconfb {
        &self.st7_tconfb
    }
    #[doc = "0x140 - State Transition Configuration a"]
    #[inline(always)]
    pub const fn st8_tconfa(&self) -> &St8Tconfa {
        &self.st8_tconfa
    }
    #[doc = "0x144 - State Transition Configuration B"]
    #[inline(always)]
    pub const fn st8_tconfb(&self) -> &St8Tconfb {
        &self.st8_tconfb
    }
    #[doc = "0x148 - State Transition Configuration a"]
    #[inline(always)]
    pub const fn st9_tconfa(&self) -> &St9Tconfa {
        &self.st9_tconfa
    }
    #[doc = "0x14c - State Transition Configuration B"]
    #[inline(always)]
    pub const fn st9_tconfb(&self) -> &St9Tconfb {
        &self.st9_tconfb
    }
    #[doc = "0x150 - State Transition Configuration a"]
    #[inline(always)]
    pub const fn st10_tconfa(&self) -> &St10Tconfa {
        &self.st10_tconfa
    }
    #[doc = "0x154 - State Transition Configuration B"]
    #[inline(always)]
    pub const fn st10_tconfb(&self) -> &St10Tconfb {
        &self.st10_tconfb
    }
    #[doc = "0x158 - State Transition Configuration a"]
    #[inline(always)]
    pub const fn st11_tconfa(&self) -> &St11Tconfa {
        &self.st11_tconfa
    }
    #[doc = "0x15c - State Transition Configuration B"]
    #[inline(always)]
    pub const fn st11_tconfb(&self) -> &St11Tconfb {
        &self.st11_tconfb
    }
    #[doc = "0x160 - State Transition Configuration a"]
    #[inline(always)]
    pub const fn st12_tconfa(&self) -> &St12Tconfa {
        &self.st12_tconfa
    }
    #[doc = "0x164 - State Transition Configuration B"]
    #[inline(always)]
    pub const fn st12_tconfb(&self) -> &St12Tconfb {
        &self.st12_tconfb
    }
    #[doc = "0x168 - State Transition Configuration a"]
    #[inline(always)]
    pub const fn st13_tconfa(&self) -> &St13Tconfa {
        &self.st13_tconfa
    }
    #[doc = "0x16c - State Transition Configuration B"]
    #[inline(always)]
    pub const fn st13_tconfb(&self) -> &St13Tconfb {
        &self.st13_tconfb
    }
    #[doc = "0x170 - State Transition Configuration a"]
    #[inline(always)]
    pub const fn st14_tconfa(&self) -> &St14Tconfa {
        &self.st14_tconfa
    }
    #[doc = "0x174 - State Transition Configuration B"]
    #[inline(always)]
    pub const fn st14_tconfb(&self) -> &St14Tconfb {
        &self.st14_tconfb
    }
    #[doc = "0x178 - State Transition Configuration a"]
    #[inline(always)]
    pub const fn st15_tconfa(&self) -> &St15Tconfa {
        &self.st15_tconfa
    }
    #[doc = "0x17c - State Transition Configuration B"]
    #[inline(always)]
    pub const fn st15_tconfb(&self) -> &St15Tconfb {
        &self.st15_tconfb
    }
    #[doc = "0x180 - State Transition Configuration a"]
    #[inline(always)]
    pub const fn st16_tconfa(&self) -> &St16Tconfa {
        &self.st16_tconfa
    }
    #[doc = "0x184 - State Transition Configuration B"]
    #[inline(always)]
    pub const fn st16_tconfb(&self) -> &St16Tconfb {
        &self.st16_tconfb
    }
    #[doc = "0x188 - State Transition Configuration a"]
    #[inline(always)]
    pub const fn st17_tconfa(&self) -> &St17Tconfa {
        &self.st17_tconfa
    }
    #[doc = "0x18c - State Transition Configuration B"]
    #[inline(always)]
    pub const fn st17_tconfb(&self) -> &St17Tconfb {
        &self.st17_tconfb
    }
    #[doc = "0x190 - State Transition Configuration a"]
    #[inline(always)]
    pub const fn st18_tconfa(&self) -> &St18Tconfa {
        &self.st18_tconfa
    }
    #[doc = "0x194 - State Transition Configuration B"]
    #[inline(always)]
    pub const fn st18_tconfb(&self) -> &St18Tconfb {
        &self.st18_tconfb
    }
    #[doc = "0x198 - State Transition Configuration a"]
    #[inline(always)]
    pub const fn st19_tconfa(&self) -> &St19Tconfa {
        &self.st19_tconfa
    }
    #[doc = "0x19c - State Transition Configuration B"]
    #[inline(always)]
    pub const fn st19_tconfb(&self) -> &St19Tconfb {
        &self.st19_tconfb
    }
    #[doc = "0x1a0 - State Transition Configuration a"]
    #[inline(always)]
    pub const fn st20_tconfa(&self) -> &St20Tconfa {
        &self.st20_tconfa
    }
    #[doc = "0x1a4 - State Transition Configuration B"]
    #[inline(always)]
    pub const fn st20_tconfb(&self) -> &St20Tconfb {
        &self.st20_tconfb
    }
    #[doc = "0x1a8 - State Transition Configuration a"]
    #[inline(always)]
    pub const fn st21_tconfa(&self) -> &St21Tconfa {
        &self.st21_tconfa
    }
    #[doc = "0x1ac - State Transition Configuration B"]
    #[inline(always)]
    pub const fn st21_tconfb(&self) -> &St21Tconfb {
        &self.st21_tconfb
    }
    #[doc = "0x1b0 - State Transition Configuration a"]
    #[inline(always)]
    pub const fn st22_tconfa(&self) -> &St22Tconfa {
        &self.st22_tconfa
    }
    #[doc = "0x1b4 - State Transition Configuration B"]
    #[inline(always)]
    pub const fn st22_tconfb(&self) -> &St22Tconfb {
        &self.st22_tconfb
    }
    #[doc = "0x1b8 - State Transition Configuration a"]
    #[inline(always)]
    pub const fn st23_tconfa(&self) -> &St23Tconfa {
        &self.st23_tconfa
    }
    #[doc = "0x1bc - State Transition Configuration B"]
    #[inline(always)]
    pub const fn st23_tconfb(&self) -> &St23Tconfb {
        &self.st23_tconfb
    }
    #[doc = "0x1c0 - State Transition Configuration a"]
    #[inline(always)]
    pub const fn st24_tconfa(&self) -> &St24Tconfa {
        &self.st24_tconfa
    }
    #[doc = "0x1c4 - State Transition Configuration B"]
    #[inline(always)]
    pub const fn st24_tconfb(&self) -> &St24Tconfb {
        &self.st24_tconfb
    }
    #[doc = "0x1c8 - State Transition Configuration a"]
    #[inline(always)]
    pub const fn st25_tconfa(&self) -> &St25Tconfa {
        &self.st25_tconfa
    }
    #[doc = "0x1cc - State Transition Configuration B"]
    #[inline(always)]
    pub const fn st25_tconfb(&self) -> &St25Tconfb {
        &self.st25_tconfb
    }
    #[doc = "0x1d0 - State Transition Configuration a"]
    #[inline(always)]
    pub const fn st26_tconfa(&self) -> &St26Tconfa {
        &self.st26_tconfa
    }
    #[doc = "0x1d4 - State Transition Configuration B"]
    #[inline(always)]
    pub const fn st26_tconfb(&self) -> &St26Tconfb {
        &self.st26_tconfb
    }
    #[doc = "0x1d8 - State Transition Configuration a"]
    #[inline(always)]
    pub const fn st27_tconfa(&self) -> &St27Tconfa {
        &self.st27_tconfa
    }
    #[doc = "0x1dc - State Transition Configuration B"]
    #[inline(always)]
    pub const fn st27_tconfb(&self) -> &St27Tconfb {
        &self.st27_tconfb
    }
    #[doc = "0x1e0 - State Transition Configuration a"]
    #[inline(always)]
    pub const fn st28_tconfa(&self) -> &St28Tconfa {
        &self.st28_tconfa
    }
    #[doc = "0x1e4 - State Transition Configuration B"]
    #[inline(always)]
    pub const fn st28_tconfb(&self) -> &St28Tconfb {
        &self.st28_tconfb
    }
    #[doc = "0x1e8 - State Transition Configuration a"]
    #[inline(always)]
    pub const fn st29_tconfa(&self) -> &St29Tconfa {
        &self.st29_tconfa
    }
    #[doc = "0x1ec - State Transition Configuration B"]
    #[inline(always)]
    pub const fn st29_tconfb(&self) -> &St29Tconfb {
        &self.st29_tconfb
    }
    #[doc = "0x1f0 - State Transition Configuration a"]
    #[inline(always)]
    pub const fn st30_tconfa(&self) -> &St30Tconfa {
        &self.st30_tconfa
    }
    #[doc = "0x1f4 - State Transition Configuration B"]
    #[inline(always)]
    pub const fn st30_tconfb(&self) -> &St30Tconfb {
        &self.st30_tconfb
    }
    #[doc = "0x1f8 - State Transition Configuration a"]
    #[inline(always)]
    pub const fn st31_tconfa(&self) -> &St31Tconfa {
        &self.st31_tconfa
    }
    #[doc = "0x1fc - State Transition Configuration B"]
    #[inline(always)]
    pub const fn st31_tconfb(&self) -> &St31Tconfb {
        &self.st31_tconfb
    }
    #[doc = "0x200 - Scan Results"]
    #[inline(always)]
    pub const fn buf0_data(&self) -> &Buf0Data {
        &self.buf0_data
    }
    #[doc = "0x204 - Scan Results"]
    #[inline(always)]
    pub const fn buf1_data(&self) -> &Buf1Data {
        &self.buf1_data
    }
    #[doc = "0x208 - Scan Results"]
    #[inline(always)]
    pub const fn buf2_data(&self) -> &Buf2Data {
        &self.buf2_data
    }
    #[doc = "0x20c - Scan Results"]
    #[inline(always)]
    pub const fn buf3_data(&self) -> &Buf3Data {
        &self.buf3_data
    }
    #[doc = "0x210 - Scan Results"]
    #[inline(always)]
    pub const fn buf4_data(&self) -> &Buf4Data {
        &self.buf4_data
    }
    #[doc = "0x214 - Scan Results"]
    #[inline(always)]
    pub const fn buf5_data(&self) -> &Buf5Data {
        &self.buf5_data
    }
    #[doc = "0x218 - Scan Results"]
    #[inline(always)]
    pub const fn buf6_data(&self) -> &Buf6Data {
        &self.buf6_data
    }
    #[doc = "0x21c - Scan Results"]
    #[inline(always)]
    pub const fn buf7_data(&self) -> &Buf7Data {
        &self.buf7_data
    }
    #[doc = "0x220 - Scan Results"]
    #[inline(always)]
    pub const fn buf8_data(&self) -> &Buf8Data {
        &self.buf8_data
    }
    #[doc = "0x224 - Scan Results"]
    #[inline(always)]
    pub const fn buf9_data(&self) -> &Buf9Data {
        &self.buf9_data
    }
    #[doc = "0x228 - Scan Results"]
    #[inline(always)]
    pub const fn buf10_data(&self) -> &Buf10Data {
        &self.buf10_data
    }
    #[doc = "0x22c - Scan Results"]
    #[inline(always)]
    pub const fn buf11_data(&self) -> &Buf11Data {
        &self.buf11_data
    }
    #[doc = "0x230 - Scan Results"]
    #[inline(always)]
    pub const fn buf12_data(&self) -> &Buf12Data {
        &self.buf12_data
    }
    #[doc = "0x234 - Scan Results"]
    #[inline(always)]
    pub const fn buf13_data(&self) -> &Buf13Data {
        &self.buf13_data
    }
    #[doc = "0x238 - Scan Results"]
    #[inline(always)]
    pub const fn buf14_data(&self) -> &Buf14Data {
        &self.buf14_data
    }
    #[doc = "0x23c - Scan Results"]
    #[inline(always)]
    pub const fn buf15_data(&self) -> &Buf15Data {
        &self.buf15_data
    }
    #[doc = "0x240 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch0_timing(&self) -> &Ch0Timing {
        &self.ch0_timing
    }
    #[doc = "0x244 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch0_interact(&self) -> &Ch0Interact {
        &self.ch0_interact
    }
    #[doc = "0x248 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch0_eval(&self) -> &Ch0Eval {
        &self.ch0_eval
    }
    #[doc = "0x250 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch1_timing(&self) -> &Ch1Timing {
        &self.ch1_timing
    }
    #[doc = "0x254 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch1_interact(&self) -> &Ch1Interact {
        &self.ch1_interact
    }
    #[doc = "0x258 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch1_eval(&self) -> &Ch1Eval {
        &self.ch1_eval
    }
    #[doc = "0x260 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch2_timing(&self) -> &Ch2Timing {
        &self.ch2_timing
    }
    #[doc = "0x264 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch2_interact(&self) -> &Ch2Interact {
        &self.ch2_interact
    }
    #[doc = "0x268 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch2_eval(&self) -> &Ch2Eval {
        &self.ch2_eval
    }
    #[doc = "0x270 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch3_timing(&self) -> &Ch3Timing {
        &self.ch3_timing
    }
    #[doc = "0x274 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch3_interact(&self) -> &Ch3Interact {
        &self.ch3_interact
    }
    #[doc = "0x278 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch3_eval(&self) -> &Ch3Eval {
        &self.ch3_eval
    }
    #[doc = "0x280 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch4_timing(&self) -> &Ch4Timing {
        &self.ch4_timing
    }
    #[doc = "0x284 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch4_interact(&self) -> &Ch4Interact {
        &self.ch4_interact
    }
    #[doc = "0x288 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch4_eval(&self) -> &Ch4Eval {
        &self.ch4_eval
    }
    #[doc = "0x290 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch5_timing(&self) -> &Ch5Timing {
        &self.ch5_timing
    }
    #[doc = "0x294 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch5_interact(&self) -> &Ch5Interact {
        &self.ch5_interact
    }
    #[doc = "0x298 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch5_eval(&self) -> &Ch5Eval {
        &self.ch5_eval
    }
    #[doc = "0x2a0 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch6_timing(&self) -> &Ch6Timing {
        &self.ch6_timing
    }
    #[doc = "0x2a4 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch6_interact(&self) -> &Ch6Interact {
        &self.ch6_interact
    }
    #[doc = "0x2a8 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch6_eval(&self) -> &Ch6Eval {
        &self.ch6_eval
    }
    #[doc = "0x2b0 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch7_timing(&self) -> &Ch7Timing {
        &self.ch7_timing
    }
    #[doc = "0x2b4 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch7_interact(&self) -> &Ch7Interact {
        &self.ch7_interact
    }
    #[doc = "0x2b8 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch7_eval(&self) -> &Ch7Eval {
        &self.ch7_eval
    }
    #[doc = "0x2c0 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch8_timing(&self) -> &Ch8Timing {
        &self.ch8_timing
    }
    #[doc = "0x2c4 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch8_interact(&self) -> &Ch8Interact {
        &self.ch8_interact
    }
    #[doc = "0x2c8 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch8_eval(&self) -> &Ch8Eval {
        &self.ch8_eval
    }
    #[doc = "0x2d0 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch9_timing(&self) -> &Ch9Timing {
        &self.ch9_timing
    }
    #[doc = "0x2d4 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch9_interact(&self) -> &Ch9Interact {
        &self.ch9_interact
    }
    #[doc = "0x2d8 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch9_eval(&self) -> &Ch9Eval {
        &self.ch9_eval
    }
    #[doc = "0x2e0 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch10_timing(&self) -> &Ch10Timing {
        &self.ch10_timing
    }
    #[doc = "0x2e4 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch10_interact(&self) -> &Ch10Interact {
        &self.ch10_interact
    }
    #[doc = "0x2e8 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch10_eval(&self) -> &Ch10Eval {
        &self.ch10_eval
    }
    #[doc = "0x2f0 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch11_timing(&self) -> &Ch11Timing {
        &self.ch11_timing
    }
    #[doc = "0x2f4 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch11_interact(&self) -> &Ch11Interact {
        &self.ch11_interact
    }
    #[doc = "0x2f8 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch11_eval(&self) -> &Ch11Eval {
        &self.ch11_eval
    }
    #[doc = "0x300 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch12_timing(&self) -> &Ch12Timing {
        &self.ch12_timing
    }
    #[doc = "0x304 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch12_interact(&self) -> &Ch12Interact {
        &self.ch12_interact
    }
    #[doc = "0x308 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch12_eval(&self) -> &Ch12Eval {
        &self.ch12_eval
    }
    #[doc = "0x310 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch13_timing(&self) -> &Ch13Timing {
        &self.ch13_timing
    }
    #[doc = "0x314 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch13_interact(&self) -> &Ch13Interact {
        &self.ch13_interact
    }
    #[doc = "0x318 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch13_eval(&self) -> &Ch13Eval {
        &self.ch13_eval
    }
    #[doc = "0x320 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch14_timing(&self) -> &Ch14Timing {
        &self.ch14_timing
    }
    #[doc = "0x324 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch14_interact(&self) -> &Ch14Interact {
        &self.ch14_interact
    }
    #[doc = "0x328 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch14_eval(&self) -> &Ch14Eval {
        &self.ch14_eval
    }
    #[doc = "0x330 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch15_timing(&self) -> &Ch15Timing {
        &self.ch15_timing
    }
    #[doc = "0x334 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch15_interact(&self) -> &Ch15Interact {
        &self.ch15_interact
    }
    #[doc = "0x338 - Scan Configuration"]
    #[inline(always)]
    pub const fn ch15_eval(&self) -> &Ch15Eval {
        &self.ch15_eval
    }
}
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "TIMCTRL (rw) register accessor: Timing Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timctrl`] module"]
#[doc(alias = "TIMCTRL")]
pub type Timctrl = crate::Reg<timctrl::TimctrlSpec>;
#[doc = "Timing Control Register"]
pub mod timctrl;
#[doc = "PERCTRL (rw) register accessor: Peripheral Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`perctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perctrl`] module"]
#[doc(alias = "PERCTRL")]
pub type Perctrl = crate::Reg<perctrl::PerctrlSpec>;
#[doc = "Peripheral Control Register"]
pub mod perctrl;
#[doc = "DECCTRL (rw) register accessor: Decoder Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`decctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`decctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@decctrl`] module"]
#[doc(alias = "DECCTRL")]
pub type Decctrl = crate::Reg<decctrl::DecctrlSpec>;
#[doc = "Decoder Control Register"]
pub mod decctrl;
#[doc = "BIASCTRL (rw) register accessor: Bias Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`biasctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`biasctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@biasctrl`] module"]
#[doc(alias = "BIASCTRL")]
pub type Biasctrl = crate::Reg<biasctrl::BiasctrlSpec>;
#[doc = "Bias Control Register"]
pub mod biasctrl;
#[doc = "EVALCTRL (rw) register accessor: LESENSE Evaluation Control\n\nYou can [`read`](crate::Reg::read) this register and get [`evalctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evalctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evalctrl`] module"]
#[doc(alias = "EVALCTRL")]
pub type Evalctrl = crate::Reg<evalctrl::EvalctrlSpec>;
#[doc = "LESENSE Evaluation Control"]
pub mod evalctrl;
#[doc = "PRSCTRL (rw) register accessor: PRS Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`prsctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prsctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prsctrl`] module"]
#[doc(alias = "PRSCTRL")]
pub type Prsctrl = crate::Reg<prsctrl::PrsctrlSpec>;
#[doc = "PRS Control Register"]
pub mod prsctrl;
#[doc = "CMD (w) register accessor: Command Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "CHEN (rw) register accessor: Channel Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`chen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chen`] module"]
#[doc(alias = "CHEN")]
pub type Chen = crate::Reg<chen::ChenSpec>;
#[doc = "Channel Enable Register"]
pub mod chen;
#[doc = "SCANRES (rw) register accessor: Scan Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scanres::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scanres::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scanres`] module"]
#[doc(alias = "SCANRES")]
pub type Scanres = crate::Reg<scanres::ScanresSpec>;
#[doc = "Scan Result Register"]
pub mod scanres;
#[doc = "STATUS (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status Register"]
pub mod status;
#[doc = "PTR (r) register accessor: Result Buffer Pointers\n\nYou can [`read`](crate::Reg::read) this register and get [`ptr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptr`] module"]
#[doc(alias = "PTR")]
pub type Ptr = crate::Reg<ptr::PtrSpec>;
#[doc = "Result Buffer Pointers"]
pub mod ptr;
#[doc = "BUFDATA (r) register accessor: Result Buffer Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bufdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@bufdata`] module"]
#[doc(alias = "BUFDATA")]
pub type Bufdata = crate::Reg<bufdata::BufdataSpec>;
#[doc = "Result Buffer Data Register"]
pub mod bufdata;
#[doc = "CURCH (r) register accessor: Current Channel Index\n\nYou can [`read`](crate::Reg::read) this register and get [`curch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@curch`] module"]
#[doc(alias = "CURCH")]
pub type Curch = crate::Reg<curch::CurchSpec>;
#[doc = "Current Channel Index"]
pub mod curch;
#[doc = "DECSTATE (rw) register accessor: Current Decoder State\n\nYou can [`read`](crate::Reg::read) this register and get [`decstate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`decstate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@decstate`] module"]
#[doc(alias = "DECSTATE")]
pub type Decstate = crate::Reg<decstate::DecstateSpec>;
#[doc = "Current Decoder State"]
pub mod decstate;
#[doc = "SENSORSTATE (rw) register accessor: Decoder Input Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sensorstate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sensorstate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sensorstate`] module"]
#[doc(alias = "SENSORSTATE")]
pub type Sensorstate = crate::Reg<sensorstate::SensorstateSpec>;
#[doc = "Decoder Input Register"]
pub mod sensorstate;
#[doc = "IDLECONF (rw) register accessor: GPIO Idle Phase Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`idleconf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idleconf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idleconf`] module"]
#[doc(alias = "IDLECONF")]
pub type Idleconf = crate::Reg<idleconf::IdleconfSpec>;
#[doc = "GPIO Idle Phase Configuration"]
pub mod idleconf;
#[doc = "ALTEXCONF (rw) register accessor: Alternative Excite Pin Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`altexconf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`altexconf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@altexconf`] module"]
#[doc(alias = "ALTEXCONF")]
pub type Altexconf = crate::Reg<altexconf::AltexconfSpec>;
#[doc = "Alternative Excite Pin Configuration"]
pub mod altexconf;
#[doc = "IF (r) register accessor: Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if_`] module"]
#[doc(alias = "IF")]
pub type If = crate::Reg<if_::IfSpec>;
#[doc = "Interrupt Flag Register"]
pub mod if_;
#[doc = "IFS (w) register accessor: Interrupt Flag Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifs::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifs`] module"]
#[doc(alias = "IFS")]
pub type Ifs = crate::Reg<ifs::IfsSpec>;
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "IFC (w) register accessor: Interrupt Flag Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifc`] module"]
#[doc(alias = "IFC")]
pub type Ifc = crate::Reg<ifc::IfcSpec>;
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "IEN (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ien`] module"]
#[doc(alias = "IEN")]
pub type Ien = crate::Reg<ien::IenSpec>;
#[doc = "Interrupt Enable Register"]
pub mod ien;
#[doc = "SYNCBUSY (r) register accessor: Synchronization Busy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`] module"]
#[doc(alias = "SYNCBUSY")]
pub type Syncbusy = crate::Reg<syncbusy::SyncbusySpec>;
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "ROUTEPEN (rw) register accessor: I/O Routing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`routepen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routepen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routepen`] module"]
#[doc(alias = "ROUTEPEN")]
pub type Routepen = crate::Reg<routepen::RoutepenSpec>;
#[doc = "I/O Routing Register"]
pub mod routepen;
#[doc = "ST0_TCONFA (rw) register accessor: State Transition Configuration a\n\nYou can [`read`](crate::Reg::read) this register and get [`st0_tconfa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st0_tconfa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st0_tconfa`] module"]
#[doc(alias = "ST0_TCONFA")]
pub type St0Tconfa = crate::Reg<st0_tconfa::St0TconfaSpec>;
#[doc = "State Transition Configuration a"]
pub mod st0_tconfa;
#[doc = "ST0_TCONFB (rw) register accessor: State Transition Configuration B\n\nYou can [`read`](crate::Reg::read) this register and get [`st0_tconfb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st0_tconfb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st0_tconfb`] module"]
#[doc(alias = "ST0_TCONFB")]
pub type St0Tconfb = crate::Reg<st0_tconfb::St0TconfbSpec>;
#[doc = "State Transition Configuration B"]
pub mod st0_tconfb;
#[doc = "ST1_TCONFA (rw) register accessor: State Transition Configuration a\n\nYou can [`read`](crate::Reg::read) this register and get [`st1_tconfa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st1_tconfa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st1_tconfa`] module"]
#[doc(alias = "ST1_TCONFA")]
pub type St1Tconfa = crate::Reg<st1_tconfa::St1TconfaSpec>;
#[doc = "State Transition Configuration a"]
pub mod st1_tconfa;
#[doc = "ST1_TCONFB (rw) register accessor: State Transition Configuration B\n\nYou can [`read`](crate::Reg::read) this register and get [`st1_tconfb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st1_tconfb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st1_tconfb`] module"]
#[doc(alias = "ST1_TCONFB")]
pub type St1Tconfb = crate::Reg<st1_tconfb::St1TconfbSpec>;
#[doc = "State Transition Configuration B"]
pub mod st1_tconfb;
#[doc = "ST2_TCONFA (rw) register accessor: State Transition Configuration a\n\nYou can [`read`](crate::Reg::read) this register and get [`st2_tconfa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st2_tconfa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st2_tconfa`] module"]
#[doc(alias = "ST2_TCONFA")]
pub type St2Tconfa = crate::Reg<st2_tconfa::St2TconfaSpec>;
#[doc = "State Transition Configuration a"]
pub mod st2_tconfa;
#[doc = "ST2_TCONFB (rw) register accessor: State Transition Configuration B\n\nYou can [`read`](crate::Reg::read) this register and get [`st2_tconfb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st2_tconfb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st2_tconfb`] module"]
#[doc(alias = "ST2_TCONFB")]
pub type St2Tconfb = crate::Reg<st2_tconfb::St2TconfbSpec>;
#[doc = "State Transition Configuration B"]
pub mod st2_tconfb;
#[doc = "ST3_TCONFA (rw) register accessor: State Transition Configuration a\n\nYou can [`read`](crate::Reg::read) this register and get [`st3_tconfa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st3_tconfa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st3_tconfa`] module"]
#[doc(alias = "ST3_TCONFA")]
pub type St3Tconfa = crate::Reg<st3_tconfa::St3TconfaSpec>;
#[doc = "State Transition Configuration a"]
pub mod st3_tconfa;
#[doc = "ST3_TCONFB (rw) register accessor: State Transition Configuration B\n\nYou can [`read`](crate::Reg::read) this register and get [`st3_tconfb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st3_tconfb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st3_tconfb`] module"]
#[doc(alias = "ST3_TCONFB")]
pub type St3Tconfb = crate::Reg<st3_tconfb::St3TconfbSpec>;
#[doc = "State Transition Configuration B"]
pub mod st3_tconfb;
#[doc = "ST4_TCONFA (rw) register accessor: State Transition Configuration a\n\nYou can [`read`](crate::Reg::read) this register and get [`st4_tconfa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st4_tconfa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st4_tconfa`] module"]
#[doc(alias = "ST4_TCONFA")]
pub type St4Tconfa = crate::Reg<st4_tconfa::St4TconfaSpec>;
#[doc = "State Transition Configuration a"]
pub mod st4_tconfa;
#[doc = "ST4_TCONFB (rw) register accessor: State Transition Configuration B\n\nYou can [`read`](crate::Reg::read) this register and get [`st4_tconfb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st4_tconfb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st4_tconfb`] module"]
#[doc(alias = "ST4_TCONFB")]
pub type St4Tconfb = crate::Reg<st4_tconfb::St4TconfbSpec>;
#[doc = "State Transition Configuration B"]
pub mod st4_tconfb;
#[doc = "ST5_TCONFA (rw) register accessor: State Transition Configuration a\n\nYou can [`read`](crate::Reg::read) this register and get [`st5_tconfa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st5_tconfa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st5_tconfa`] module"]
#[doc(alias = "ST5_TCONFA")]
pub type St5Tconfa = crate::Reg<st5_tconfa::St5TconfaSpec>;
#[doc = "State Transition Configuration a"]
pub mod st5_tconfa;
#[doc = "ST5_TCONFB (rw) register accessor: State Transition Configuration B\n\nYou can [`read`](crate::Reg::read) this register and get [`st5_tconfb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st5_tconfb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st5_tconfb`] module"]
#[doc(alias = "ST5_TCONFB")]
pub type St5Tconfb = crate::Reg<st5_tconfb::St5TconfbSpec>;
#[doc = "State Transition Configuration B"]
pub mod st5_tconfb;
#[doc = "ST6_TCONFA (rw) register accessor: State Transition Configuration a\n\nYou can [`read`](crate::Reg::read) this register and get [`st6_tconfa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st6_tconfa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st6_tconfa`] module"]
#[doc(alias = "ST6_TCONFA")]
pub type St6Tconfa = crate::Reg<st6_tconfa::St6TconfaSpec>;
#[doc = "State Transition Configuration a"]
pub mod st6_tconfa;
#[doc = "ST6_TCONFB (rw) register accessor: State Transition Configuration B\n\nYou can [`read`](crate::Reg::read) this register and get [`st6_tconfb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st6_tconfb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st6_tconfb`] module"]
#[doc(alias = "ST6_TCONFB")]
pub type St6Tconfb = crate::Reg<st6_tconfb::St6TconfbSpec>;
#[doc = "State Transition Configuration B"]
pub mod st6_tconfb;
#[doc = "ST7_TCONFA (rw) register accessor: State Transition Configuration a\n\nYou can [`read`](crate::Reg::read) this register and get [`st7_tconfa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st7_tconfa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st7_tconfa`] module"]
#[doc(alias = "ST7_TCONFA")]
pub type St7Tconfa = crate::Reg<st7_tconfa::St7TconfaSpec>;
#[doc = "State Transition Configuration a"]
pub mod st7_tconfa;
#[doc = "ST7_TCONFB (rw) register accessor: State Transition Configuration B\n\nYou can [`read`](crate::Reg::read) this register and get [`st7_tconfb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st7_tconfb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st7_tconfb`] module"]
#[doc(alias = "ST7_TCONFB")]
pub type St7Tconfb = crate::Reg<st7_tconfb::St7TconfbSpec>;
#[doc = "State Transition Configuration B"]
pub mod st7_tconfb;
#[doc = "ST8_TCONFA (rw) register accessor: State Transition Configuration a\n\nYou can [`read`](crate::Reg::read) this register and get [`st8_tconfa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st8_tconfa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st8_tconfa`] module"]
#[doc(alias = "ST8_TCONFA")]
pub type St8Tconfa = crate::Reg<st8_tconfa::St8TconfaSpec>;
#[doc = "State Transition Configuration a"]
pub mod st8_tconfa;
#[doc = "ST8_TCONFB (rw) register accessor: State Transition Configuration B\n\nYou can [`read`](crate::Reg::read) this register and get [`st8_tconfb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st8_tconfb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st8_tconfb`] module"]
#[doc(alias = "ST8_TCONFB")]
pub type St8Tconfb = crate::Reg<st8_tconfb::St8TconfbSpec>;
#[doc = "State Transition Configuration B"]
pub mod st8_tconfb;
#[doc = "ST9_TCONFA (rw) register accessor: State Transition Configuration a\n\nYou can [`read`](crate::Reg::read) this register and get [`st9_tconfa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st9_tconfa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st9_tconfa`] module"]
#[doc(alias = "ST9_TCONFA")]
pub type St9Tconfa = crate::Reg<st9_tconfa::St9TconfaSpec>;
#[doc = "State Transition Configuration a"]
pub mod st9_tconfa;
#[doc = "ST9_TCONFB (rw) register accessor: State Transition Configuration B\n\nYou can [`read`](crate::Reg::read) this register and get [`st9_tconfb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st9_tconfb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st9_tconfb`] module"]
#[doc(alias = "ST9_TCONFB")]
pub type St9Tconfb = crate::Reg<st9_tconfb::St9TconfbSpec>;
#[doc = "State Transition Configuration B"]
pub mod st9_tconfb;
#[doc = "ST10_TCONFA (rw) register accessor: State Transition Configuration a\n\nYou can [`read`](crate::Reg::read) this register and get [`st10_tconfa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st10_tconfa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st10_tconfa`] module"]
#[doc(alias = "ST10_TCONFA")]
pub type St10Tconfa = crate::Reg<st10_tconfa::St10TconfaSpec>;
#[doc = "State Transition Configuration a"]
pub mod st10_tconfa;
#[doc = "ST10_TCONFB (rw) register accessor: State Transition Configuration B\n\nYou can [`read`](crate::Reg::read) this register and get [`st10_tconfb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st10_tconfb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st10_tconfb`] module"]
#[doc(alias = "ST10_TCONFB")]
pub type St10Tconfb = crate::Reg<st10_tconfb::St10TconfbSpec>;
#[doc = "State Transition Configuration B"]
pub mod st10_tconfb;
#[doc = "ST11_TCONFA (rw) register accessor: State Transition Configuration a\n\nYou can [`read`](crate::Reg::read) this register and get [`st11_tconfa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st11_tconfa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st11_tconfa`] module"]
#[doc(alias = "ST11_TCONFA")]
pub type St11Tconfa = crate::Reg<st11_tconfa::St11TconfaSpec>;
#[doc = "State Transition Configuration a"]
pub mod st11_tconfa;
#[doc = "ST11_TCONFB (rw) register accessor: State Transition Configuration B\n\nYou can [`read`](crate::Reg::read) this register and get [`st11_tconfb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st11_tconfb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st11_tconfb`] module"]
#[doc(alias = "ST11_TCONFB")]
pub type St11Tconfb = crate::Reg<st11_tconfb::St11TconfbSpec>;
#[doc = "State Transition Configuration B"]
pub mod st11_tconfb;
#[doc = "ST12_TCONFA (rw) register accessor: State Transition Configuration a\n\nYou can [`read`](crate::Reg::read) this register and get [`st12_tconfa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st12_tconfa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st12_tconfa`] module"]
#[doc(alias = "ST12_TCONFA")]
pub type St12Tconfa = crate::Reg<st12_tconfa::St12TconfaSpec>;
#[doc = "State Transition Configuration a"]
pub mod st12_tconfa;
#[doc = "ST12_TCONFB (rw) register accessor: State Transition Configuration B\n\nYou can [`read`](crate::Reg::read) this register and get [`st12_tconfb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st12_tconfb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st12_tconfb`] module"]
#[doc(alias = "ST12_TCONFB")]
pub type St12Tconfb = crate::Reg<st12_tconfb::St12TconfbSpec>;
#[doc = "State Transition Configuration B"]
pub mod st12_tconfb;
#[doc = "ST13_TCONFA (rw) register accessor: State Transition Configuration a\n\nYou can [`read`](crate::Reg::read) this register and get [`st13_tconfa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st13_tconfa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st13_tconfa`] module"]
#[doc(alias = "ST13_TCONFA")]
pub type St13Tconfa = crate::Reg<st13_tconfa::St13TconfaSpec>;
#[doc = "State Transition Configuration a"]
pub mod st13_tconfa;
#[doc = "ST13_TCONFB (rw) register accessor: State Transition Configuration B\n\nYou can [`read`](crate::Reg::read) this register and get [`st13_tconfb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st13_tconfb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st13_tconfb`] module"]
#[doc(alias = "ST13_TCONFB")]
pub type St13Tconfb = crate::Reg<st13_tconfb::St13TconfbSpec>;
#[doc = "State Transition Configuration B"]
pub mod st13_tconfb;
#[doc = "ST14_TCONFA (rw) register accessor: State Transition Configuration a\n\nYou can [`read`](crate::Reg::read) this register and get [`st14_tconfa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st14_tconfa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st14_tconfa`] module"]
#[doc(alias = "ST14_TCONFA")]
pub type St14Tconfa = crate::Reg<st14_tconfa::St14TconfaSpec>;
#[doc = "State Transition Configuration a"]
pub mod st14_tconfa;
#[doc = "ST14_TCONFB (rw) register accessor: State Transition Configuration B\n\nYou can [`read`](crate::Reg::read) this register and get [`st14_tconfb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st14_tconfb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st14_tconfb`] module"]
#[doc(alias = "ST14_TCONFB")]
pub type St14Tconfb = crate::Reg<st14_tconfb::St14TconfbSpec>;
#[doc = "State Transition Configuration B"]
pub mod st14_tconfb;
#[doc = "ST15_TCONFA (rw) register accessor: State Transition Configuration a\n\nYou can [`read`](crate::Reg::read) this register and get [`st15_tconfa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st15_tconfa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st15_tconfa`] module"]
#[doc(alias = "ST15_TCONFA")]
pub type St15Tconfa = crate::Reg<st15_tconfa::St15TconfaSpec>;
#[doc = "State Transition Configuration a"]
pub mod st15_tconfa;
#[doc = "ST15_TCONFB (rw) register accessor: State Transition Configuration B\n\nYou can [`read`](crate::Reg::read) this register and get [`st15_tconfb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st15_tconfb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st15_tconfb`] module"]
#[doc(alias = "ST15_TCONFB")]
pub type St15Tconfb = crate::Reg<st15_tconfb::St15TconfbSpec>;
#[doc = "State Transition Configuration B"]
pub mod st15_tconfb;
#[doc = "ST16_TCONFA (rw) register accessor: State Transition Configuration a\n\nYou can [`read`](crate::Reg::read) this register and get [`st16_tconfa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st16_tconfa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st16_tconfa`] module"]
#[doc(alias = "ST16_TCONFA")]
pub type St16Tconfa = crate::Reg<st16_tconfa::St16TconfaSpec>;
#[doc = "State Transition Configuration a"]
pub mod st16_tconfa;
#[doc = "ST16_TCONFB (rw) register accessor: State Transition Configuration B\n\nYou can [`read`](crate::Reg::read) this register and get [`st16_tconfb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st16_tconfb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st16_tconfb`] module"]
#[doc(alias = "ST16_TCONFB")]
pub type St16Tconfb = crate::Reg<st16_tconfb::St16TconfbSpec>;
#[doc = "State Transition Configuration B"]
pub mod st16_tconfb;
#[doc = "ST17_TCONFA (rw) register accessor: State Transition Configuration a\n\nYou can [`read`](crate::Reg::read) this register and get [`st17_tconfa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st17_tconfa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st17_tconfa`] module"]
#[doc(alias = "ST17_TCONFA")]
pub type St17Tconfa = crate::Reg<st17_tconfa::St17TconfaSpec>;
#[doc = "State Transition Configuration a"]
pub mod st17_tconfa;
#[doc = "ST17_TCONFB (rw) register accessor: State Transition Configuration B\n\nYou can [`read`](crate::Reg::read) this register and get [`st17_tconfb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st17_tconfb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st17_tconfb`] module"]
#[doc(alias = "ST17_TCONFB")]
pub type St17Tconfb = crate::Reg<st17_tconfb::St17TconfbSpec>;
#[doc = "State Transition Configuration B"]
pub mod st17_tconfb;
#[doc = "ST18_TCONFA (rw) register accessor: State Transition Configuration a\n\nYou can [`read`](crate::Reg::read) this register and get [`st18_tconfa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st18_tconfa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st18_tconfa`] module"]
#[doc(alias = "ST18_TCONFA")]
pub type St18Tconfa = crate::Reg<st18_tconfa::St18TconfaSpec>;
#[doc = "State Transition Configuration a"]
pub mod st18_tconfa;
#[doc = "ST18_TCONFB (rw) register accessor: State Transition Configuration B\n\nYou can [`read`](crate::Reg::read) this register and get [`st18_tconfb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st18_tconfb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st18_tconfb`] module"]
#[doc(alias = "ST18_TCONFB")]
pub type St18Tconfb = crate::Reg<st18_tconfb::St18TconfbSpec>;
#[doc = "State Transition Configuration B"]
pub mod st18_tconfb;
#[doc = "ST19_TCONFA (rw) register accessor: State Transition Configuration a\n\nYou can [`read`](crate::Reg::read) this register and get [`st19_tconfa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st19_tconfa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st19_tconfa`] module"]
#[doc(alias = "ST19_TCONFA")]
pub type St19Tconfa = crate::Reg<st19_tconfa::St19TconfaSpec>;
#[doc = "State Transition Configuration a"]
pub mod st19_tconfa;
#[doc = "ST19_TCONFB (rw) register accessor: State Transition Configuration B\n\nYou can [`read`](crate::Reg::read) this register and get [`st19_tconfb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st19_tconfb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st19_tconfb`] module"]
#[doc(alias = "ST19_TCONFB")]
pub type St19Tconfb = crate::Reg<st19_tconfb::St19TconfbSpec>;
#[doc = "State Transition Configuration B"]
pub mod st19_tconfb;
#[doc = "ST20_TCONFA (rw) register accessor: State Transition Configuration a\n\nYou can [`read`](crate::Reg::read) this register and get [`st20_tconfa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st20_tconfa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st20_tconfa`] module"]
#[doc(alias = "ST20_TCONFA")]
pub type St20Tconfa = crate::Reg<st20_tconfa::St20TconfaSpec>;
#[doc = "State Transition Configuration a"]
pub mod st20_tconfa;
#[doc = "ST20_TCONFB (rw) register accessor: State Transition Configuration B\n\nYou can [`read`](crate::Reg::read) this register and get [`st20_tconfb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st20_tconfb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st20_tconfb`] module"]
#[doc(alias = "ST20_TCONFB")]
pub type St20Tconfb = crate::Reg<st20_tconfb::St20TconfbSpec>;
#[doc = "State Transition Configuration B"]
pub mod st20_tconfb;
#[doc = "ST21_TCONFA (rw) register accessor: State Transition Configuration a\n\nYou can [`read`](crate::Reg::read) this register and get [`st21_tconfa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st21_tconfa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st21_tconfa`] module"]
#[doc(alias = "ST21_TCONFA")]
pub type St21Tconfa = crate::Reg<st21_tconfa::St21TconfaSpec>;
#[doc = "State Transition Configuration a"]
pub mod st21_tconfa;
#[doc = "ST21_TCONFB (rw) register accessor: State Transition Configuration B\n\nYou can [`read`](crate::Reg::read) this register and get [`st21_tconfb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st21_tconfb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st21_tconfb`] module"]
#[doc(alias = "ST21_TCONFB")]
pub type St21Tconfb = crate::Reg<st21_tconfb::St21TconfbSpec>;
#[doc = "State Transition Configuration B"]
pub mod st21_tconfb;
#[doc = "ST22_TCONFA (rw) register accessor: State Transition Configuration a\n\nYou can [`read`](crate::Reg::read) this register and get [`st22_tconfa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st22_tconfa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st22_tconfa`] module"]
#[doc(alias = "ST22_TCONFA")]
pub type St22Tconfa = crate::Reg<st22_tconfa::St22TconfaSpec>;
#[doc = "State Transition Configuration a"]
pub mod st22_tconfa;
#[doc = "ST22_TCONFB (rw) register accessor: State Transition Configuration B\n\nYou can [`read`](crate::Reg::read) this register and get [`st22_tconfb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st22_tconfb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st22_tconfb`] module"]
#[doc(alias = "ST22_TCONFB")]
pub type St22Tconfb = crate::Reg<st22_tconfb::St22TconfbSpec>;
#[doc = "State Transition Configuration B"]
pub mod st22_tconfb;
#[doc = "ST23_TCONFA (rw) register accessor: State Transition Configuration a\n\nYou can [`read`](crate::Reg::read) this register and get [`st23_tconfa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st23_tconfa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st23_tconfa`] module"]
#[doc(alias = "ST23_TCONFA")]
pub type St23Tconfa = crate::Reg<st23_tconfa::St23TconfaSpec>;
#[doc = "State Transition Configuration a"]
pub mod st23_tconfa;
#[doc = "ST23_TCONFB (rw) register accessor: State Transition Configuration B\n\nYou can [`read`](crate::Reg::read) this register and get [`st23_tconfb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st23_tconfb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st23_tconfb`] module"]
#[doc(alias = "ST23_TCONFB")]
pub type St23Tconfb = crate::Reg<st23_tconfb::St23TconfbSpec>;
#[doc = "State Transition Configuration B"]
pub mod st23_tconfb;
#[doc = "ST24_TCONFA (rw) register accessor: State Transition Configuration a\n\nYou can [`read`](crate::Reg::read) this register and get [`st24_tconfa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st24_tconfa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st24_tconfa`] module"]
#[doc(alias = "ST24_TCONFA")]
pub type St24Tconfa = crate::Reg<st24_tconfa::St24TconfaSpec>;
#[doc = "State Transition Configuration a"]
pub mod st24_tconfa;
#[doc = "ST24_TCONFB (rw) register accessor: State Transition Configuration B\n\nYou can [`read`](crate::Reg::read) this register and get [`st24_tconfb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st24_tconfb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st24_tconfb`] module"]
#[doc(alias = "ST24_TCONFB")]
pub type St24Tconfb = crate::Reg<st24_tconfb::St24TconfbSpec>;
#[doc = "State Transition Configuration B"]
pub mod st24_tconfb;
#[doc = "ST25_TCONFA (rw) register accessor: State Transition Configuration a\n\nYou can [`read`](crate::Reg::read) this register and get [`st25_tconfa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st25_tconfa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st25_tconfa`] module"]
#[doc(alias = "ST25_TCONFA")]
pub type St25Tconfa = crate::Reg<st25_tconfa::St25TconfaSpec>;
#[doc = "State Transition Configuration a"]
pub mod st25_tconfa;
#[doc = "ST25_TCONFB (rw) register accessor: State Transition Configuration B\n\nYou can [`read`](crate::Reg::read) this register and get [`st25_tconfb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st25_tconfb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st25_tconfb`] module"]
#[doc(alias = "ST25_TCONFB")]
pub type St25Tconfb = crate::Reg<st25_tconfb::St25TconfbSpec>;
#[doc = "State Transition Configuration B"]
pub mod st25_tconfb;
#[doc = "ST26_TCONFA (rw) register accessor: State Transition Configuration a\n\nYou can [`read`](crate::Reg::read) this register and get [`st26_tconfa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st26_tconfa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st26_tconfa`] module"]
#[doc(alias = "ST26_TCONFA")]
pub type St26Tconfa = crate::Reg<st26_tconfa::St26TconfaSpec>;
#[doc = "State Transition Configuration a"]
pub mod st26_tconfa;
#[doc = "ST26_TCONFB (rw) register accessor: State Transition Configuration B\n\nYou can [`read`](crate::Reg::read) this register and get [`st26_tconfb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st26_tconfb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st26_tconfb`] module"]
#[doc(alias = "ST26_TCONFB")]
pub type St26Tconfb = crate::Reg<st26_tconfb::St26TconfbSpec>;
#[doc = "State Transition Configuration B"]
pub mod st26_tconfb;
#[doc = "ST27_TCONFA (rw) register accessor: State Transition Configuration a\n\nYou can [`read`](crate::Reg::read) this register and get [`st27_tconfa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st27_tconfa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st27_tconfa`] module"]
#[doc(alias = "ST27_TCONFA")]
pub type St27Tconfa = crate::Reg<st27_tconfa::St27TconfaSpec>;
#[doc = "State Transition Configuration a"]
pub mod st27_tconfa;
#[doc = "ST27_TCONFB (rw) register accessor: State Transition Configuration B\n\nYou can [`read`](crate::Reg::read) this register and get [`st27_tconfb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st27_tconfb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st27_tconfb`] module"]
#[doc(alias = "ST27_TCONFB")]
pub type St27Tconfb = crate::Reg<st27_tconfb::St27TconfbSpec>;
#[doc = "State Transition Configuration B"]
pub mod st27_tconfb;
#[doc = "ST28_TCONFA (rw) register accessor: State Transition Configuration a\n\nYou can [`read`](crate::Reg::read) this register and get [`st28_tconfa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st28_tconfa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st28_tconfa`] module"]
#[doc(alias = "ST28_TCONFA")]
pub type St28Tconfa = crate::Reg<st28_tconfa::St28TconfaSpec>;
#[doc = "State Transition Configuration a"]
pub mod st28_tconfa;
#[doc = "ST28_TCONFB (rw) register accessor: State Transition Configuration B\n\nYou can [`read`](crate::Reg::read) this register and get [`st28_tconfb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st28_tconfb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st28_tconfb`] module"]
#[doc(alias = "ST28_TCONFB")]
pub type St28Tconfb = crate::Reg<st28_tconfb::St28TconfbSpec>;
#[doc = "State Transition Configuration B"]
pub mod st28_tconfb;
#[doc = "ST29_TCONFA (rw) register accessor: State Transition Configuration a\n\nYou can [`read`](crate::Reg::read) this register and get [`st29_tconfa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st29_tconfa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st29_tconfa`] module"]
#[doc(alias = "ST29_TCONFA")]
pub type St29Tconfa = crate::Reg<st29_tconfa::St29TconfaSpec>;
#[doc = "State Transition Configuration a"]
pub mod st29_tconfa;
#[doc = "ST29_TCONFB (rw) register accessor: State Transition Configuration B\n\nYou can [`read`](crate::Reg::read) this register and get [`st29_tconfb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st29_tconfb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st29_tconfb`] module"]
#[doc(alias = "ST29_TCONFB")]
pub type St29Tconfb = crate::Reg<st29_tconfb::St29TconfbSpec>;
#[doc = "State Transition Configuration B"]
pub mod st29_tconfb;
#[doc = "ST30_TCONFA (rw) register accessor: State Transition Configuration a\n\nYou can [`read`](crate::Reg::read) this register and get [`st30_tconfa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st30_tconfa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st30_tconfa`] module"]
#[doc(alias = "ST30_TCONFA")]
pub type St30Tconfa = crate::Reg<st30_tconfa::St30TconfaSpec>;
#[doc = "State Transition Configuration a"]
pub mod st30_tconfa;
#[doc = "ST30_TCONFB (rw) register accessor: State Transition Configuration B\n\nYou can [`read`](crate::Reg::read) this register and get [`st30_tconfb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st30_tconfb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st30_tconfb`] module"]
#[doc(alias = "ST30_TCONFB")]
pub type St30Tconfb = crate::Reg<st30_tconfb::St30TconfbSpec>;
#[doc = "State Transition Configuration B"]
pub mod st30_tconfb;
#[doc = "ST31_TCONFA (rw) register accessor: State Transition Configuration a\n\nYou can [`read`](crate::Reg::read) this register and get [`st31_tconfa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st31_tconfa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st31_tconfa`] module"]
#[doc(alias = "ST31_TCONFA")]
pub type St31Tconfa = crate::Reg<st31_tconfa::St31TconfaSpec>;
#[doc = "State Transition Configuration a"]
pub mod st31_tconfa;
#[doc = "ST31_TCONFB (rw) register accessor: State Transition Configuration B\n\nYou can [`read`](crate::Reg::read) this register and get [`st31_tconfb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st31_tconfb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st31_tconfb`] module"]
#[doc(alias = "ST31_TCONFB")]
pub type St31Tconfb = crate::Reg<st31_tconfb::St31TconfbSpec>;
#[doc = "State Transition Configuration B"]
pub mod st31_tconfb;
#[doc = "BUF0_DATA (rw) register accessor: Scan Results\n\nYou can [`read`](crate::Reg::read) this register and get [`buf0_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf0_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf0_data`] module"]
#[doc(alias = "BUF0_DATA")]
pub type Buf0Data = crate::Reg<buf0_data::Buf0DataSpec>;
#[doc = "Scan Results"]
pub mod buf0_data;
#[doc = "BUF1_DATA (rw) register accessor: Scan Results\n\nYou can [`read`](crate::Reg::read) this register and get [`buf1_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf1_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf1_data`] module"]
#[doc(alias = "BUF1_DATA")]
pub type Buf1Data = crate::Reg<buf1_data::Buf1DataSpec>;
#[doc = "Scan Results"]
pub mod buf1_data;
#[doc = "BUF2_DATA (rw) register accessor: Scan Results\n\nYou can [`read`](crate::Reg::read) this register and get [`buf2_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf2_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf2_data`] module"]
#[doc(alias = "BUF2_DATA")]
pub type Buf2Data = crate::Reg<buf2_data::Buf2DataSpec>;
#[doc = "Scan Results"]
pub mod buf2_data;
#[doc = "BUF3_DATA (rw) register accessor: Scan Results\n\nYou can [`read`](crate::Reg::read) this register and get [`buf3_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf3_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf3_data`] module"]
#[doc(alias = "BUF3_DATA")]
pub type Buf3Data = crate::Reg<buf3_data::Buf3DataSpec>;
#[doc = "Scan Results"]
pub mod buf3_data;
#[doc = "BUF4_DATA (rw) register accessor: Scan Results\n\nYou can [`read`](crate::Reg::read) this register and get [`buf4_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf4_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf4_data`] module"]
#[doc(alias = "BUF4_DATA")]
pub type Buf4Data = crate::Reg<buf4_data::Buf4DataSpec>;
#[doc = "Scan Results"]
pub mod buf4_data;
#[doc = "BUF5_DATA (rw) register accessor: Scan Results\n\nYou can [`read`](crate::Reg::read) this register and get [`buf5_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf5_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf5_data`] module"]
#[doc(alias = "BUF5_DATA")]
pub type Buf5Data = crate::Reg<buf5_data::Buf5DataSpec>;
#[doc = "Scan Results"]
pub mod buf5_data;
#[doc = "BUF6_DATA (rw) register accessor: Scan Results\n\nYou can [`read`](crate::Reg::read) this register and get [`buf6_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf6_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf6_data`] module"]
#[doc(alias = "BUF6_DATA")]
pub type Buf6Data = crate::Reg<buf6_data::Buf6DataSpec>;
#[doc = "Scan Results"]
pub mod buf6_data;
#[doc = "BUF7_DATA (rw) register accessor: Scan Results\n\nYou can [`read`](crate::Reg::read) this register and get [`buf7_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf7_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf7_data`] module"]
#[doc(alias = "BUF7_DATA")]
pub type Buf7Data = crate::Reg<buf7_data::Buf7DataSpec>;
#[doc = "Scan Results"]
pub mod buf7_data;
#[doc = "BUF8_DATA (rw) register accessor: Scan Results\n\nYou can [`read`](crate::Reg::read) this register and get [`buf8_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf8_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf8_data`] module"]
#[doc(alias = "BUF8_DATA")]
pub type Buf8Data = crate::Reg<buf8_data::Buf8DataSpec>;
#[doc = "Scan Results"]
pub mod buf8_data;
#[doc = "BUF9_DATA (rw) register accessor: Scan Results\n\nYou can [`read`](crate::Reg::read) this register and get [`buf9_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf9_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf9_data`] module"]
#[doc(alias = "BUF9_DATA")]
pub type Buf9Data = crate::Reg<buf9_data::Buf9DataSpec>;
#[doc = "Scan Results"]
pub mod buf9_data;
#[doc = "BUF10_DATA (rw) register accessor: Scan Results\n\nYou can [`read`](crate::Reg::read) this register and get [`buf10_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf10_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf10_data`] module"]
#[doc(alias = "BUF10_DATA")]
pub type Buf10Data = crate::Reg<buf10_data::Buf10DataSpec>;
#[doc = "Scan Results"]
pub mod buf10_data;
#[doc = "BUF11_DATA (rw) register accessor: Scan Results\n\nYou can [`read`](crate::Reg::read) this register and get [`buf11_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf11_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf11_data`] module"]
#[doc(alias = "BUF11_DATA")]
pub type Buf11Data = crate::Reg<buf11_data::Buf11DataSpec>;
#[doc = "Scan Results"]
pub mod buf11_data;
#[doc = "BUF12_DATA (rw) register accessor: Scan Results\n\nYou can [`read`](crate::Reg::read) this register and get [`buf12_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf12_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf12_data`] module"]
#[doc(alias = "BUF12_DATA")]
pub type Buf12Data = crate::Reg<buf12_data::Buf12DataSpec>;
#[doc = "Scan Results"]
pub mod buf12_data;
#[doc = "BUF13_DATA (rw) register accessor: Scan Results\n\nYou can [`read`](crate::Reg::read) this register and get [`buf13_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf13_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf13_data`] module"]
#[doc(alias = "BUF13_DATA")]
pub type Buf13Data = crate::Reg<buf13_data::Buf13DataSpec>;
#[doc = "Scan Results"]
pub mod buf13_data;
#[doc = "BUF14_DATA (rw) register accessor: Scan Results\n\nYou can [`read`](crate::Reg::read) this register and get [`buf14_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf14_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf14_data`] module"]
#[doc(alias = "BUF14_DATA")]
pub type Buf14Data = crate::Reg<buf14_data::Buf14DataSpec>;
#[doc = "Scan Results"]
pub mod buf14_data;
#[doc = "BUF15_DATA (rw) register accessor: Scan Results\n\nYou can [`read`](crate::Reg::read) this register and get [`buf15_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf15_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf15_data`] module"]
#[doc(alias = "BUF15_DATA")]
pub type Buf15Data = crate::Reg<buf15_data::Buf15DataSpec>;
#[doc = "Scan Results"]
pub mod buf15_data;
#[doc = "CH0_TIMING (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_timing`] module"]
#[doc(alias = "CH0_TIMING")]
pub type Ch0Timing = crate::Reg<ch0_timing::Ch0TimingSpec>;
#[doc = "Scan Configuration"]
pub mod ch0_timing;
#[doc = "CH0_INTERACT (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0_interact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0_interact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_interact`] module"]
#[doc(alias = "CH0_INTERACT")]
pub type Ch0Interact = crate::Reg<ch0_interact::Ch0InteractSpec>;
#[doc = "Scan Configuration"]
pub mod ch0_interact;
#[doc = "CH0_EVAL (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0_eval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0_eval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_eval`] module"]
#[doc(alias = "CH0_EVAL")]
pub type Ch0Eval = crate::Reg<ch0_eval::Ch0EvalSpec>;
#[doc = "Scan Configuration"]
pub mod ch0_eval;
#[doc = "CH1_TIMING (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_timing`] module"]
#[doc(alias = "CH1_TIMING")]
pub type Ch1Timing = crate::Reg<ch1_timing::Ch1TimingSpec>;
#[doc = "Scan Configuration"]
pub mod ch1_timing;
#[doc = "CH1_INTERACT (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1_interact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_interact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_interact`] module"]
#[doc(alias = "CH1_INTERACT")]
pub type Ch1Interact = crate::Reg<ch1_interact::Ch1InteractSpec>;
#[doc = "Scan Configuration"]
pub mod ch1_interact;
#[doc = "CH1_EVAL (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1_eval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_eval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_eval`] module"]
#[doc(alias = "CH1_EVAL")]
pub type Ch1Eval = crate::Reg<ch1_eval::Ch1EvalSpec>;
#[doc = "Scan Configuration"]
pub mod ch1_eval;
#[doc = "CH2_TIMING (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_timing`] module"]
#[doc(alias = "CH2_TIMING")]
pub type Ch2Timing = crate::Reg<ch2_timing::Ch2TimingSpec>;
#[doc = "Scan Configuration"]
pub mod ch2_timing;
#[doc = "CH2_INTERACT (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_interact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_interact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_interact`] module"]
#[doc(alias = "CH2_INTERACT")]
pub type Ch2Interact = crate::Reg<ch2_interact::Ch2InteractSpec>;
#[doc = "Scan Configuration"]
pub mod ch2_interact;
#[doc = "CH2_EVAL (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_eval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_eval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_eval`] module"]
#[doc(alias = "CH2_EVAL")]
pub type Ch2Eval = crate::Reg<ch2_eval::Ch2EvalSpec>;
#[doc = "Scan Configuration"]
pub mod ch2_eval;
#[doc = "CH3_TIMING (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_timing`] module"]
#[doc(alias = "CH3_TIMING")]
pub type Ch3Timing = crate::Reg<ch3_timing::Ch3TimingSpec>;
#[doc = "Scan Configuration"]
pub mod ch3_timing;
#[doc = "CH3_INTERACT (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_interact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_interact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_interact`] module"]
#[doc(alias = "CH3_INTERACT")]
pub type Ch3Interact = crate::Reg<ch3_interact::Ch3InteractSpec>;
#[doc = "Scan Configuration"]
pub mod ch3_interact;
#[doc = "CH3_EVAL (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_eval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_eval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_eval`] module"]
#[doc(alias = "CH3_EVAL")]
pub type Ch3Eval = crate::Reg<ch3_eval::Ch3EvalSpec>;
#[doc = "Scan Configuration"]
pub mod ch3_eval;
#[doc = "CH4_TIMING (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_timing`] module"]
#[doc(alias = "CH4_TIMING")]
pub type Ch4Timing = crate::Reg<ch4_timing::Ch4TimingSpec>;
#[doc = "Scan Configuration"]
pub mod ch4_timing;
#[doc = "CH4_INTERACT (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_interact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_interact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_interact`] module"]
#[doc(alias = "CH4_INTERACT")]
pub type Ch4Interact = crate::Reg<ch4_interact::Ch4InteractSpec>;
#[doc = "Scan Configuration"]
pub mod ch4_interact;
#[doc = "CH4_EVAL (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_eval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_eval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_eval`] module"]
#[doc(alias = "CH4_EVAL")]
pub type Ch4Eval = crate::Reg<ch4_eval::Ch4EvalSpec>;
#[doc = "Scan Configuration"]
pub mod ch4_eval;
#[doc = "CH5_TIMING (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_timing`] module"]
#[doc(alias = "CH5_TIMING")]
pub type Ch5Timing = crate::Reg<ch5_timing::Ch5TimingSpec>;
#[doc = "Scan Configuration"]
pub mod ch5_timing;
#[doc = "CH5_INTERACT (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5_interact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5_interact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_interact`] module"]
#[doc(alias = "CH5_INTERACT")]
pub type Ch5Interact = crate::Reg<ch5_interact::Ch5InteractSpec>;
#[doc = "Scan Configuration"]
pub mod ch5_interact;
#[doc = "CH5_EVAL (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5_eval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5_eval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_eval`] module"]
#[doc(alias = "CH5_EVAL")]
pub type Ch5Eval = crate::Reg<ch5_eval::Ch5EvalSpec>;
#[doc = "Scan Configuration"]
pub mod ch5_eval;
#[doc = "CH6_TIMING (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_timing`] module"]
#[doc(alias = "CH6_TIMING")]
pub type Ch6Timing = crate::Reg<ch6_timing::Ch6TimingSpec>;
#[doc = "Scan Configuration"]
pub mod ch6_timing;
#[doc = "CH6_INTERACT (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6_interact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6_interact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_interact`] module"]
#[doc(alias = "CH6_INTERACT")]
pub type Ch6Interact = crate::Reg<ch6_interact::Ch6InteractSpec>;
#[doc = "Scan Configuration"]
pub mod ch6_interact;
#[doc = "CH6_EVAL (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6_eval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6_eval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_eval`] module"]
#[doc(alias = "CH6_EVAL")]
pub type Ch6Eval = crate::Reg<ch6_eval::Ch6EvalSpec>;
#[doc = "Scan Configuration"]
pub mod ch6_eval;
#[doc = "CH7_TIMING (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_timing`] module"]
#[doc(alias = "CH7_TIMING")]
pub type Ch7Timing = crate::Reg<ch7_timing::Ch7TimingSpec>;
#[doc = "Scan Configuration"]
pub mod ch7_timing;
#[doc = "CH7_INTERACT (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7_interact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7_interact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_interact`] module"]
#[doc(alias = "CH7_INTERACT")]
pub type Ch7Interact = crate::Reg<ch7_interact::Ch7InteractSpec>;
#[doc = "Scan Configuration"]
pub mod ch7_interact;
#[doc = "CH7_EVAL (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7_eval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7_eval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_eval`] module"]
#[doc(alias = "CH7_EVAL")]
pub type Ch7Eval = crate::Reg<ch7_eval::Ch7EvalSpec>;
#[doc = "Scan Configuration"]
pub mod ch7_eval;
#[doc = "CH8_TIMING (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch8_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch8_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch8_timing`] module"]
#[doc(alias = "CH8_TIMING")]
pub type Ch8Timing = crate::Reg<ch8_timing::Ch8TimingSpec>;
#[doc = "Scan Configuration"]
pub mod ch8_timing;
#[doc = "CH8_INTERACT (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch8_interact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch8_interact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch8_interact`] module"]
#[doc(alias = "CH8_INTERACT")]
pub type Ch8Interact = crate::Reg<ch8_interact::Ch8InteractSpec>;
#[doc = "Scan Configuration"]
pub mod ch8_interact;
#[doc = "CH8_EVAL (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch8_eval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch8_eval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch8_eval`] module"]
#[doc(alias = "CH8_EVAL")]
pub type Ch8Eval = crate::Reg<ch8_eval::Ch8EvalSpec>;
#[doc = "Scan Configuration"]
pub mod ch8_eval;
#[doc = "CH9_TIMING (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch9_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch9_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch9_timing`] module"]
#[doc(alias = "CH9_TIMING")]
pub type Ch9Timing = crate::Reg<ch9_timing::Ch9TimingSpec>;
#[doc = "Scan Configuration"]
pub mod ch9_timing;
#[doc = "CH9_INTERACT (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch9_interact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch9_interact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch9_interact`] module"]
#[doc(alias = "CH9_INTERACT")]
pub type Ch9Interact = crate::Reg<ch9_interact::Ch9InteractSpec>;
#[doc = "Scan Configuration"]
pub mod ch9_interact;
#[doc = "CH9_EVAL (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch9_eval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch9_eval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch9_eval`] module"]
#[doc(alias = "CH9_EVAL")]
pub type Ch9Eval = crate::Reg<ch9_eval::Ch9EvalSpec>;
#[doc = "Scan Configuration"]
pub mod ch9_eval;
#[doc = "CH10_TIMING (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch10_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch10_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch10_timing`] module"]
#[doc(alias = "CH10_TIMING")]
pub type Ch10Timing = crate::Reg<ch10_timing::Ch10TimingSpec>;
#[doc = "Scan Configuration"]
pub mod ch10_timing;
#[doc = "CH10_INTERACT (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch10_interact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch10_interact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch10_interact`] module"]
#[doc(alias = "CH10_INTERACT")]
pub type Ch10Interact = crate::Reg<ch10_interact::Ch10InteractSpec>;
#[doc = "Scan Configuration"]
pub mod ch10_interact;
#[doc = "CH10_EVAL (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch10_eval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch10_eval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch10_eval`] module"]
#[doc(alias = "CH10_EVAL")]
pub type Ch10Eval = crate::Reg<ch10_eval::Ch10EvalSpec>;
#[doc = "Scan Configuration"]
pub mod ch10_eval;
#[doc = "CH11_TIMING (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch11_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch11_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch11_timing`] module"]
#[doc(alias = "CH11_TIMING")]
pub type Ch11Timing = crate::Reg<ch11_timing::Ch11TimingSpec>;
#[doc = "Scan Configuration"]
pub mod ch11_timing;
#[doc = "CH11_INTERACT (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch11_interact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch11_interact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch11_interact`] module"]
#[doc(alias = "CH11_INTERACT")]
pub type Ch11Interact = crate::Reg<ch11_interact::Ch11InteractSpec>;
#[doc = "Scan Configuration"]
pub mod ch11_interact;
#[doc = "CH11_EVAL (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch11_eval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch11_eval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch11_eval`] module"]
#[doc(alias = "CH11_EVAL")]
pub type Ch11Eval = crate::Reg<ch11_eval::Ch11EvalSpec>;
#[doc = "Scan Configuration"]
pub mod ch11_eval;
#[doc = "CH12_TIMING (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch12_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch12_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch12_timing`] module"]
#[doc(alias = "CH12_TIMING")]
pub type Ch12Timing = crate::Reg<ch12_timing::Ch12TimingSpec>;
#[doc = "Scan Configuration"]
pub mod ch12_timing;
#[doc = "CH12_INTERACT (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch12_interact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch12_interact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch12_interact`] module"]
#[doc(alias = "CH12_INTERACT")]
pub type Ch12Interact = crate::Reg<ch12_interact::Ch12InteractSpec>;
#[doc = "Scan Configuration"]
pub mod ch12_interact;
#[doc = "CH12_EVAL (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch12_eval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch12_eval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch12_eval`] module"]
#[doc(alias = "CH12_EVAL")]
pub type Ch12Eval = crate::Reg<ch12_eval::Ch12EvalSpec>;
#[doc = "Scan Configuration"]
pub mod ch12_eval;
#[doc = "CH13_TIMING (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch13_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch13_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch13_timing`] module"]
#[doc(alias = "CH13_TIMING")]
pub type Ch13Timing = crate::Reg<ch13_timing::Ch13TimingSpec>;
#[doc = "Scan Configuration"]
pub mod ch13_timing;
#[doc = "CH13_INTERACT (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch13_interact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch13_interact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch13_interact`] module"]
#[doc(alias = "CH13_INTERACT")]
pub type Ch13Interact = crate::Reg<ch13_interact::Ch13InteractSpec>;
#[doc = "Scan Configuration"]
pub mod ch13_interact;
#[doc = "CH13_EVAL (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch13_eval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch13_eval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch13_eval`] module"]
#[doc(alias = "CH13_EVAL")]
pub type Ch13Eval = crate::Reg<ch13_eval::Ch13EvalSpec>;
#[doc = "Scan Configuration"]
pub mod ch13_eval;
#[doc = "CH14_TIMING (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch14_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch14_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch14_timing`] module"]
#[doc(alias = "CH14_TIMING")]
pub type Ch14Timing = crate::Reg<ch14_timing::Ch14TimingSpec>;
#[doc = "Scan Configuration"]
pub mod ch14_timing;
#[doc = "CH14_INTERACT (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch14_interact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch14_interact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch14_interact`] module"]
#[doc(alias = "CH14_INTERACT")]
pub type Ch14Interact = crate::Reg<ch14_interact::Ch14InteractSpec>;
#[doc = "Scan Configuration"]
pub mod ch14_interact;
#[doc = "CH14_EVAL (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch14_eval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch14_eval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch14_eval`] module"]
#[doc(alias = "CH14_EVAL")]
pub type Ch14Eval = crate::Reg<ch14_eval::Ch14EvalSpec>;
#[doc = "Scan Configuration"]
pub mod ch14_eval;
#[doc = "CH15_TIMING (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch15_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch15_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch15_timing`] module"]
#[doc(alias = "CH15_TIMING")]
pub type Ch15Timing = crate::Reg<ch15_timing::Ch15TimingSpec>;
#[doc = "Scan Configuration"]
pub mod ch15_timing;
#[doc = "CH15_INTERACT (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch15_interact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch15_interact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch15_interact`] module"]
#[doc(alias = "CH15_INTERACT")]
pub type Ch15Interact = crate::Reg<ch15_interact::Ch15InteractSpec>;
#[doc = "Scan Configuration"]
pub mod ch15_interact;
#[doc = "CH15_EVAL (rw) register accessor: Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch15_eval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch15_eval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch15_eval`] module"]
#[doc(alias = "CH15_EVAL")]
pub type Ch15Eval = crate::Reg<ch15_eval::Ch15EvalSpec>;
#[doc = "Scan Configuration"]
pub mod ch15_eval;
