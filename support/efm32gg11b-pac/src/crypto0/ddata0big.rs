#[doc = "Register `DDATA0BIG` reader"]
pub type R = crate::R<DDATA0BIG_SPEC>;
#[doc = "Register `DDATA0BIG` writer"]
pub type W = crate::W<DDATA0BIG_SPEC>;
#[doc = "Field `DDATA0BIG` reader - Double Data 0 Big Endian Access"]
pub type DDATA0BIG_R = crate::FieldReader<u32>;
#[doc = "Field `DDATA0BIG` writer - Double Data 0 Big Endian Access"]
pub type DDATA0BIG_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Double Data 0 Big Endian Access"]
    #[inline(always)]
    pub fn ddata0big(&self) -> DDATA0BIG_R {
        DDATA0BIG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Double Data 0 Big Endian Access"]
    #[inline(always)]
    #[must_use]
    pub fn ddata0big(&mut self) -> DDATA0BIG_W<DDATA0BIG_SPEC> {
        DDATA0BIG_W::new(self, 0)
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
#[doc = "DDATA0 Register Big Endian Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddata0big::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddata0big::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDATA0BIG_SPEC;
impl crate::RegisterSpec for DDATA0BIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddata0big::R`](R) reader structure"]
impl crate::Readable for DDATA0BIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ddata0big::W`](W) writer structure"]
impl crate::Writable for DDATA0BIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDATA0BIG to value 0"]
impl crate::Resettable for DDATA0BIG_SPEC {
    const RESET_VALUE: u32 = 0;
}
