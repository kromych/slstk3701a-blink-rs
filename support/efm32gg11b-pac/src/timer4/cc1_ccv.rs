#[doc = "Register `CC1_CCV` reader"]
pub type R = crate::R<CC1_CCV_SPEC>;
#[doc = "Register `CC1_CCV` writer"]
pub type W = crate::W<CC1_CCV_SPEC>;
#[doc = "Field `CCV` reader - CC Channel Value"]
pub type CCV_R = crate::FieldReader<u32>;
#[doc = "Field `CCV` writer - CC Channel Value"]
pub type CCV_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CC Channel Value"]
    #[inline(always)]
    pub fn ccv(&self) -> CCV_R {
        CCV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CC Channel Value"]
    #[inline(always)]
    #[must_use]
    pub fn ccv(&mut self) -> CCV_W<CC1_CCV_SPEC> {
        CCV_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "CC Channel Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc1_ccv::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc1_ccv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CC1_CCV_SPEC;
impl crate::RegisterSpec for CC1_CCV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc1_ccv::R`](R) reader structure"]
impl crate::Readable for CC1_CCV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cc1_ccv::W`](W) writer structure"]
impl crate::Writable for CC1_CCV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CC1_CCV to value 0"]
impl crate::Resettable for CC1_CCV_SPEC {
    const RESET_VALUE: u32 = 0;
}
