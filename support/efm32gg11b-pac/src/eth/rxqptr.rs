#[doc = "Register `RXQPTR` reader"]
pub type R = crate::R<RxqptrSpec>;
#[doc = "Register `RXQPTR` writer"]
pub type W = crate::W<RxqptrSpec>;
#[doc = "Field `DMARXQPTR` reader - Receive buffer queue base address"]
pub type DmarxqptrR = crate::FieldReader<u32>;
#[doc = "Field `DMARXQPTR` writer - Receive buffer queue base address"]
pub type DmarxqptrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Receive buffer queue base address"]
    #[inline(always)]
    pub fn dmarxqptr(&self) -> DmarxqptrR {
        DmarxqptrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Receive buffer queue base address"]
    #[inline(always)]
    pub fn dmarxqptr(&mut self) -> DmarxqptrW<'_, RxqptrSpec> {
        DmarxqptrW::new(self, 2)
    }
}
#[doc = "Start address of the receive buffer queue\n\nYou can [`read`](crate::Reg::read) this register and get [`rxqptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxqptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxqptrSpec;
impl crate::RegisterSpec for RxqptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxqptr::R`](R) reader structure"]
impl crate::Readable for RxqptrSpec {}
#[doc = "`write(|w| ..)` method takes [`rxqptr::W`](W) writer structure"]
impl crate::Writable for RxqptrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXQPTR to value 0"]
impl crate::Resettable for RxqptrSpec {}
