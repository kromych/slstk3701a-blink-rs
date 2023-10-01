#[doc = "Register `ETMCLAIMSET` reader"]
pub type R = crate::R<ETMCLAIMSET_SPEC>;
#[doc = "Register `ETMCLAIMSET` writer"]
pub type W = crate::W<ETMCLAIMSET_SPEC>;
#[doc = "Field `SETTAG` reader - Tag Bits"]
pub type SETTAG_R = crate::FieldReader;
#[doc = "Field `SETTAG` writer - Tag Bits"]
pub type SETTAG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Tag Bits"]
    #[inline(always)]
    pub fn settag(&self) -> SETTAG_R {
        SETTAG_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Tag Bits"]
    #[inline(always)]
    #[must_use]
    pub fn settag(&mut self) -> SETTAG_W<ETMCLAIMSET_SPEC, 0> {
        SETTAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ETM Claim Tag Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmclaimset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etmclaimset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETMCLAIMSET_SPEC;
impl crate::RegisterSpec for ETMCLAIMSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmclaimset::R`](R) reader structure"]
impl crate::Readable for ETMCLAIMSET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etmclaimset::W`](W) writer structure"]
impl crate::Writable for ETMCLAIMSET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETMCLAIMSET to value 0x0f"]
impl crate::Resettable for ETMCLAIMSET_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}
