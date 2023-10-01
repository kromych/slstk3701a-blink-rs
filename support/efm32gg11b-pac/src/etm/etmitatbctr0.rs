#[doc = "Register `ETMITATBCTR0` reader"]
pub type R = crate::R<ETMITATBCTR0_SPEC>;
#[doc = "Register `ETMITATBCTR0` writer"]
pub type W = crate::W<ETMITATBCTR0_SPEC>;
#[doc = "Field `ATVALID` reader - ATVALID Output Value"]
pub type ATVALID_R = crate::BitReader;
#[doc = "Field `ATVALID` writer - ATVALID Output Value"]
pub type ATVALID_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - ATVALID Output Value"]
    #[inline(always)]
    pub fn atvalid(&self) -> ATVALID_R {
        ATVALID_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ATVALID Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn atvalid(&mut self) -> ATVALID_W<ETMITATBCTR0_SPEC, 0> {
        ATVALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ETM Integration Test ATB Control 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmitatbctr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etmitatbctr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETMITATBCTR0_SPEC;
impl crate::RegisterSpec for ETMITATBCTR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmitatbctr0::R`](R) reader structure"]
impl crate::Readable for ETMITATBCTR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etmitatbctr0::W`](W) writer structure"]
impl crate::Writable for ETMITATBCTR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETMITATBCTR0 to value 0"]
impl crate::Resettable for ETMITATBCTR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
