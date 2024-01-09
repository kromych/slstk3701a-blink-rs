#[doc = "Register `LFBCLKEN0` reader"]
pub type R = crate::R<LFBCLKEN0_SPEC>;
#[doc = "Register `LFBCLKEN0` writer"]
pub type W = crate::W<LFBCLKEN0_SPEC>;
#[doc = "Field `LEUART0` reader - Low Energy UART 0 Clock Enable"]
pub type LEUART0_R = crate::BitReader;
#[doc = "Field `LEUART0` writer - Low Energy UART 0 Clock Enable"]
pub type LEUART0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEUART1` reader - Low Energy UART 1 Clock Enable"]
pub type LEUART1_R = crate::BitReader;
#[doc = "Field `LEUART1` writer - Low Energy UART 1 Clock Enable"]
pub type LEUART1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSTICK` reader - Clock Enable"]
pub type SYSTICK_R = crate::BitReader;
#[doc = "Field `SYSTICK` writer - Clock Enable"]
pub type SYSTICK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSEN` reader - Capacitive touch sense module Clock Enable"]
pub type CSEN_R = crate::BitReader;
#[doc = "Field `CSEN` writer - Capacitive touch sense module Clock Enable"]
pub type CSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Low Energy UART 0 Clock Enable"]
    #[inline(always)]
    pub fn leuart0(&self) -> LEUART0_R {
        LEUART0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low Energy UART 1 Clock Enable"]
    #[inline(always)]
    pub fn leuart1(&self) -> LEUART1_R {
        LEUART1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clock Enable"]
    #[inline(always)]
    pub fn systick(&self) -> SYSTICK_R {
        SYSTICK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capacitive touch sense module Clock Enable"]
    #[inline(always)]
    pub fn csen(&self) -> CSEN_R {
        CSEN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low Energy UART 0 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn leuart0(&mut self) -> LEUART0_W<LFBCLKEN0_SPEC> {
        LEUART0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Low Energy UART 1 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn leuart1(&mut self) -> LEUART1_W<LFBCLKEN0_SPEC> {
        LEUART1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn systick(&mut self) -> SYSTICK_W<LFBCLKEN0_SPEC> {
        SYSTICK_W::new(self, 2)
    }
    #[doc = "Bit 3 - Capacitive touch sense module Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn csen(&mut self) -> CSEN_W<LFBCLKEN0_SPEC> {
        CSEN_W::new(self, 3)
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
#[doc = "Low Frequency B Clock Enable Register 0 (Async Reg)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfbclken0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfbclken0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LFBCLKEN0_SPEC;
impl crate::RegisterSpec for LFBCLKEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfbclken0::R`](R) reader structure"]
impl crate::Readable for LFBCLKEN0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lfbclken0::W`](W) writer structure"]
impl crate::Writable for LFBCLKEN0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LFBCLKEN0 to value 0"]
impl crate::Resettable for LFBCLKEN0_SPEC {
    const RESET_VALUE: u32 = 0;
}
