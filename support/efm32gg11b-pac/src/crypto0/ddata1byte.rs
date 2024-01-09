#[doc = "Register `DDATA1BYTE` reader"]
pub type R = crate::R<DDATA1BYTE_SPEC>;
#[doc = "Register `DDATA1BYTE` writer"]
pub type W = crate::W<DDATA1BYTE_SPEC>;
#[doc = "Field `DDATA1BYTE` reader - Ddata 1 Byte Access"]
pub type DDATA1BYTE_R = crate::FieldReader;
#[doc = "Field `DDATA1BYTE` writer - Ddata 1 Byte Access"]
pub type DDATA1BYTE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Ddata 1 Byte Access"]
    #[inline(always)]
    pub fn ddata1byte(&self) -> DDATA1BYTE_R {
        DDATA1BYTE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Ddata 1 Byte Access"]
    #[inline(always)]
    #[must_use]
    pub fn ddata1byte(&mut self) -> DDATA1BYTE_W<DDATA1BYTE_SPEC> {
        DDATA1BYTE_W::new(self, 0)
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
#[doc = "DDATA1 Register Byte Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddata1byte::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddata1byte::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDATA1BYTE_SPEC;
impl crate::RegisterSpec for DDATA1BYTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddata1byte::R`](R) reader structure"]
impl crate::Readable for DDATA1BYTE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ddata1byte::W`](W) writer structure"]
impl crate::Writable for DDATA1BYTE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDATA1BYTE to value 0"]
impl crate::Resettable for DDATA1BYTE_SPEC {
    const RESET_VALUE: u32 = 0;
}
