#[doc = "Register `AUTOFLUSHEDPKTS` reader"]
pub type R = crate::R<AutoflushedpktsSpec>;
#[doc = "Register `AUTOFLUSHEDPKTS` writer"]
pub type W = crate::W<AutoflushedpktsSpec>;
#[doc = "Field `COUNT` reader - Flushed RX pkts counter"]
pub type CountR = crate::FieldReader<u16>;
#[doc = "Field `COUNT` writer - Flushed RX pkts counter"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Flushed RX pkts counter"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Flushed RX pkts counter"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, AutoflushedpktsSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Receive DMA Flushed Packets\n\nYou can [`read`](crate::Reg::read) this register and get [`autoflushedpkts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autoflushedpkts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AutoflushedpktsSpec;
impl crate::RegisterSpec for AutoflushedpktsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`autoflushedpkts::R`](R) reader structure"]
impl crate::Readable for AutoflushedpktsSpec {}
#[doc = "`write(|w| ..)` method takes [`autoflushedpkts::W`](W) writer structure"]
impl crate::Writable for AutoflushedpktsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AUTOFLUSHEDPKTS to value 0"]
impl crate::Resettable for AutoflushedpktsSpec {}
