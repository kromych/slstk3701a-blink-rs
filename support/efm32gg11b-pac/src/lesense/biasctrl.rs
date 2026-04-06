#[doc = "Register `BIASCTRL` reader"]
pub type R = crate::R<BiasctrlSpec>;
#[doc = "Register `BIASCTRL` writer"]
pub type W = crate::W<BiasctrlSpec>;
#[doc = "Select Bias Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Biasmode {
    #[doc = "0: Bias module is controlled by the EMU and is not affected by LESENSE"]
    Donttouch = 0,
    #[doc = "1: Bias module duty cycled between low power and high accuracy mode"]
    Dutycycle = 1,
    #[doc = "2: Bias module always in high accuracy mode"]
    Highacc = 2,
}
impl From<Biasmode> for u8 {
    #[inline(always)]
    fn from(variant: Biasmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Biasmode {
    type Ux = u8;
}
impl crate::IsEnum for Biasmode {}
#[doc = "Field `BIASMODE` reader - Select Bias Mode"]
pub type BiasmodeR = crate::FieldReader<Biasmode>;
impl BiasmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Biasmode> {
        match self.bits {
            0 => Some(Biasmode::Donttouch),
            1 => Some(Biasmode::Dutycycle),
            2 => Some(Biasmode::Highacc),
            _ => None,
        }
    }
    #[doc = "Bias module is controlled by the EMU and is not affected by LESENSE"]
    #[inline(always)]
    pub fn is_donttouch(&self) -> bool {
        *self == Biasmode::Donttouch
    }
    #[doc = "Bias module duty cycled between low power and high accuracy mode"]
    #[inline(always)]
    pub fn is_dutycycle(&self) -> bool {
        *self == Biasmode::Dutycycle
    }
    #[doc = "Bias module always in high accuracy mode"]
    #[inline(always)]
    pub fn is_highacc(&self) -> bool {
        *self == Biasmode::Highacc
    }
}
#[doc = "Field `BIASMODE` writer - Select Bias Mode"]
pub type BiasmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Biasmode>;
impl<'a, REG> BiasmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Bias module is controlled by the EMU and is not affected by LESENSE"]
    #[inline(always)]
    pub fn donttouch(self) -> &'a mut crate::W<REG> {
        self.variant(Biasmode::Donttouch)
    }
    #[doc = "Bias module duty cycled between low power and high accuracy mode"]
    #[inline(always)]
    pub fn dutycycle(self) -> &'a mut crate::W<REG> {
        self.variant(Biasmode::Dutycycle)
    }
    #[doc = "Bias module always in high accuracy mode"]
    #[inline(always)]
    pub fn highacc(self) -> &'a mut crate::W<REG> {
        self.variant(Biasmode::Highacc)
    }
}
impl R {
    #[doc = "Bits 0:1 - Select Bias Mode"]
    #[inline(always)]
    pub fn biasmode(&self) -> BiasmodeR {
        BiasmodeR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Select Bias Mode"]
    #[inline(always)]
    pub fn biasmode(&mut self) -> BiasmodeW<'_, BiasctrlSpec> {
        BiasmodeW::new(self, 0)
    }
}
#[doc = "Bias Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`biasctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`biasctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BiasctrlSpec;
impl crate::RegisterSpec for BiasctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`biasctrl::R`](R) reader structure"]
impl crate::Readable for BiasctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`biasctrl::W`](W) writer structure"]
impl crate::Writable for BiasctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BIASCTRL to value 0"]
impl crate::Resettable for BiasctrlSpec {}
