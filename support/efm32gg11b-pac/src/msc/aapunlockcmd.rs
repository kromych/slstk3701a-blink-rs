#[doc = "Register `AAPUNLOCKCMD` writer"]
pub type W = crate::W<AAPUNLOCKCMD_SPEC>;
#[doc = "Field `UNLOCKAAP` writer - Software Unlock AAP Command"]
pub type UNLOCKAAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Software Unlock AAP Command"]
    #[inline(always)]
    #[must_use]
    pub fn unlockaap(&mut self) -> UNLOCKAAP_W<AAPUNLOCKCMD_SPEC, 0> {
        UNLOCKAAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Software Unlock AAP Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aapunlockcmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AAPUNLOCKCMD_SPEC;
impl crate::RegisterSpec for AAPUNLOCKCMD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`aapunlockcmd::W`](W) writer structure"]
impl crate::Writable for AAPUNLOCKCMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AAPUNLOCKCMD to value 0"]
impl crate::Resettable for AAPUNLOCKCMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
