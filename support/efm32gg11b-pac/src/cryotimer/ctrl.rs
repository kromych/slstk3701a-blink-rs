#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `EN` reader - Enable CRYOTIMER"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Enable CRYOTIMER"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEBUGRUN` reader - Debug Mode Run Enable"]
pub type DebugrunR = crate::BitReader;
#[doc = "Field `DEBUGRUN` writer - Debug Mode Run Enable"]
pub type DebugrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Select Low Frequency Oscillator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Oscsel {
    #[doc = "0: Output is driven low"]
    Disabled = 0,
    #[doc = "1: Select Low Frequency RC Oscillator"]
    Lfrco = 1,
    #[doc = "2: Select Low Frequency Crystal Oscillator"]
    Lfxo = 2,
    #[doc = "3: Select Ultra Low Frequency RC Oscillator"]
    Ulfrco = 3,
}
impl From<Oscsel> for u8 {
    #[inline(always)]
    fn from(variant: Oscsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Oscsel {
    type Ux = u8;
}
impl crate::IsEnum for Oscsel {}
#[doc = "Field `OSCSEL` reader - Select Low Frequency Oscillator"]
pub type OscselR = crate::FieldReader<Oscsel>;
impl OscselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oscsel {
        match self.bits {
            0 => Oscsel::Disabled,
            1 => Oscsel::Lfrco,
            2 => Oscsel::Lfxo,
            3 => Oscsel::Ulfrco,
            _ => unreachable!(),
        }
    }
    #[doc = "Output is driven low"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Oscsel::Disabled
    }
    #[doc = "Select Low Frequency RC Oscillator"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == Oscsel::Lfrco
    }
    #[doc = "Select Low Frequency Crystal Oscillator"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == Oscsel::Lfxo
    }
    #[doc = "Select Ultra Low Frequency RC Oscillator"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == Oscsel::Ulfrco
    }
}
#[doc = "Field `OSCSEL` writer - Select Low Frequency Oscillator"]
pub type OscselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Oscsel, crate::Safe>;
impl<'a, REG> OscselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output is driven low"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Oscsel::Disabled)
    }
    #[doc = "Select Low Frequency RC Oscillator"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Oscsel::Lfrco)
    }
    #[doc = "Select Low Frequency Crystal Oscillator"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Oscsel::Lfxo)
    }
    #[doc = "Select Ultra Low Frequency RC Oscillator"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Oscsel::Ulfrco)
    }
}
#[doc = "Prescaler Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Presc {
    #[doc = "0: LF Oscillator frequency undivided"]
    Div1 = 0,
    #[doc = "1: LF Oscillator frequency divided by 2"]
    Div2 = 1,
    #[doc = "2: LF Oscillator frequency divided by 4"]
    Div4 = 2,
    #[doc = "3: LF Oscillator frequency divided by 8"]
    Div8 = 3,
    #[doc = "4: LF Oscillator frequency divided by 16"]
    Div16 = 4,
    #[doc = "5: LF Oscillator frequency divided by 32"]
    Div32 = 5,
    #[doc = "6: LF Oscillator frequency divided by 64"]
    Div64 = 6,
    #[doc = "7: LF Oscillator frequency divided by 128"]
    Div128 = 7,
}
impl From<Presc> for u8 {
    #[inline(always)]
    fn from(variant: Presc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Presc {
    type Ux = u8;
}
impl crate::IsEnum for Presc {}
#[doc = "Field `PRESC` reader - Prescaler Setting"]
pub type PrescR = crate::FieldReader<Presc>;
impl PrescR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Presc {
        match self.bits {
            0 => Presc::Div1,
            1 => Presc::Div2,
            2 => Presc::Div4,
            3 => Presc::Div8,
            4 => Presc::Div16,
            5 => Presc::Div32,
            6 => Presc::Div64,
            7 => Presc::Div128,
            _ => unreachable!(),
        }
    }
    #[doc = "LF Oscillator frequency undivided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Presc::Div1
    }
    #[doc = "LF Oscillator frequency divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Presc::Div2
    }
    #[doc = "LF Oscillator frequency divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Presc::Div4
    }
    #[doc = "LF Oscillator frequency divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Presc::Div8
    }
    #[doc = "LF Oscillator frequency divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Presc::Div16
    }
    #[doc = "LF Oscillator frequency divided by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Presc::Div32
    }
    #[doc = "LF Oscillator frequency divided by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Presc::Div64
    }
    #[doc = "LF Oscillator frequency divided by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Presc::Div128
    }
}
#[doc = "Field `PRESC` writer - Prescaler Setting"]
pub type PrescW<'a, REG> = crate::FieldWriter<'a, REG, 3, Presc, crate::Safe>;
impl<'a, REG> PrescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LF Oscillator frequency undivided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div1)
    }
    #[doc = "LF Oscillator frequency divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div2)
    }
    #[doc = "LF Oscillator frequency divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div4)
    }
    #[doc = "LF Oscillator frequency divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div8)
    }
    #[doc = "LF Oscillator frequency divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div16)
    }
    #[doc = "LF Oscillator frequency divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div32)
    }
    #[doc = "LF Oscillator frequency divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div64)
    }
    #[doc = "LF Oscillator frequency divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div128)
    }
}
impl R {
    #[doc = "Bit 0 - Enable CRYOTIMER"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&self) -> DebugrunR {
        DebugrunR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Select Low Frequency Oscillator"]
    #[inline(always)]
    pub fn oscsel(&self) -> OscselR {
        OscselR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 5:7 - Prescaler Setting"]
    #[inline(always)]
    pub fn presc(&self) -> PrescR {
        PrescR::new(((self.bits >> 5) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable CRYOTIMER"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, CtrlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&mut self) -> DebugrunW<'_, CtrlSpec> {
        DebugrunW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Select Low Frequency Oscillator"]
    #[inline(always)]
    pub fn oscsel(&mut self) -> OscselW<'_, CtrlSpec> {
        OscselW::new(self, 2)
    }
    #[doc = "Bits 5:7 - Prescaler Setting"]
    #[inline(always)]
    pub fn presc(&mut self) -> PrescW<'_, CtrlSpec> {
        PrescW::new(self, 5)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
