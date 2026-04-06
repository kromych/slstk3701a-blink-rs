#[doc = "Register `CH11_INTERACT` reader"]
pub type R = crate::R<Ch11InteractSpec>;
#[doc = "Register `CH11_INTERACT` writer"]
pub type W = crate::W<Ch11InteractSpec>;
#[doc = "Field `THRES` reader - ACMP Threshold or VDAC Data"]
pub type ThresR = crate::FieldReader<u16>;
#[doc = "Field `THRES` writer - ACMP Threshold or VDAC Data"]
pub type ThresW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Select Sample Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sample {
    #[doc = "0: Counter output will be used in evaluation"]
    Acmpcount = 0,
    #[doc = "1: ACMP output will be used in evaluation"]
    Acmp = 1,
    #[doc = "2: ADC output will be used in evaluation"]
    Adc = 2,
    #[doc = "3: Differential ADC output will be used in evaluation"]
    Adcdiff = 3,
}
impl From<Sample> for u8 {
    #[inline(always)]
    fn from(variant: Sample) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sample {
    type Ux = u8;
}
impl crate::IsEnum for Sample {}
#[doc = "Field `SAMPLE` reader - Select Sample Mode"]
pub type SampleR = crate::FieldReader<Sample>;
impl SampleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sample {
        match self.bits {
            0 => Sample::Acmpcount,
            1 => Sample::Acmp,
            2 => Sample::Adc,
            3 => Sample::Adcdiff,
            _ => unreachable!(),
        }
    }
    #[doc = "Counter output will be used in evaluation"]
    #[inline(always)]
    pub fn is_acmpcount(&self) -> bool {
        *self == Sample::Acmpcount
    }
    #[doc = "ACMP output will be used in evaluation"]
    #[inline(always)]
    pub fn is_acmp(&self) -> bool {
        *self == Sample::Acmp
    }
    #[doc = "ADC output will be used in evaluation"]
    #[inline(always)]
    pub fn is_adc(&self) -> bool {
        *self == Sample::Adc
    }
    #[doc = "Differential ADC output will be used in evaluation"]
    #[inline(always)]
    pub fn is_adcdiff(&self) -> bool {
        *self == Sample::Adcdiff
    }
}
#[doc = "Field `SAMPLE` writer - Select Sample Mode"]
pub type SampleW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sample, crate::Safe>;
impl<'a, REG> SampleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Counter output will be used in evaluation"]
    #[inline(always)]
    pub fn acmpcount(self) -> &'a mut crate::W<REG> {
        self.variant(Sample::Acmpcount)
    }
    #[doc = "ACMP output will be used in evaluation"]
    #[inline(always)]
    pub fn acmp(self) -> &'a mut crate::W<REG> {
        self.variant(Sample::Acmp)
    }
    #[doc = "ADC output will be used in evaluation"]
    #[inline(always)]
    pub fn adc(self) -> &'a mut crate::W<REG> {
        self.variant(Sample::Adc)
    }
    #[doc = "Differential ADC output will be used in evaluation"]
    #[inline(always)]
    pub fn adcdiff(self) -> &'a mut crate::W<REG> {
        self.variant(Sample::Adcdiff)
    }
}
#[doc = "Enable Interrupt Generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Setif {
    #[doc = "0: No interrupt is generated"]
    None = 0,
    #[doc = "1: Set interrupt flag if the sensor triggers."]
    Level = 1,
    #[doc = "2: Set interrupt flag on positive edge of the sensor state"]
    Posedge = 2,
    #[doc = "3: Set interrupt flag on negative edge of the sensor state"]
    Negedge = 3,
    #[doc = "4: Set interrupt flag on both edges of the sensor state"]
    Bothedges = 4,
}
impl From<Setif> for u8 {
    #[inline(always)]
    fn from(variant: Setif) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Setif {
    type Ux = u8;
}
impl crate::IsEnum for Setif {}
#[doc = "Field `SETIF` reader - Enable Interrupt Generation"]
pub type SetifR = crate::FieldReader<Setif>;
impl SetifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Setif> {
        match self.bits {
            0 => Some(Setif::None),
            1 => Some(Setif::Level),
            2 => Some(Setif::Posedge),
            3 => Some(Setif::Negedge),
            4 => Some(Setif::Bothedges),
            _ => None,
        }
    }
    #[doc = "No interrupt is generated"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Setif::None
    }
    #[doc = "Set interrupt flag if the sensor triggers."]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Setif::Level
    }
    #[doc = "Set interrupt flag on positive edge of the sensor state"]
    #[inline(always)]
    pub fn is_posedge(&self) -> bool {
        *self == Setif::Posedge
    }
    #[doc = "Set interrupt flag on negative edge of the sensor state"]
    #[inline(always)]
    pub fn is_negedge(&self) -> bool {
        *self == Setif::Negedge
    }
    #[doc = "Set interrupt flag on both edges of the sensor state"]
    #[inline(always)]
    pub fn is_bothedges(&self) -> bool {
        *self == Setif::Bothedges
    }
}
#[doc = "Field `SETIF` writer - Enable Interrupt Generation"]
pub type SetifW<'a, REG> = crate::FieldWriter<'a, REG, 3, Setif>;
impl<'a, REG> SetifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No interrupt is generated"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Setif::None)
    }
    #[doc = "Set interrupt flag if the sensor triggers."]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Setif::Level)
    }
    #[doc = "Set interrupt flag on positive edge of the sensor state"]
    #[inline(always)]
    pub fn posedge(self) -> &'a mut crate::W<REG> {
        self.variant(Setif::Posedge)
    }
    #[doc = "Set interrupt flag on negative edge of the sensor state"]
    #[inline(always)]
    pub fn negedge(self) -> &'a mut crate::W<REG> {
        self.variant(Setif::Negedge)
    }
    #[doc = "Set interrupt flag on both edges of the sensor state"]
    #[inline(always)]
    pub fn bothedges(self) -> &'a mut crate::W<REG> {
        self.variant(Setif::Bothedges)
    }
}
#[doc = "Set GPIO Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exmode {
    #[doc = "0: Disabled"]
    Disable = 0,
    #[doc = "1: Push Pull, GPIO is driven high"]
    High = 1,
    #[doc = "2: Push Pull, GPIO is driven low"]
    Low = 2,
    #[doc = "3: VDAC output"]
    Dacout = 3,
}
impl From<Exmode> for u8 {
    #[inline(always)]
    fn from(variant: Exmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exmode {
    type Ux = u8;
}
impl crate::IsEnum for Exmode {}
#[doc = "Field `EXMODE` reader - Set GPIO Mode"]
pub type ExmodeR = crate::FieldReader<Exmode>;
impl ExmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exmode {
        match self.bits {
            0 => Exmode::Disable,
            1 => Exmode::High,
            2 => Exmode::Low,
            3 => Exmode::Dacout,
            _ => unreachable!(),
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Exmode::Disable
    }
    #[doc = "Push Pull, GPIO is driven high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Exmode::High
    }
    #[doc = "Push Pull, GPIO is driven low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Exmode::Low
    }
    #[doc = "VDAC output"]
    #[inline(always)]
    pub fn is_dacout(&self) -> bool {
        *self == Exmode::Dacout
    }
}
#[doc = "Field `EXMODE` writer - Set GPIO Mode"]
pub type ExmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Exmode, crate::Safe>;
impl<'a, REG> ExmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Exmode::Disable)
    }
    #[doc = "Push Pull, GPIO is driven high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Exmode::High)
    }
    #[doc = "Push Pull, GPIO is driven low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Exmode::Low)
    }
    #[doc = "VDAC output"]
    #[inline(always)]
    pub fn dacout(self) -> &'a mut crate::W<REG> {
        self.variant(Exmode::Dacout)
    }
}
#[doc = "Field `EXCLK` reader - Select Clock Used for Excitation Timing"]
pub type ExclkR = crate::BitReader;
#[doc = "Field `EXCLK` writer - Select Clock Used for Excitation Timing"]
pub type ExclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAMPLECLK` reader - Select Clock Used for Timing of Sample Delay"]
pub type SampleclkR = crate::BitReader;
#[doc = "Field `SAMPLECLK` writer - Select Clock Used for Timing of Sample Delay"]
pub type SampleclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALTEX` reader - Use Alternative Excite Pin"]
pub type AltexR = crate::BitReader;
#[doc = "Field `ALTEX` writer - Use Alternative Excite Pin"]
pub type AltexW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - ACMP Threshold or VDAC Data"]
    #[inline(always)]
    pub fn thres(&self) -> ThresR {
        ThresR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:13 - Select Sample Mode"]
    #[inline(always)]
    pub fn sample(&self) -> SampleR {
        SampleR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:16 - Enable Interrupt Generation"]
    #[inline(always)]
    pub fn setif(&self) -> SetifR {
        SetifR::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bits 17:18 - Set GPIO Mode"]
    #[inline(always)]
    pub fn exmode(&self) -> ExmodeR {
        ExmodeR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - Select Clock Used for Excitation Timing"]
    #[inline(always)]
    pub fn exclk(&self) -> ExclkR {
        ExclkR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Select Clock Used for Timing of Sample Delay"]
    #[inline(always)]
    pub fn sampleclk(&self) -> SampleclkR {
        SampleclkR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Use Alternative Excite Pin"]
    #[inline(always)]
    pub fn altex(&self) -> AltexR {
        AltexR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - ACMP Threshold or VDAC Data"]
    #[inline(always)]
    pub fn thres(&mut self) -> ThresW<'_, Ch11InteractSpec> {
        ThresW::new(self, 0)
    }
    #[doc = "Bits 12:13 - Select Sample Mode"]
    #[inline(always)]
    pub fn sample(&mut self) -> SampleW<'_, Ch11InteractSpec> {
        SampleW::new(self, 12)
    }
    #[doc = "Bits 14:16 - Enable Interrupt Generation"]
    #[inline(always)]
    pub fn setif(&mut self) -> SetifW<'_, Ch11InteractSpec> {
        SetifW::new(self, 14)
    }
    #[doc = "Bits 17:18 - Set GPIO Mode"]
    #[inline(always)]
    pub fn exmode(&mut self) -> ExmodeW<'_, Ch11InteractSpec> {
        ExmodeW::new(self, 17)
    }
    #[doc = "Bit 19 - Select Clock Used for Excitation Timing"]
    #[inline(always)]
    pub fn exclk(&mut self) -> ExclkW<'_, Ch11InteractSpec> {
        ExclkW::new(self, 19)
    }
    #[doc = "Bit 20 - Select Clock Used for Timing of Sample Delay"]
    #[inline(always)]
    pub fn sampleclk(&mut self) -> SampleclkW<'_, Ch11InteractSpec> {
        SampleclkW::new(self, 20)
    }
    #[doc = "Bit 21 - Use Alternative Excite Pin"]
    #[inline(always)]
    pub fn altex(&mut self) -> AltexW<'_, Ch11InteractSpec> {
        AltexW::new(self, 21)
    }
}
#[doc = "Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch11_interact::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch11_interact::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch11InteractSpec;
impl crate::RegisterSpec for Ch11InteractSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch11_interact::R`](R) reader structure"]
impl crate::Readable for Ch11InteractSpec {}
#[doc = "`write(|w| ..)` method takes [`ch11_interact::W`](W) writer structure"]
impl crate::Writable for Ch11InteractSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH11_INTERACT to value 0"]
impl crate::Resettable for Ch11InteractSpec {}
