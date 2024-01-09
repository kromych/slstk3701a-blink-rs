#[doc = "Register `HC7_DMAADDR` reader"]
pub type R = crate::R<HC7_DMAADDR_SPEC>;
#[doc = "Register `HC7_DMAADDR` writer"]
pub type W = crate::W<HC7_DMAADDR_SPEC>;
#[doc = "Field `DMAADDR` reader - DMA Address"]
pub type DMAADDR_R = crate::FieldReader<u32>;
#[doc = "Field `DMAADDR` writer - DMA Address"]
pub type DMAADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    pub fn dmaaddr(&self) -> DMAADDR_R {
        DMAADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    #[must_use]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<HC7_DMAADDR_SPEC> {
        DMAADDR_W::new(self, 0)
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
#[doc = "Host Channel x DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc7_dmaaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc7_dmaaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HC7_DMAADDR_SPEC;
impl crate::RegisterSpec for HC7_DMAADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hc7_dmaaddr::R`](R) reader structure"]
impl crate::Readable for HC7_DMAADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hc7_dmaaddr::W`](W) writer structure"]
impl crate::Writable for HC7_DMAADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HC7_DMAADDR to value 0"]
impl crate::Resettable for HC7_DMAADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
