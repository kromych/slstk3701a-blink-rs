#[doc = "Register `R5VCTRL` reader"]
pub type R = crate::R<R5vctrlSpec>;
#[doc = "Register `R5VCTRL` writer"]
pub type W = crate::W<R5vctrlSpec>;
#[doc = "Field `BYPASS` reader - 5V Regulator Bypass"]
pub type BypassR = crate::BitReader;
#[doc = "Field `BYPASS` writer - 5V Regulator Bypass"]
pub type BypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM4WUEN` reader - Enable EM4 Wakeup Due to VBUS Detection"]
pub type Em4wuenR = crate::BitReader;
#[doc = "Field `EM4WUEN` writer - Enable EM4 Wakeup Due to VBUS Detection"]
pub type Em4wuenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMONEN` reader - Enable the Regulator Current Monitor for Selected Current Path to Either VREGI or VBUS"]
pub type ImonenR = crate::BitReader;
#[doc = "Field `IMONEN` writer - Enable the Regulator Current Monitor for Selected Current Path to Either VREGI or VBUS"]
pub type ImonenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "5V Input Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Inputmode {
    #[doc = "0: Regulator input supply switched automatically to the highest voltage of either VBUS or VREGI"]
    Auto = 0,
    #[doc = "1: Force VBUS pin as the regulator input"]
    Vbus = 1,
    #[doc = "2: Force VREGI pin as the regulator input"]
    Vregi = 2,
}
impl From<Inputmode> for u8 {
    #[inline(always)]
    fn from(variant: Inputmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Inputmode {
    type Ux = u8;
}
impl crate::IsEnum for Inputmode {}
#[doc = "Field `INPUTMODE` reader - 5V Input Mode"]
pub type InputmodeR = crate::FieldReader<Inputmode>;
impl InputmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Inputmode> {
        match self.bits {
            0 => Some(Inputmode::Auto),
            1 => Some(Inputmode::Vbus),
            2 => Some(Inputmode::Vregi),
            _ => None,
        }
    }
    #[doc = "Regulator input supply switched automatically to the highest voltage of either VBUS or VREGI"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == Inputmode::Auto
    }
    #[doc = "Force VBUS pin as the regulator input"]
    #[inline(always)]
    pub fn is_vbus(&self) -> bool {
        *self == Inputmode::Vbus
    }
    #[doc = "Force VREGI pin as the regulator input"]
    #[inline(always)]
    pub fn is_vregi(&self) -> bool {
        *self == Inputmode::Vregi
    }
}
#[doc = "Field `INPUTMODE` writer - 5V Input Mode"]
pub type InputmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Inputmode>;
impl<'a, REG> InputmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Regulator input supply switched automatically to the highest voltage of either VBUS or VREGI"]
    #[inline(always)]
    pub fn auto(self) -> &'a mut crate::W<REG> {
        self.variant(Inputmode::Auto)
    }
    #[doc = "Force VBUS pin as the regulator input"]
    #[inline(always)]
    pub fn vbus(self) -> &'a mut crate::W<REG> {
        self.variant(Inputmode::Vbus)
    }
    #[doc = "Force VREGI pin as the regulator input"]
    #[inline(always)]
    pub fn vregi(self) -> &'a mut crate::W<REG> {
        self.variant(Inputmode::Vregi)
    }
}
impl R {
    #[doc = "Bit 0 - 5V Regulator Bypass"]
    #[inline(always)]
    pub fn bypass(&self) -> BypassR {
        BypassR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable EM4 Wakeup Due to VBUS Detection"]
    #[inline(always)]
    pub fn em4wuen(&self) -> Em4wuenR {
        Em4wuenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable the Regulator Current Monitor for Selected Current Path to Either VREGI or VBUS"]
    #[inline(always)]
    pub fn imonen(&self) -> ImonenR {
        ImonenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:9 - 5V Input Mode"]
    #[inline(always)]
    pub fn inputmode(&self) -> InputmodeR {
        InputmodeR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 5V Regulator Bypass"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BypassW<'_, R5vctrlSpec> {
        BypassW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable EM4 Wakeup Due to VBUS Detection"]
    #[inline(always)]
    pub fn em4wuen(&mut self) -> Em4wuenW<'_, R5vctrlSpec> {
        Em4wuenW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable the Regulator Current Monitor for Selected Current Path to Either VREGI or VBUS"]
    #[inline(always)]
    pub fn imonen(&mut self) -> ImonenW<'_, R5vctrlSpec> {
        ImonenW::new(self, 2)
    }
    #[doc = "Bits 8:9 - 5V Input Mode"]
    #[inline(always)]
    pub fn inputmode(&mut self) -> InputmodeW<'_, R5vctrlSpec> {
        InputmodeW::new(self, 8)
    }
}
#[doc = "5V Regulator Control\n\nYou can [`read`](crate::Reg::read) this register and get [`r5vctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5vctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R5vctrlSpec;
impl crate::RegisterSpec for R5vctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r5vctrl::R`](R) reader structure"]
impl crate::Readable for R5vctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`r5vctrl::W`](W) writer structure"]
impl crate::Writable for R5vctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R5VCTRL to value 0"]
impl crate::Resettable for R5vctrlSpec {}
