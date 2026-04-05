#[doc = "Register `CC3_CCVB` reader"]
pub type R = crate::R<Cc3CcvbSpec>;
#[doc = "Register `CC3_CCVB` writer"]
pub type W = crate::W<Cc3CcvbSpec>;
#[doc = "Field `CCVB` reader - CC Channel Value Buffer"]
pub type CcvbR = crate::FieldReader<u32>;
#[doc = "Field `CCVB` writer - CC Channel Value Buffer"]
pub type CcvbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CC Channel Value Buffer"]
    #[inline(always)]
    pub fn ccvb(&self) -> CcvbR {
        CcvbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CC Channel Value Buffer"]
    #[inline(always)]
    pub fn ccvb(&mut self) -> CcvbW<'_, Cc3CcvbSpec> {
        CcvbW::new(self, 0)
    }
}
#[doc = "CC Channel Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc3_ccvb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc3_ccvb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cc3CcvbSpec;
impl crate::RegisterSpec for Cc3CcvbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc3_ccvb::R`](R) reader structure"]
impl crate::Readable for Cc3CcvbSpec {}
#[doc = "`write(|w| ..)` method takes [`cc3_ccvb::W`](W) writer structure"]
impl crate::Writable for Cc3CcvbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CC3_CCVB to value 0"]
impl crate::Resettable for Cc3CcvbSpec {}
