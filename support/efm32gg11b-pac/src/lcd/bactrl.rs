#[doc = "Register `BACTRL` reader"]
pub type R = crate::R<BACTRL_SPEC>;
#[doc = "Register `BACTRL` writer"]
pub type W = crate::W<BACTRL_SPEC>;
#[doc = "Field `BLINKEN` reader - Blink Enable"]
pub type BLINKEN_R = crate::BitReader;
#[doc = "Field `BLINKEN` writer - Blink Enable"]
pub type BLINKEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BLANK` reader - Blank Display"]
pub type BLANK_R = crate::BitReader;
#[doc = "Field `BLANK` writer - Blank Display"]
pub type BLANK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AEN` reader - Animation Enable"]
pub type AEN_R = crate::BitReader;
#[doc = "Field `AEN` writer - Animation Enable"]
pub type AEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AREGASC` reader - Animate Register a Shift Control"]
pub type AREGASC_R = crate::FieldReader<AREGASC_A>;
#[doc = "Animate Register a Shift Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AREGASC_A {
    #[doc = "0: No Shift operation on Animation Register A"]
    NOSHIFT = 0,
    #[doc = "1: Animation Register A is shifted left"]
    SHIFTLEFT = 1,
    #[doc = "2: Animation Register A is shifted right"]
    SHIFTRIGHT = 2,
}
impl From<AREGASC_A> for u8 {
    #[inline(always)]
    fn from(variant: AREGASC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AREGASC_A {
    type Ux = u8;
}
impl AREGASC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AREGASC_A> {
        match self.bits {
            0 => Some(AREGASC_A::NOSHIFT),
            1 => Some(AREGASC_A::SHIFTLEFT),
            2 => Some(AREGASC_A::SHIFTRIGHT),
            _ => None,
        }
    }
    #[doc = "No Shift operation on Animation Register A"]
    #[inline(always)]
    pub fn is_noshift(&self) -> bool {
        *self == AREGASC_A::NOSHIFT
    }
    #[doc = "Animation Register A is shifted left"]
    #[inline(always)]
    pub fn is_shiftleft(&self) -> bool {
        *self == AREGASC_A::SHIFTLEFT
    }
    #[doc = "Animation Register A is shifted right"]
    #[inline(always)]
    pub fn is_shiftright(&self) -> bool {
        *self == AREGASC_A::SHIFTRIGHT
    }
}
#[doc = "Field `AREGASC` writer - Animate Register a Shift Control"]
pub type AREGASC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, AREGASC_A>;
impl<'a, REG, const O: u8> AREGASC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Shift operation on Animation Register A"]
    #[inline(always)]
    pub fn noshift(self) -> &'a mut crate::W<REG> {
        self.variant(AREGASC_A::NOSHIFT)
    }
    #[doc = "Animation Register A is shifted left"]
    #[inline(always)]
    pub fn shiftleft(self) -> &'a mut crate::W<REG> {
        self.variant(AREGASC_A::SHIFTLEFT)
    }
    #[doc = "Animation Register A is shifted right"]
    #[inline(always)]
    pub fn shiftright(self) -> &'a mut crate::W<REG> {
        self.variant(AREGASC_A::SHIFTRIGHT)
    }
}
#[doc = "Field `AREGBSC` reader - Animate Register B Shift Control"]
pub type AREGBSC_R = crate::FieldReader<AREGBSC_A>;
#[doc = "Animate Register B Shift Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AREGBSC_A {
    #[doc = "0: No Shift operation on Animation Register B"]
    NOSHIFT = 0,
    #[doc = "1: Animation Register B is shifted left"]
    SHIFTLEFT = 1,
    #[doc = "2: Animation Register B is shifted right"]
    SHIFTRIGHT = 2,
}
impl From<AREGBSC_A> for u8 {
    #[inline(always)]
    fn from(variant: AREGBSC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AREGBSC_A {
    type Ux = u8;
}
impl AREGBSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AREGBSC_A> {
        match self.bits {
            0 => Some(AREGBSC_A::NOSHIFT),
            1 => Some(AREGBSC_A::SHIFTLEFT),
            2 => Some(AREGBSC_A::SHIFTRIGHT),
            _ => None,
        }
    }
    #[doc = "No Shift operation on Animation Register B"]
    #[inline(always)]
    pub fn is_noshift(&self) -> bool {
        *self == AREGBSC_A::NOSHIFT
    }
    #[doc = "Animation Register B is shifted left"]
    #[inline(always)]
    pub fn is_shiftleft(&self) -> bool {
        *self == AREGBSC_A::SHIFTLEFT
    }
    #[doc = "Animation Register B is shifted right"]
    #[inline(always)]
    pub fn is_shiftright(&self) -> bool {
        *self == AREGBSC_A::SHIFTRIGHT
    }
}
#[doc = "Field `AREGBSC` writer - Animate Register B Shift Control"]
pub type AREGBSC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, AREGBSC_A>;
impl<'a, REG, const O: u8> AREGBSC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Shift operation on Animation Register B"]
    #[inline(always)]
    pub fn noshift(self) -> &'a mut crate::W<REG> {
        self.variant(AREGBSC_A::NOSHIFT)
    }
    #[doc = "Animation Register B is shifted left"]
    #[inline(always)]
    pub fn shiftleft(self) -> &'a mut crate::W<REG> {
        self.variant(AREGBSC_A::SHIFTLEFT)
    }
    #[doc = "Animation Register B is shifted right"]
    #[inline(always)]
    pub fn shiftright(self) -> &'a mut crate::W<REG> {
        self.variant(AREGBSC_A::SHIFTRIGHT)
    }
}
#[doc = "Field `ALOGSEL` reader - Animate Logic Function Select"]
pub type ALOGSEL_R = crate::BitReader;
#[doc = "Field `ALOGSEL` writer - Animate Logic Function Select"]
pub type ALOGSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FCEN` reader - Frame Counter Enable"]
pub type FCEN_R = crate::BitReader;
#[doc = "Field `FCEN` writer - Frame Counter Enable"]
pub type FCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FCPRESC` reader - Frame Counter Prescaler"]
pub type FCPRESC_R = crate::FieldReader<FCPRESC_A>;
#[doc = "Frame Counter Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FCPRESC_A {
    #[doc = "0: CLKFC = CLKFRAME / 1"]
    DIV1 = 0,
    #[doc = "1: CLKFC = CLKFRAME / 2"]
    DIV2 = 1,
    #[doc = "2: CLKFC = CLKFRAME / 4"]
    DIV4 = 2,
    #[doc = "3: CLKFC = CLKFRAME / 8"]
    DIV8 = 3,
}
impl From<FCPRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: FCPRESC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FCPRESC_A {
    type Ux = u8;
}
impl FCPRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCPRESC_A {
        match self.bits {
            0 => FCPRESC_A::DIV1,
            1 => FCPRESC_A::DIV2,
            2 => FCPRESC_A::DIV4,
            3 => FCPRESC_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "CLKFC = CLKFRAME / 1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == FCPRESC_A::DIV1
    }
    #[doc = "CLKFC = CLKFRAME / 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == FCPRESC_A::DIV2
    }
    #[doc = "CLKFC = CLKFRAME / 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == FCPRESC_A::DIV4
    }
    #[doc = "CLKFC = CLKFRAME / 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == FCPRESC_A::DIV8
    }
}
#[doc = "Field `FCPRESC` writer - Frame Counter Prescaler"]
pub type FCPRESC_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, FCPRESC_A>;
impl<'a, REG, const O: u8> FCPRESC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CLKFC = CLKFRAME / 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(FCPRESC_A::DIV1)
    }
    #[doc = "CLKFC = CLKFRAME / 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(FCPRESC_A::DIV2)
    }
    #[doc = "CLKFC = CLKFRAME / 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(FCPRESC_A::DIV4)
    }
    #[doc = "CLKFC = CLKFRAME / 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(FCPRESC_A::DIV8)
    }
}
#[doc = "Field `FCTOP` reader - Frame Counter Top Value"]
pub type FCTOP_R = crate::FieldReader;
#[doc = "Field `FCTOP` writer - Frame Counter Top Value"]
pub type FCTOP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `ALOC` reader - Animation Location"]
pub type ALOC_R = crate::BitReader;
#[doc = "Field `ALOC` writer - Animation Location"]
pub type ALOC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Blink Enable"]
    #[inline(always)]
    pub fn blinken(&self) -> BLINKEN_R {
        BLINKEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Blank Display"]
    #[inline(always)]
    pub fn blank(&self) -> BLANK_R {
        BLANK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Animation Enable"]
    #[inline(always)]
    pub fn aen(&self) -> AEN_R {
        AEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Animate Register a Shift Control"]
    #[inline(always)]
    pub fn aregasc(&self) -> AREGASC_R {
        AREGASC_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Animate Register B Shift Control"]
    #[inline(always)]
    pub fn aregbsc(&self) -> AREGBSC_R {
        AREGBSC_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Animate Logic Function Select"]
    #[inline(always)]
    pub fn alogsel(&self) -> ALOGSEL_R {
        ALOGSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Frame Counter Enable"]
    #[inline(always)]
    pub fn fcen(&self) -> FCEN_R {
        FCEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Frame Counter Prescaler"]
    #[inline(always)]
    pub fn fcpresc(&self) -> FCPRESC_R {
        FCPRESC_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:23 - Frame Counter Top Value"]
    #[inline(always)]
    pub fn fctop(&self) -> FCTOP_R {
        FCTOP_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bit 28 - Animation Location"]
    #[inline(always)]
    pub fn aloc(&self) -> ALOC_R {
        ALOC_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Blink Enable"]
    #[inline(always)]
    #[must_use]
    pub fn blinken(&mut self) -> BLINKEN_W<BACTRL_SPEC, 0> {
        BLINKEN_W::new(self)
    }
    #[doc = "Bit 1 - Blank Display"]
    #[inline(always)]
    #[must_use]
    pub fn blank(&mut self) -> BLANK_W<BACTRL_SPEC, 1> {
        BLANK_W::new(self)
    }
    #[doc = "Bit 2 - Animation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aen(&mut self) -> AEN_W<BACTRL_SPEC, 2> {
        AEN_W::new(self)
    }
    #[doc = "Bits 3:4 - Animate Register a Shift Control"]
    #[inline(always)]
    #[must_use]
    pub fn aregasc(&mut self) -> AREGASC_W<BACTRL_SPEC, 3> {
        AREGASC_W::new(self)
    }
    #[doc = "Bits 5:6 - Animate Register B Shift Control"]
    #[inline(always)]
    #[must_use]
    pub fn aregbsc(&mut self) -> AREGBSC_W<BACTRL_SPEC, 5> {
        AREGBSC_W::new(self)
    }
    #[doc = "Bit 7 - Animate Logic Function Select"]
    #[inline(always)]
    #[must_use]
    pub fn alogsel(&mut self) -> ALOGSEL_W<BACTRL_SPEC, 7> {
        ALOGSEL_W::new(self)
    }
    #[doc = "Bit 8 - Frame Counter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fcen(&mut self) -> FCEN_W<BACTRL_SPEC, 8> {
        FCEN_W::new(self)
    }
    #[doc = "Bits 16:17 - Frame Counter Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn fcpresc(&mut self) -> FCPRESC_W<BACTRL_SPEC, 16> {
        FCPRESC_W::new(self)
    }
    #[doc = "Bits 18:23 - Frame Counter Top Value"]
    #[inline(always)]
    #[must_use]
    pub fn fctop(&mut self) -> FCTOP_W<BACTRL_SPEC, 18> {
        FCTOP_W::new(self)
    }
    #[doc = "Bit 28 - Animation Location"]
    #[inline(always)]
    #[must_use]
    pub fn aloc(&mut self) -> ALOC_W<BACTRL_SPEC, 28> {
        ALOC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Blink and Animation Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bactrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bactrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BACTRL_SPEC;
impl crate::RegisterSpec for BACTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bactrl::R`](R) reader structure"]
impl crate::Readable for BACTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bactrl::W`](W) writer structure"]
impl crate::Writable for BACTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BACTRL to value 0"]
impl crate::Resettable for BACTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
