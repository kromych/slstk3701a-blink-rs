#[doc = "Register `TXPTPUNICAST` reader"]
pub type R = crate::R<TxptpunicastSpec>;
#[doc = "Register `TXPTPUNICAST` writer"]
pub type W = crate::W<TxptpunicastSpec>;
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
    pub fn addr(&mut self) -> AddrW<'_, TxptpunicastSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "PTP TX unicast IP destination address\n\nYou can [`read`](crate::Reg::read) this register and get [`txptpunicast::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txptpunicast::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxptpunicastSpec;
impl crate::RegisterSpec for TxptpunicastSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txptpunicast::R`](R) reader structure"]
impl crate::Readable for TxptpunicastSpec {}
#[doc = "`write(|w| ..)` method takes [`txptpunicast::W`](W) writer structure"]
impl crate::Writable for TxptpunicastSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXPTPUNICAST to value 0"]
impl crate::Resettable for TxptpunicastSpec {}
