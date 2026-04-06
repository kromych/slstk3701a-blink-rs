#[doc = "Register `RXLENERRS` reader"]
pub type R = crate::R<RxlenerrsSpec>;
#[doc = "Register `RXLENERRS` writer"]
pub type W = crate::W<RxlenerrsSpec>;
#[doc = "Field `COUNT` reader - Length field frame errors"]
pub type CountR = crate::FieldReader<u16>;
#[doc = "Field `COUNT` writer - Length field frame errors"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Length field frame errors"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Length field frame errors"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, RxlenerrsSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Length Field Frame Errors\n\nYou can [`read`](crate::Reg::read) this register and get [`rxlenerrs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxlenerrs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxlenerrsSpec;
impl crate::RegisterSpec for RxlenerrsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxlenerrs::R`](R) reader structure"]
impl crate::Readable for RxlenerrsSpec {}
#[doc = "`write(|w| ..)` method takes [`rxlenerrs::W`](W) writer structure"]
impl crate::Writable for RxlenerrsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXLENERRS to value 0"]
impl crate::Resettable for RxlenerrsSpec {}
