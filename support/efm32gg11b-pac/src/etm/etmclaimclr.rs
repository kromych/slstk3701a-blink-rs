#[doc = "Register `ETMCLAIMCLR` reader"]
pub type R = crate::R<ETMCLAIMCLR_SPEC>;
#[doc = "Register `ETMCLAIMCLR` writer"]
pub type W = crate::W<ETMCLAIMCLR_SPEC>;
#[doc = "Field `CLRTAG` reader - Tag Bits"]
pub type CLRTAG_R = crate::BitReader;
#[doc = "Field `CLRTAG` writer - Tag Bits"]
pub type CLRTAG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Tag Bits"]
    #[inline(always)]
    pub fn clrtag(&self) -> CLRTAG_R {
        CLRTAG_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tag Bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrtag(&mut self) -> CLRTAG_W<ETMCLAIMCLR_SPEC> {
        CLRTAG_W::new(self, 0)
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
#[doc = "ETM Claim Tag Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmclaimclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etmclaimclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETMCLAIMCLR_SPEC;
impl crate::RegisterSpec for ETMCLAIMCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmclaimclr::R`](R) reader structure"]
impl crate::Readable for ETMCLAIMCLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etmclaimclr::W`](W) writer structure"]
impl crate::Writable for ETMCLAIMCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETMCLAIMCLR to value 0"]
impl crate::Resettable for ETMCLAIMCLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
