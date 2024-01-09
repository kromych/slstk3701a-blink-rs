#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `EN` reader - Enable CRYOTIMER"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Enable CRYOTIMER"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEBUGRUN` reader - Debug Mode Run Enable"]
pub type DEBUGRUN_R = crate::BitReader;
#[doc = "Field `DEBUGRUN` writer - Debug Mode Run Enable"]
pub type DEBUGRUN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCSEL` reader - Select Low Frequency Oscillator"]
pub type OSCSEL_R = crate::FieldReader<OSCSEL_A>;
#[doc = "Select Low Frequency Oscillator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSCSEL_A {
    #[doc = "0: Output is driven low"]
    DISABLED = 0,
    #[doc = "1: Select Low Frequency RC Oscillator"]
    LFRCO = 1,
    #[doc = "2: Select Low Frequency Crystal Oscillator"]
    LFXO = 2,
    #[doc = "3: Select Ultra Low Frequency RC Oscillator"]
    ULFRCO = 3,
}
impl From<OSCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: OSCSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSCSEL_A {
    type Ux = u8;
}
impl OSCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSCSEL_A {
        match self.bits {
            0 => OSCSEL_A::DISABLED,
            1 => OSCSEL_A::LFRCO,
            2 => OSCSEL_A::LFXO,
            3 => OSCSEL_A::ULFRCO,
            _ => unreachable!(),
        }
    }
    #[doc = "Output is driven low"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSCSEL_A::DISABLED
    }
    #[doc = "Select Low Frequency RC Oscillator"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == OSCSEL_A::LFRCO
    }
    #[doc = "Select Low Frequency Crystal Oscillator"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == OSCSEL_A::LFXO
    }
    #[doc = "Select Ultra Low Frequency RC Oscillator"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == OSCSEL_A::ULFRCO
    }
}
#[doc = "Field `OSCSEL` writer - Select Low Frequency Oscillator"]
pub type OSCSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, OSCSEL_A>;
impl<'a, REG> OSCSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output is driven low"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OSCSEL_A::DISABLED)
    }
    #[doc = "Select Low Frequency RC Oscillator"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(OSCSEL_A::LFRCO)
    }
    #[doc = "Select Low Frequency Crystal Oscillator"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(OSCSEL_A::LFXO)
    }
    #[doc = "Select Ultra Low Frequency RC Oscillator"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut crate::W<REG> {
        self.variant(OSCSEL_A::ULFRCO)
    }
}
#[doc = "Field `PRESC` reader - Prescaler Setting"]
pub type PRESC_R = crate::FieldReader<PRESC_A>;
#[doc = "Prescaler Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESC_A {
    #[doc = "0: LF Oscillator frequency undivided"]
    DIV1 = 0,
    #[doc = "1: LF Oscillator frequency divided by 2"]
    DIV2 = 1,
    #[doc = "2: LF Oscillator frequency divided by 4"]
    DIV4 = 2,
    #[doc = "3: LF Oscillator frequency divided by 8"]
    DIV8 = 3,
    #[doc = "4: LF Oscillator frequency divided by 16"]
    DIV16 = 4,
    #[doc = "5: LF Oscillator frequency divided by 32"]
    DIV32 = 5,
    #[doc = "6: LF Oscillator frequency divided by 64"]
    DIV64 = 6,
    #[doc = "7: LF Oscillator frequency divided by 128"]
    DIV128 = 7,
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
    pub const fn variant(&self) -> PRESC_A {
        match self.bits {
            0 => PRESC_A::DIV1,
            1 => PRESC_A::DIV2,
            2 => PRESC_A::DIV4,
            3 => PRESC_A::DIV8,
            4 => PRESC_A::DIV16,
            5 => PRESC_A::DIV32,
            6 => PRESC_A::DIV64,
            7 => PRESC_A::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "LF Oscillator frequency undivided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PRESC_A::DIV1
    }
    #[doc = "LF Oscillator frequency divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESC_A::DIV2
    }
    #[doc = "LF Oscillator frequency divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESC_A::DIV4
    }
    #[doc = "LF Oscillator frequency divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESC_A::DIV8
    }
    #[doc = "LF Oscillator frequency divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESC_A::DIV16
    }
    #[doc = "LF Oscillator frequency divided by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PRESC_A::DIV32
    }
    #[doc = "LF Oscillator frequency divided by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESC_A::DIV64
    }
    #[doc = "LF Oscillator frequency divided by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PRESC_A::DIV128
    }
}
#[doc = "Field `PRESC` writer - Prescaler Setting"]
pub type PRESC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, PRESC_A>;
impl<'a, REG> PRESC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LF Oscillator frequency undivided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::DIV1)
    }
    #[doc = "LF Oscillator frequency divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::DIV2)
    }
    #[doc = "LF Oscillator frequency divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::DIV4)
    }
    #[doc = "LF Oscillator frequency divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::DIV8)
    }
    #[doc = "LF Oscillator frequency divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::DIV16)
    }
    #[doc = "LF Oscillator frequency divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::DIV32)
    }
    #[doc = "LF Oscillator frequency divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::DIV64)
    }
    #[doc = "LF Oscillator frequency divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::DIV128)
    }
}
impl R {
    #[doc = "Bit 0 - Enable CRYOTIMER"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&self) -> DEBUGRUN_R {
        DEBUGRUN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Select Low Frequency Oscillator"]
    #[inline(always)]
    pub fn oscsel(&self) -> OSCSEL_R {
        OSCSEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 5:7 - Prescaler Setting"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 5) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable CRYOTIMER"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CTRL_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Debug Mode Run Enable"]
    #[inline(always)]
    #[must_use]
    pub fn debugrun(&mut self) -> DEBUGRUN_W<CTRL_SPEC> {
        DEBUGRUN_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Select Low Frequency Oscillator"]
    #[inline(always)]
    #[must_use]
    pub fn oscsel(&mut self) -> OSCSEL_W<CTRL_SPEC> {
        OSCSEL_W::new(self, 2)
    }
    #[doc = "Bits 5:7 - Prescaler Setting"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<CTRL_SPEC> {
        PRESC_W::new(self, 5)
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
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
