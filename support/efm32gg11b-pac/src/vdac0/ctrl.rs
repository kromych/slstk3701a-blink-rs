#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `DIFF` reader - Differential Mode"]
pub type DiffR = crate::BitReader;
#[doc = "Field `DIFF` writer - Differential Mode"]
pub type DiffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINEMODE` reader - Sine Mode"]
pub type SinemodeR = crate::BitReader;
#[doc = "Field `SINEMODE` writer - Sine Mode"]
pub type SinemodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTENPRS` reader - PRS Controlled Output Enable"]
pub type OutenprsR = crate::BitReader;
#[doc = "Field `OUTENPRS` writer - PRS Controlled Output Enable"]
pub type OutenprsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0PRESCRST` reader - Channel 0 Start Reset Prescaler"]
pub type Ch0prescrstR = crate::BitReader;
#[doc = "Field `CH0PRESCRST` writer - Channel 0 Start Reset Prescaler"]
pub type Ch0prescrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Refsel {
    #[doc = "0: Internal low noise 1.25 V bandgap reference"]
    _1v25ln = 0,
    #[doc = "1: Internal low noise 2.5 V bandgap reference"]
    _2v5ln = 1,
    #[doc = "2: Internal 1.25 V bandgap reference"]
    _1v25 = 2,
    #[doc = "3: Internal 2.5 V bandgap reference"]
    _2v5 = 3,
    #[doc = "4: AVDD reference"]
    Vdd = 4,
    #[doc = "6: External pin reference"]
    Ext = 6,
}
impl From<Refsel> for u8 {
    #[inline(always)]
    fn from(variant: Refsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Refsel {
    type Ux = u8;
}
impl crate::IsEnum for Refsel {}
#[doc = "Field `REFSEL` reader - Reference Selection"]
pub type RefselR = crate::FieldReader<Refsel>;
impl RefselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Refsel> {
        match self.bits {
            0 => Some(Refsel::_1v25ln),
            1 => Some(Refsel::_2v5ln),
            2 => Some(Refsel::_1v25),
            3 => Some(Refsel::_2v5),
            4 => Some(Refsel::Vdd),
            6 => Some(Refsel::Ext),
            _ => None,
        }
    }
    #[doc = "Internal low noise 1.25 V bandgap reference"]
    #[inline(always)]
    pub fn is_1v25ln(&self) -> bool {
        *self == Refsel::_1v25ln
    }
    #[doc = "Internal low noise 2.5 V bandgap reference"]
    #[inline(always)]
    pub fn is_2v5ln(&self) -> bool {
        *self == Refsel::_2v5ln
    }
    #[doc = "Internal 1.25 V bandgap reference"]
    #[inline(always)]
    pub fn is_1v25(&self) -> bool {
        *self == Refsel::_1v25
    }
    #[doc = "Internal 2.5 V bandgap reference"]
    #[inline(always)]
    pub fn is_2v5(&self) -> bool {
        *self == Refsel::_2v5
    }
    #[doc = "AVDD reference"]
    #[inline(always)]
    pub fn is_vdd(&self) -> bool {
        *self == Refsel::Vdd
    }
    #[doc = "External pin reference"]
    #[inline(always)]
    pub fn is_ext(&self) -> bool {
        *self == Refsel::Ext
    }
}
#[doc = "Field `REFSEL` writer - Reference Selection"]
pub type RefselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Refsel>;
impl<'a, REG> RefselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal low noise 1.25 V bandgap reference"]
    #[inline(always)]
    pub fn _1v25ln(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::_1v25ln)
    }
    #[doc = "Internal low noise 2.5 V bandgap reference"]
    #[inline(always)]
    pub fn _2v5ln(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::_2v5ln)
    }
    #[doc = "Internal 1.25 V bandgap reference"]
    #[inline(always)]
    pub fn _1v25(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::_1v25)
    }
    #[doc = "Internal 2.5 V bandgap reference"]
    #[inline(always)]
    pub fn _2v5(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::_2v5)
    }
    #[doc = "AVDD reference"]
    #[inline(always)]
    pub fn vdd(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Vdd)
    }
    #[doc = "External pin reference"]
    #[inline(always)]
    pub fn ext(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Ext)
    }
}
#[doc = "Prescaler Setting for DAC Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Presc {
    #[doc = "0: `0`"]
    Nodivision = 0,
}
impl From<Presc> for u8 {
    #[inline(always)]
    fn from(variant: Presc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Presc {
    type Ux = u8;
}
impl crate::IsEnum for Presc {}
#[doc = "Field `PRESC` reader - Prescaler Setting for DAC Clock"]
pub type PrescR = crate::FieldReader<Presc>;
impl PrescR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Presc> {
        match self.bits {
            0 => Some(Presc::Nodivision),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_nodivision(&self) -> bool {
        *self == Presc::Nodivision
    }
}
#[doc = "Field `PRESC` writer - Prescaler Setting for DAC Clock"]
pub type PrescW<'a, REG> = crate::FieldWriter<'a, REG, 7, Presc>;
impl<'a, REG> PrescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn nodivision(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Nodivision)
    }
}
#[doc = "Refresh Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Refreshperiod {
    #[doc = "0: All channels with enabled refresh are refreshed every 8 DAC_CLK cycles"]
    _8cycles = 0,
    #[doc = "1: All channels with enabled refresh are refreshed every 16 DAC_CLK cycles"]
    _16cycles = 1,
    #[doc = "2: All channels with enabled refresh are refreshed every 32 DAC_CLK cycles"]
    _32cycles = 2,
    #[doc = "3: All channels with enabled refresh are refreshed every 64 DAC_CLK cycles"]
    _64cycles = 3,
}
impl From<Refreshperiod> for u8 {
    #[inline(always)]
    fn from(variant: Refreshperiod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Refreshperiod {
    type Ux = u8;
}
impl crate::IsEnum for Refreshperiod {}
#[doc = "Field `REFRESHPERIOD` reader - Refresh Period"]
pub type RefreshperiodR = crate::FieldReader<Refreshperiod>;
impl RefreshperiodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refreshperiod {
        match self.bits {
            0 => Refreshperiod::_8cycles,
            1 => Refreshperiod::_16cycles,
            2 => Refreshperiod::_32cycles,
            3 => Refreshperiod::_64cycles,
            _ => unreachable!(),
        }
    }
    #[doc = "All channels with enabled refresh are refreshed every 8 DAC_CLK cycles"]
    #[inline(always)]
    pub fn is_8cycles(&self) -> bool {
        *self == Refreshperiod::_8cycles
    }
    #[doc = "All channels with enabled refresh are refreshed every 16 DAC_CLK cycles"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == Refreshperiod::_16cycles
    }
    #[doc = "All channels with enabled refresh are refreshed every 32 DAC_CLK cycles"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == Refreshperiod::_32cycles
    }
    #[doc = "All channels with enabled refresh are refreshed every 64 DAC_CLK cycles"]
    #[inline(always)]
    pub fn is_64cycles(&self) -> bool {
        *self == Refreshperiod::_64cycles
    }
}
#[doc = "Field `REFRESHPERIOD` writer - Refresh Period"]
pub type RefreshperiodW<'a, REG> = crate::FieldWriter<'a, REG, 2, Refreshperiod, crate::Safe>;
impl<'a, REG> RefreshperiodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "All channels with enabled refresh are refreshed every 8 DAC_CLK cycles"]
    #[inline(always)]
    pub fn _8cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Refreshperiod::_8cycles)
    }
    #[doc = "All channels with enabled refresh are refreshed every 16 DAC_CLK cycles"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Refreshperiod::_16cycles)
    }
    #[doc = "All channels with enabled refresh are refreshed every 32 DAC_CLK cycles"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Refreshperiod::_32cycles)
    }
    #[doc = "All channels with enabled refresh are refreshed every 64 DAC_CLK cycles"]
    #[inline(always)]
    pub fn _64cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Refreshperiod::_64cycles)
    }
}
#[doc = "Field `WARMUPMODE` reader - Warm-up Mode"]
pub type WarmupmodeR = crate::BitReader;
#[doc = "Field `WARMUPMODE` writer - Warm-up Mode"]
pub type WarmupmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DACCLKMODE` reader - Clock Mode"]
pub type DacclkmodeR = crate::BitReader;
#[doc = "Field `DACCLKMODE` writer - Clock Mode"]
pub type DacclkmodeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Differential Mode"]
    #[inline(always)]
    pub fn diff(&self) -> DiffR {
        DiffR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Sine Mode"]
    #[inline(always)]
    pub fn sinemode(&self) -> SinemodeR {
        SinemodeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PRS Controlled Output Enable"]
    #[inline(always)]
    pub fn outenprs(&self) -> OutenprsR {
        OutenprsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 0 Start Reset Prescaler"]
    #[inline(always)]
    pub fn ch0prescrst(&self) -> Ch0prescrstR {
        Ch0prescrstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Reference Selection"]
    #[inline(always)]
    pub fn refsel(&self) -> RefselR {
        RefselR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:22 - Prescaler Setting for DAC Clock"]
    #[inline(always)]
    pub fn presc(&self) -> PrescR {
        PrescR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:25 - Refresh Period"]
    #[inline(always)]
    pub fn refreshperiod(&self) -> RefreshperiodR {
        RefreshperiodR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28 - Warm-up Mode"]
    #[inline(always)]
    pub fn warmupmode(&self) -> WarmupmodeR {
        WarmupmodeR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - Clock Mode"]
    #[inline(always)]
    pub fn dacclkmode(&self) -> DacclkmodeR {
        DacclkmodeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Differential Mode"]
    #[inline(always)]
    pub fn diff(&mut self) -> DiffW<'_, CtrlSpec> {
        DiffW::new(self, 0)
    }
    #[doc = "Bit 4 - Sine Mode"]
    #[inline(always)]
    pub fn sinemode(&mut self) -> SinemodeW<'_, CtrlSpec> {
        SinemodeW::new(self, 4)
    }
    #[doc = "Bit 5 - PRS Controlled Output Enable"]
    #[inline(always)]
    pub fn outenprs(&mut self) -> OutenprsW<'_, CtrlSpec> {
        OutenprsW::new(self, 5)
    }
    #[doc = "Bit 6 - Channel 0 Start Reset Prescaler"]
    #[inline(always)]
    pub fn ch0prescrst(&mut self) -> Ch0prescrstW<'_, CtrlSpec> {
        Ch0prescrstW::new(self, 6)
    }
    #[doc = "Bits 8:10 - Reference Selection"]
    #[inline(always)]
    pub fn refsel(&mut self) -> RefselW<'_, CtrlSpec> {
        RefselW::new(self, 8)
    }
    #[doc = "Bits 16:22 - Prescaler Setting for DAC Clock"]
    #[inline(always)]
    pub fn presc(&mut self) -> PrescW<'_, CtrlSpec> {
        PrescW::new(self, 16)
    }
    #[doc = "Bits 24:25 - Refresh Period"]
    #[inline(always)]
    pub fn refreshperiod(&mut self) -> RefreshperiodW<'_, CtrlSpec> {
        RefreshperiodW::new(self, 24)
    }
    #[doc = "Bit 28 - Warm-up Mode"]
    #[inline(always)]
    pub fn warmupmode(&mut self) -> WarmupmodeW<'_, CtrlSpec> {
        WarmupmodeW::new(self, 28)
    }
    #[doc = "Bit 31 - Clock Mode"]
    #[inline(always)]
    pub fn dacclkmode(&mut self) -> DacclkmodeW<'_, CtrlSpec> {
        DacclkmodeW::new(self, 31)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
