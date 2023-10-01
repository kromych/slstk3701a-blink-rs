#[doc = "Register `DATA3` reader"]
pub type R = crate::R<DATA3_SPEC>;
#[doc = "Register `DATA3` writer"]
pub type W = crate::W<DATA3_SPEC>;
#[doc = "Field `DATA3` reader - Data 3 Access"]
pub type DATA3_R = crate::FieldReader<u32>;
#[doc = "Field `DATA3` writer - Data 3 Access"]
pub type DATA3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Data 3 Access"]
    #[inline(always)]
    pub fn data3(&self) -> DATA3_R {
        DATA3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data 3 Access"]
    #[inline(always)]
    #[must_use]
    pub fn data3(&mut self) -> DATA3_W<DATA3_SPEC, 0> {
        DATA3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DATA3 Register Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data3::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA3_SPEC;
impl crate::RegisterSpec for DATA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data3::R`](R) reader structure"]
impl crate::Readable for DATA3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data3::W`](W) writer structure"]
impl crate::Writable for DATA3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA3 to value 0"]
impl crate::Resettable for DATA3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
