#[doc = "Register `DISPCTRL` reader"]
pub type R = crate::R<DISPCTRL_SPEC>;
#[doc = "Register `DISPCTRL` writer"]
pub type W = crate::W<DISPCTRL_SPEC>;
#[doc = "Field `MUX` reader - Mux Configuration"]
pub type MUX_R = crate::FieldReader<MUX_A>;
#[doc = "Mux Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MUX_A {
    #[doc = "0: Static"]
    STATIC = 0,
    #[doc = "1: Duplex"]
    DUPLEX = 1,
    #[doc = "2: Triplex"]
    TRIPLEX = 2,
    #[doc = "3: Quadruplex"]
    QUADRUPLEX = 3,
    #[doc = "5: Sextaplex"]
    SEXTAPLEX = 5,
    #[doc = "7: Octaplex"]
    OCTAPLEX = 7,
}
impl From<MUX_A> for u8 {
    #[inline(always)]
    fn from(variant: MUX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MUX_A {
    type Ux = u8;
}
impl MUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MUX_A> {
        match self.bits {
            0 => Some(MUX_A::STATIC),
            1 => Some(MUX_A::DUPLEX),
            2 => Some(MUX_A::TRIPLEX),
            3 => Some(MUX_A::QUADRUPLEX),
            5 => Some(MUX_A::SEXTAPLEX),
            7 => Some(MUX_A::OCTAPLEX),
            _ => None,
        }
    }
    #[doc = "Static"]
    #[inline(always)]
    pub fn is_static(&self) -> bool {
        *self == MUX_A::STATIC
    }
    #[doc = "Duplex"]
    #[inline(always)]
    pub fn is_duplex(&self) -> bool {
        *self == MUX_A::DUPLEX
    }
    #[doc = "Triplex"]
    #[inline(always)]
    pub fn is_triplex(&self) -> bool {
        *self == MUX_A::TRIPLEX
    }
    #[doc = "Quadruplex"]
    #[inline(always)]
    pub fn is_quadruplex(&self) -> bool {
        *self == MUX_A::QUADRUPLEX
    }
    #[doc = "Sextaplex"]
    #[inline(always)]
    pub fn is_sextaplex(&self) -> bool {
        *self == MUX_A::SEXTAPLEX
    }
    #[doc = "Octaplex"]
    #[inline(always)]
    pub fn is_octaplex(&self) -> bool {
        *self == MUX_A::OCTAPLEX
    }
}
#[doc = "Field `MUX` writer - Mux Configuration"]
pub type MUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, MUX_A>;
impl<'a, REG, const O: u8> MUX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Static"]
    #[inline(always)]
    pub fn static_(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::STATIC)
    }
    #[doc = "Duplex"]
    #[inline(always)]
    pub fn duplex(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::DUPLEX)
    }
    #[doc = "Triplex"]
    #[inline(always)]
    pub fn triplex(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::TRIPLEX)
    }
    #[doc = "Quadruplex"]
    #[inline(always)]
    pub fn quadruplex(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::QUADRUPLEX)
    }
    #[doc = "Sextaplex"]
    #[inline(always)]
    pub fn sextaplex(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::SEXTAPLEX)
    }
    #[doc = "Octaplex"]
    #[inline(always)]
    pub fn octaplex(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::OCTAPLEX)
    }
}
#[doc = "Field `WAVE` reader - Waveform Selection"]
pub type WAVE_R = crate::BitReader;
#[doc = "Field `WAVE` writer - Waveform Selection"]
pub type WAVE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CONTRAST` reader - Contrast Control"]
pub type CONTRAST_R = crate::FieldReader;
#[doc = "Field `CONTRAST` writer - Contrast Control"]
pub type CONTRAST_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `CHGRDST` reader - Charge Redistribution Cycles"]
pub type CHGRDST_R = crate::FieldReader<CHGRDST_A>;
#[doc = "Charge Redistribution Cycles\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHGRDST_A {
    #[doc = "0: Disable charge redistribution."]
    DISABLE = 0,
    #[doc = "1: Use 1 prescaled low frequency clock cycle for charge redistribution."]
    ONE = 1,
    #[doc = "2: Use 2 prescaled low frequency clock cycles for charge redistribution."]
    TWO = 2,
    #[doc = "3: Use 3 prescaled low frequency clock cycles for charge redistribution."]
    THREE = 3,
    #[doc = "4: Use 4 prescaled low frequency clock cycles for charge redistribution."]
    FOUR = 4,
}
impl From<CHGRDST_A> for u8 {
    #[inline(always)]
    fn from(variant: CHGRDST_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CHGRDST_A {
    type Ux = u8;
}
impl CHGRDST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CHGRDST_A> {
        match self.bits {
            0 => Some(CHGRDST_A::DISABLE),
            1 => Some(CHGRDST_A::ONE),
            2 => Some(CHGRDST_A::TWO),
            3 => Some(CHGRDST_A::THREE),
            4 => Some(CHGRDST_A::FOUR),
            _ => None,
        }
    }
    #[doc = "Disable charge redistribution."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHGRDST_A::DISABLE
    }
    #[doc = "Use 1 prescaled low frequency clock cycle for charge redistribution."]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == CHGRDST_A::ONE
    }
    #[doc = "Use 2 prescaled low frequency clock cycles for charge redistribution."]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == CHGRDST_A::TWO
    }
    #[doc = "Use 3 prescaled low frequency clock cycles for charge redistribution."]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == CHGRDST_A::THREE
    }
    #[doc = "Use 4 prescaled low frequency clock cycles for charge redistribution."]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == CHGRDST_A::FOUR
    }
}
#[doc = "Field `CHGRDST` writer - Charge Redistribution Cycles"]
pub type CHGRDST_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, CHGRDST_A>;
impl<'a, REG, const O: u8> CHGRDST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable charge redistribution."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CHGRDST_A::DISABLE)
    }
    #[doc = "Use 1 prescaled low frequency clock cycle for charge redistribution."]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(CHGRDST_A::ONE)
    }
    #[doc = "Use 2 prescaled low frequency clock cycles for charge redistribution."]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(CHGRDST_A::TWO)
    }
    #[doc = "Use 3 prescaled low frequency clock cycles for charge redistribution."]
    #[inline(always)]
    pub fn three(self) -> &'a mut crate::W<REG> {
        self.variant(CHGRDST_A::THREE)
    }
    #[doc = "Use 4 prescaled low frequency clock cycles for charge redistribution."]
    #[inline(always)]
    pub fn four(self) -> &'a mut crate::W<REG> {
        self.variant(CHGRDST_A::FOUR)
    }
}
#[doc = "Field `BIAS` reader - Bias Configuration"]
pub type BIAS_R = crate::FieldReader<BIAS_A>;
#[doc = "Bias Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BIAS_A {
    #[doc = "0: Static"]
    STATIC = 0,
    #[doc = "1: 1/2 Bias"]
    ONEHALF = 1,
    #[doc = "2: 1/3 Bias"]
    ONETHIRD = 2,
    #[doc = "3: 1/4 Bias"]
    ONEFOURTH = 3,
}
impl From<BIAS_A> for u8 {
    #[inline(always)]
    fn from(variant: BIAS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BIAS_A {
    type Ux = u8;
}
impl BIAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIAS_A {
        match self.bits {
            0 => BIAS_A::STATIC,
            1 => BIAS_A::ONEHALF,
            2 => BIAS_A::ONETHIRD,
            3 => BIAS_A::ONEFOURTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Static"]
    #[inline(always)]
    pub fn is_static(&self) -> bool {
        *self == BIAS_A::STATIC
    }
    #[doc = "1/2 Bias"]
    #[inline(always)]
    pub fn is_onehalf(&self) -> bool {
        *self == BIAS_A::ONEHALF
    }
    #[doc = "1/3 Bias"]
    #[inline(always)]
    pub fn is_onethird(&self) -> bool {
        *self == BIAS_A::ONETHIRD
    }
    #[doc = "1/4 Bias"]
    #[inline(always)]
    pub fn is_onefourth(&self) -> bool {
        *self == BIAS_A::ONEFOURTH
    }
}
#[doc = "Field `BIAS` writer - Bias Configuration"]
pub type BIAS_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, BIAS_A>;
impl<'a, REG, const O: u8> BIAS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Static"]
    #[inline(always)]
    pub fn static_(self) -> &'a mut crate::W<REG> {
        self.variant(BIAS_A::STATIC)
    }
    #[doc = "1/2 Bias"]
    #[inline(always)]
    pub fn onehalf(self) -> &'a mut crate::W<REG> {
        self.variant(BIAS_A::ONEHALF)
    }
    #[doc = "1/3 Bias"]
    #[inline(always)]
    pub fn onethird(self) -> &'a mut crate::W<REG> {
        self.variant(BIAS_A::ONETHIRD)
    }
    #[doc = "1/4 Bias"]
    #[inline(always)]
    pub fn onefourth(self) -> &'a mut crate::W<REG> {
        self.variant(BIAS_A::ONEFOURTH)
    }
}
#[doc = "Field `MODE` reader - Mode Setting"]
pub type MODE_R = crate::FieldReader<MODE_A>;
#[doc = "Mode Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: No External Cap. Uses an internal current source to generate VLCD. Use CONTRAST\\[4:0\\] to control VLCD."]
    NOEXTCAP = 0,
    #[doc = "1: Use step down control with VLCD less than VDD. Use CONTRAST\\[5:0\\] to control VLCD level, and use SPEED to adjust VLCD drive strength."]
    STEPDOWN = 1,
    #[doc = "2: Charge pump used with internal oscillator. Use CONTRAST\\[5:0\\] to control VLCD level, and use SPEED to adjust oscillator frequency."]
    CPINTOSC = 2,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE_A {
    type Ux = u8;
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::NOEXTCAP),
            1 => Some(MODE_A::STEPDOWN),
            2 => Some(MODE_A::CPINTOSC),
            _ => None,
        }
    }
    #[doc = "No External Cap. Uses an internal current source to generate VLCD. Use CONTRAST\\[4:0\\] to control VLCD."]
    #[inline(always)]
    pub fn is_noextcap(&self) -> bool {
        *self == MODE_A::NOEXTCAP
    }
    #[doc = "Use step down control with VLCD less than VDD. Use CONTRAST\\[5:0\\] to control VLCD level, and use SPEED to adjust VLCD drive strength."]
    #[inline(always)]
    pub fn is_stepdown(&self) -> bool {
        *self == MODE_A::STEPDOWN
    }
    #[doc = "Charge pump used with internal oscillator. Use CONTRAST\\[5:0\\] to control VLCD level, and use SPEED to adjust oscillator frequency."]
    #[inline(always)]
    pub fn is_cpintosc(&self) -> bool {
        *self == MODE_A::CPINTOSC
    }
}
#[doc = "Field `MODE` writer - Mode Setting"]
pub type MODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, MODE_A>;
impl<'a, REG, const O: u8> MODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No External Cap. Uses an internal current source to generate VLCD. Use CONTRAST\\[4:0\\] to control VLCD."]
    #[inline(always)]
    pub fn noextcap(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::NOEXTCAP)
    }
    #[doc = "Use step down control with VLCD less than VDD. Use CONTRAST\\[5:0\\] to control VLCD level, and use SPEED to adjust VLCD drive strength."]
    #[inline(always)]
    pub fn stepdown(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::STEPDOWN)
    }
    #[doc = "Charge pump used with internal oscillator. Use CONTRAST\\[5:0\\] to control VLCD level, and use SPEED to adjust oscillator frequency."]
    #[inline(always)]
    pub fn cpintosc(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::CPINTOSC)
    }
}
impl R {
    #[doc = "Bits 0:2 - Mux Configuration"]
    #[inline(always)]
    pub fn mux(&self) -> MUX_R {
        MUX_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Waveform Selection"]
    #[inline(always)]
    pub fn wave(&self) -> WAVE_R {
        WAVE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:13 - Contrast Control"]
    #[inline(always)]
    pub fn contrast(&self) -> CONTRAST_R {
        CONTRAST_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 20:22 - Charge Redistribution Cycles"]
    #[inline(always)]
    pub fn chgrdst(&self) -> CHGRDST_R {
        CHGRDST_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:25 - Bias Configuration"]
    #[inline(always)]
    pub fn bias(&self) -> BIAS_R {
        BIAS_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Mode Setting"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Mux Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn mux(&mut self) -> MUX_W<DISPCTRL_SPEC, 0> {
        MUX_W::new(self)
    }
    #[doc = "Bit 4 - Waveform Selection"]
    #[inline(always)]
    #[must_use]
    pub fn wave(&mut self) -> WAVE_W<DISPCTRL_SPEC, 4> {
        WAVE_W::new(self)
    }
    #[doc = "Bits 8:13 - Contrast Control"]
    #[inline(always)]
    #[must_use]
    pub fn contrast(&mut self) -> CONTRAST_W<DISPCTRL_SPEC, 8> {
        CONTRAST_W::new(self)
    }
    #[doc = "Bits 20:22 - Charge Redistribution Cycles"]
    #[inline(always)]
    #[must_use]
    pub fn chgrdst(&mut self) -> CHGRDST_W<DISPCTRL_SPEC, 20> {
        CHGRDST_W::new(self)
    }
    #[doc = "Bits 24:25 - Bias Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn bias(&mut self) -> BIAS_W<DISPCTRL_SPEC, 24> {
        BIAS_W::new(self)
    }
    #[doc = "Bits 28:29 - Mode Setting"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<DISPCTRL_SPEC, 28> {
        MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Display Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dispctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dispctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DISPCTRL_SPEC;
impl crate::RegisterSpec for DISPCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dispctrl::R`](R) reader structure"]
impl crate::Readable for DISPCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dispctrl::W`](W) writer structure"]
impl crate::Writable for DISPCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DISPCTRL to value 0x0010_3f00"]
impl crate::Resettable for DISPCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0010_3f00;
}
