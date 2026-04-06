#[doc = "Register `RXJABBERS` reader"]
pub type R = crate::R<RxjabbersSpec>;
#[doc = "Register `RXJABBERS` writer"]
pub type W = crate::W<RxjabbersSpec>;
#[doc = "Field `COUNT` reader - Jabbers received"]
pub type CountR = crate::FieldReader<u16>;
#[doc = "Field `COUNT` writer - Jabbers received"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Jabbers received"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Jabbers received"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, RxjabbersSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Jabbers Received\n\nYou can [`read`](crate::Reg::read) this register and get [`rxjabbers::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxjabbers::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxjabbersSpec;
impl crate::RegisterSpec for RxjabbersSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxjabbers::R`](R) reader structure"]
impl crate::Readable for RxjabbersSpec {}
#[doc = "`write(|w| ..)` method takes [`rxjabbers::W`](W) writer structure"]
impl crate::Writable for RxjabbersSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXJABBERS to value 0"]
impl crate::Resettable for RxjabbersSpec {}
