#[doc = "Register `CC2_CCV` reader"]
pub type R = crate::R<Cc2CcvSpec>;
#[doc = "Register `CC2_CCV` writer"]
pub type W = crate::W<Cc2CcvSpec>;
#[doc = "Field `CCV` reader - Capture/Compare Value"]
pub type CcvR = crate::FieldReader<u32>;
#[doc = "Field `CCV` writer - Capture/Compare Value"]
pub type CcvW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Capture/Compare Value"]
    #[inline(always)]
    pub fn ccv(&self) -> CcvR {
        CcvR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Capture/Compare Value"]
    #[inline(always)]
    pub fn ccv(&mut self) -> CcvW<'_, Cc2CcvSpec> {
        CcvW::new(self, 0)
    }
}
#[doc = "Capture/Compare Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc2_ccv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc2_ccv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cc2CcvSpec;
impl crate::RegisterSpec for Cc2CcvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc2_ccv::R`](R) reader structure"]
impl crate::Readable for Cc2CcvSpec {}
#[doc = "`write(|w| ..)` method takes [`cc2_ccv::W`](W) writer structure"]
impl crate::Writable for Cc2CcvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CC2_CCV to value 0"]
impl crate::Resettable for Cc2CcvSpec {}
