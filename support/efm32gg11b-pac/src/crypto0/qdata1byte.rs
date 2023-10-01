#[doc = "Register `QDATA1BYTE` reader"]
pub type R = crate::R<QDATA1BYTE_SPEC>;
#[doc = "Register `QDATA1BYTE` writer"]
pub type W = crate::W<QDATA1BYTE_SPEC>;
#[doc = "Field `QDATA1BYTE` reader - Qdata 1 Byte Access"]
pub type QDATA1BYTE_R = crate::FieldReader;
#[doc = "Field `QDATA1BYTE` writer - Qdata 1 Byte Access"]
pub type QDATA1BYTE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Qdata 1 Byte Access"]
    #[inline(always)]
    pub fn qdata1byte(&self) -> QDATA1BYTE_R {
        QDATA1BYTE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Qdata 1 Byte Access"]
    #[inline(always)]
    #[must_use]
    pub fn qdata1byte(&mut self) -> QDATA1BYTE_W<QDATA1BYTE_SPEC, 0> {
        QDATA1BYTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "QDATA1 Register Byte Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qdata1byte::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qdata1byte::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QDATA1BYTE_SPEC;
impl crate::RegisterSpec for QDATA1BYTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qdata1byte::R`](R) reader structure"]
impl crate::Readable for QDATA1BYTE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`qdata1byte::W`](W) writer structure"]
impl crate::Writable for QDATA1BYTE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets QDATA1BYTE to value 0"]
impl crate::Resettable for QDATA1BYTE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
