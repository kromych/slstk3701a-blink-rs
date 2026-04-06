#[doc = "Register `CC3_CCV` reader"]
pub type R = crate::R<Cc3CcvSpec>;
#[doc = "Register `CC3_CCV` writer"]
pub type W = crate::W<Cc3CcvSpec>;
#[doc = "Field `CCV` reader - CC Channel Value"]
pub type CcvR = crate::FieldReader<u32>;
#[doc = "Field `CCV` writer - CC Channel Value"]
pub type CcvW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CC Channel Value"]
    #[inline(always)]
    pub fn ccv(&self) -> CcvR {
        CcvR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CC Channel Value"]
    #[inline(always)]
    pub fn ccv(&mut self) -> CcvW<'_, Cc3CcvSpec> {
        CcvW::new(self, 0)
    }
}
#[doc = "CC Channel Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc3_ccv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc3_ccv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct Cc3CcvSpec;
impl crate::RegisterSpec for Cc3CcvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc3_ccv::R`](R) reader structure"]
impl crate::Readable for Cc3CcvSpec {}
#[doc = "`write(|w| ..)` method takes [`cc3_ccv::W`](W) writer structure"]
impl crate::Writable for Cc3CcvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CC3_CCV to value 0"]
impl crate::Resettable for Cc3CcvSpec {}
