#[doc = "Register `HC8_DMAADDR` reader"]
pub type R = crate::R<Hc8DmaaddrSpec>;
#[doc = "Register `HC8_DMAADDR` writer"]
pub type W = crate::W<Hc8DmaaddrSpec>;
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
    pub fn dmaaddr(&mut self) -> DmaaddrW<'_, Hc8DmaaddrSpec> {
        DmaaddrW::new(self, 0)
    }
}
#[doc = "Host Channel x DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc8_dmaaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc8_dmaaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hc8DmaaddrSpec;
impl crate::RegisterSpec for Hc8DmaaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hc8_dmaaddr::R`](R) reader structure"]
impl crate::Readable for Hc8DmaaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`hc8_dmaaddr::W`](W) writer structure"]
impl crate::Writable for Hc8DmaaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HC8_DMAADDR to value 0"]
impl crate::Resettable for Hc8DmaaddrSpec {}
