#[doc = "Register `ADCCTRL` reader"]
pub type R = crate::R<ADCCTRL_SPEC>;
#[doc = "Register `ADCCTRL` writer"]
pub type W = crate::W<ADCCTRL_SPEC>;
#[doc = "Field `ADC0CLKDIV` reader - ADC0 Clock Prescaler"]
pub type ADC0CLKDIV_R = crate::FieldReader<ADC0CLKDIV_A>;
#[doc = "ADC0 Clock Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC0CLKDIV_A {
    #[doc = "0: `0`"]
    NODIVISION = 0,
}
impl From<ADC0CLKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC0CLKDIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADC0CLKDIV_A {
    type Ux = u8;
}
impl ADC0CLKDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC0CLKDIV_A> {
        match self.bits {
            0 => Some(ADC0CLKDIV_A::NODIVISION),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_nodivision(&self) -> bool {
        *self == ADC0CLKDIV_A::NODIVISION
    }
}
#[doc = "Field `ADC0CLKDIV` writer - ADC0 Clock Prescaler"]
pub type ADC0CLKDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, ADC0CLKDIV_A>;
impl<'a, REG, const O: u8> ADC0CLKDIV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn nodivision(self) -> &'a mut crate::W<REG> {
        self.variant(ADC0CLKDIV_A::NODIVISION)
    }
}
#[doc = "Field `ADC0CLKSEL` reader - ADC0 Clock Select"]
pub type ADC0CLKSEL_R = crate::FieldReader<ADC0CLKSEL_A>;
#[doc = "ADC0 Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC0CLKSEL_A {
    #[doc = "0: ADC0 is not clocked"]
    DISABLED = 0,
    #[doc = "1: AUXHFRCO is clocking ADC0"]
    AUXHFRCO = 1,
    #[doc = "2: HFXO is clocking ADC0"]
    HFXO = 2,
    #[doc = "3: HFSRCCLK is clocking ADC0"]
    HFSRCCLK = 3,
}
impl From<ADC0CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC0CLKSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADC0CLKSEL_A {
    type Ux = u8;
}
impl ADC0CLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC0CLKSEL_A {
        match self.bits {
            0 => ADC0CLKSEL_A::DISABLED,
            1 => ADC0CLKSEL_A::AUXHFRCO,
            2 => ADC0CLKSEL_A::HFXO,
            3 => ADC0CLKSEL_A::HFSRCCLK,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC0 is not clocked"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADC0CLKSEL_A::DISABLED
    }
    #[doc = "AUXHFRCO is clocking ADC0"]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == ADC0CLKSEL_A::AUXHFRCO
    }
    #[doc = "HFXO is clocking ADC0"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == ADC0CLKSEL_A::HFXO
    }
    #[doc = "HFSRCCLK is clocking ADC0"]
    #[inline(always)]
    pub fn is_hfsrcclk(&self) -> bool {
        *self == ADC0CLKSEL_A::HFSRCCLK
    }
}
#[doc = "Field `ADC0CLKSEL` writer - ADC0 Clock Select"]
pub type ADC0CLKSEL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, ADC0CLKSEL_A>;
impl<'a, REG, const O: u8> ADC0CLKSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC0 is not clocked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADC0CLKSEL_A::DISABLED)
    }
    #[doc = "AUXHFRCO is clocking ADC0"]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut crate::W<REG> {
        self.variant(ADC0CLKSEL_A::AUXHFRCO)
    }
    #[doc = "HFXO is clocking ADC0"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(ADC0CLKSEL_A::HFXO)
    }
    #[doc = "HFSRCCLK is clocking ADC0"]
    #[inline(always)]
    pub fn hfsrcclk(self) -> &'a mut crate::W<REG> {
        self.variant(ADC0CLKSEL_A::HFSRCCLK)
    }
}
#[doc = "Field `ADC0CLKINV` reader - Invert Clock Selected By ADC0CLKSEL"]
pub type ADC0CLKINV_R = crate::BitReader;
#[doc = "Field `ADC0CLKINV` writer - Invert Clock Selected By ADC0CLKSEL"]
pub type ADC0CLKINV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC1CLKDIV` reader - ADC1 Clock Prescaler"]
pub type ADC1CLKDIV_R = crate::FieldReader<ADC1CLKDIV_A>;
#[doc = "ADC1 Clock Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC1CLKDIV_A {
    #[doc = "0: `0`"]
    NODIVISION = 0,
}
impl From<ADC1CLKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC1CLKDIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADC1CLKDIV_A {
    type Ux = u8;
}
impl ADC1CLKDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC1CLKDIV_A> {
        match self.bits {
            0 => Some(ADC1CLKDIV_A::NODIVISION),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_nodivision(&self) -> bool {
        *self == ADC1CLKDIV_A::NODIVISION
    }
}
#[doc = "Field `ADC1CLKDIV` writer - ADC1 Clock Prescaler"]
pub type ADC1CLKDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, ADC1CLKDIV_A>;
impl<'a, REG, const O: u8> ADC1CLKDIV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn nodivision(self) -> &'a mut crate::W<REG> {
        self.variant(ADC1CLKDIV_A::NODIVISION)
    }
}
#[doc = "Field `ADC1CLKSEL` reader - ADC1 Clock Select"]
pub type ADC1CLKSEL_R = crate::FieldReader<ADC1CLKSEL_A>;
#[doc = "ADC1 Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC1CLKSEL_A {
    #[doc = "0: ADC1 is not clocked"]
    DISABLED = 0,
    #[doc = "1: AUXHFRCO is clocking ADC1"]
    AUXHFRCO = 1,
    #[doc = "2: HFXO is clocking ADC1"]
    HFXO = 2,
    #[doc = "3: HFSRCCLK is clocking ADC1"]
    HFSRCCLK = 3,
}
impl From<ADC1CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC1CLKSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADC1CLKSEL_A {
    type Ux = u8;
}
impl ADC1CLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC1CLKSEL_A {
        match self.bits {
            0 => ADC1CLKSEL_A::DISABLED,
            1 => ADC1CLKSEL_A::AUXHFRCO,
            2 => ADC1CLKSEL_A::HFXO,
            3 => ADC1CLKSEL_A::HFSRCCLK,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC1 is not clocked"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADC1CLKSEL_A::DISABLED
    }
    #[doc = "AUXHFRCO is clocking ADC1"]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == ADC1CLKSEL_A::AUXHFRCO
    }
    #[doc = "HFXO is clocking ADC1"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == ADC1CLKSEL_A::HFXO
    }
    #[doc = "HFSRCCLK is clocking ADC1"]
    #[inline(always)]
    pub fn is_hfsrcclk(&self) -> bool {
        *self == ADC1CLKSEL_A::HFSRCCLK
    }
}
#[doc = "Field `ADC1CLKSEL` writer - ADC1 Clock Select"]
pub type ADC1CLKSEL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, ADC1CLKSEL_A>;
impl<'a, REG, const O: u8> ADC1CLKSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC1 is not clocked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADC1CLKSEL_A::DISABLED)
    }
    #[doc = "AUXHFRCO is clocking ADC1"]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut crate::W<REG> {
        self.variant(ADC1CLKSEL_A::AUXHFRCO)
    }
    #[doc = "HFXO is clocking ADC1"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(ADC1CLKSEL_A::HFXO)
    }
    #[doc = "HFSRCCLK is clocking ADC1"]
    #[inline(always)]
    pub fn hfsrcclk(self) -> &'a mut crate::W<REG> {
        self.variant(ADC1CLKSEL_A::HFSRCCLK)
    }
}
#[doc = "Field `ADC1CLKINV` reader - Invert Clock Selected By ADC1CLKSEL"]
pub type ADC1CLKINV_R = crate::BitReader;
#[doc = "Field `ADC1CLKINV` writer - Invert Clock Selected By ADC1CLKSEL"]
pub type ADC1CLKINV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - ADC0 Clock Prescaler"]
    #[inline(always)]
    pub fn adc0clkdiv(&self) -> ADC0CLKDIV_R {
        ADC0CLKDIV_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - ADC0 Clock Select"]
    #[inline(always)]
    pub fn adc0clksel(&self) -> ADC0CLKSEL_R {
        ADC0CLKSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Invert Clock Selected By ADC0CLKSEL"]
    #[inline(always)]
    pub fn adc0clkinv(&self) -> ADC0CLKINV_R {
        ADC0CLKINV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17 - ADC1 Clock Prescaler"]
    #[inline(always)]
    pub fn adc1clkdiv(&self) -> ADC1CLKDIV_R {
        ADC1CLKDIV_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - ADC1 Clock Select"]
    #[inline(always)]
    pub fn adc1clksel(&self) -> ADC1CLKSEL_R {
        ADC1CLKSEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 24 - Invert Clock Selected By ADC1CLKSEL"]
    #[inline(always)]
    pub fn adc1clkinv(&self) -> ADC1CLKINV_R {
        ADC1CLKINV_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADC0 Clock Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn adc0clkdiv(&mut self) -> ADC0CLKDIV_W<ADCCTRL_SPEC, 0> {
        ADC0CLKDIV_W::new(self)
    }
    #[doc = "Bits 4:5 - ADC0 Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc0clksel(&mut self) -> ADC0CLKSEL_W<ADCCTRL_SPEC, 4> {
        ADC0CLKSEL_W::new(self)
    }
    #[doc = "Bit 8 - Invert Clock Selected By ADC0CLKSEL"]
    #[inline(always)]
    #[must_use]
    pub fn adc0clkinv(&mut self) -> ADC0CLKINV_W<ADCCTRL_SPEC, 8> {
        ADC0CLKINV_W::new(self)
    }
    #[doc = "Bits 16:17 - ADC1 Clock Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn adc1clkdiv(&mut self) -> ADC1CLKDIV_W<ADCCTRL_SPEC, 16> {
        ADC1CLKDIV_W::new(self)
    }
    #[doc = "Bits 20:21 - ADC1 Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc1clksel(&mut self) -> ADC1CLKSEL_W<ADCCTRL_SPEC, 20> {
        ADC1CLKSEL_W::new(self)
    }
    #[doc = "Bit 24 - Invert Clock Selected By ADC1CLKSEL"]
    #[inline(always)]
    #[must_use]
    pub fn adc1clkinv(&mut self) -> ADC1CLKINV_W<ADCCTRL_SPEC, 24> {
        ADC1CLKINV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ADC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADCCTRL_SPEC;
impl crate::RegisterSpec for ADCCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcctrl::R`](R) reader structure"]
impl crate::Readable for ADCCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adcctrl::W`](W) writer structure"]
impl crate::Writable for ADCCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCCTRL to value 0"]
impl crate::Resettable for ADCCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
