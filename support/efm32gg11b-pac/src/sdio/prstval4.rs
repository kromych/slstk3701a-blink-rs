#[doc = "Register `PRSTVAL4` reader"]
pub type R = crate::R<Prstval4Spec>;
#[doc = "Field `SDR25SDCLKFREQVAL` reader - SD_CLK Frequency Select Value for SDR25"]
pub type Sdr25sdclkfreqvalR = crate::FieldReader<u16>;
#[doc = "Field `SDR25CLKGENVAL` reader - Clock Generator Select Value for SDR25"]
pub type Sdr25clkgenvalR = crate::BitReader;
#[doc = "Driver Strength Select Value for SDR25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sdr25drvstval {
    #[doc = "0: Driver Type B is selected (Default)"]
    Typeb = 0,
    #[doc = "1: Driver Type A is selected"]
    Typea = 1,
    #[doc = "2: Driver Type C is selected"]
    Typec = 2,
    #[doc = "3: Driver Type D is selected"]
    Typed = 3,
}
impl From<Sdr25drvstval> for u8 {
    #[inline(always)]
    fn from(variant: Sdr25drvstval) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sdr25drvstval {
    type Ux = u8;
}
impl crate::IsEnum for Sdr25drvstval {}
#[doc = "Field `SDR25DRVSTVAL` reader - Driver Strength Select Value for SDR25"]
pub type Sdr25drvstvalR = crate::FieldReader<Sdr25drvstval>;
impl Sdr25drvstvalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdr25drvstval {
        match self.bits {
            0 => Sdr25drvstval::Typeb,
            1 => Sdr25drvstval::Typea,
            2 => Sdr25drvstval::Typec,
            3 => Sdr25drvstval::Typed,
            _ => unreachable!(),
        }
    }
    #[doc = "Driver Type B is selected (Default)"]
    #[inline(always)]
    pub fn is_typeb(&self) -> bool {
        *self == Sdr25drvstval::Typeb
    }
    #[doc = "Driver Type A is selected"]
    #[inline(always)]
    pub fn is_typea(&self) -> bool {
        *self == Sdr25drvstval::Typea
    }
    #[doc = "Driver Type C is selected"]
    #[inline(always)]
    pub fn is_typec(&self) -> bool {
        *self == Sdr25drvstval::Typec
    }
    #[doc = "Driver Type D is selected"]
    #[inline(always)]
    pub fn is_typed(&self) -> bool {
        *self == Sdr25drvstval::Typed
    }
}
#[doc = "Field `SDR50SDCLKFREQVAL` reader - SD_CLK Frequency Select Value for SDR50"]
pub type Sdr50sdclkfreqvalR = crate::FieldReader<u16>;
#[doc = "Field `SDR50CLCKGENVAL` reader - Clock Generator Select Value for SDR50"]
pub type Sdr50clckgenvalR = crate::BitReader;
#[doc = "Driver Strength Select Value for SDR50\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sdr50drvstval {
    #[doc = "0: Driver Type B is selected (Default)"]
    Typeb = 0,
    #[doc = "1: Driver Type A is selected"]
    Typea = 1,
    #[doc = "2: Driver Type C is selected"]
    Typec = 2,
    #[doc = "3: Driver Type D is selected"]
    Typed = 3,
}
impl From<Sdr50drvstval> for u8 {
    #[inline(always)]
    fn from(variant: Sdr50drvstval) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sdr50drvstval {
    type Ux = u8;
}
impl crate::IsEnum for Sdr50drvstval {}
#[doc = "Field `SDR50DRVSTVAL` reader - Driver Strength Select Value for SDR50"]
pub type Sdr50drvstvalR = crate::FieldReader<Sdr50drvstval>;
impl Sdr50drvstvalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdr50drvstval {
        match self.bits {
            0 => Sdr50drvstval::Typeb,
            1 => Sdr50drvstval::Typea,
            2 => Sdr50drvstval::Typec,
            3 => Sdr50drvstval::Typed,
            _ => unreachable!(),
        }
    }
    #[doc = "Driver Type B is selected (Default)"]
    #[inline(always)]
    pub fn is_typeb(&self) -> bool {
        *self == Sdr50drvstval::Typeb
    }
    #[doc = "Driver Type A is selected"]
    #[inline(always)]
    pub fn is_typea(&self) -> bool {
        *self == Sdr50drvstval::Typea
    }
    #[doc = "Driver Type C is selected"]
    #[inline(always)]
    pub fn is_typec(&self) -> bool {
        *self == Sdr50drvstval::Typec
    }
    #[doc = "Driver Type D is selected"]
    #[inline(always)]
    pub fn is_typed(&self) -> bool {
        *self == Sdr50drvstval::Typed
    }
}
impl R {
    #[doc = "Bits 0:9 - SD_CLK Frequency Select Value for SDR25"]
    #[inline(always)]
    pub fn sdr25sdclkfreqval(&self) -> Sdr25sdclkfreqvalR {
        Sdr25sdclkfreqvalR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Clock Generator Select Value for SDR25"]
    #[inline(always)]
    pub fn sdr25clkgenval(&self) -> Sdr25clkgenvalR {
        Sdr25clkgenvalR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Driver Strength Select Value for SDR25"]
    #[inline(always)]
    pub fn sdr25drvstval(&self) -> Sdr25drvstvalR {
        Sdr25drvstvalR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:25 - SD_CLK Frequency Select Value for SDR50"]
    #[inline(always)]
    pub fn sdr50sdclkfreqval(&self) -> Sdr50sdclkfreqvalR {
        Sdr50sdclkfreqvalR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - Clock Generator Select Value for SDR50"]
    #[inline(always)]
    pub fn sdr50clckgenval(&self) -> Sdr50clckgenvalR {
        Sdr50clckgenvalR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Driver Strength Select Value for SDR50"]
    #[inline(always)]
    pub fn sdr50drvstval(&self) -> Sdr50drvstvalR {
        Sdr50drvstvalR::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "Preset Value for SDR25 and SDR50 Modes\n\nYou can [`read`](crate::Reg::read) this register and get [`prstval4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Prstval4Spec;
impl crate::RegisterSpec for Prstval4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prstval4::R`](R) reader structure"]
impl crate::Readable for Prstval4Spec {}
#[doc = "`reset()` method sets PRSTVAL4 to value 0"]
impl crate::Resettable for Prstval4Spec {}
