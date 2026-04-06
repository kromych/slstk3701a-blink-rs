#[doc = "Register `PRSTVAL6` reader"]
pub type R = crate::R<Prstval6Spec>;
#[doc = "Field `SDR104SDCLKFREQVAL` reader - SD_CLK Frequency Select Value for SDR104"]
pub type Sdr104sdclkfreqvalR = crate::FieldReader<u16>;
#[doc = "Field `SDR104CLKGENVAL` reader - Clock Generator Select Value for SDR104"]
pub type Sdr104clkgenvalR = crate::BitReader;
#[doc = "Driver Strength Select Value for SDR104\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sdr104drvstval {
    #[doc = "0: Driver Type B is selected (Default)"]
    Typeb = 0,
    #[doc = "1: Driver Type A is selected"]
    Typea = 1,
    #[doc = "2: Driver Type C is selected"]
    Typec = 2,
    #[doc = "3: Driver Type D is selected"]
    Typed = 3,
}
impl From<Sdr104drvstval> for u8 {
    #[inline(always)]
    fn from(variant: Sdr104drvstval) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sdr104drvstval {
    type Ux = u8;
}
impl crate::IsEnum for Sdr104drvstval {}
#[doc = "Field `SDR104DRVSTVAL` reader - Driver Strength Select Value for SDR104"]
pub type Sdr104drvstvalR = crate::FieldReader<Sdr104drvstval>;
impl Sdr104drvstvalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdr104drvstval {
        match self.bits {
            0 => Sdr104drvstval::Typeb,
            1 => Sdr104drvstval::Typea,
            2 => Sdr104drvstval::Typec,
            3 => Sdr104drvstval::Typed,
            _ => unreachable!(),
        }
    }
    #[doc = "Driver Type B is selected (Default)"]
    #[inline(always)]
    pub fn is_typeb(&self) -> bool {
        *self == Sdr104drvstval::Typeb
    }
    #[doc = "Driver Type A is selected"]
    #[inline(always)]
    pub fn is_typea(&self) -> bool {
        *self == Sdr104drvstval::Typea
    }
    #[doc = "Driver Type C is selected"]
    #[inline(always)]
    pub fn is_typec(&self) -> bool {
        *self == Sdr104drvstval::Typec
    }
    #[doc = "Driver Type D is selected"]
    #[inline(always)]
    pub fn is_typed(&self) -> bool {
        *self == Sdr104drvstval::Typed
    }
}
#[doc = "Field `DDR50SDCLKFREQVAL` reader - SD_CLK Frequency Select Value for DDR50"]
pub type Ddr50sdclkfreqvalR = crate::FieldReader<u16>;
#[doc = "Field `DDR50CLKGENVAL` reader - Clock Generator Select Value for DDR50"]
pub type Ddr50clkgenvalR = crate::BitReader;
#[doc = "Driver Strength Select Value for DDR50\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ddr50drvstval {
    #[doc = "0: Driver Type B is selected (Default)"]
    Typeb = 0,
    #[doc = "1: Driver Type A is selected"]
    Typea = 1,
    #[doc = "2: Driver Type C is selected"]
    Typec = 2,
    #[doc = "3: Driver Type D is selected"]
    Typed = 3,
}
impl From<Ddr50drvstval> for u8 {
    #[inline(always)]
    fn from(variant: Ddr50drvstval) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ddr50drvstval {
    type Ux = u8;
}
impl crate::IsEnum for Ddr50drvstval {}
#[doc = "Field `DDR50DRVSTVAL` reader - Driver Strength Select Value for DDR50"]
pub type Ddr50drvstvalR = crate::FieldReader<Ddr50drvstval>;
impl Ddr50drvstvalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ddr50drvstval {
        match self.bits {
            0 => Ddr50drvstval::Typeb,
            1 => Ddr50drvstval::Typea,
            2 => Ddr50drvstval::Typec,
            3 => Ddr50drvstval::Typed,
            _ => unreachable!(),
        }
    }
    #[doc = "Driver Type B is selected (Default)"]
    #[inline(always)]
    pub fn is_typeb(&self) -> bool {
        *self == Ddr50drvstval::Typeb
    }
    #[doc = "Driver Type A is selected"]
    #[inline(always)]
    pub fn is_typea(&self) -> bool {
        *self == Ddr50drvstval::Typea
    }
    #[doc = "Driver Type C is selected"]
    #[inline(always)]
    pub fn is_typec(&self) -> bool {
        *self == Ddr50drvstval::Typec
    }
    #[doc = "Driver Type D is selected"]
    #[inline(always)]
    pub fn is_typed(&self) -> bool {
        *self == Ddr50drvstval::Typed
    }
}
impl R {
    #[doc = "Bits 0:9 - SD_CLK Frequency Select Value for SDR104"]
    #[inline(always)]
    pub fn sdr104sdclkfreqval(&self) -> Sdr104sdclkfreqvalR {
        Sdr104sdclkfreqvalR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Clock Generator Select Value for SDR104"]
    #[inline(always)]
    pub fn sdr104clkgenval(&self) -> Sdr104clkgenvalR {
        Sdr104clkgenvalR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Driver Strength Select Value for SDR104"]
    #[inline(always)]
    pub fn sdr104drvstval(&self) -> Sdr104drvstvalR {
        Sdr104drvstvalR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:25 - SD_CLK Frequency Select Value for DDR50"]
    #[inline(always)]
    pub fn ddr50sdclkfreqval(&self) -> Ddr50sdclkfreqvalR {
        Ddr50sdclkfreqvalR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - Clock Generator Select Value for DDR50"]
    #[inline(always)]
    pub fn ddr50clkgenval(&self) -> Ddr50clkgenvalR {
        Ddr50clkgenvalR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Driver Strength Select Value for DDR50"]
    #[inline(always)]
    pub fn ddr50drvstval(&self) -> Ddr50drvstvalR {
        Ddr50drvstvalR::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "Preset Value for SDR104 and DDR50 Modes\n\nYou can [`read`](crate::Reg::read) this register and get [`prstval6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Prstval6Spec;
impl crate::RegisterSpec for Prstval6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prstval6::R`](R) reader structure"]
impl crate::Readable for Prstval6Spec {}
#[doc = "`reset()` method sets PRSTVAL6 to value 0"]
impl crate::Resettable for Prstval6Spec {}
