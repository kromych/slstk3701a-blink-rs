#[doc = "Register `LFBCLKSEL` reader"]
pub type R = crate::R<LFBCLKSEL_SPEC>;
#[doc = "Register `LFBCLKSEL` writer"]
pub type W = crate::W<LFBCLKSEL_SPEC>;
#[doc = "Field `LFB` reader - Clock Select for LFB"]
pub type LFB_R = crate::FieldReader<LFB_A>;
#[doc = "Clock Select for LFB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LFB_A {
    #[doc = "0: LFBCLK is disabled"]
    DISABLED = 0,
    #[doc = "1: LFRCO selected as LFBCLK"]
    LFRCO = 1,
    #[doc = "2: LFXO selected as LFBCLK"]
    LFXO = 2,
    #[doc = "3: HFCLK divided by two/four is selected as LFBCLK"]
    HFCLKLE = 3,
    #[doc = "4: ULFRCO selected as LFBCLK"]
    ULFRCO = 4,
}
impl From<LFB_A> for u8 {
    #[inline(always)]
    fn from(variant: LFB_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LFB_A {
    type Ux = u8;
}
impl LFB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LFB_A> {
        match self.bits {
            0 => Some(LFB_A::DISABLED),
            1 => Some(LFB_A::LFRCO),
            2 => Some(LFB_A::LFXO),
            3 => Some(LFB_A::HFCLKLE),
            4 => Some(LFB_A::ULFRCO),
            _ => None,
        }
    }
    #[doc = "LFBCLK is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LFB_A::DISABLED
    }
    #[doc = "LFRCO selected as LFBCLK"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == LFB_A::LFRCO
    }
    #[doc = "LFXO selected as LFBCLK"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == LFB_A::LFXO
    }
    #[doc = "HFCLK divided by two/four is selected as LFBCLK"]
    #[inline(always)]
    pub fn is_hfclkle(&self) -> bool {
        *self == LFB_A::HFCLKLE
    }
    #[doc = "ULFRCO selected as LFBCLK"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == LFB_A::ULFRCO
    }
}
#[doc = "Field `LFB` writer - Clock Select for LFB"]
pub type LFB_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LFB_A>;
impl<'a, REG> LFB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LFBCLK is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LFB_A::DISABLED)
    }
    #[doc = "LFRCO selected as LFBCLK"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(LFB_A::LFRCO)
    }
    #[doc = "LFXO selected as LFBCLK"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(LFB_A::LFXO)
    }
    #[doc = "HFCLK divided by two/four is selected as LFBCLK"]
    #[inline(always)]
    pub fn hfclkle(self) -> &'a mut crate::W<REG> {
        self.variant(LFB_A::HFCLKLE)
    }
    #[doc = "ULFRCO selected as LFBCLK"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut crate::W<REG> {
        self.variant(LFB_A::ULFRCO)
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Select for LFB"]
    #[inline(always)]
    pub fn lfb(&self) -> LFB_R {
        LFB_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Select for LFB"]
    #[inline(always)]
    #[must_use]
    pub fn lfb(&mut self) -> LFB_W<LFBCLKSEL_SPEC> {
        LFB_W::new(self, 0)
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
#[doc = "Low Frequency B Clock Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfbclksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfbclksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LFBCLKSEL_SPEC;
impl crate::RegisterSpec for LFBCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfbclksel::R`](R) reader structure"]
impl crate::Readable for LFBCLKSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lfbclksel::W`](W) writer structure"]
impl crate::Writable for LFBCLKSEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LFBCLKSEL to value 0"]
impl crate::Resettable for LFBCLKSEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
