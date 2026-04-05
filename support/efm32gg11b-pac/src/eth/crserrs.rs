#[doc = "Register `CRSERRS` reader"]
pub type R = crate::R<CrserrsSpec>;
#[doc = "Register `CRSERRS` writer"]
pub type W = crate::W<CrserrsSpec>;
#[doc = "Field `COUNT` reader - Carrier sense errors"]
pub type CountR = crate::FieldReader<u16>;
#[doc = "Field `COUNT` writer - Carrier sense errors"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Carrier sense errors"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Carrier sense errors"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, CrserrsSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Carrier Sense Errors\n\nYou can [`read`](crate::Reg::read) this register and get [`crserrs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crserrs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrserrsSpec;
impl crate::RegisterSpec for CrserrsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crserrs::R`](R) reader structure"]
impl crate::Readable for CrserrsSpec {}
#[doc = "`write(|w| ..)` method takes [`crserrs::W`](W) writer structure"]
impl crate::Writable for CrserrsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRSERRS to value 0"]
impl crate::Resettable for CrserrsSpec {}
