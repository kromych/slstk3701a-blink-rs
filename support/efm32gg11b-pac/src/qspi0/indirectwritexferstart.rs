#[doc = "Register `INDIRECTWRITEXFERSTART` reader"]
pub type R = crate::R<INDIRECTWRITEXFERSTART_SPEC>;
#[doc = "Register `INDIRECTWRITEXFERSTART` writer"]
pub type W = crate::W<INDIRECTWRITEXFERSTART_SPEC>;
#[doc = "Field `ADDR` reader - Start of Indirect Access"]
pub type ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Start of Indirect Access"]
pub type ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Start of Indirect Access"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start of Indirect Access"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<INDIRECTWRITEXFERSTART_SPEC, 0> {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Indirect Write Transfer Start Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indirectwritexferstart::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indirectwritexferstart::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INDIRECTWRITEXFERSTART_SPEC;
impl crate::RegisterSpec for INDIRECTWRITEXFERSTART_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`indirectwritexferstart::R`](R) reader structure"]
impl crate::Readable for INDIRECTWRITEXFERSTART_SPEC {}
#[doc = "`write(|w| ..)` method takes [`indirectwritexferstart::W`](W) writer structure"]
impl crate::Writable for INDIRECTWRITEXFERSTART_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INDIRECTWRITEXFERSTART to value 0"]
impl crate::Resettable for INDIRECTWRITEXFERSTART_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
