#[doc = "Register `RXIPCKERRS` reader"]
pub type R = crate::R<RxipckerrsSpec>;
#[doc = "Register `RXIPCKERRS` writer"]
pub type W = crate::W<RxipckerrsSpec>;
#[doc = "Field `COUNT` reader - IP header checksum errors"]
pub type CountR = crate::FieldReader;
#[doc = "Field `COUNT` writer - IP header checksum errors"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - IP header checksum errors"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - IP header checksum errors"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, RxipckerrsSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "IP Header Checksum Errors\n\nYou can [`read`](crate::Reg::read) this register and get [`rxipckerrs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxipckerrs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxipckerrsSpec;
impl crate::RegisterSpec for RxipckerrsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxipckerrs::R`](R) reader structure"]
impl crate::Readable for RxipckerrsSpec {}
#[doc = "`write(|w| ..)` method takes [`rxipckerrs::W`](W) writer structure"]
impl crate::Writable for RxipckerrsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXIPCKERRS to value 0"]
impl crate::Resettable for RxipckerrsSpec {}
