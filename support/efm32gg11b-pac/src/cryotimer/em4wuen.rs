#[doc = "Register `EM4WUEN` reader"]
pub type R = crate::R<EM4WUEN_SPEC>;
#[doc = "Register `EM4WUEN` writer"]
pub type W = crate::W<EM4WUEN_SPEC>;
#[doc = "Field `EM4WU` reader - EM4 Wake-up Enable"]
pub type EM4WU_R = crate::BitReader;
#[doc = "Field `EM4WU` writer - EM4 Wake-up Enable"]
pub type EM4WU_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EM4 Wake-up Enable"]
    #[inline(always)]
    pub fn em4wu(&self) -> EM4WU_R {
        EM4WU_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EM4 Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn em4wu(&mut self) -> EM4WU_W<EM4WUEN_SPEC> {
        EM4WU_W::new(self, 0)
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
#[doc = "Wake Up Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`em4wuen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`em4wuen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EM4WUEN_SPEC;
impl crate::RegisterSpec for EM4WUEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`em4wuen::R`](R) reader structure"]
impl crate::Readable for EM4WUEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`em4wuen::W`](W) writer structure"]
impl crate::Writable for EM4WUEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EM4WUEN to value 0"]
impl crate::Resettable for EM4WUEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
