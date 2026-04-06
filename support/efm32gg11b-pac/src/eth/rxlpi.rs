#[doc = "Register `RXLPI` reader"]
pub type R = crate::R<RxlpiSpec>;
#[doc = "Register `RXLPI` writer"]
pub type W = crate::W<RxlpiSpec>;
#[doc = "Field `COUNT` reader - Count of RX LPI transitions"]
pub type CountR = crate::FieldReader<u16>;
#[doc = "Field `COUNT` writer - Count of RX LPI transitions"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Count of RX LPI transitions"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Count of RX LPI transitions"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, RxlpiSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Received LPI transitions\n\nYou can [`read`](crate::Reg::read) this register and get [`rxlpi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxlpi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxlpiSpec;
impl crate::RegisterSpec for RxlpiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxlpi::R`](R) reader structure"]
impl crate::Readable for RxlpiSpec {}
#[doc = "`write(|w| ..)` method takes [`rxlpi::W`](W) writer structure"]
impl crate::Writable for RxlpiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXLPI to value 0"]
impl crate::Resettable for RxlpiSpec {}
