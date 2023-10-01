#[doc = "Register `DDATA3` reader"]
pub type R = crate::R<DDATA3_SPEC>;
#[doc = "Register `DDATA3` writer"]
pub type W = crate::W<DDATA3_SPEC>;
#[doc = "Field `DDATA3` reader - Double Data 0 Access"]
pub type DDATA3_R = crate::FieldReader<u32>;
#[doc = "Field `DDATA3` writer - Double Data 0 Access"]
pub type DDATA3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Double Data 0 Access"]
    #[inline(always)]
    pub fn ddata3(&self) -> DDATA3_R {
        DDATA3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Double Data 0 Access"]
    #[inline(always)]
    #[must_use]
    pub fn ddata3(&mut self) -> DDATA3_W<DDATA3_SPEC, 0> {
        DDATA3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DDATA3 Register Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddata3::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddata3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDATA3_SPEC;
impl crate::RegisterSpec for DDATA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddata3::R`](R) reader structure"]
impl crate::Readable for DDATA3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ddata3::W`](W) writer structure"]
impl crate::Writable for DDATA3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDATA3 to value 0"]
impl crate::Resettable for DDATA3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
