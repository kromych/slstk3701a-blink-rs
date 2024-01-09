#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `EN` reader - CSEN Enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - CSEN Enable"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPPOL` reader - CSEN Digital Comparator Polarity Select"]
pub type CMPPOL_R = crate::BitReader;
#[doc = "Field `CMPPOL` writer - CSEN Digital Comparator Polarity Select"]
pub type CMPPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CM` reader - CSEN Conversion Mode Select"]
pub type CM_R = crate::FieldReader<CM_A>;
#[doc = "CSEN Conversion Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CM_A {
    #[doc = "0: Single Channel Mode: One conversion of a single channel (when MCE = 0) or set of bonded channels (when MCE = 1) per conversion trigger."]
    SGL = 0,
    #[doc = "1: Scan Mode: Scans multiple selected channels once per conversion trigger."]
    SCAN = 1,
    #[doc = "2: Continuous Single Channel: Continuous conversion of a single channel (when MCE = 0) or set of bonded channels (when MCE = 1)."]
    CONTSGL = 2,
    #[doc = "3: Continuous Scan Mode: Continuously scans multiple selected channels."]
    CONTSCAN = 3,
}
impl From<CM_A> for u8 {
    #[inline(always)]
    fn from(variant: CM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CM_A {
    type Ux = u8;
}
impl CM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CM_A {
        match self.bits {
            0 => CM_A::SGL,
            1 => CM_A::SCAN,
            2 => CM_A::CONTSGL,
            3 => CM_A::CONTSCAN,
            _ => unreachable!(),
        }
    }
    #[doc = "Single Channel Mode: One conversion of a single channel (when MCE = 0) or set of bonded channels (when MCE = 1) per conversion trigger."]
    #[inline(always)]
    pub fn is_sgl(&self) -> bool {
        *self == CM_A::SGL
    }
    #[doc = "Scan Mode: Scans multiple selected channels once per conversion trigger."]
    #[inline(always)]
    pub fn is_scan(&self) -> bool {
        *self == CM_A::SCAN
    }
    #[doc = "Continuous Single Channel: Continuous conversion of a single channel (when MCE = 0) or set of bonded channels (when MCE = 1)."]
    #[inline(always)]
    pub fn is_contsgl(&self) -> bool {
        *self == CM_A::CONTSGL
    }
    #[doc = "Continuous Scan Mode: Continuously scans multiple selected channels."]
    #[inline(always)]
    pub fn is_contscan(&self) -> bool {
        *self == CM_A::CONTSCAN
    }
}
#[doc = "Field `CM` writer - CSEN Conversion Mode Select"]
pub type CM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CM_A>;
impl<'a, REG> CM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single Channel Mode: One conversion of a single channel (when MCE = 0) or set of bonded channels (when MCE = 1) per conversion trigger."]
    #[inline(always)]
    pub fn sgl(self) -> &'a mut crate::W<REG> {
        self.variant(CM_A::SGL)
    }
    #[doc = "Scan Mode: Scans multiple selected channels once per conversion trigger."]
    #[inline(always)]
    pub fn scan(self) -> &'a mut crate::W<REG> {
        self.variant(CM_A::SCAN)
    }
    #[doc = "Continuous Single Channel: Continuous conversion of a single channel (when MCE = 0) or set of bonded channels (when MCE = 1)."]
    #[inline(always)]
    pub fn contsgl(self) -> &'a mut crate::W<REG> {
        self.variant(CM_A::CONTSGL)
    }
    #[doc = "Continuous Scan Mode: Continuously scans multiple selected channels."]
    #[inline(always)]
    pub fn contscan(self) -> &'a mut crate::W<REG> {
        self.variant(CM_A::CONTSCAN)
    }
}
#[doc = "Field `SARCR` reader - SAR Conversion Resolution."]
pub type SARCR_R = crate::FieldReader<SARCR_A>;
#[doc = "SAR Conversion Resolution.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SARCR_A {
    #[doc = "0: Conversions last 10 internal CSEN clocks and are 10-bits in length."]
    CLK10 = 0,
    #[doc = "1: Conversions last 12 internal CSEN clocks and are 12-bits in length."]
    CLK12 = 1,
    #[doc = "2: Conversions last 14 internal CSEN clocks and are 14-bits in length."]
    CLK14 = 2,
    #[doc = "3: Conversions last 16 internal CSEN clocks and are 16-bits in length."]
    CLK16 = 3,
}
impl From<SARCR_A> for u8 {
    #[inline(always)]
    fn from(variant: SARCR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SARCR_A {
    type Ux = u8;
}
impl SARCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SARCR_A {
        match self.bits {
            0 => SARCR_A::CLK10,
            1 => SARCR_A::CLK12,
            2 => SARCR_A::CLK14,
            3 => SARCR_A::CLK16,
            _ => unreachable!(),
        }
    }
    #[doc = "Conversions last 10 internal CSEN clocks and are 10-bits in length."]
    #[inline(always)]
    pub fn is_clk10(&self) -> bool {
        *self == SARCR_A::CLK10
    }
    #[doc = "Conversions last 12 internal CSEN clocks and are 12-bits in length."]
    #[inline(always)]
    pub fn is_clk12(&self) -> bool {
        *self == SARCR_A::CLK12
    }
    #[doc = "Conversions last 14 internal CSEN clocks and are 14-bits in length."]
    #[inline(always)]
    pub fn is_clk14(&self) -> bool {
        *self == SARCR_A::CLK14
    }
    #[doc = "Conversions last 16 internal CSEN clocks and are 16-bits in length."]
    #[inline(always)]
    pub fn is_clk16(&self) -> bool {
        *self == SARCR_A::CLK16
    }
}
#[doc = "Field `SARCR` writer - SAR Conversion Resolution."]
pub type SARCR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SARCR_A>;
impl<'a, REG> SARCR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Conversions last 10 internal CSEN clocks and are 10-bits in length."]
    #[inline(always)]
    pub fn clk10(self) -> &'a mut crate::W<REG> {
        self.variant(SARCR_A::CLK10)
    }
    #[doc = "Conversions last 12 internal CSEN clocks and are 12-bits in length."]
    #[inline(always)]
    pub fn clk12(self) -> &'a mut crate::W<REG> {
        self.variant(SARCR_A::CLK12)
    }
    #[doc = "Conversions last 14 internal CSEN clocks and are 14-bits in length."]
    #[inline(always)]
    pub fn clk14(self) -> &'a mut crate::W<REG> {
        self.variant(SARCR_A::CLK14)
    }
    #[doc = "Conversions last 16 internal CSEN clocks and are 16-bits in length."]
    #[inline(always)]
    pub fn clk16(self) -> &'a mut crate::W<REG> {
        self.variant(SARCR_A::CLK16)
    }
}
#[doc = "Field `ACU` reader - CSEN Accumulator Mode Select"]
pub type ACU_R = crate::FieldReader<ACU_A>;
#[doc = "CSEN Accumulator Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACU_A {
    #[doc = "0: Accumulate 1 sample."]
    ACC1 = 0,
    #[doc = "1: Accumulate 2 sample."]
    ACC2 = 1,
    #[doc = "2: Accumulate 4 sample."]
    ACC4 = 2,
    #[doc = "3: Accumulate 8 sample."]
    ACC8 = 3,
    #[doc = "4: Accumulate 16 sample."]
    ACC16 = 4,
    #[doc = "5: Accumulate 32 sample."]
    ACC32 = 5,
    #[doc = "6: Accumulate 64 sample."]
    ACC64 = 6,
}
impl From<ACU_A> for u8 {
    #[inline(always)]
    fn from(variant: ACU_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ACU_A {
    type Ux = u8;
}
impl ACU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ACU_A> {
        match self.bits {
            0 => Some(ACU_A::ACC1),
            1 => Some(ACU_A::ACC2),
            2 => Some(ACU_A::ACC4),
            3 => Some(ACU_A::ACC8),
            4 => Some(ACU_A::ACC16),
            5 => Some(ACU_A::ACC32),
            6 => Some(ACU_A::ACC64),
            _ => None,
        }
    }
    #[doc = "Accumulate 1 sample."]
    #[inline(always)]
    pub fn is_acc1(&self) -> bool {
        *self == ACU_A::ACC1
    }
    #[doc = "Accumulate 2 sample."]
    #[inline(always)]
    pub fn is_acc2(&self) -> bool {
        *self == ACU_A::ACC2
    }
    #[doc = "Accumulate 4 sample."]
    #[inline(always)]
    pub fn is_acc4(&self) -> bool {
        *self == ACU_A::ACC4
    }
    #[doc = "Accumulate 8 sample."]
    #[inline(always)]
    pub fn is_acc8(&self) -> bool {
        *self == ACU_A::ACC8
    }
    #[doc = "Accumulate 16 sample."]
    #[inline(always)]
    pub fn is_acc16(&self) -> bool {
        *self == ACU_A::ACC16
    }
    #[doc = "Accumulate 32 sample."]
    #[inline(always)]
    pub fn is_acc32(&self) -> bool {
        *self == ACU_A::ACC32
    }
    #[doc = "Accumulate 64 sample."]
    #[inline(always)]
    pub fn is_acc64(&self) -> bool {
        *self == ACU_A::ACC64
    }
}
#[doc = "Field `ACU` writer - CSEN Accumulator Mode Select"]
pub type ACU_W<'a, REG> = crate::FieldWriter<'a, REG, 3, ACU_A>;
impl<'a, REG> ACU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Accumulate 1 sample."]
    #[inline(always)]
    pub fn acc1(self) -> &'a mut crate::W<REG> {
        self.variant(ACU_A::ACC1)
    }
    #[doc = "Accumulate 2 sample."]
    #[inline(always)]
    pub fn acc2(self) -> &'a mut crate::W<REG> {
        self.variant(ACU_A::ACC2)
    }
    #[doc = "Accumulate 4 sample."]
    #[inline(always)]
    pub fn acc4(self) -> &'a mut crate::W<REG> {
        self.variant(ACU_A::ACC4)
    }
    #[doc = "Accumulate 8 sample."]
    #[inline(always)]
    pub fn acc8(self) -> &'a mut crate::W<REG> {
        self.variant(ACU_A::ACC8)
    }
    #[doc = "Accumulate 16 sample."]
    #[inline(always)]
    pub fn acc16(self) -> &'a mut crate::W<REG> {
        self.variant(ACU_A::ACC16)
    }
    #[doc = "Accumulate 32 sample."]
    #[inline(always)]
    pub fn acc32(self) -> &'a mut crate::W<REG> {
        self.variant(ACU_A::ACC32)
    }
    #[doc = "Accumulate 64 sample."]
    #[inline(always)]
    pub fn acc64(self) -> &'a mut crate::W<REG> {
        self.variant(ACU_A::ACC64)
    }
}
#[doc = "Field `MCEN` reader - CSEN Multiple Channel Enable"]
pub type MCEN_R = crate::BitReader;
#[doc = "Field `MCEN` writer - CSEN Multiple Channel Enable"]
pub type MCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STM` reader - Start Trigger Select"]
pub type STM_R = crate::FieldReader<STM_A>;
#[doc = "Start Trigger Select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STM_A {
    #[doc = "0: PRS Triggering. Conversions are triggered by the PRS channel selected in PRSSEL."]
    PRS = 0,
    #[doc = "1: Timer Triggering. Conversions are triggered by a local CSEN timer reload."]
    TIMER = 1,
    #[doc = "2: Software Triggering. Conversions are triggered by writing a 1 to the START field of the CMD register."]
    START = 2,
}
impl From<STM_A> for u8 {
    #[inline(always)]
    fn from(variant: STM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STM_A {
    type Ux = u8;
}
impl STM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<STM_A> {
        match self.bits {
            0 => Some(STM_A::PRS),
            1 => Some(STM_A::TIMER),
            2 => Some(STM_A::START),
            _ => None,
        }
    }
    #[doc = "PRS Triggering. Conversions are triggered by the PRS channel selected in PRSSEL."]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == STM_A::PRS
    }
    #[doc = "Timer Triggering. Conversions are triggered by a local CSEN timer reload."]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == STM_A::TIMER
    }
    #[doc = "Software Triggering. Conversions are triggered by writing a 1 to the START field of the CMD register."]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == STM_A::START
    }
}
#[doc = "Field `STM` writer - Start Trigger Select"]
pub type STM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, STM_A>;
impl<'a, REG> STM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Triggering. Conversions are triggered by the PRS channel selected in PRSSEL."]
    #[inline(always)]
    pub fn prs(self) -> &'a mut crate::W<REG> {
        self.variant(STM_A::PRS)
    }
    #[doc = "Timer Triggering. Conversions are triggered by a local CSEN timer reload."]
    #[inline(always)]
    pub fn timer(self) -> &'a mut crate::W<REG> {
        self.variant(STM_A::TIMER)
    }
    #[doc = "Software Triggering. Conversions are triggered by writing a 1 to the START field of the CMD register."]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(STM_A::START)
    }
}
#[doc = "Field `CMPEN` reader - CSEN Digital Comparator Enable"]
pub type CMPEN_R = crate::BitReader;
#[doc = "Field `CMPEN` writer - CSEN Digital Comparator Enable"]
pub type CMPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRSF` reader - CSEN Disable Right-Shift"]
pub type DRSF_R = crate::BitReader;
#[doc = "Field `DRSF` writer - CSEN Disable Right-Shift"]
pub type DRSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEN` reader - CSEN DMA Enable Bit"]
pub type DMAEN_R = crate::BitReader;
#[doc = "Field `DMAEN` writer - CSEN DMA Enable Bit"]
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONVSEL` reader - CSEN Converter Select"]
pub type CONVSEL_R = crate::BitReader;
#[doc = "Field `CONVSEL` writer - CSEN Converter Select"]
pub type CONVSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHOPEN` reader - CSEN Chop Enable"]
pub type CHOPEN_R = crate::BitReader;
#[doc = "Field `CHOPEN` writer - CSEN Chop Enable"]
pub type CHOPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOGND` reader - CSEN Automatic Ground Enable"]
pub type AUTOGND_R = crate::BitReader;
#[doc = "Field `AUTOGND` writer - CSEN Automatic Ground Enable"]
pub type AUTOGND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MXUC` reader - CSEN Mux Disconnect"]
pub type MXUC_R = crate::BitReader;
#[doc = "Field `MXUC` writer - CSEN Mux Disconnect"]
pub type MXUC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMACMPEN` reader - Greater and Less Than Comparison Using the Exponential Moving Average (EMA) is Enabled"]
pub type EMACMPEN_R = crate::BitReader;
#[doc = "Field `EMACMPEN` writer - Greater and Less Than Comparison Using the Exponential Moving Average (EMA) is Enabled"]
pub type EMACMPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WARMUPMODE` reader - Select Warmup Mode for CSEN"]
pub type WARMUPMODE_R = crate::BitReader;
#[doc = "Field `WARMUPMODE` writer - Select Warmup Mode for CSEN"]
pub type WARMUPMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCALSENS` reader - Local Sensing Enable"]
pub type LOCALSENS_R = crate::BitReader;
#[doc = "Field `LOCALSENS` writer - Local Sensing Enable"]
pub type LOCALSENS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPACCURACY` reader - Charge Pump Accuracy"]
pub type CPACCURACY_R = crate::BitReader;
#[doc = "Field `CPACCURACY` writer - Charge Pump Accuracy"]
pub type CPACCURACY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - CSEN Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CSEN Digital Comparator Polarity Select"]
    #[inline(always)]
    pub fn cmppol(&self) -> CMPPOL_R {
        CMPPOL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - CSEN Conversion Mode Select"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - SAR Conversion Resolution."]
    #[inline(always)]
    pub fn sarcr(&self) -> SARCR_R {
        SARCR_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:14 - CSEN Accumulator Mode Select"]
    #[inline(always)]
    pub fn acu(&self) -> ACU_R {
        ACU_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - CSEN Multiple Channel Enable"]
    #[inline(always)]
    pub fn mcen(&self) -> MCEN_R {
        MCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Start Trigger Select"]
    #[inline(always)]
    pub fn stm(&self) -> STM_R {
        STM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - CSEN Digital Comparator Enable"]
    #[inline(always)]
    pub fn cmpen(&self) -> CMPEN_R {
        CMPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CSEN Disable Right-Shift"]
    #[inline(always)]
    pub fn drsf(&self) -> DRSF_R {
        DRSF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CSEN DMA Enable Bit"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CSEN Converter Select"]
    #[inline(always)]
    pub fn convsel(&self) -> CONVSEL_R {
        CONVSEL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - CSEN Chop Enable"]
    #[inline(always)]
    pub fn chopen(&self) -> CHOPEN_R {
        CHOPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CSEN Automatic Ground Enable"]
    #[inline(always)]
    pub fn autognd(&self) -> AUTOGND_R {
        AUTOGND_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CSEN Mux Disconnect"]
    #[inline(always)]
    pub fn mxuc(&self) -> MXUC_R {
        MXUC_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Greater and Less Than Comparison Using the Exponential Moving Average (EMA) is Enabled"]
    #[inline(always)]
    pub fn emacmpen(&self) -> EMACMPEN_R {
        EMACMPEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Select Warmup Mode for CSEN"]
    #[inline(always)]
    pub fn warmupmode(&self) -> WARMUPMODE_R {
        WARMUPMODE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Local Sensing Enable"]
    #[inline(always)]
    pub fn localsens(&self) -> LOCALSENS_R {
        LOCALSENS_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Charge Pump Accuracy"]
    #[inline(always)]
    pub fn cpaccuracy(&self) -> CPACCURACY_R {
        CPACCURACY_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - CSEN Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CTRL_SPEC> {
        EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - CSEN Digital Comparator Polarity Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmppol(&mut self) -> CMPPOL_W<CTRL_SPEC> {
        CMPPOL_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - CSEN Conversion Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn cm(&mut self) -> CM_W<CTRL_SPEC> {
        CM_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - SAR Conversion Resolution."]
    #[inline(always)]
    #[must_use]
    pub fn sarcr(&mut self) -> SARCR_W<CTRL_SPEC> {
        SARCR_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - CSEN Accumulator Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn acu(&mut self) -> ACU_W<CTRL_SPEC> {
        ACU_W::new(self, 12)
    }
    #[doc = "Bit 15 - CSEN Multiple Channel Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mcen(&mut self) -> MCEN_W<CTRL_SPEC> {
        MCEN_W::new(self, 15)
    }
    #[doc = "Bits 16:17 - Start Trigger Select"]
    #[inline(always)]
    #[must_use]
    pub fn stm(&mut self) -> STM_W<CTRL_SPEC> {
        STM_W::new(self, 16)
    }
    #[doc = "Bit 18 - CSEN Digital Comparator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpen(&mut self) -> CMPEN_W<CTRL_SPEC> {
        CMPEN_W::new(self, 18)
    }
    #[doc = "Bit 19 - CSEN Disable Right-Shift"]
    #[inline(always)]
    #[must_use]
    pub fn drsf(&mut self) -> DRSF_W<CTRL_SPEC> {
        DRSF_W::new(self, 19)
    }
    #[doc = "Bit 20 - CSEN DMA Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<CTRL_SPEC> {
        DMAEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - CSEN Converter Select"]
    #[inline(always)]
    #[must_use]
    pub fn convsel(&mut self) -> CONVSEL_W<CTRL_SPEC> {
        CONVSEL_W::new(self, 21)
    }
    #[doc = "Bit 22 - CSEN Chop Enable"]
    #[inline(always)]
    #[must_use]
    pub fn chopen(&mut self) -> CHOPEN_W<CTRL_SPEC> {
        CHOPEN_W::new(self, 22)
    }
    #[doc = "Bit 23 - CSEN Automatic Ground Enable"]
    #[inline(always)]
    #[must_use]
    pub fn autognd(&mut self) -> AUTOGND_W<CTRL_SPEC> {
        AUTOGND_W::new(self, 23)
    }
    #[doc = "Bit 24 - CSEN Mux Disconnect"]
    #[inline(always)]
    #[must_use]
    pub fn mxuc(&mut self) -> MXUC_W<CTRL_SPEC> {
        MXUC_W::new(self, 24)
    }
    #[doc = "Bit 25 - Greater and Less Than Comparison Using the Exponential Moving Average (EMA) is Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn emacmpen(&mut self) -> EMACMPEN_W<CTRL_SPEC> {
        EMACMPEN_W::new(self, 25)
    }
    #[doc = "Bit 26 - Select Warmup Mode for CSEN"]
    #[inline(always)]
    #[must_use]
    pub fn warmupmode(&mut self) -> WARMUPMODE_W<CTRL_SPEC> {
        WARMUPMODE_W::new(self, 26)
    }
    #[doc = "Bit 27 - Local Sensing Enable"]
    #[inline(always)]
    #[must_use]
    pub fn localsens(&mut self) -> LOCALSENS_W<CTRL_SPEC> {
        LOCALSENS_W::new(self, 27)
    }
    #[doc = "Bit 28 - Charge Pump Accuracy"]
    #[inline(always)]
    #[must_use]
    pub fn cpaccuracy(&mut self) -> CPACCURACY_W<CTRL_SPEC> {
        CPACCURACY_W::new(self, 28)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x0003_0000"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0003_0000;
}
