#[doc = "Register `PL_OVTDIS` reader"]
pub type R = crate::R<PL_OVTDIS_SPEC>;
#[doc = "Register `PL_OVTDIS` writer"]
pub type W = crate::W<PL_OVTDIS_SPEC>;
#[doc = "Field `OVTDIS` reader - Disable Over Voltage Capability"]
pub type OVTDIS_R = crate::FieldReader<u16>;
#[doc = "Field `OVTDIS` writer - Disable Over Voltage Capability"]
pub type OVTDIS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovtdis(&self) -> OVTDIS_R {
        OVTDIS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Disable Over Voltage Capability"]
    #[inline(always)]
    #[must_use]
    pub fn ovtdis(&mut self) -> OVTDIS_W<PL_OVTDIS_SPEC, 0> {
        OVTDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Over Voltage Disable for All Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pl_ovtdis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pl_ovtdis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PL_OVTDIS_SPEC;
impl crate::RegisterSpec for PL_OVTDIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pl_ovtdis::R`](R) reader structure"]
impl crate::Readable for PL_OVTDIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pl_ovtdis::W`](W) writer structure"]
impl crate::Writable for PL_OVTDIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PL_OVTDIS to value 0"]
impl crate::Resettable for PL_OVTDIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
