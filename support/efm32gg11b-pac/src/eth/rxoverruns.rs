#[doc = "Register `RXOVERRUNS` reader"]
pub type R = crate::R<RxoverrunsSpec>;
#[doc = "Register `RXOVERRUNS` writer"]
pub type W = crate::W<RxoverrunsSpec>;
#[doc = "Field `COUNT` reader - Receive overruns"]
pub type CountR = crate::FieldReader<u16>;
#[doc = "Field `COUNT` writer - Receive overruns"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Receive overruns"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Receive overruns"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, RxoverrunsSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Receive Overruns\n\nYou can [`read`](crate::Reg::read) this register and get [`rxoverruns::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxoverruns::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxoverrunsSpec;
impl crate::RegisterSpec for RxoverrunsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxoverruns::R`](R) reader structure"]
impl crate::Readable for RxoverrunsSpec {}
#[doc = "`write(|w| ..)` method takes [`rxoverruns::W`](W) writer structure"]
impl crate::Writable for RxoverrunsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXOVERRUNS to value 0"]
impl crate::Resettable for RxoverrunsSpec {}
