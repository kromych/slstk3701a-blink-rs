#[doc = "Register `TIMCTRL` reader"]
pub type R = crate::R<TIMCTRL_SPEC>;
#[doc = "Register `TIMCTRL` writer"]
pub type W = crate::W<TIMCTRL_SPEC>;
#[doc = "Field `PCPRESC` reader - Period Counter Prescaler"]
pub type PCPRESC_R = crate::FieldReader<PCPRESC_A>;
#[doc = "Period Counter Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCPRESC_A {
    #[doc = "0: The period counter clock frequency is LFBCLKCSEN/1"]
    DIV1 = 0,
    #[doc = "1: The period counter clock frequency is LFBCLKCSEN/2"]
    DIV2 = 1,
    #[doc = "2: The period counter clock frequency is LFBCLKCSEN/4"]
    DIV4 = 2,
    #[doc = "3: The period counter clock frequency is LFBCLKCSEN/8"]
    DIV8 = 3,
    #[doc = "4: The period counter clock frequency is LFBCLKCSEN/16"]
    DIV16 = 4,
    #[doc = "5: The period counter clock frequency is LFBCLKCSEN/32"]
    DIV32 = 5,
    #[doc = "6: The period counter clock frequency is LFBCLKCSEN/64"]
    DIV64 = 6,
    #[doc = "7: The period counter clock frequency is LFBCLKCSEN/128"]
    DIV128 = 7,
}
impl From<PCPRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: PCPRESC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCPRESC_A {
    type Ux = u8;
}
impl PCPRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCPRESC_A {
        match self.bits {
            0 => PCPRESC_A::DIV1,
            1 => PCPRESC_A::DIV2,
            2 => PCPRESC_A::DIV4,
            3 => PCPRESC_A::DIV8,
            4 => PCPRESC_A::DIV16,
            5 => PCPRESC_A::DIV32,
            6 => PCPRESC_A::DIV64,
            7 => PCPRESC_A::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PCPRESC_A::DIV1
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PCPRESC_A::DIV2
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PCPRESC_A::DIV4
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PCPRESC_A::DIV8
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PCPRESC_A::DIV16
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PCPRESC_A::DIV32
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PCPRESC_A::DIV64
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PCPRESC_A::DIV128
    }
}
#[doc = "Field `PCPRESC` writer - Period Counter Prescaler"]
pub type PCPRESC_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, PCPRESC_A>;
impl<'a, REG, const O: u8> PCPRESC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The period counter clock frequency is LFBCLKCSEN/1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PCPRESC_A::DIV1)
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PCPRESC_A::DIV2)
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PCPRESC_A::DIV4)
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PCPRESC_A::DIV8)
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PCPRESC_A::DIV16)
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(PCPRESC_A::DIV32)
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(PCPRESC_A::DIV64)
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(PCPRESC_A::DIV128)
    }
}
#[doc = "Field `PCTOP` reader - Period Counter Top Value"]
pub type PCTOP_R = crate::FieldReader;
#[doc = "Field `PCTOP` writer - Period Counter Top Value"]
pub type PCTOP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `WARMUPCNT` reader - Warmup Period Counter"]
pub type WARMUPCNT_R = crate::FieldReader;
#[doc = "Field `WARMUPCNT` writer - Warmup Period Counter"]
pub type WARMUPCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:2 - Period Counter Prescaler"]
    #[inline(always)]
    pub fn pcpresc(&self) -> PCPRESC_R {
        PCPRESC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:15 - Period Counter Top Value"]
    #[inline(always)]
    pub fn pctop(&self) -> PCTOP_R {
        PCTOP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Warmup Period Counter"]
    #[inline(always)]
    pub fn warmupcnt(&self) -> WARMUPCNT_R {
        WARMUPCNT_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Period Counter Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn pcpresc(&mut self) -> PCPRESC_W<TIMCTRL_SPEC, 0> {
        PCPRESC_W::new(self)
    }
    #[doc = "Bits 8:15 - Period Counter Top Value"]
    #[inline(always)]
    #[must_use]
    pub fn pctop(&mut self) -> PCTOP_W<TIMCTRL_SPEC, 8> {
        PCTOP_W::new(self)
    }
    #[doc = "Bits 16:17 - Warmup Period Counter"]
    #[inline(always)]
    #[must_use]
    pub fn warmupcnt(&mut self) -> WARMUPCNT_W<TIMCTRL_SPEC, 16> {
        WARMUPCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timing Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMCTRL_SPEC;
impl crate::RegisterSpec for TIMCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timctrl::R`](R) reader structure"]
impl crate::Readable for TIMCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timctrl::W`](W) writer structure"]
impl crate::Writable for TIMCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMCTRL to value 0"]
impl crate::Resettable for TIMCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
