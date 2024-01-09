#[doc = "Register `FRAMESTXEDOK` reader"]
pub type R = crate::R<FRAMESTXEDOK_SPEC>;
#[doc = "Register `FRAMESTXEDOK` writer"]
pub type W = crate::W<FRAMESTXEDOK_SPEC>;
#[doc = "Field `COUNT` reader - Frames transmitted without error"]
pub type COUNT_R = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - Frames transmitted without error"]
pub type COUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Frames transmitted without error"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Frames transmitted without error"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<FRAMESTXEDOK_SPEC> {
        COUNT_W::new(self, 0)
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
#[doc = "Frames Transmitted\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`framestxedok::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`framestxedok::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRAMESTXEDOK_SPEC;
impl crate::RegisterSpec for FRAMESTXEDOK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`framestxedok::R`](R) reader structure"]
impl crate::Readable for FRAMESTXEDOK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`framestxedok::W`](W) writer structure"]
impl crate::Writable for FRAMESTXEDOK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FRAMESTXEDOK to value 0"]
impl crate::Resettable for FRAMESTXEDOK_SPEC {
    const RESET_VALUE: u32 = 0;
}
