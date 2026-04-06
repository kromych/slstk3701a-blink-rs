#[doc = "Register `RXRESOURCEERRS` reader"]
pub type R = crate::R<RxresourceerrsSpec>;
#[doc = "Register `RXRESOURCEERRS` writer"]
pub type W = crate::W<RxresourceerrsSpec>;
#[doc = "Field `COUNT` reader - Receive resource errors"]
pub type CountR = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - Receive resource errors"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - Receive resource errors"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - Receive resource errors"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, RxresourceerrsSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Receive Resource Errors\n\nYou can [`read`](crate::Reg::read) this register and get [`rxresourceerrs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxresourceerrs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxresourceerrsSpec;
impl crate::RegisterSpec for RxresourceerrsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxresourceerrs::R`](R) reader structure"]
impl crate::Readable for RxresourceerrsSpec {}
#[doc = "`write(|w| ..)` method takes [`rxresourceerrs::W`](W) writer structure"]
impl crate::Writable for RxresourceerrsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXRESOURCEERRS to value 0"]
impl crate::Resettable for RxresourceerrsSpec {}
