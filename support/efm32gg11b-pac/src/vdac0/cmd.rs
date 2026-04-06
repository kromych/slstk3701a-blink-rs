#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Field `CH0EN` writer - DAC Channel 0 Enable"]
pub type Ch0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0DIS` writer - DAC Channel 0 Disable"]
pub type Ch0disW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1EN` writer - DAC Channel 1 Enable"]
pub type Ch1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1DIS` writer - DAC Channel 1 Disable"]
pub type Ch1disW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA0EN` writer - OPA0 Enable"]
pub type Opa0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA0DIS` writer - OPA0 Disable"]
pub type Opa0disW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA1EN` writer - OPA1 Enable"]
pub type Opa1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA1DIS` writer - OPA1 Disable"]
pub type Opa1disW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA2EN` writer - OPA2 Enable"]
pub type Opa2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA2DIS` writer - OPA2 Disable"]
pub type Opa2disW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA3EN` writer - OPA3 Enable"]
pub type Opa3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA3DIS` writer - OPA3 Disable"]
pub type Opa3disW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - DAC Channel 0 Enable"]
    #[inline(always)]
    pub fn ch0en(&mut self) -> Ch0enW<'_, CmdSpec> {
        Ch0enW::new(self, 0)
    }
    #[doc = "Bit 1 - DAC Channel 0 Disable"]
    #[inline(always)]
    pub fn ch0dis(&mut self) -> Ch0disW<'_, CmdSpec> {
        Ch0disW::new(self, 1)
    }
    #[doc = "Bit 2 - DAC Channel 1 Enable"]
    #[inline(always)]
    pub fn ch1en(&mut self) -> Ch1enW<'_, CmdSpec> {
        Ch1enW::new(self, 2)
    }
    #[doc = "Bit 3 - DAC Channel 1 Disable"]
    #[inline(always)]
    pub fn ch1dis(&mut self) -> Ch1disW<'_, CmdSpec> {
        Ch1disW::new(self, 3)
    }
    #[doc = "Bit 16 - OPA0 Enable"]
    #[inline(always)]
    pub fn opa0en(&mut self) -> Opa0enW<'_, CmdSpec> {
        Opa0enW::new(self, 16)
    }
    #[doc = "Bit 17 - OPA0 Disable"]
    #[inline(always)]
    pub fn opa0dis(&mut self) -> Opa0disW<'_, CmdSpec> {
        Opa0disW::new(self, 17)
    }
    #[doc = "Bit 18 - OPA1 Enable"]
    #[inline(always)]
    pub fn opa1en(&mut self) -> Opa1enW<'_, CmdSpec> {
        Opa1enW::new(self, 18)
    }
    #[doc = "Bit 19 - OPA1 Disable"]
    #[inline(always)]
    pub fn opa1dis(&mut self) -> Opa1disW<'_, CmdSpec> {
        Opa1disW::new(self, 19)
    }
    #[doc = "Bit 20 - OPA2 Enable"]
    #[inline(always)]
    pub fn opa2en(&mut self) -> Opa2enW<'_, CmdSpec> {
        Opa2enW::new(self, 20)
    }
    #[doc = "Bit 21 - OPA2 Disable"]
    #[inline(always)]
    pub fn opa2dis(&mut self) -> Opa2disW<'_, CmdSpec> {
        Opa2disW::new(self, 21)
    }
    #[doc = "Bit 22 - OPA3 Enable"]
    #[inline(always)]
    pub fn opa3en(&mut self) -> Opa3enW<'_, CmdSpec> {
        Opa3enW::new(self, 22)
    }
    #[doc = "Bit 23 - OPA3 Disable"]
    #[inline(always)]
    pub fn opa3dis(&mut self) -> Opa3disW<'_, CmdSpec> {
        Opa3disW::new(self, 23)
    }
}
#[doc = "Command Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdSpec;
impl crate::RegisterSpec for CmdSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CmdSpec {}
