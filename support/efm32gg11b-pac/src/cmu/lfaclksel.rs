#[doc = "Register `LFACLKSEL` reader"]
pub type R = crate::R<LFACLKSEL_SPEC>;
#[doc = "Register `LFACLKSEL` writer"]
pub type W = crate::W<LFACLKSEL_SPEC>;
#[doc = "Field `LFA` reader - Clock Select for LFA"]
pub type LFA_R = crate::FieldReader<LFA_A>;
#[doc = "Clock Select for LFA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LFA_A {
    #[doc = "0: LFACLK is disabled"]
    DISABLED = 0,
    #[doc = "1: LFRCO selected as LFACLK"]
    LFRCO = 1,
    #[doc = "2: LFXO selected as LFACLK"]
    LFXO = 2,
    #[doc = "4: ULFRCO selected as LFACLK"]
    ULFRCO = 4,
}
impl From<LFA_A> for u8 {
    #[inline(always)]
    fn from(variant: LFA_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LFA_A {
    type Ux = u8;
}
impl LFA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LFA_A> {
        match self.bits {
            0 => Some(LFA_A::DISABLED),
            1 => Some(LFA_A::LFRCO),
            2 => Some(LFA_A::LFXO),
            4 => Some(LFA_A::ULFRCO),
            _ => None,
        }
    }
    #[doc = "LFACLK is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LFA_A::DISABLED
    }
    #[doc = "LFRCO selected as LFACLK"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == LFA_A::LFRCO
    }
    #[doc = "LFXO selected as LFACLK"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == LFA_A::LFXO
    }
    #[doc = "ULFRCO selected as LFACLK"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == LFA_A::ULFRCO
    }
}
#[doc = "Field `LFA` writer - Clock Select for LFA"]
pub type LFA_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LFA_A>;
impl<'a, REG> LFA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LFACLK is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LFA_A::DISABLED)
    }
    #[doc = "LFRCO selected as LFACLK"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(LFA_A::LFRCO)
    }
    #[doc = "LFXO selected as LFACLK"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(LFA_A::LFXO)
    }
    #[doc = "ULFRCO selected as LFACLK"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut crate::W<REG> {
        self.variant(LFA_A::ULFRCO)
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Select for LFA"]
    #[inline(always)]
    pub fn lfa(&self) -> LFA_R {
        LFA_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Select for LFA"]
    #[inline(always)]
    #[must_use]
    pub fn lfa(&mut self) -> LFA_W<LFACLKSEL_SPEC> {
        LFA_W::new(self, 0)
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
#[doc = "Low Frequency A Clock Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfaclksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfaclksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LFACLKSEL_SPEC;
impl crate::RegisterSpec for LFACLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfaclksel::R`](R) reader structure"]
impl crate::Readable for LFACLKSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lfaclksel::W`](W) writer structure"]
impl crate::Writable for LFACLKSEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LFACLKSEL to value 0"]
impl crate::Resettable for LFACLKSEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
