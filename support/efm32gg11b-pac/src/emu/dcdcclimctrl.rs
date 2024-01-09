#[doc = "Register `DCDCCLIMCTRL` reader"]
pub type R = crate::R<DCDCCLIMCTRL_SPEC>;
#[doc = "Register `DCDCCLIMCTRL` writer"]
pub type W = crate::W<DCDCCLIMCTRL_SPEC>;
#[doc = "Field `CLIMBLANKDLY` reader - Reserved for internal use. Do not change."]
pub type CLIMBLANKDLY_R = crate::FieldReader;
#[doc = "Field `CLIMBLANKDLY` writer - Reserved for internal use. Do not change."]
pub type CLIMBLANKDLY_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BYPLIMEN` reader - Bypass Current Limit Enable"]
pub type BYPLIMEN_R = crate::BitReader;
#[doc = "Field `BYPLIMEN` writer - Bypass Current Limit Enable"]
pub type BYPLIMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 8:9 - Reserved for internal use. Do not change."]
    #[inline(always)]
    pub fn climblankdly(&self) -> CLIMBLANKDLY_R {
        CLIMBLANKDLY_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 13 - Bypass Current Limit Enable"]
    #[inline(always)]
    pub fn byplimen(&self) -> BYPLIMEN_R {
        BYPLIMEN_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:9 - Reserved for internal use. Do not change."]
    #[inline(always)]
    #[must_use]
    pub fn climblankdly(&mut self) -> CLIMBLANKDLY_W<DCDCCLIMCTRL_SPEC> {
        CLIMBLANKDLY_W::new(self, 8)
    }
    #[doc = "Bit 13 - Bypass Current Limit Enable"]
    #[inline(always)]
    #[must_use]
    pub fn byplimen(&mut self) -> BYPLIMEN_W<DCDCCLIMCTRL_SPEC> {
        BYPLIMEN_W::new(self, 13)
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
#[doc = "DCDC Power Train PFET Current Limiter Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdcclimctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdcclimctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCDCCLIMCTRL_SPEC;
impl crate::RegisterSpec for DCDCCLIMCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdcclimctrl::R`](R) reader structure"]
impl crate::Readable for DCDCCLIMCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcdcclimctrl::W`](W) writer structure"]
impl crate::Writable for DCDCCLIMCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCDCCLIMCTRL to value 0x0100"]
impl crate::Resettable for DCDCCLIMCTRL_SPEC {
    const RESET_VALUE: u32 = 0x0100;
}
