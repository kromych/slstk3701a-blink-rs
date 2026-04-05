#[doc = "Register `HC9_DMAADDR` reader"]
pub type R = crate::R<Hc9DmaaddrSpec>;
#[doc = "Register `HC9_DMAADDR` writer"]
pub type W = crate::W<Hc9DmaaddrSpec>;
#[doc = "Field `DMAADDR` reader - DMA Address"]
pub type DmaaddrR = crate::FieldReader<u32>;
#[doc = "Field `DMAADDR` writer - DMA Address"]
pub type DmaaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    pub fn dmaaddr(&self) -> DmaaddrR {
        DmaaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    pub fn dmaaddr(&mut self) -> DmaaddrW<'_, Hc9DmaaddrSpec> {
        DmaaddrW::new(self, 0)
    }
}
#[doc = "Host Channel x DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc9_dmaaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc9_dmaaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hc9DmaaddrSpec;
impl crate::RegisterSpec for Hc9DmaaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hc9_dmaaddr::R`](R) reader structure"]
impl crate::Readable for Hc9DmaaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`hc9_dmaaddr::W`](W) writer structure"]
impl crate::Writable for Hc9DmaaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HC9_DMAADDR to value 0"]
impl crate::Resettable for Hc9DmaaddrSpec {}
