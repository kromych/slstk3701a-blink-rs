#[doc = "Register `POWERDOWN` reader"]
pub type R = crate::R<POWERDOWN_SPEC>;
#[doc = "Register `POWERDOWN` writer"]
pub type W = crate::W<POWERDOWN_SPEC>;
#[doc = "Field `RAM` reader - Retention RAM Power-down"]
pub type RAM_R = crate::BitReader;
#[doc = "Field `RAM` writer - Retention RAM Power-down"]
pub type RAM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Retention RAM Power-down"]
    #[inline(always)]
    pub fn ram(&self) -> RAM_R {
        RAM_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Retention RAM Power-down"]
    #[inline(always)]
    #[must_use]
    pub fn ram(&mut self) -> RAM_W<POWERDOWN_SPEC> {
        RAM_W::new(self, 0)
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
#[doc = "Retention RAM Power-down Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`powerdown::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`powerdown::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POWERDOWN_SPEC;
impl crate::RegisterSpec for POWERDOWN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`powerdown::R`](R) reader structure"]
impl crate::Readable for POWERDOWN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`powerdown::W`](W) writer structure"]
impl crate::Writable for POWERDOWN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POWERDOWN to value 0"]
impl crate::Resettable for POWERDOWN_SPEC {
    const RESET_VALUE: u32 = 0;
}
