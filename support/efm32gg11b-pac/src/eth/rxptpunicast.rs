#[doc = "Register `RXPTPUNICAST` reader"]
pub type R = crate::R<RxptpunicastSpec>;
#[doc = "Register `RXPTPUNICAST` writer"]
pub type W = crate::W<RxptpunicastSpec>;
#[doc = "Field `ADDR` reader - Unicast IP destination address"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Unicast IP destination address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Unicast IP destination address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Unicast IP destination address"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, RxptpunicastSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "PTP RX unicast IP destination address\n\nYou can [`read`](crate::Reg::read) this register and get [`rxptpunicast::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxptpunicast::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxptpunicastSpec;
impl crate::RegisterSpec for RxptpunicastSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxptpunicast::R`](R) reader structure"]
impl crate::Readable for RxptpunicastSpec {}
#[doc = "`write(|w| ..)` method takes [`rxptpunicast::W`](W) writer structure"]
impl crate::Writable for RxptpunicastSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXPTPUNICAST to value 0"]
impl crate::Resettable for RxptpunicastSpec {}
