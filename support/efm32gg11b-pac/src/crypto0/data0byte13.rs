#[doc = "Register `DATA0BYTE13` reader"]
pub type R = crate::R<DATA0BYTE13_SPEC>;
#[doc = "Register `DATA0BYTE13` writer"]
pub type W = crate::W<DATA0BYTE13_SPEC>;
#[doc = "Field `DATA0BYTE13` reader - Data 0 Byte 13 Access"]
pub type DATA0BYTE13_R = crate::FieldReader;
#[doc = "Field `DATA0BYTE13` writer - Data 0 Byte 13 Access"]
pub type DATA0BYTE13_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Data 0 Byte 13 Access"]
    #[inline(always)]
    pub fn data0byte13(&self) -> DATA0BYTE13_R {
        DATA0BYTE13_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data 0 Byte 13 Access"]
    #[inline(always)]
    #[must_use]
    pub fn data0byte13(&mut self) -> DATA0BYTE13_W<DATA0BYTE13_SPEC, 0> {
        DATA0BYTE13_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DATA0 Register Byte 13 Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data0byte13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data0byte13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA0BYTE13_SPEC;
impl crate::RegisterSpec for DATA0BYTE13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data0byte13::R`](R) reader structure"]
impl crate::Readable for DATA0BYTE13_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data0byte13::W`](W) writer structure"]
impl crate::Writable for DATA0BYTE13_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA0BYTE13 to value 0"]
impl crate::Resettable for DATA0BYTE13_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
