#[doc = "Register `DCDCLNFREQCTRL` reader"]
pub type R = crate::R<DCDCLNFREQCTRL_SPEC>;
#[doc = "Register `DCDCLNFREQCTRL` writer"]
pub type W = crate::W<DCDCLNFREQCTRL_SPEC>;
#[doc = "Field `RCOBAND` reader - LN Mode RCO Frequency Band Selection"]
pub type RCOBAND_R = crate::FieldReader;
#[doc = "Field `RCOBAND` writer - LN Mode RCO Frequency Band Selection"]
pub type RCOBAND_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RCOTRIM` reader - Reserved for internal use. Do not change."]
pub type RCOTRIM_R = crate::FieldReader;
#[doc = "Field `RCOTRIM` writer - Reserved for internal use. Do not change."]
pub type RCOTRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:2 - LN Mode RCO Frequency Band Selection"]
    #[inline(always)]
    pub fn rcoband(&self) -> RCOBAND_R {
        RCOBAND_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 24:28 - Reserved for internal use. Do not change."]
    #[inline(always)]
    pub fn rcotrim(&self) -> RCOTRIM_R {
        RCOTRIM_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - LN Mode RCO Frequency Band Selection"]
    #[inline(always)]
    #[must_use]
    pub fn rcoband(&mut self) -> RCOBAND_W<DCDCLNFREQCTRL_SPEC> {
        RCOBAND_W::new(self, 0)
    }
    #[doc = "Bits 24:28 - Reserved for internal use. Do not change."]
    #[inline(always)]
    #[must_use]
    pub fn rcotrim(&mut self) -> RCOTRIM_W<DCDCLNFREQCTRL_SPEC> {
        RCOTRIM_W::new(self, 24)
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
#[doc = "DCDC Low Noise Controller Frequency Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdclnfreqctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdclnfreqctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCDCLNFREQCTRL_SPEC;
impl crate::RegisterSpec for DCDCLNFREQCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdclnfreqctrl::R`](R) reader structure"]
impl crate::Readable for DCDCLNFREQCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcdclnfreqctrl::W`](W) writer structure"]
impl crate::Writable for DCDCLNFREQCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCDCLNFREQCTRL to value 0x1000_0000"]
impl crate::Resettable for DCDCLNFREQCTRL_SPEC {
    const RESET_VALUE: u32 = 0x1000_0000;
}
