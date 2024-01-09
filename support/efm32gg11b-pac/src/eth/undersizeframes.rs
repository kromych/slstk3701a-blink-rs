#[doc = "Register `UNDERSIZEFRAMES` reader"]
pub type R = crate::R<UNDERSIZEFRAMES_SPEC>;
#[doc = "Register `UNDERSIZEFRAMES` writer"]
pub type W = crate::W<UNDERSIZEFRAMES_SPEC>;
#[doc = "Field `COUNT` reader - Undersize frames received"]
pub type COUNT_R = crate::FieldReader<u16>;
#[doc = "Field `COUNT` writer - Undersize frames received"]
pub type COUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Undersize frames received"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Undersize frames received"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<UNDERSIZEFRAMES_SPEC> {
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
#[doc = "Undersized Frames Received\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`undersizeframes::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`undersizeframes::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UNDERSIZEFRAMES_SPEC;
impl crate::RegisterSpec for UNDERSIZEFRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`undersizeframes::R`](R) reader structure"]
impl crate::Readable for UNDERSIZEFRAMES_SPEC {}
#[doc = "`write(|w| ..)` method takes [`undersizeframes::W`](W) writer structure"]
impl crate::Writable for UNDERSIZEFRAMES_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UNDERSIZEFRAMES to value 0"]
impl crate::Resettable for UNDERSIZEFRAMES_SPEC {
    const RESET_VALUE: u32 = 0;
}
