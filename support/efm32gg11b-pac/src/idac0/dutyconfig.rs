#[doc = "Register `DUTYCONFIG` reader"]
pub type R = crate::R<DUTYCONFIG_SPEC>;
#[doc = "Register `DUTYCONFIG` writer"]
pub type W = crate::W<DUTYCONFIG_SPEC>;
#[doc = "Field `EM2DUTYCYCLEDIS` reader - Duty Cycle Enable"]
pub type EM2DUTYCYCLEDIS_R = crate::BitReader;
#[doc = "Field `EM2DUTYCYCLEDIS` writer - Duty Cycle Enable"]
pub type EM2DUTYCYCLEDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Duty Cycle Enable"]
    #[inline(always)]
    pub fn em2dutycycledis(&self) -> EM2DUTYCYCLEDIS_R {
        EM2DUTYCYCLEDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Duty Cycle Enable"]
    #[inline(always)]
    #[must_use]
    pub fn em2dutycycledis(&mut self) -> EM2DUTYCYCLEDIS_W<DUTYCONFIG_SPEC> {
        EM2DUTYCYCLEDIS_W::new(self, 1)
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
#[doc = "Duty Cycle Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dutyconfig::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dutyconfig::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DUTYCONFIG_SPEC;
impl crate::RegisterSpec for DUTYCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dutyconfig::R`](R) reader structure"]
impl crate::Readable for DUTYCONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dutyconfig::W`](W) writer structure"]
impl crate::Writable for DUTYCONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DUTYCONFIG to value 0"]
impl crate::Resettable for DUTYCONFIG_SPEC {
    const RESET_VALUE: u32 = 0;
}
