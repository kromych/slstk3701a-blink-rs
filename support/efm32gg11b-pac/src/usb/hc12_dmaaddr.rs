#[doc = "Register `HC12_DMAADDR` reader"]
pub type R = crate::R<Hc12DmaaddrSpec>;
#[doc = "Register `HC12_DMAADDR` writer"]
pub type W = crate::W<Hc12DmaaddrSpec>;
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
    pub fn dmaaddr(&mut self) -> DmaaddrW<'_, Hc12DmaaddrSpec> {
        DmaaddrW::new(self, 0)
    }
}
#[doc = "Host Channel x DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc12_dmaaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc12_dmaaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hc12DmaaddrSpec;
impl crate::RegisterSpec for Hc12DmaaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hc12_dmaaddr::R`](R) reader structure"]
impl crate::Readable for Hc12DmaaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`hc12_dmaaddr::W`](W) writer structure"]
impl crate::Writable for Hc12DmaaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HC12_DMAADDR to value 0"]
impl crate::Resettable for Hc12DmaaddrSpec {}
