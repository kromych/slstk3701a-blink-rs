#[doc = "Register `ADCCTRL` reader"]
pub type R = crate::R<AdcctrlSpec>;
#[doc = "Register `ADCCTRL` writer"]
pub type W = crate::W<AdcctrlSpec>;
#[doc = "ADC0 Clock Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc0clkdiv {
    #[doc = "0: `0`"]
    Nodivision = 0,
}
impl From<Adc0clkdiv> for u8 {
    #[inline(always)]
    fn from(variant: Adc0clkdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc0clkdiv {
    type Ux = u8;
}
impl crate::IsEnum for Adc0clkdiv {}
#[doc = "Field `ADC0CLKDIV` reader - ADC0 Clock Prescaler"]
pub type Adc0clkdivR = crate::FieldReader<Adc0clkdiv>;
impl Adc0clkdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Adc0clkdiv> {
        match self.bits {
            0 => Some(Adc0clkdiv::Nodivision),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_nodivision(&self) -> bool {
        *self == Adc0clkdiv::Nodivision
    }
}
#[doc = "Field `ADC0CLKDIV` writer - ADC0 Clock Prescaler"]
pub type Adc0clkdivW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adc0clkdiv>;
impl<'a, REG> Adc0clkdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn nodivision(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0clkdiv::Nodivision)
    }
}
#[doc = "ADC0 Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc0clksel {
    #[doc = "0: ADC0 is not clocked"]
    Disabled = 0,
    #[doc = "1: AUXHFRCO is clocking ADC0"]
    Auxhfrco = 1,
    #[doc = "2: HFXO is clocking ADC0"]
    Hfxo = 2,
    #[doc = "3: HFSRCCLK is clocking ADC0"]
    Hfsrcclk = 3,
}
impl From<Adc0clksel> for u8 {
    #[inline(always)]
    fn from(variant: Adc0clksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc0clksel {
    type Ux = u8;
}
impl crate::IsEnum for Adc0clksel {}
#[doc = "Field `ADC0CLKSEL` reader - ADC0 Clock Select"]
pub type Adc0clkselR = crate::FieldReader<Adc0clksel>;
impl Adc0clkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc0clksel {
        match self.bits {
            0 => Adc0clksel::Disabled,
            1 => Adc0clksel::Auxhfrco,
            2 => Adc0clksel::Hfxo,
            3 => Adc0clksel::Hfsrcclk,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC0 is not clocked"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Adc0clksel::Disabled
    }
    #[doc = "AUXHFRCO is clocking ADC0"]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == Adc0clksel::Auxhfrco
    }
    #[doc = "HFXO is clocking ADC0"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == Adc0clksel::Hfxo
    }
    #[doc = "HFSRCCLK is clocking ADC0"]
    #[inline(always)]
    pub fn is_hfsrcclk(&self) -> bool {
        *self == Adc0clksel::Hfsrcclk
    }
}
#[doc = "Field `ADC0CLKSEL` writer - ADC0 Clock Select"]
pub type Adc0clkselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adc0clksel, crate::Safe>;
impl<'a, REG> Adc0clkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC0 is not clocked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0clksel::Disabled)
    }
    #[doc = "AUXHFRCO is clocking ADC0"]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0clksel::Auxhfrco)
    }
    #[doc = "HFXO is clocking ADC0"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0clksel::Hfxo)
    }
    #[doc = "HFSRCCLK is clocking ADC0"]
    #[inline(always)]
    pub fn hfsrcclk(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0clksel::Hfsrcclk)
    }
}
#[doc = "Field `ADC0CLKINV` reader - Invert Clock Selected By ADC0CLKSEL"]
pub type Adc0clkinvR = crate::BitReader;
#[doc = "Field `ADC0CLKINV` writer - Invert Clock Selected By ADC0CLKSEL"]
pub type Adc0clkinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "ADC1 Clock Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc1clkdiv {
    #[doc = "0: `0`"]
    Nodivision = 0,
}
impl From<Adc1clkdiv> for u8 {
    #[inline(always)]
    fn from(variant: Adc1clkdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc1clkdiv {
    type Ux = u8;
}
impl crate::IsEnum for Adc1clkdiv {}
#[doc = "Field `ADC1CLKDIV` reader - ADC1 Clock Prescaler"]
pub type Adc1clkdivR = crate::FieldReader<Adc1clkdiv>;
impl Adc1clkdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Adc1clkdiv> {
        match self.bits {
            0 => Some(Adc1clkdiv::Nodivision),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_nodivision(&self) -> bool {
        *self == Adc1clkdiv::Nodivision
    }
}
#[doc = "Field `ADC1CLKDIV` writer - ADC1 Clock Prescaler"]
pub type Adc1clkdivW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adc1clkdiv>;
impl<'a, REG> Adc1clkdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn nodivision(self) -> &'a mut crate::W<REG> {
        self.variant(Adc1clkdiv::Nodivision)
    }
}
#[doc = "ADC1 Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc1clksel {
    #[doc = "0: ADC1 is not clocked"]
    Disabled = 0,
    #[doc = "1: AUXHFRCO is clocking ADC1"]
    Auxhfrco = 1,
    #[doc = "2: HFXO is clocking ADC1"]
    Hfxo = 2,
    #[doc = "3: HFSRCCLK is clocking ADC1"]
    Hfsrcclk = 3,
}
impl From<Adc1clksel> for u8 {
    #[inline(always)]
    fn from(variant: Adc1clksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc1clksel {
    type Ux = u8;
}
impl crate::IsEnum for Adc1clksel {}
#[doc = "Field `ADC1CLKSEL` reader - ADC1 Clock Select"]
pub type Adc1clkselR = crate::FieldReader<Adc1clksel>;
impl Adc1clkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc1clksel {
        match self.bits {
            0 => Adc1clksel::Disabled,
            1 => Adc1clksel::Auxhfrco,
            2 => Adc1clksel::Hfxo,
            3 => Adc1clksel::Hfsrcclk,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC1 is not clocked"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Adc1clksel::Disabled
    }
    #[doc = "AUXHFRCO is clocking ADC1"]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == Adc1clksel::Auxhfrco
    }
    #[doc = "HFXO is clocking ADC1"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == Adc1clksel::Hfxo
    }
    #[doc = "HFSRCCLK is clocking ADC1"]
    #[inline(always)]
    pub fn is_hfsrcclk(&self) -> bool {
        *self == Adc1clksel::Hfsrcclk
    }
}
#[doc = "Field `ADC1CLKSEL` writer - ADC1 Clock Select"]
pub type Adc1clkselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adc1clksel, crate::Safe>;
impl<'a, REG> Adc1clkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC1 is not clocked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Adc1clksel::Disabled)
    }
    #[doc = "AUXHFRCO is clocking ADC1"]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Adc1clksel::Auxhfrco)
    }
    #[doc = "HFXO is clocking ADC1"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Adc1clksel::Hfxo)
    }
    #[doc = "HFSRCCLK is clocking ADC1"]
    #[inline(always)]
    pub fn hfsrcclk(self) -> &'a mut crate::W<REG> {
        self.variant(Adc1clksel::Hfsrcclk)
    }
}
#[doc = "Field `ADC1CLKINV` reader - Invert Clock Selected By ADC1CLKSEL"]
pub type Adc1clkinvR = crate::BitReader;
#[doc = "Field `ADC1CLKINV` writer - Invert Clock Selected By ADC1CLKSEL"]
pub type Adc1clkinvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - ADC0 Clock Prescaler"]
    #[inline(always)]
    pub fn adc0clkdiv(&self) -> Adc0clkdivR {
        Adc0clkdivR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - ADC0 Clock Select"]
    #[inline(always)]
    pub fn adc0clksel(&self) -> Adc0clkselR {
        Adc0clkselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Invert Clock Selected By ADC0CLKSEL"]
    #[inline(always)]
    pub fn adc0clkinv(&self) -> Adc0clkinvR {
        Adc0clkinvR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17 - ADC1 Clock Prescaler"]
    #[inline(always)]
    pub fn adc1clkdiv(&self) -> Adc1clkdivR {
        Adc1clkdivR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - ADC1 Clock Select"]
    #[inline(always)]
    pub fn adc1clksel(&self) -> Adc1clkselR {
        Adc1clkselR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 24 - Invert Clock Selected By ADC1CLKSEL"]
    #[inline(always)]
    pub fn adc1clkinv(&self) -> Adc1clkinvR {
        Adc1clkinvR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADC0 Clock Prescaler"]
    #[inline(always)]
    pub fn adc0clkdiv(&mut self) -> Adc0clkdivW<'_, AdcctrlSpec> {
        Adc0clkdivW::new(self, 0)
    }
    #[doc = "Bits 4:5 - ADC0 Clock Select"]
    #[inline(always)]
    pub fn adc0clksel(&mut self) -> Adc0clkselW<'_, AdcctrlSpec> {
        Adc0clkselW::new(self, 4)
    }
    #[doc = "Bit 8 - Invert Clock Selected By ADC0CLKSEL"]
    #[inline(always)]
    pub fn adc0clkinv(&mut self) -> Adc0clkinvW<'_, AdcctrlSpec> {
        Adc0clkinvW::new(self, 8)
    }
    #[doc = "Bits 16:17 - ADC1 Clock Prescaler"]
    #[inline(always)]
    pub fn adc1clkdiv(&mut self) -> Adc1clkdivW<'_, AdcctrlSpec> {
        Adc1clkdivW::new(self, 16)
    }
    #[doc = "Bits 20:21 - ADC1 Clock Select"]
    #[inline(always)]
    pub fn adc1clksel(&mut self) -> Adc1clkselW<'_, AdcctrlSpec> {
        Adc1clkselW::new(self, 20)
    }
    #[doc = "Bit 24 - Invert Clock Selected By ADC1CLKSEL"]
    #[inline(always)]
    pub fn adc1clkinv(&mut self) -> Adc1clkinvW<'_, AdcctrlSpec> {
        Adc1clkinvW::new(self, 24)
    }
}
#[doc = "ADC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcctrlSpec;
impl crate::RegisterSpec for AdcctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcctrl::R`](R) reader structure"]
impl crate::Readable for AdcctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`adcctrl::W`](W) writer structure"]
impl crate::Writable for AdcctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCCTRL to value 0"]
impl crate::Resettable for AdcctrlSpec {}
