#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `VBUSENAP` reader - VBUSEN Active Polarity"]
pub type VbusenapR = crate::BitReader;
#[doc = "Field `VBUSENAP` writer - VBUSEN Active Polarity"]
pub type VbusenapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SELFPOWERED` reader - PHY Power"]
pub type SelfpoweredR = crate::BitReader;
#[doc = "Field `SELFPOWERED` writer - PHY Power"]
pub type SelfpoweredW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Low Energy Mode Oscillator Control\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lemoscctrl {
    #[doc = "0: Low Energy Mode has no effect on neither USBC or USHFRCO."]
    None = 0,
    #[doc = "1: The USBC clock is gated when Low Energy Mode is active."]
    Gate = 1,
}
impl From<Lemoscctrl> for u8 {
    #[inline(always)]
    fn from(variant: Lemoscctrl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lemoscctrl {
    type Ux = u8;
}
impl crate::IsEnum for Lemoscctrl {}
#[doc = "Field `LEMOSCCTRL` reader - Low Energy Mode Oscillator Control"]
pub type LemoscctrlR = crate::FieldReader<Lemoscctrl>;
impl LemoscctrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lemoscctrl> {
        match self.bits {
            0 => Some(Lemoscctrl::None),
            1 => Some(Lemoscctrl::Gate),
            _ => None,
        }
    }
    #[doc = "Low Energy Mode has no effect on neither USBC or USHFRCO."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Lemoscctrl::None
    }
    #[doc = "The USBC clock is gated when Low Energy Mode is active."]
    #[inline(always)]
    pub fn is_gate(&self) -> bool {
        *self == Lemoscctrl::Gate
    }
}
#[doc = "Field `LEMOSCCTRL` writer - Low Energy Mode Oscillator Control"]
pub type LemoscctrlW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lemoscctrl>;
impl<'a, REG> LemoscctrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low Energy Mode has no effect on neither USBC or USHFRCO."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Lemoscctrl::None)
    }
    #[doc = "The USBC clock is gated when Low Energy Mode is active."]
    #[inline(always)]
    pub fn gate(self) -> &'a mut crate::W<REG> {
        self.variant(Lemoscctrl::Gate)
    }
}
#[doc = "Field `LEMPHYCTRL` reader - Low Energy Mode USB PHY Control"]
pub type LemphyctrlR = crate::BitReader;
#[doc = "Field `LEMPHYCTRL` writer - Low Energy Mode USB PHY Control"]
pub type LemphyctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEMIDLEEN` reader - Low Energy Mode on Bus Idle Enable"]
pub type LemidleenR = crate::BitReader;
#[doc = "Field `LEMIDLEEN` writer - Low Energy Mode on Bus Idle Enable"]
pub type LemidleenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDCDEN` reader - ID Pull-up Enable"]
pub type IdcdenR = crate::BitReader;
#[doc = "Field `IDCDEN` writer - ID Pull-up Enable"]
pub type IdcdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGCLKCDIS` reader - OTG CLKC Disable"]
pub type OtgclkcdisR = crate::BitReader;
#[doc = "Field `OTGCLKCDIS` writer - OTG CLKC Disable"]
pub type OtgclkcdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGIDINDIS` reader - OTG ID Input Disable"]
pub type OtgidindisR = crate::BitReader;
#[doc = "Field `OTGIDINDIS` writer - OTG ID Input Disable"]
pub type OtgidindisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGPHYCTRLDIS` reader - OTG Control Signals to PHY Disable"]
pub type OtgphyctrldisR = crate::BitReader;
#[doc = "Field `OTGPHYCTRLDIS` writer - OTG Control Signals to PHY Disable"]
pub type OtgphyctrldisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Data Contact Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dcden {
    #[doc = "0: DCD is disabled."]
    Disabled = 0,
    #[doc = "2: Only DCD timeout will be initiated."]
    Timeout = 2,
    #[doc = "3: Full DCD operation (physical contact and timeout) will be initiated."]
    Enabled = 3,
}
impl From<Dcden> for u8 {
    #[inline(always)]
    fn from(variant: Dcden) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dcden {
    type Ux = u8;
}
impl crate::IsEnum for Dcden {}
#[doc = "Field `DCDEN` reader - Data Contact Detection Enable"]
pub type DcdenR = crate::FieldReader<Dcden>;
impl DcdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dcden> {
        match self.bits {
            0 => Some(Dcden::Disabled),
            2 => Some(Dcden::Timeout),
            3 => Some(Dcden::Enabled),
            _ => None,
        }
    }
    #[doc = "DCD is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dcden::Disabled
    }
    #[doc = "Only DCD timeout will be initiated."]
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == Dcden::Timeout
    }
    #[doc = "Full DCD operation (physical contact and timeout) will be initiated."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dcden::Enabled
    }
}
#[doc = "Field `DCDEN` writer - Data Contact Detection Enable"]
pub type DcdenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dcden>;
impl<'a, REG> DcdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DCD is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dcden::Disabled)
    }
    #[doc = "Only DCD timeout will be initiated."]
    #[inline(always)]
    pub fn timeout(self) -> &'a mut crate::W<REG> {
        self.variant(Dcden::Timeout)
    }
    #[doc = "Full DCD operation (physical contact and timeout) will be initiated."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dcden::Enabled)
    }
}
#[doc = "Field `PDEN` reader - Primary Detection Enable"]
pub type PdenR = crate::BitReader;
#[doc = "Field `PDEN` writer - Primary Detection Enable"]
pub type PdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDEN` reader - Secondary Detection Enable"]
pub type SdenR = crate::BitReader;
#[doc = "Field `SDEN` writer - Secondary Detection Enable"]
pub type SdenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - VBUSEN Active Polarity"]
    #[inline(always)]
    pub fn vbusenap(&self) -> VbusenapR {
        VbusenapR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - PHY Power"]
    #[inline(always)]
    pub fn selfpowered(&self) -> SelfpoweredR {
        SelfpoweredR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Low Energy Mode Oscillator Control"]
    #[inline(always)]
    pub fn lemoscctrl(&self) -> LemoscctrlR {
        LemoscctrlR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Low Energy Mode USB PHY Control"]
    #[inline(always)]
    pub fn lemphyctrl(&self) -> LemphyctrlR {
        LemphyctrlR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Low Energy Mode on Bus Idle Enable"]
    #[inline(always)]
    pub fn lemidleen(&self) -> LemidleenR {
        LemidleenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - ID Pull-up Enable"]
    #[inline(always)]
    pub fn idcden(&self) -> IdcdenR {
        IdcdenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 25 - OTG CLKC Disable"]
    #[inline(always)]
    pub fn otgclkcdis(&self) -> OtgclkcdisR {
        OtgclkcdisR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - OTG ID Input Disable"]
    #[inline(always)]
    pub fn otgidindis(&self) -> OtgidindisR {
        OtgidindisR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - OTG Control Signals to PHY Disable"]
    #[inline(always)]
    pub fn otgphyctrldis(&self) -> OtgphyctrldisR {
        OtgphyctrldisR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Data Contact Detection Enable"]
    #[inline(always)]
    pub fn dcden(&self) -> DcdenR {
        DcdenR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Primary Detection Enable"]
    #[inline(always)]
    pub fn pden(&self) -> PdenR {
        PdenR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Secondary Detection Enable"]
    #[inline(always)]
    pub fn sden(&self) -> SdenR {
        SdenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VBUSEN Active Polarity"]
    #[inline(always)]
    pub fn vbusenap(&mut self) -> VbusenapW<'_, CtrlSpec> {
        VbusenapW::new(self, 0)
    }
    #[doc = "Bit 3 - PHY Power"]
    #[inline(always)]
    pub fn selfpowered(&mut self) -> SelfpoweredW<'_, CtrlSpec> {
        SelfpoweredW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Low Energy Mode Oscillator Control"]
    #[inline(always)]
    pub fn lemoscctrl(&mut self) -> LemoscctrlW<'_, CtrlSpec> {
        LemoscctrlW::new(self, 4)
    }
    #[doc = "Bit 7 - Low Energy Mode USB PHY Control"]
    #[inline(always)]
    pub fn lemphyctrl(&mut self) -> LemphyctrlW<'_, CtrlSpec> {
        LemphyctrlW::new(self, 7)
    }
    #[doc = "Bit 9 - Low Energy Mode on Bus Idle Enable"]
    #[inline(always)]
    pub fn lemidleen(&mut self) -> LemidleenW<'_, CtrlSpec> {
        LemidleenW::new(self, 9)
    }
    #[doc = "Bit 12 - ID Pull-up Enable"]
    #[inline(always)]
    pub fn idcden(&mut self) -> IdcdenW<'_, CtrlSpec> {
        IdcdenW::new(self, 12)
    }
    #[doc = "Bit 25 - OTG CLKC Disable"]
    #[inline(always)]
    pub fn otgclkcdis(&mut self) -> OtgclkcdisW<'_, CtrlSpec> {
        OtgclkcdisW::new(self, 25)
    }
    #[doc = "Bit 26 - OTG ID Input Disable"]
    #[inline(always)]
    pub fn otgidindis(&mut self) -> OtgidindisW<'_, CtrlSpec> {
        OtgidindisW::new(self, 26)
    }
    #[doc = "Bit 27 - OTG Control Signals to PHY Disable"]
    #[inline(always)]
    pub fn otgphyctrldis(&mut self) -> OtgphyctrldisW<'_, CtrlSpec> {
        OtgphyctrldisW::new(self, 27)
    }
    #[doc = "Bits 28:29 - Data Contact Detection Enable"]
    #[inline(always)]
    pub fn dcden(&mut self) -> DcdenW<'_, CtrlSpec> {
        DcdenW::new(self, 28)
    }
    #[doc = "Bit 30 - Primary Detection Enable"]
    #[inline(always)]
    pub fn pden(&mut self) -> PdenW<'_, CtrlSpec> {
        PdenW::new(self, 30)
    }
    #[doc = "Bit 31 - Secondary Detection Enable"]
    #[inline(always)]
    pub fn sden(&mut self) -> SdenW<'_, CtrlSpec> {
        SdenW::new(self, 31)
    }
}
#[doc = "System Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTRL to value 0x20"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x20;
}
