#[doc = "Register `ETMITCTRL` reader"]
pub type R = crate::R<ETMITCTRL_SPEC>;
#[doc = "Register `ETMITCTRL` writer"]
pub type W = crate::W<ETMITCTRL_SPEC>;
#[doc = "Field `ITEN` reader - Integration Mode Enable"]
pub type ITEN_R = crate::BitReader;
#[doc = "Field `ITEN` writer - Integration Mode Enable"]
pub type ITEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Integration Mode Enable"]
    #[inline(always)]
    pub fn iten(&self) -> ITEN_R {
        ITEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Integration Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn iten(&mut self) -> ITEN_W<ETMITCTRL_SPEC> {
        ITEN_W::new(self, 0)
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
#[doc = "ETM Integration Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmitctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etmitctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETMITCTRL_SPEC;
impl crate::RegisterSpec for ETMITCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmitctrl::R`](R) reader structure"]
impl crate::Readable for ETMITCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etmitctrl::W`](W) writer structure"]
impl crate::Writable for ETMITCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETMITCTRL to value 0"]
impl crate::Resettable for ETMITCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
