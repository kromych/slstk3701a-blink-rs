#[doc = "Register `ADSADDR` reader"]
pub type R = crate::R<AdsaddrSpec>;
#[doc = "Register `ADSADDR` writer"]
pub type W = crate::W<AdsaddrSpec>;
#[doc = "Field `ADSADDR` reader - ADMA System Address"]
pub type AdsaddrR = crate::FieldReader<u32>;
#[doc = "Field `ADSADDR` writer - ADMA System Address"]
pub type AdsaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ADMA System Address"]
    #[inline(always)]
    pub fn adsaddr(&self) -> AdsaddrR {
        AdsaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ADMA System Address"]
    #[inline(always)]
    pub fn adsaddr(&mut self) -> AdsaddrW<'_, AdsaddrSpec> {
        AdsaddrW::new(self, 0)
    }
}
#[doc = "ADMA System Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adsaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adsaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdsaddrSpec;
impl crate::RegisterSpec for AdsaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adsaddr::R`](R) reader structure"]
impl crate::Readable for AdsaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`adsaddr::W`](W) writer structure"]
impl crate::Writable for AdsaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADSADDR to value 0"]
impl crate::Resettable for AdsaddrSpec {}
