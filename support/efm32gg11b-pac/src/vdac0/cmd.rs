#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMD_SPEC>;
#[doc = "Field `CH0EN` writer - DAC Channel 0 Enable"]
pub type CH0EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0DIS` writer - DAC Channel 0 Disable"]
pub type CH0DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1EN` writer - DAC Channel 1 Enable"]
pub type CH1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1DIS` writer - DAC Channel 1 Disable"]
pub type CH1DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA0EN` writer - OPA0 Enable"]
pub type OPA0EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA0DIS` writer - OPA0 Disable"]
pub type OPA0DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA1EN` writer - OPA1 Enable"]
pub type OPA1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA1DIS` writer - OPA1 Disable"]
pub type OPA1DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA2EN` writer - OPA2 Enable"]
pub type OPA2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA2DIS` writer - OPA2 Disable"]
pub type OPA2DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA3EN` writer - OPA3 Enable"]
pub type OPA3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA3DIS` writer - OPA3 Disable"]
pub type OPA3DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - DAC Channel 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0en(&mut self) -> CH0EN_W<CMD_SPEC> {
        CH0EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - DAC Channel 0 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0dis(&mut self) -> CH0DIS_W<CMD_SPEC> {
        CH0DIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - DAC Channel 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1en(&mut self) -> CH1EN_W<CMD_SPEC> {
        CH1EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - DAC Channel 1 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1dis(&mut self) -> CH1DIS_W<CMD_SPEC> {
        CH1DIS_W::new(self, 3)
    }
    #[doc = "Bit 16 - OPA0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn opa0en(&mut self) -> OPA0EN_W<CMD_SPEC> {
        OPA0EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - OPA0 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn opa0dis(&mut self) -> OPA0DIS_W<CMD_SPEC> {
        OPA0DIS_W::new(self, 17)
    }
    #[doc = "Bit 18 - OPA1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn opa1en(&mut self) -> OPA1EN_W<CMD_SPEC> {
        OPA1EN_W::new(self, 18)
    }
    #[doc = "Bit 19 - OPA1 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn opa1dis(&mut self) -> OPA1DIS_W<CMD_SPEC> {
        OPA1DIS_W::new(self, 19)
    }
    #[doc = "Bit 20 - OPA2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn opa2en(&mut self) -> OPA2EN_W<CMD_SPEC> {
        OPA2EN_W::new(self, 20)
    }
    #[doc = "Bit 21 - OPA2 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn opa2dis(&mut self) -> OPA2DIS_W<CMD_SPEC> {
        OPA2DIS_W::new(self, 21)
    }
    #[doc = "Bit 22 - OPA3 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn opa3en(&mut self) -> OPA3EN_W<CMD_SPEC> {
        OPA3EN_W::new(self, 22)
    }
    #[doc = "Bit 23 - OPA3 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn opa3dis(&mut self) -> OPA3DIS_W<CMD_SPEC> {
        OPA3DIS_W::new(self, 23)
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
#[doc = "Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: u32 = 0;
}
