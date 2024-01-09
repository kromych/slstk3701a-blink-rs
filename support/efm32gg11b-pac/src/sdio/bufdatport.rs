#[doc = "Register `BUFDATPORT` reader"]
pub type R = crate::R<BUFDATPORT_SPEC>;
#[doc = "Register `BUFDATPORT` writer"]
pub type W = crate::W<BUFDATPORT_SPEC>;
#[doc = "Field `BUFDAT` reader - Buffer Data"]
pub type BUFDAT_R = crate::FieldReader<u32>;
#[doc = "Field `BUFDAT` writer - Buffer Data"]
pub type BUFDAT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer Data"]
    #[inline(always)]
    pub fn bufdat(&self) -> BUFDAT_R {
        BUFDAT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer Data"]
    #[inline(always)]
    #[must_use]
    pub fn bufdat(&mut self) -> BUFDAT_W<BUFDATPORT_SPEC> {
        BUFDAT_W::new(self, 0)
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
#[doc = "Buffer Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bufdatport::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bufdatport::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUFDATPORT_SPEC;
impl crate::RegisterSpec for BUFDATPORT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bufdatport::R`](R) reader structure"]
impl crate::Readable for BUFDATPORT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bufdatport::W`](W) writer structure"]
impl crate::Writable for BUFDATPORT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BUFDATPORT to value 0"]
impl crate::Resettable for BUFDATPORT_SPEC {
    const RESET_VALUE: u32 = 0;
}
