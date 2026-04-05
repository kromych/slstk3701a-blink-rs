#[doc = "Register `FCSERRS` reader"]
pub type R = crate::R<FcserrsSpec>;
#[doc = "Register `FCSERRS` writer"]
pub type W = crate::W<FcserrsSpec>;
#[doc = "Field `COUNT` reader - Frame check sequence errors"]
pub type CountR = crate::FieldReader<u16>;
#[doc = "Field `COUNT` writer - Frame check sequence errors"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Frame check sequence errors"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Frame check sequence errors"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, FcserrsSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Frame Check Sequence Errors\n\nYou can [`read`](crate::Reg::read) this register and get [`fcserrs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcserrs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcserrsSpec;
impl crate::RegisterSpec for FcserrsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcserrs::R`](R) reader structure"]
impl crate::Readable for FcserrsSpec {}
#[doc = "`write(|w| ..)` method takes [`fcserrs::W`](W) writer structure"]
impl crate::Writable for FcserrsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCSERRS to value 0"]
impl crate::Resettable for FcserrsSpec {}
