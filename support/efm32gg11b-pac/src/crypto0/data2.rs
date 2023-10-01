#[doc = "Register `DATA2` reader"]
pub type R = crate::R<DATA2_SPEC>;
#[doc = "Register `DATA2` writer"]
pub type W = crate::W<DATA2_SPEC>;
#[doc = "Field `DATA2` reader - Data 2 Access"]
pub type DATA2_R = crate::FieldReader<u32>;
#[doc = "Field `DATA2` writer - Data 2 Access"]
pub type DATA2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Data 2 Access"]
    #[inline(always)]
    pub fn data2(&self) -> DATA2_R {
        DATA2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data 2 Access"]
    #[inline(always)]
    #[must_use]
    pub fn data2(&mut self) -> DATA2_W<DATA2_SPEC, 0> {
        DATA2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DATA2 Register Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data2::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA2_SPEC;
impl crate::RegisterSpec for DATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data2::R`](R) reader structure"]
impl crate::Readable for DATA2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data2::W`](W) writer structure"]
impl crate::Writable for DATA2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA2 to value 0"]
impl crate::Resettable for DATA2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
