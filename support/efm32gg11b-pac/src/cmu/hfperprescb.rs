#[doc = "Register `HFPERPRESCB` reader"]
pub type R = crate::R<HFPERPRESCB_SPEC>;
#[doc = "Register `HFPERPRESCB` writer"]
pub type W = crate::W<HFPERPRESCB_SPEC>;
#[doc = "Field `PRESC` reader - HFPERCLK Prescaler"]
pub type PRESC_R = crate::FieldReader<PRESC_A>;
#[doc = "HFPERCLK Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PRESC_A {
    #[doc = "0: `0`"]
    NODIVISION = 0,
}
impl From<PRESC_A> for u16 {
    #[inline(always)]
    fn from(variant: PRESC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRESC_A {
    type Ux = u16;
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
#[doc = "Field `PRESC` writer - HFPERCLK Prescaler"]
pub type PRESC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, PRESC_A>;
impl<'a, REG, const O: u8> PRESC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn nodivision(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::NODIVISION)
    }
}
impl R {
    #[doc = "Bits 8:16 - HFPERCLK Prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 8) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:16 - HFPERCLK Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<HFPERPRESCB_SPEC, 8> {
        PRESC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "High Frequency Peripheral Clock Prescaler B Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfperprescb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfperprescb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFPERPRESCB_SPEC;
impl crate::RegisterSpec for HFPERPRESCB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfperprescb::R`](R) reader structure"]
impl crate::Readable for HFPERPRESCB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hfperprescb::W`](W) writer structure"]
impl crate::Writable for HFPERPRESCB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HFPERPRESCB to value 0"]
impl crate::Resettable for HFPERPRESCB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
