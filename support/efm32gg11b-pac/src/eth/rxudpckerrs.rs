#[doc = "Register `RXUDPCKERRS` reader"]
pub type R = crate::R<RxudpckerrsSpec>;
#[doc = "Register `RXUDPCKERRS` writer"]
pub type W = crate::W<RxudpckerrsSpec>;
#[doc = "Field `COUNT` reader - UDP checksum errors"]
pub type CountR = crate::FieldReader;
#[doc = "Field `COUNT` writer - UDP checksum errors"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - UDP checksum errors"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - UDP checksum errors"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, RxudpckerrsSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "UDP Checksum Errors\n\nYou can [`read`](crate::Reg::read) this register and get [`rxudpckerrs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxudpckerrs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxudpckerrsSpec;
impl crate::RegisterSpec for RxudpckerrsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxudpckerrs::R`](R) reader structure"]
impl crate::Readable for RxudpckerrsSpec {}
#[doc = "`write(|w| ..)` method takes [`rxudpckerrs::W`](W) writer structure"]
impl crate::Writable for RxudpckerrsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXUDPCKERRS to value 0"]
impl crate::Resettable for RxudpckerrsSpec {}
