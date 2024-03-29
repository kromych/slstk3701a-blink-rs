#[doc = "Register `FRAMESTXED64` reader"]
pub type R = crate::R<FRAMESTXED64_SPEC>;
#[doc = "Register `FRAMESTXED64` writer"]
pub type W = crate::W<FRAMESTXED64_SPEC>;
#[doc = "Field `COUNT` reader - 64 byte frames transmitted without error"]
pub type COUNT_R = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - 64 byte frames transmitted without error"]
pub type COUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 64 byte frames transmitted without error"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 64 byte frames transmitted without error"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<FRAMESTXED64_SPEC> {
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
#[doc = "64 Byte Frames Transmitted\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`framestxed64::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`framestxed64::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRAMESTXED64_SPEC;
impl crate::RegisterSpec for FRAMESTXED64_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`framestxed64::R`](R) reader structure"]
impl crate::Readable for FRAMESTXED64_SPEC {}
#[doc = "`write(|w| ..)` method takes [`framestxed64::W`](W) writer structure"]
impl crate::Writable for FRAMESTXED64_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FRAMESTXED64 to value 0"]
impl crate::Resettable for FRAMESTXED64_SPEC {
    const RESET_VALUE: u32 = 0;
}
