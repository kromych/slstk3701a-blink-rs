#[doc = "Register `SDMASYSADDR` reader"]
pub type R = crate::R<SDMASYSADDR_SPEC>;
#[doc = "Register `SDMASYSADDR` writer"]
pub type W = crate::W<SDMASYSADDR_SPEC>;
#[doc = "Field `SDMASYSADDRARG` reader - Physical SYS Memory ADDR Used for DMA Transfers or the Second Argument for the Auto CMD23"]
pub type SDMASYSADDRARG_R = crate::FieldReader<u32>;
#[doc = "Field `SDMASYSADDRARG` writer - Physical SYS Memory ADDR Used for DMA Transfers or the Second Argument for the Auto CMD23"]
pub type SDMASYSADDRARG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Physical SYS Memory ADDR Used for DMA Transfers or the Second Argument for the Auto CMD23"]
    #[inline(always)]
    pub fn sdmasysaddrarg(&self) -> SDMASYSADDRARG_R {
        SDMASYSADDRARG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Physical SYS Memory ADDR Used for DMA Transfers or the Second Argument for the Auto CMD23"]
    #[inline(always)]
    #[must_use]
    pub fn sdmasysaddrarg(&mut self) -> SDMASYSADDRARG_W<SDMASYSADDR_SPEC, 0> {
        SDMASYSADDRARG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SDMA System Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmasysaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmasysaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDMASYSADDR_SPEC;
impl crate::RegisterSpec for SDMASYSADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmasysaddr::R`](R) reader structure"]
impl crate::Readable for SDMASYSADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdmasysaddr::W`](W) writer structure"]
impl crate::Writable for SDMASYSADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDMASYSADDR to value 0"]
impl crate::Resettable for SDMASYSADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
