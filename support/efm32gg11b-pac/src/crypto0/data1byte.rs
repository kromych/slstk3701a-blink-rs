#[doc = "Register `DATA1BYTE` reader"]
pub type R = crate::R<DATA1BYTE_SPEC>;
#[doc = "Register `DATA1BYTE` writer"]
pub type W = crate::W<DATA1BYTE_SPEC>;
#[doc = "Field `DATA1BYTE` reader - Data 1 Byte Access"]
pub type DATA1BYTE_R = crate::FieldReader;
#[doc = "Field `DATA1BYTE` writer - Data 1 Byte Access"]
pub type DATA1BYTE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Data 1 Byte Access"]
    #[inline(always)]
    pub fn data1byte(&self) -> DATA1BYTE_R {
        DATA1BYTE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data 1 Byte Access"]
    #[inline(always)]
    #[must_use]
    pub fn data1byte(&mut self) -> DATA1BYTE_W<DATA1BYTE_SPEC, 0> {
        DATA1BYTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DATA1 Register Byte Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data1byte::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data1byte::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA1BYTE_SPEC;
impl crate::RegisterSpec for DATA1BYTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data1byte::R`](R) reader structure"]
impl crate::Readable for DATA1BYTE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data1byte::W`](W) writer structure"]
impl crate::Writable for DATA1BYTE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA1BYTE to value 0"]
impl crate::Resettable for DATA1BYTE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
