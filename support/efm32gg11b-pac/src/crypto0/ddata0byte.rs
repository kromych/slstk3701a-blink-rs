#[doc = "Register `DDATA0BYTE` reader"]
pub type R = crate::R<DDATA0BYTE_SPEC>;
#[doc = "Register `DDATA0BYTE` writer"]
pub type W = crate::W<DDATA0BYTE_SPEC>;
#[doc = "Field `DDATA0BYTE` reader - Ddata 0 Byte Access"]
pub type DDATA0BYTE_R = crate::FieldReader;
#[doc = "Field `DDATA0BYTE` writer - Ddata 0 Byte Access"]
pub type DDATA0BYTE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Ddata 0 Byte Access"]
    #[inline(always)]
    pub fn ddata0byte(&self) -> DDATA0BYTE_R {
        DDATA0BYTE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Ddata 0 Byte Access"]
    #[inline(always)]
    #[must_use]
    pub fn ddata0byte(&mut self) -> DDATA0BYTE_W<DDATA0BYTE_SPEC> {
        DDATA0BYTE_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DDATA0 Register Byte Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddata0byte::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddata0byte::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDATA0BYTE_SPEC;
impl crate::RegisterSpec for DDATA0BYTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddata0byte::R`](R) reader structure"]
impl crate::Readable for DDATA0BYTE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ddata0byte::W`](W) writer structure"]
impl crate::Writable for DDATA0BYTE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDATA0BYTE to value 0"]
impl crate::Resettable for DDATA0BYTE_SPEC {
    const RESET_VALUE: u32 = 0;
}
