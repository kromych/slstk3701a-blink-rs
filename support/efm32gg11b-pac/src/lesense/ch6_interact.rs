#[doc = "Register `CH6_INTERACT` reader"]
pub type R = crate::R<CH6_INTERACT_SPEC>;
#[doc = "Register `CH6_INTERACT` writer"]
pub type W = crate::W<CH6_INTERACT_SPEC>;
#[doc = "Field `THRES` reader - ACMP Threshold or VDAC Data"]
pub type THRES_R = crate::FieldReader<u16>;
#[doc = "Field `THRES` writer - ACMP Threshold or VDAC Data"]
pub type THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SAMPLE` reader - Select Sample Mode"]
pub type SAMPLE_R = crate::FieldReader<SAMPLE_A>;
#[doc = "Select Sample Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAMPLE_A {
    #[doc = "0: Counter output will be used in evaluation"]
    ACMPCOUNT = 0,
    #[doc = "1: ACMP output will be used in evaluation"]
    ACMP = 1,
    #[doc = "2: ADC output will be used in evaluation"]
    ADC = 2,
    #[doc = "3: Differential ADC output will be used in evaluation"]
    ADCDIFF = 3,
}
impl From<SAMPLE_A> for u8 {
    #[inline(always)]
    fn from(variant: SAMPLE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SAMPLE_A {
    type Ux = u8;
}
impl SAMPLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SAMPLE_A {
        match self.bits {
            0 => SAMPLE_A::ACMPCOUNT,
            1 => SAMPLE_A::ACMP,
            2 => SAMPLE_A::ADC,
            3 => SAMPLE_A::ADCDIFF,
            _ => unreachable!(),
        }
    }
    #[doc = "Counter output will be used in evaluation"]
    #[inline(always)]
    pub fn is_acmpcount(&self) -> bool {
        *self == SAMPLE_A::ACMPCOUNT
    }
    #[doc = "ACMP output will be used in evaluation"]
    #[inline(always)]
    pub fn is_acmp(&self) -> bool {
        *self == SAMPLE_A::ACMP
    }
    #[doc = "ADC output will be used in evaluation"]
    #[inline(always)]
    pub fn is_adc(&self) -> bool {
        *self == SAMPLE_A::ADC
    }
    #[doc = "Differential ADC output will be used in evaluation"]
    #[inline(always)]
    pub fn is_adcdiff(&self) -> bool {
        *self == SAMPLE_A::ADCDIFF
    }
}
#[doc = "Field `SAMPLE` writer - Select Sample Mode"]
pub type SAMPLE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SAMPLE_A>;
impl<'a, REG> SAMPLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Counter output will be used in evaluation"]
    #[inline(always)]
    pub fn acmpcount(self) -> &'a mut crate::W<REG> {
        self.variant(SAMPLE_A::ACMPCOUNT)
    }
    #[doc = "ACMP output will be used in evaluation"]
    #[inline(always)]
    pub fn acmp(self) -> &'a mut crate::W<REG> {
        self.variant(SAMPLE_A::ACMP)
    }
    #[doc = "ADC output will be used in evaluation"]
    #[inline(always)]
    pub fn adc(self) -> &'a mut crate::W<REG> {
        self.variant(SAMPLE_A::ADC)
    }
    #[doc = "Differential ADC output will be used in evaluation"]
    #[inline(always)]
    pub fn adcdiff(self) -> &'a mut crate::W<REG> {
        self.variant(SAMPLE_A::ADCDIFF)
    }
}
#[doc = "Field `SETIF` reader - Enable Interrupt Generation"]
pub type SETIF_R = crate::FieldReader<SETIF_A>;
#[doc = "Enable Interrupt Generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SETIF_A {
    #[doc = "0: No interrupt is generated"]
    NONE = 0,
    #[doc = "1: Set interrupt flag if the sensor triggers."]
    LEVEL = 1,
    #[doc = "2: Set interrupt flag on positive edge of the sensor state"]
    POSEDGE = 2,
    #[doc = "3: Set interrupt flag on negative edge of the sensor state"]
    NEGEDGE = 3,
    #[doc = "4: Set interrupt flag on both edges of the sensor state"]
    BOTHEDGES = 4,
}
impl From<SETIF_A> for u8 {
    #[inline(always)]
    fn from(variant: SETIF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SETIF_A {
    type Ux = u8;
}
impl SETIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SETIF_A> {
        match self.bits {
            0 => Some(SETIF_A::NONE),
            1 => Some(SETIF_A::LEVEL),
            2 => Some(SETIF_A::POSEDGE),
            3 => Some(SETIF_A::NEGEDGE),
            4 => Some(SETIF_A::BOTHEDGES),
            _ => None,
        }
    }
    #[doc = "No interrupt is generated"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SETIF_A::NONE
    }
    #[doc = "Set interrupt flag if the sensor triggers."]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == SETIF_A::LEVEL
    }
    #[doc = "Set interrupt flag on positive edge of the sensor state"]
    #[inline(always)]
    pub fn is_posedge(&self) -> bool {
        *self == SETIF_A::POSEDGE
    }
    #[doc = "Set interrupt flag on negative edge of the sensor state"]
    #[inline(always)]
    pub fn is_negedge(&self) -> bool {
        *self == SETIF_A::NEGEDGE
    }
    #[doc = "Set interrupt flag on both edges of the sensor state"]
    #[inline(always)]
    pub fn is_bothedges(&self) -> bool {
        *self == SETIF_A::BOTHEDGES
    }
}
#[doc = "Field `SETIF` writer - Enable Interrupt Generation"]
pub type SETIF_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SETIF_A>;
impl<'a, REG> SETIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No interrupt is generated"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(SETIF_A::NONE)
    }
    #[doc = "Set interrupt flag if the sensor triggers."]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(SETIF_A::LEVEL)
    }
    #[doc = "Set interrupt flag on positive edge of the sensor state"]
    #[inline(always)]
    pub fn posedge(self) -> &'a mut crate::W<REG> {
        self.variant(SETIF_A::POSEDGE)
    }
    #[doc = "Set interrupt flag on negative edge of the sensor state"]
    #[inline(always)]
    pub fn negedge(self) -> &'a mut crate::W<REG> {
        self.variant(SETIF_A::NEGEDGE)
    }
    #[doc = "Set interrupt flag on both edges of the sensor state"]
    #[inline(always)]
    pub fn bothedges(self) -> &'a mut crate::W<REG> {
        self.variant(SETIF_A::BOTHEDGES)
    }
}
#[doc = "Field `EXMODE` reader - Set GPIO Mode"]
pub type EXMODE_R = crate::FieldReader<EXMODE_A>;
#[doc = "Set GPIO Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXMODE_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Push Pull, GPIO is driven high"]
    HIGH = 1,
    #[doc = "2: Push Pull, GPIO is driven low"]
    LOW = 2,
    #[doc = "3: VDAC output"]
    DACOUT = 3,
}
impl From<EXMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: EXMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXMODE_A {
    type Ux = u8;
}
impl EXMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXMODE_A {
        match self.bits {
            0 => EXMODE_A::DISABLE,
            1 => EXMODE_A::HIGH,
            2 => EXMODE_A::LOW,
            3 => EXMODE_A::DACOUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EXMODE_A::DISABLE
    }
    #[doc = "Push Pull, GPIO is driven high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == EXMODE_A::HIGH
    }
    #[doc = "Push Pull, GPIO is driven low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == EXMODE_A::LOW
    }
    #[doc = "VDAC output"]
    #[inline(always)]
    pub fn is_dacout(&self) -> bool {
        *self == EXMODE_A::DACOUT
    }
}
#[doc = "Field `EXMODE` writer - Set GPIO Mode"]
pub type EXMODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, EXMODE_A>;
impl<'a, REG> EXMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EXMODE_A::DISABLE)
    }
    #[doc = "Push Pull, GPIO is driven high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(EXMODE_A::HIGH)
    }
    #[doc = "Push Pull, GPIO is driven low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(EXMODE_A::LOW)
    }
    #[doc = "VDAC output"]
    #[inline(always)]
    pub fn dacout(self) -> &'a mut crate::W<REG> {
        self.variant(EXMODE_A::DACOUT)
    }
}
#[doc = "Field `EXCLK` reader - Select Clock Used for Excitation Timing"]
pub type EXCLK_R = crate::BitReader;
#[doc = "Field `EXCLK` writer - Select Clock Used for Excitation Timing"]
pub type EXCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAMPLECLK` reader - Select Clock Used for Timing of Sample Delay"]
pub type SAMPLECLK_R = crate::BitReader;
#[doc = "Field `SAMPLECLK` writer - Select Clock Used for Timing of Sample Delay"]
pub type SAMPLECLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALTEX` reader - Use Alternative Excite Pin"]
pub type ALTEX_R = crate::BitReader;
#[doc = "Field `ALTEX` writer - Use Alternative Excite Pin"]
pub type ALTEX_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - ACMP Threshold or VDAC Data"]
    #[inline(always)]
    pub fn thres(&self) -> THRES_R {
        THRES_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:13 - Select Sample Mode"]
    #[inline(always)]
    pub fn sample(&self) -> SAMPLE_R {
        SAMPLE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:16 - Enable Interrupt Generation"]
    #[inline(always)]
    pub fn setif(&self) -> SETIF_R {
        SETIF_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bits 17:18 - Set GPIO Mode"]
    #[inline(always)]
    pub fn exmode(&self) -> EXMODE_R {
        EXMODE_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - Select Clock Used for Excitation Timing"]
    #[inline(always)]
    pub fn exclk(&self) -> EXCLK_R {
        EXCLK_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Select Clock Used for Timing of Sample Delay"]
    #[inline(always)]
    pub fn sampleclk(&self) -> SAMPLECLK_R {
        SAMPLECLK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Use Alternative Excite Pin"]
    #[inline(always)]
    pub fn altex(&self) -> ALTEX_R {
        ALTEX_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - ACMP Threshold or VDAC Data"]
    #[inline(always)]
    #[must_use]
    pub fn thres(&mut self) -> THRES_W<CH6_INTERACT_SPEC> {
        THRES_W::new(self, 0)
    }
    #[doc = "Bits 12:13 - Select Sample Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sample(&mut self) -> SAMPLE_W<CH6_INTERACT_SPEC> {
        SAMPLE_W::new(self, 12)
    }
    #[doc = "Bits 14:16 - Enable Interrupt Generation"]
    #[inline(always)]
    #[must_use]
    pub fn setif(&mut self) -> SETIF_W<CH6_INTERACT_SPEC> {
        SETIF_W::new(self, 14)
    }
    #[doc = "Bits 17:18 - Set GPIO Mode"]
    #[inline(always)]
    #[must_use]
    pub fn exmode(&mut self) -> EXMODE_W<CH6_INTERACT_SPEC> {
        EXMODE_W::new(self, 17)
    }
    #[doc = "Bit 19 - Select Clock Used for Excitation Timing"]
    #[inline(always)]
    #[must_use]
    pub fn exclk(&mut self) -> EXCLK_W<CH6_INTERACT_SPEC> {
        EXCLK_W::new(self, 19)
    }
    #[doc = "Bit 20 - Select Clock Used for Timing of Sample Delay"]
    #[inline(always)]
    #[must_use]
    pub fn sampleclk(&mut self) -> SAMPLECLK_W<CH6_INTERACT_SPEC> {
        SAMPLECLK_W::new(self, 20)
    }
    #[doc = "Bit 21 - Use Alternative Excite Pin"]
    #[inline(always)]
    #[must_use]
    pub fn altex(&mut self) -> ALTEX_W<CH6_INTERACT_SPEC> {
        ALTEX_W::new(self, 21)
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
#[doc = "Scan Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6_interact::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6_interact::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH6_INTERACT_SPEC;
impl crate::RegisterSpec for CH6_INTERACT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch6_interact::R`](R) reader structure"]
impl crate::Readable for CH6_INTERACT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch6_interact::W`](W) writer structure"]
impl crate::Writable for CH6_INTERACT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH6_INTERACT to value 0"]
impl crate::Resettable for CH6_INTERACT_SPEC {
    const RESET_VALUE: u32 = 0;
}
