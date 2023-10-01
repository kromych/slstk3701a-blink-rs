#[doc = "Register `REQDIS` reader"]
pub type R = crate::R<REQDIS_SPEC>;
#[doc = "Register `REQDIS` writer"]
pub type W = crate::W<REQDIS_SPEC>;
#[doc = "Field `REQDIS` reader - DMA Request Disables"]
pub type REQDIS_R = crate::FieldReader<u32>;
#[doc = "Field `REQDIS` writer - DMA Request Disables"]
pub type REQDIS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
impl R {
    #[doc = "Bits 0:23 - DMA Request Disables"]
    #[inline(always)]
    pub fn reqdis(&self) -> REQDIS_R {
        REQDIS_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - DMA Request Disables"]
    #[inline(always)]
    #[must_use]
    pub fn reqdis(&mut self) -> REQDIS_W<REQDIS_SPEC, 0> {
        REQDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA Channel Request Disable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reqdis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reqdis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REQDIS_SPEC;
impl crate::RegisterSpec for REQDIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reqdis::R`](R) reader structure"]
impl crate::Readable for REQDIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reqdis::W`](W) writer structure"]
impl crate::Writable for REQDIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REQDIS to value 0"]
impl crate::Resettable for REQDIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
