#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `EN` reader - CSEN Enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - CSEN Enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPPOL` reader - CSEN Digital Comparator Polarity Select"]
pub type CmppolR = crate::BitReader;
#[doc = "Field `CMPPOL` writer - CSEN Digital Comparator Polarity Select"]
pub type CmppolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "CSEN Conversion Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cm {
    #[doc = "0: Single Channel Mode: One conversion of a single channel (when MCE = 0) or set of bonded channels (when MCE = 1) per conversion trigger."]
    Sgl = 0,
    #[doc = "1: Scan Mode: Scans multiple selected channels once per conversion trigger."]
    Scan = 1,
    #[doc = "2: Continuous Single Channel: Continuous conversion of a single channel (when MCE = 0) or set of bonded channels (when MCE = 1)."]
    Contsgl = 2,
    #[doc = "3: Continuous Scan Mode: Continuously scans multiple selected channels."]
    Contscan = 3,
}
impl From<Cm> for u8 {
    #[inline(always)]
    fn from(variant: Cm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cm {
    type Ux = u8;
}
impl crate::IsEnum for Cm {}
#[doc = "Field `CM` reader - CSEN Conversion Mode Select"]
pub type CmR = crate::FieldReader<Cm>;
impl CmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cm {
        match self.bits {
            0 => Cm::Sgl,
            1 => Cm::Scan,
            2 => Cm::Contsgl,
            3 => Cm::Contscan,
            _ => unreachable!(),
        }
    }
    #[doc = "Single Channel Mode: One conversion of a single channel (when MCE = 0) or set of bonded channels (when MCE = 1) per conversion trigger."]
    #[inline(always)]
    pub fn is_sgl(&self) -> bool {
        *self == Cm::Sgl
    }
    #[doc = "Scan Mode: Scans multiple selected channels once per conversion trigger."]
    #[inline(always)]
    pub fn is_scan(&self) -> bool {
        *self == Cm::Scan
    }
    #[doc = "Continuous Single Channel: Continuous conversion of a single channel (when MCE = 0) or set of bonded channels (when MCE = 1)."]
    #[inline(always)]
    pub fn is_contsgl(&self) -> bool {
        *self == Cm::Contsgl
    }
    #[doc = "Continuous Scan Mode: Continuously scans multiple selected channels."]
    #[inline(always)]
    pub fn is_contscan(&self) -> bool {
        *self == Cm::Contscan
    }
}
#[doc = "Field `CM` writer - CSEN Conversion Mode Select"]
pub type CmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cm, crate::Safe>;
impl<'a, REG> CmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single Channel Mode: One conversion of a single channel (when MCE = 0) or set of bonded channels (when MCE = 1) per conversion trigger."]
    #[inline(always)]
    pub fn sgl(self) -> &'a mut crate::W<REG> {
        self.variant(Cm::Sgl)
    }
    #[doc = "Scan Mode: Scans multiple selected channels once per conversion trigger."]
    #[inline(always)]
    pub fn scan(self) -> &'a mut crate::W<REG> {
        self.variant(Cm::Scan)
    }
    #[doc = "Continuous Single Channel: Continuous conversion of a single channel (when MCE = 0) or set of bonded channels (when MCE = 1)."]
    #[inline(always)]
    pub fn contsgl(self) -> &'a mut crate::W<REG> {
        self.variant(Cm::Contsgl)
    }
    #[doc = "Continuous Scan Mode: Continuously scans multiple selected channels."]
    #[inline(always)]
    pub fn contscan(self) -> &'a mut crate::W<REG> {
        self.variant(Cm::Contscan)
    }
}
#[doc = "SAR Conversion Resolution.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sarcr {
    #[doc = "0: Conversions last 10 internal CSEN clocks and are 10-bits in length."]
    Clk10 = 0,
    #[doc = "1: Conversions last 12 internal CSEN clocks and are 12-bits in length."]
    Clk12 = 1,
    #[doc = "2: Conversions last 14 internal CSEN clocks and are 14-bits in length."]
    Clk14 = 2,
    #[doc = "3: Conversions last 16 internal CSEN clocks and are 16-bits in length."]
    Clk16 = 3,
}
impl From<Sarcr> for u8 {
    #[inline(always)]
    fn from(variant: Sarcr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sarcr {
    type Ux = u8;
}
impl crate::IsEnum for Sarcr {}
#[doc = "Field `SARCR` reader - SAR Conversion Resolution."]
pub type SarcrR = crate::FieldReader<Sarcr>;
impl SarcrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sarcr {
        match self.bits {
            0 => Sarcr::Clk10,
            1 => Sarcr::Clk12,
            2 => Sarcr::Clk14,
            3 => Sarcr::Clk16,
            _ => unreachable!(),
        }
    }
    #[doc = "Conversions last 10 internal CSEN clocks and are 10-bits in length."]
    #[inline(always)]
    pub fn is_clk10(&self) -> bool {
        *self == Sarcr::Clk10
    }
    #[doc = "Conversions last 12 internal CSEN clocks and are 12-bits in length."]
    #[inline(always)]
    pub fn is_clk12(&self) -> bool {
        *self == Sarcr::Clk12
    }
    #[doc = "Conversions last 14 internal CSEN clocks and are 14-bits in length."]
    #[inline(always)]
    pub fn is_clk14(&self) -> bool {
        *self == Sarcr::Clk14
    }
    #[doc = "Conversions last 16 internal CSEN clocks and are 16-bits in length."]
    #[inline(always)]
    pub fn is_clk16(&self) -> bool {
        *self == Sarcr::Clk16
    }
}
#[doc = "Field `SARCR` writer - SAR Conversion Resolution."]
pub type SarcrW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sarcr, crate::Safe>;
impl<'a, REG> SarcrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Conversions last 10 internal CSEN clocks and are 10-bits in length."]
    #[inline(always)]
    pub fn clk10(self) -> &'a mut crate::W<REG> {
        self.variant(Sarcr::Clk10)
    }
    #[doc = "Conversions last 12 internal CSEN clocks and are 12-bits in length."]
    #[inline(always)]
    pub fn clk12(self) -> &'a mut crate::W<REG> {
        self.variant(Sarcr::Clk12)
    }
    #[doc = "Conversions last 14 internal CSEN clocks and are 14-bits in length."]
    #[inline(always)]
    pub fn clk14(self) -> &'a mut crate::W<REG> {
        self.variant(Sarcr::Clk14)
    }
    #[doc = "Conversions last 16 internal CSEN clocks and are 16-bits in length."]
    #[inline(always)]
    pub fn clk16(self) -> &'a mut crate::W<REG> {
        self.variant(Sarcr::Clk16)
    }
}
#[doc = "CSEN Accumulator Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Acu {
    #[doc = "0: Accumulate 1 sample."]
    Acc1 = 0,
    #[doc = "1: Accumulate 2 sample."]
    Acc2 = 1,
    #[doc = "2: Accumulate 4 sample."]
    Acc4 = 2,
    #[doc = "3: Accumulate 8 sample."]
    Acc8 = 3,
    #[doc = "4: Accumulate 16 sample."]
    Acc16 = 4,
    #[doc = "5: Accumulate 32 sample."]
    Acc32 = 5,
    #[doc = "6: Accumulate 64 sample."]
    Acc64 = 6,
}
impl From<Acu> for u8 {
    #[inline(always)]
    fn from(variant: Acu) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Acu {
    type Ux = u8;
}
impl crate::IsEnum for Acu {}
#[doc = "Field `ACU` reader - CSEN Accumulator Mode Select"]
pub type AcuR = crate::FieldReader<Acu>;
impl AcuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Acu> {
        match self.bits {
            0 => Some(Acu::Acc1),
            1 => Some(Acu::Acc2),
            2 => Some(Acu::Acc4),
            3 => Some(Acu::Acc8),
            4 => Some(Acu::Acc16),
            5 => Some(Acu::Acc32),
            6 => Some(Acu::Acc64),
            _ => None,
        }
    }
    #[doc = "Accumulate 1 sample."]
    #[inline(always)]
    pub fn is_acc1(&self) -> bool {
        *self == Acu::Acc1
    }
    #[doc = "Accumulate 2 sample."]
    #[inline(always)]
    pub fn is_acc2(&self) -> bool {
        *self == Acu::Acc2
    }
    #[doc = "Accumulate 4 sample."]
    #[inline(always)]
    pub fn is_acc4(&self) -> bool {
        *self == Acu::Acc4
    }
    #[doc = "Accumulate 8 sample."]
    #[inline(always)]
    pub fn is_acc8(&self) -> bool {
        *self == Acu::Acc8
    }
    #[doc = "Accumulate 16 sample."]
    #[inline(always)]
    pub fn is_acc16(&self) -> bool {
        *self == Acu::Acc16
    }
    #[doc = "Accumulate 32 sample."]
    #[inline(always)]
    pub fn is_acc32(&self) -> bool {
        *self == Acu::Acc32
    }
    #[doc = "Accumulate 64 sample."]
    #[inline(always)]
    pub fn is_acc64(&self) -> bool {
        *self == Acu::Acc64
    }
}
#[doc = "Field `ACU` writer - CSEN Accumulator Mode Select"]
pub type AcuW<'a, REG> = crate::FieldWriter<'a, REG, 3, Acu>;
impl<'a, REG> AcuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Accumulate 1 sample."]
    #[inline(always)]
    pub fn acc1(self) -> &'a mut crate::W<REG> {
        self.variant(Acu::Acc1)
    }
    #[doc = "Accumulate 2 sample."]
    #[inline(always)]
    pub fn acc2(self) -> &'a mut crate::W<REG> {
        self.variant(Acu::Acc2)
    }
    #[doc = "Accumulate 4 sample."]
    #[inline(always)]
    pub fn acc4(self) -> &'a mut crate::W<REG> {
        self.variant(Acu::Acc4)
    }
    #[doc = "Accumulate 8 sample."]
    #[inline(always)]
    pub fn acc8(self) -> &'a mut crate::W<REG> {
        self.variant(Acu::Acc8)
    }
    #[doc = "Accumulate 16 sample."]
    #[inline(always)]
    pub fn acc16(self) -> &'a mut crate::W<REG> {
        self.variant(Acu::Acc16)
    }
    #[doc = "Accumulate 32 sample."]
    #[inline(always)]
    pub fn acc32(self) -> &'a mut crate::W<REG> {
        self.variant(Acu::Acc32)
    }
    #[doc = "Accumulate 64 sample."]
    #[inline(always)]
    pub fn acc64(self) -> &'a mut crate::W<REG> {
        self.variant(Acu::Acc64)
    }
}
#[doc = "Field `MCEN` reader - CSEN Multiple Channel Enable"]
pub type McenR = crate::BitReader;
#[doc = "Field `MCEN` writer - CSEN Multiple Channel Enable"]
pub type McenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Start Trigger Select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stm {
    #[doc = "0: PRS Triggering. Conversions are triggered by the PRS channel selected in PRSSEL."]
    Prs = 0,
    #[doc = "1: Timer Triggering. Conversions are triggered by a local CSEN timer reload."]
    Timer = 1,
    #[doc = "2: Software Triggering. Conversions are triggered by writing a 1 to the START field of the CMD register."]
    Start = 2,
}
impl From<Stm> for u8 {
    #[inline(always)]
    fn from(variant: Stm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Stm {
    type Ux = u8;
}
impl crate::IsEnum for Stm {}
#[doc = "Field `STM` reader - Start Trigger Select"]
pub type StmR = crate::FieldReader<Stm>;
impl StmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Stm> {
        match self.bits {
            0 => Some(Stm::Prs),
            1 => Some(Stm::Timer),
            2 => Some(Stm::Start),
            _ => None,
        }
    }
    #[doc = "PRS Triggering. Conversions are triggered by the PRS channel selected in PRSSEL."]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == Stm::Prs
    }
    #[doc = "Timer Triggering. Conversions are triggered by a local CSEN timer reload."]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == Stm::Timer
    }
    #[doc = "Software Triggering. Conversions are triggered by writing a 1 to the START field of the CMD register."]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == Stm::Start
    }
}
#[doc = "Field `STM` writer - Start Trigger Select"]
pub type StmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Stm>;
impl<'a, REG> StmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Triggering. Conversions are triggered by the PRS channel selected in PRSSEL."]
    #[inline(always)]
    pub fn prs(self) -> &'a mut crate::W<REG> {
        self.variant(Stm::Prs)
    }
    #[doc = "Timer Triggering. Conversions are triggered by a local CSEN timer reload."]
    #[inline(always)]
    pub fn timer(self) -> &'a mut crate::W<REG> {
        self.variant(Stm::Timer)
    }
    #[doc = "Software Triggering. Conversions are triggered by writing a 1 to the START field of the CMD register."]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(Stm::Start)
    }
}
#[doc = "Field `CMPEN` reader - CSEN Digital Comparator Enable"]
pub type CmpenR = crate::BitReader;
#[doc = "Field `CMPEN` writer - CSEN Digital Comparator Enable"]
pub type CmpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRSF` reader - CSEN Disable Right-Shift"]
pub type DrsfR = crate::BitReader;
#[doc = "Field `DRSF` writer - CSEN Disable Right-Shift"]
pub type DrsfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEN` reader - CSEN DMA Enable Bit"]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - CSEN DMA Enable Bit"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONVSEL` reader - CSEN Converter Select"]
pub type ConvselR = crate::BitReader;
#[doc = "Field `CONVSEL` writer - CSEN Converter Select"]
pub type ConvselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHOPEN` reader - CSEN Chop Enable"]
pub type ChopenR = crate::BitReader;
#[doc = "Field `CHOPEN` writer - CSEN Chop Enable"]
pub type ChopenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOGND` reader - CSEN Automatic Ground Enable"]
pub type AutogndR = crate::BitReader;
#[doc = "Field `AUTOGND` writer - CSEN Automatic Ground Enable"]
pub type AutogndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MXUC` reader - CSEN Mux Disconnect"]
pub type MxucR = crate::BitReader;
#[doc = "Field `MXUC` writer - CSEN Mux Disconnect"]
pub type MxucW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMACMPEN` reader - Greater and Less Than Comparison Using the Exponential Moving Average (EMA) is Enabled"]
pub type EmacmpenR = crate::BitReader;
#[doc = "Field `EMACMPEN` writer - Greater and Less Than Comparison Using the Exponential Moving Average (EMA) is Enabled"]
pub type EmacmpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WARMUPMODE` reader - Select Warmup Mode for CSEN"]
pub type WarmupmodeR = crate::BitReader;
#[doc = "Field `WARMUPMODE` writer - Select Warmup Mode for CSEN"]
pub type WarmupmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCALSENS` reader - Local Sensing Enable"]
pub type LocalsensR = crate::BitReader;
#[doc = "Field `LOCALSENS` writer - Local Sensing Enable"]
pub type LocalsensW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPACCURACY` reader - Charge Pump Accuracy"]
pub type CpaccuracyR = crate::BitReader;
#[doc = "Field `CPACCURACY` writer - Charge Pump Accuracy"]
pub type CpaccuracyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - CSEN Enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CSEN Digital Comparator Polarity Select"]
    #[inline(always)]
    pub fn cmppol(&self) -> CmppolR {
        CmppolR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - CSEN Conversion Mode Select"]
    #[inline(always)]
    pub fn cm(&self) -> CmR {
        CmR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - SAR Conversion Resolution."]
    #[inline(always)]
    pub fn sarcr(&self) -> SarcrR {
        SarcrR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:14 - CSEN Accumulator Mode Select"]
    #[inline(always)]
    pub fn acu(&self) -> AcuR {
        AcuR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - CSEN Multiple Channel Enable"]
    #[inline(always)]
    pub fn mcen(&self) -> McenR {
        McenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Start Trigger Select"]
    #[inline(always)]
    pub fn stm(&self) -> StmR {
        StmR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - CSEN Digital Comparator Enable"]
    #[inline(always)]
    pub fn cmpen(&self) -> CmpenR {
        CmpenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CSEN Disable Right-Shift"]
    #[inline(always)]
    pub fn drsf(&self) -> DrsfR {
        DrsfR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CSEN DMA Enable Bit"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CSEN Converter Select"]
    #[inline(always)]
    pub fn convsel(&self) -> ConvselR {
        ConvselR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - CSEN Chop Enable"]
    #[inline(always)]
    pub fn chopen(&self) -> ChopenR {
        ChopenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CSEN Automatic Ground Enable"]
    #[inline(always)]
    pub fn autognd(&self) -> AutogndR {
        AutogndR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CSEN Mux Disconnect"]
    #[inline(always)]
    pub fn mxuc(&self) -> MxucR {
        MxucR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Greater and Less Than Comparison Using the Exponential Moving Average (EMA) is Enabled"]
    #[inline(always)]
    pub fn emacmpen(&self) -> EmacmpenR {
        EmacmpenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Select Warmup Mode for CSEN"]
    #[inline(always)]
    pub fn warmupmode(&self) -> WarmupmodeR {
        WarmupmodeR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Local Sensing Enable"]
    #[inline(always)]
    pub fn localsens(&self) -> LocalsensR {
        LocalsensR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Charge Pump Accuracy"]
    #[inline(always)]
    pub fn cpaccuracy(&self) -> CpaccuracyR {
        CpaccuracyR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - CSEN Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, CtrlSpec> {
        EnW::new(self, 1)
    }
    #[doc = "Bit 2 - CSEN Digital Comparator Polarity Select"]
    #[inline(always)]
    pub fn cmppol(&mut self) -> CmppolW<'_, CtrlSpec> {
        CmppolW::new(self, 2)
    }
    #[doc = "Bits 4:5 - CSEN Conversion Mode Select"]
    #[inline(always)]
    pub fn cm(&mut self) -> CmW<'_, CtrlSpec> {
        CmW::new(self, 4)
    }
    #[doc = "Bits 8:9 - SAR Conversion Resolution."]
    #[inline(always)]
    pub fn sarcr(&mut self) -> SarcrW<'_, CtrlSpec> {
        SarcrW::new(self, 8)
    }
    #[doc = "Bits 12:14 - CSEN Accumulator Mode Select"]
    #[inline(always)]
    pub fn acu(&mut self) -> AcuW<'_, CtrlSpec> {
        AcuW::new(self, 12)
    }
    #[doc = "Bit 15 - CSEN Multiple Channel Enable"]
    #[inline(always)]
    pub fn mcen(&mut self) -> McenW<'_, CtrlSpec> {
        McenW::new(self, 15)
    }
    #[doc = "Bits 16:17 - Start Trigger Select"]
    #[inline(always)]
    pub fn stm(&mut self) -> StmW<'_, CtrlSpec> {
        StmW::new(self, 16)
    }
    #[doc = "Bit 18 - CSEN Digital Comparator Enable"]
    #[inline(always)]
    pub fn cmpen(&mut self) -> CmpenW<'_, CtrlSpec> {
        CmpenW::new(self, 18)
    }
    #[doc = "Bit 19 - CSEN Disable Right-Shift"]
    #[inline(always)]
    pub fn drsf(&mut self) -> DrsfW<'_, CtrlSpec> {
        DrsfW::new(self, 19)
    }
    #[doc = "Bit 20 - CSEN DMA Enable Bit"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<'_, CtrlSpec> {
        DmaenW::new(self, 20)
    }
    #[doc = "Bit 21 - CSEN Converter Select"]
    #[inline(always)]
    pub fn convsel(&mut self) -> ConvselW<'_, CtrlSpec> {
        ConvselW::new(self, 21)
    }
    #[doc = "Bit 22 - CSEN Chop Enable"]
    #[inline(always)]
    pub fn chopen(&mut self) -> ChopenW<'_, CtrlSpec> {
        ChopenW::new(self, 22)
    }
    #[doc = "Bit 23 - CSEN Automatic Ground Enable"]
    #[inline(always)]
    pub fn autognd(&mut self) -> AutogndW<'_, CtrlSpec> {
        AutogndW::new(self, 23)
    }
    #[doc = "Bit 24 - CSEN Mux Disconnect"]
    #[inline(always)]
    pub fn mxuc(&mut self) -> MxucW<'_, CtrlSpec> {
        MxucW::new(self, 24)
    }
    #[doc = "Bit 25 - Greater and Less Than Comparison Using the Exponential Moving Average (EMA) is Enabled"]
    #[inline(always)]
    pub fn emacmpen(&mut self) -> EmacmpenW<'_, CtrlSpec> {
        EmacmpenW::new(self, 25)
    }
    #[doc = "Bit 26 - Select Warmup Mode for CSEN"]
    #[inline(always)]
    pub fn warmupmode(&mut self) -> WarmupmodeW<'_, CtrlSpec> {
        WarmupmodeW::new(self, 26)
    }
    #[doc = "Bit 27 - Local Sensing Enable"]
    #[inline(always)]
    pub fn localsens(&mut self) -> LocalsensW<'_, CtrlSpec> {
        LocalsensW::new(self, 27)
    }
    #[doc = "Bit 28 - Charge Pump Accuracy"]
    #[inline(always)]
    pub fn cpaccuracy(&mut self) -> CpaccuracyW<'_, CtrlSpec> {
        CpaccuracyW::new(self, 28)
    }
}
#[doc = "Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0x0003_0000"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x0003_0000;
}
