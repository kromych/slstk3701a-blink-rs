#[doc = "Register `EVALCTRL` reader"]
pub type R = crate::R<EVALCTRL_SPEC>;
#[doc = "Register `EVALCTRL` writer"]
pub type W = crate::W<EVALCTRL_SPEC>;
#[doc = "Field `WINSIZE` reader - Sliding Window and Step Detection Size"]
pub type WINSIZE_R = crate::FieldReader<u16>;
#[doc = "Field `WINSIZE` writer - Sliding Window and Step Detection Size"]
pub type WINSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Sliding Window and Step Detection Size"]
    #[inline(always)]
    pub fn winsize(&self) -> WINSIZE_R {
        WINSIZE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Sliding Window and Step Detection Size"]
    #[inline(always)]
    #[must_use]
    pub fn winsize(&mut self) -> WINSIZE_W<EVALCTRL_SPEC> {
        WINSIZE_W::new(self, 0)
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
#[doc = "LESENSE Evaluation Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evalctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evalctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVALCTRL_SPEC;
impl crate::RegisterSpec for EVALCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evalctrl::R`](R) reader structure"]
impl crate::Readable for EVALCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`evalctrl::W`](W) writer structure"]
impl crate::Writable for EVALCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVALCTRL to value 0"]
impl crate::Resettable for EVALCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
