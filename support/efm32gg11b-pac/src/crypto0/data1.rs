#[doc = "Register `DATA1` reader"]
pub type R = crate::R<DATA1_SPEC>;
#[doc = "Register `DATA1` writer"]
pub type W = crate::W<DATA1_SPEC>;
#[doc = "Field `DATA1` reader - Data 1 Access"]
pub type DATA1_R = crate::FieldReader<u32>;
#[doc = "Field `DATA1` writer - Data 1 Access"]
pub type DATA1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Data 1 Access"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data 1 Access"]
    #[inline(always)]
    #[must_use]
    pub fn data1(&mut self) -> DATA1_W<DATA1_SPEC, 0> {
        DATA1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DATA1 Register Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data1::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA1_SPEC;
impl crate::RegisterSpec for DATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data1::R`](R) reader structure"]
impl crate::Readable for DATA1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data1::W`](W) writer structure"]
impl crate::Writable for DATA1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA1 to value 0"]
impl crate::Resettable for DATA1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
