#[doc = "Register `RXSYMBOLERRS` reader"]
pub type R = crate::R<RxsymbolerrsSpec>;
#[doc = "Register `RXSYMBOLERRS` writer"]
pub type W = crate::W<RxsymbolerrsSpec>;
#[doc = "Field `COUNT` reader - Receive symbol errors"]
pub type CountR = crate::FieldReader<u16>;
#[doc = "Field `COUNT` writer - Receive symbol errors"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Receive symbol errors"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Receive symbol errors"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, RxsymbolerrsSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Receive Symbol Errors\n\nYou can [`read`](crate::Reg::read) this register and get [`rxsymbolerrs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxsymbolerrs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxsymbolerrsSpec;
impl crate::RegisterSpec for RxsymbolerrsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxsymbolerrs::R`](R) reader structure"]
impl crate::Readable for RxsymbolerrsSpec {}
#[doc = "`write(|w| ..)` method takes [`rxsymbolerrs::W`](W) writer structure"]
impl crate::Writable for RxsymbolerrsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXSYMBOLERRS to value 0"]
impl crate::Resettable for RxsymbolerrsSpec {}
