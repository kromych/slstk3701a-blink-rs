#[doc = "Register `CHDONE` reader"]
pub type R = crate::R<CHDONE_SPEC>;
#[doc = "Register `CHDONE` writer"]
pub type W = crate::W<CHDONE_SPEC>;
#[doc = "Field `CHDONE` reader - DMA Channel Linking or Done"]
pub type CHDONE_R = crate::FieldReader<u32>;
#[doc = "Field `CHDONE` writer - DMA Channel Linking or Done"]
pub type CHDONE_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - DMA Channel Linking or Done"]
    #[inline(always)]
    pub fn chdone(&self) -> CHDONE_R {
        CHDONE_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - DMA Channel Linking or Done"]
    #[inline(always)]
    #[must_use]
    pub fn chdone(&mut self) -> CHDONE_W<CHDONE_SPEC> {
        CHDONE_W::new(self, 0)
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
#[doc = "DMA Channel Linking Done Register (Single-Cycle RMW)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chdone::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chdone::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHDONE_SPEC;
impl crate::RegisterSpec for CHDONE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chdone::R`](R) reader structure"]
impl crate::Readable for CHDONE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chdone::W`](W) writer structure"]
impl crate::Writable for CHDONE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHDONE to value 0"]
impl crate::Resettable for CHDONE_SPEC {
    const RESET_VALUE: u32 = 0;
}
