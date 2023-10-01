#[doc = "Register `DATA0XORBYTE` reader"]
pub type R = crate::R<DATA0XORBYTE_SPEC>;
#[doc = "Register `DATA0XORBYTE` writer"]
pub type W = crate::W<DATA0XORBYTE_SPEC>;
#[doc = "Field `DATA0XORBYTE` reader - Data 0 XOR Byte Access"]
pub type DATA0XORBYTE_R = crate::FieldReader;
#[doc = "Field `DATA0XORBYTE` writer - Data 0 XOR Byte Access"]
pub type DATA0XORBYTE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Data 0 XOR Byte Access"]
    #[inline(always)]
    pub fn data0xorbyte(&self) -> DATA0XORBYTE_R {
        DATA0XORBYTE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data 0 XOR Byte Access"]
    #[inline(always)]
    #[must_use]
    pub fn data0xorbyte(&mut self) -> DATA0XORBYTE_W<DATA0XORBYTE_SPEC, 0> {
        DATA0XORBYTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DATA0 Register Byte XOR Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data0xorbyte::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data0xorbyte::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA0XORBYTE_SPEC;
impl crate::RegisterSpec for DATA0XORBYTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data0xorbyte::R`](R) reader structure"]
impl crate::Readable for DATA0XORBYTE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data0xorbyte::W`](W) writer structure"]
impl crate::Writable for DATA0XORBYTE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA0XORBYTE to value 0"]
impl crate::Resettable for DATA0XORBYTE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
