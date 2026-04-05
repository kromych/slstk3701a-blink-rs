#[doc = "Register `ETMFFLR` reader"]
pub type R = crate::R<EtmfflrSpec>;
#[doc = "Register `ETMFFLR` writer"]
pub type W = crate::W<EtmfflrSpec>;
#[doc = "Field `BYTENUM` reader - Bytes left in FIFO"]
pub type BytenumR = crate::FieldReader;
#[doc = "Field `BYTENUM` writer - Bytes left in FIFO"]
pub type BytenumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Bytes left in FIFO"]
    #[inline(always)]
    pub fn bytenum(&self) -> BytenumR {
        BytenumR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bytes left in FIFO"]
    #[inline(always)]
    pub fn bytenum(&mut self) -> BytenumW<'_, EtmfflrSpec> {
        BytenumW::new(self, 0)
    }
}
#[doc = "ETM Fifo Full Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmfflr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etmfflr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtmfflrSpec;
impl crate::RegisterSpec for EtmfflrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmfflr::R`](R) reader structure"]
impl crate::Readable for EtmfflrSpec {}
#[doc = "`write(|w| ..)` method takes [`etmfflr::W`](W) writer structure"]
impl crate::Writable for EtmfflrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETMFFLR to value 0"]
impl crate::Resettable for EtmfflrSpec {}
