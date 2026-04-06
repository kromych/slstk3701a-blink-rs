#[doc = "Register `DIEP3_DMAADDR` reader"]
pub type R = crate::R<Diep3DmaaddrSpec>;
#[doc = "Register `DIEP3_DMAADDR` writer"]
pub type W = crate::W<Diep3DmaaddrSpec>;
#[doc = "Field `DMAADDR` reader - "]
pub type DmaaddrR = crate::FieldReader<u32>;
#[doc = "Field `DMAADDR` writer - "]
pub type DmaaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dmaaddr(&self) -> DmaaddrR {
        DmaaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dmaaddr(&mut self) -> DmaaddrW<'_, Diep3DmaaddrSpec> {
        DmaaddrW::new(self, 0)
    }
}
#[doc = "Device IN Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep3_dmaaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep3_dmaaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Diep3DmaaddrSpec;
impl crate::RegisterSpec for Diep3DmaaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diep3_dmaaddr::R`](R) reader structure"]
impl crate::Readable for Diep3DmaaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`diep3_dmaaddr::W`](W) writer structure"]
impl crate::Writable for Diep3DmaaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEP3_DMAADDR to value 0"]
impl crate::Resettable for Diep3DmaaddrSpec {}
