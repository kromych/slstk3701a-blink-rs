#[doc = "Register `HFPRESC` reader"]
pub type R = crate::R<HFPRESC_SPEC>;
#[doc = "Register `HFPRESC` writer"]
pub type W = crate::W<HFPRESC_SPEC>;
#[doc = "Field `PRESC` reader - HFCLK Prescaler"]
pub type PRESC_R = crate::FieldReader<PRESC_A>;
#[doc = "HFCLK Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESC_A {
    #[doc = "0: `0`"]
    NODIVISION = 0,
}
impl From<PRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRESC_A {
    type Ux = u8;
}
impl PRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRESC_A> {
        match self.bits {
            0 => Some(PRESC_A::NODIVISION),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_nodivision(&self) -> bool {
        *self == PRESC_A::NODIVISION
    }
}
#[doc = "Field `PRESC` writer - HFCLK Prescaler"]
pub type PRESC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O, PRESC_A>;
impl<'a, REG, const O: u8> PRESC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn nodivision(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::NODIVISION)
    }
}
#[doc = "Field `HFCLKLEPRESC` reader - HFCLKLE Prescaler"]
pub type HFCLKLEPRESC_R = crate::FieldReader<HFCLKLEPRESC_A>;
#[doc = "HFCLKLE Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HFCLKLEPRESC_A {
    #[doc = "0: HFCLKLE is HFBUSCLKLE divided by 2."]
    DIV2 = 0,
    #[doc = "1: HFCLKLE is HFBUSCLKLE divided by 4."]
    DIV4 = 1,
    #[doc = "2: HFCLKLE is HFBUSCLKLE divided by 8."]
    DIV8 = 2,
}
impl From<HFCLKLEPRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: HFCLKLEPRESC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HFCLKLEPRESC_A {
    type Ux = u8;
}
impl HFCLKLEPRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HFCLKLEPRESC_A> {
        match self.bits {
            0 => Some(HFCLKLEPRESC_A::DIV2),
            1 => Some(HFCLKLEPRESC_A::DIV4),
            2 => Some(HFCLKLEPRESC_A::DIV8),
            _ => None,
        }
    }
    #[doc = "HFCLKLE is HFBUSCLKLE divided by 2."]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == HFCLKLEPRESC_A::DIV2
    }
    #[doc = "HFCLKLE is HFBUSCLKLE divided by 4."]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HFCLKLEPRESC_A::DIV4
    }
    #[doc = "HFCLKLE is HFBUSCLKLE divided by 8."]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == HFCLKLEPRESC_A::DIV8
    }
}
#[doc = "Field `HFCLKLEPRESC` writer - HFCLKLE Prescaler"]
pub type HFCLKLEPRESC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, HFCLKLEPRESC_A>;
impl<'a, REG, const O: u8> HFCLKLEPRESC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HFCLKLE is HFBUSCLKLE divided by 2."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(HFCLKLEPRESC_A::DIV2)
    }
    #[doc = "HFCLKLE is HFBUSCLKLE divided by 4."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(HFCLKLEPRESC_A::DIV4)
    }
    #[doc = "HFCLKLE is HFBUSCLKLE divided by 8."]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(HFCLKLEPRESC_A::DIV8)
    }
}
impl R {
    #[doc = "Bits 8:12 - HFCLK Prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 24:25 - HFCLKLE Prescaler"]
    #[inline(always)]
    pub fn hfclklepresc(&self) -> HFCLKLEPRESC_R {
        HFCLKLEPRESC_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 8:12 - HFCLK Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<HFPRESC_SPEC, 8> {
        PRESC_W::new(self)
    }
    #[doc = "Bits 24:25 - HFCLKLE Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn hfclklepresc(&mut self) -> HFCLKLEPRESC_W<HFPRESC_SPEC, 24> {
        HFCLKLEPRESC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "High Frequency Clock Prescaler Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfpresc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfpresc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFPRESC_SPEC;
impl crate::RegisterSpec for HFPRESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfpresc::R`](R) reader structure"]
impl crate::Readable for HFPRESC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hfpresc::W`](W) writer structure"]
impl crate::Writable for HFPRESC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HFPRESC to value 0"]
impl crate::Resettable for HFPRESC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
