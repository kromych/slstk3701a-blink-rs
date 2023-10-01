#[doc = "Register `QDATA0BYTE` reader"]
pub type R = crate::R<QDATA0BYTE_SPEC>;
#[doc = "Register `QDATA0BYTE` writer"]
pub type W = crate::W<QDATA0BYTE_SPEC>;
#[doc = "Field `QDATA0BYTE` reader - Qdata 0 Byte Access"]
pub type QDATA0BYTE_R = crate::FieldReader;
#[doc = "Field `QDATA0BYTE` writer - Qdata 0 Byte Access"]
pub type QDATA0BYTE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Qdata 0 Byte Access"]
    #[inline(always)]
    pub fn qdata0byte(&self) -> QDATA0BYTE_R {
        QDATA0BYTE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Qdata 0 Byte Access"]
    #[inline(always)]
    #[must_use]
    pub fn qdata0byte(&mut self) -> QDATA0BYTE_W<QDATA0BYTE_SPEC, 0> {
        QDATA0BYTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "QDATA0 Register Byte Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qdata0byte::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qdata0byte::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QDATA0BYTE_SPEC;
impl crate::RegisterSpec for QDATA0BYTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qdata0byte::R`](R) reader structure"]
impl crate::Readable for QDATA0BYTE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`qdata0byte::W`](W) writer structure"]
impl crate::Writable for QDATA0BYTE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets QDATA0BYTE to value 0"]
impl crate::Resettable for QDATA0BYTE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
