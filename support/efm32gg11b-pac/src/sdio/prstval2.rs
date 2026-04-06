#[doc = "Register `PRSTVAL2` reader"]
pub type R = crate::R<Prstval2Spec>;
#[doc = "Field `HSPSDCLKFREQVAL` reader - SD_CLK Frequency Select Value for High Speed"]
pub type HspsdclkfreqvalR = crate::FieldReader<u16>;
#[doc = "Field `HSPCLKGENVAL` reader - Clock Generator Select Value for High Speed"]
pub type HspclkgenvalR = crate::BitReader;
#[doc = "Driver Strength Select Value for High Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hspdrvstval {
    #[doc = "0: Driver Type B is selected (Default)"]
    Typeb = 0,
    #[doc = "1: Driver Type A is selected"]
    Typea = 1,
    #[doc = "2: Driver Type C is selected"]
    Typec = 2,
    #[doc = "3: Driver Type D is selected"]
    Typed = 3,
}
impl From<Hspdrvstval> for u8 {
    #[inline(always)]
    fn from(variant: Hspdrvstval) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hspdrvstval {
    type Ux = u8;
}
impl crate::IsEnum for Hspdrvstval {}
#[doc = "Field `HSPDRVSTVAL` reader - Driver Strength Select Value for High Speed"]
pub type HspdrvstvalR = crate::FieldReader<Hspdrvstval>;
impl HspdrvstvalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hspdrvstval {
        match self.bits {
            0 => Hspdrvstval::Typeb,
            1 => Hspdrvstval::Typea,
            2 => Hspdrvstval::Typec,
            3 => Hspdrvstval::Typed,
            _ => unreachable!(),
        }
    }
    #[doc = "Driver Type B is selected (Default)"]
    #[inline(always)]
    pub fn is_typeb(&self) -> bool {
        *self == Hspdrvstval::Typeb
    }
    #[doc = "Driver Type A is selected"]
    #[inline(always)]
    pub fn is_typea(&self) -> bool {
        *self == Hspdrvstval::Typea
    }
    #[doc = "Driver Type C is selected"]
    #[inline(always)]
    pub fn is_typec(&self) -> bool {
        *self == Hspdrvstval::Typec
    }
    #[doc = "Driver Type D is selected"]
    #[inline(always)]
    pub fn is_typed(&self) -> bool {
        *self == Hspdrvstval::Typed
    }
}
#[doc = "Field `SDR12SDCLKFREQVAL` reader - SD_CLK Frequency Select Value for SDR12"]
pub type Sdr12sdclkfreqvalR = crate::FieldReader<u16>;
#[doc = "Field `SDR12CLKGENVAL` reader - Clock Generator Select Value for SDR12"]
pub type Sdr12clkgenvalR = crate::BitReader;
#[doc = "Driver Strength Select Value for SDR12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sdr12drvstval {
    #[doc = "0: Driver Type B is selected (Default)"]
    Typeb = 0,
    #[doc = "1: Driver Type A is selected"]
    Typea = 1,
    #[doc = "2: Driver Type C is selected"]
    Typec = 2,
    #[doc = "3: Driver Type D is selected"]
    Typed = 3,
}
impl From<Sdr12drvstval> for u8 {
    #[inline(always)]
    fn from(variant: Sdr12drvstval) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sdr12drvstval {
    type Ux = u8;
}
impl crate::IsEnum for Sdr12drvstval {}
#[doc = "Field `SDR12DRVSTVAL` reader - Driver Strength Select Value for SDR12"]
pub type Sdr12drvstvalR = crate::FieldReader<Sdr12drvstval>;
impl Sdr12drvstvalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdr12drvstval {
        match self.bits {
            0 => Sdr12drvstval::Typeb,
            1 => Sdr12drvstval::Typea,
            2 => Sdr12drvstval::Typec,
            3 => Sdr12drvstval::Typed,
            _ => unreachable!(),
        }
    }
    #[doc = "Driver Type B is selected (Default)"]
    #[inline(always)]
    pub fn is_typeb(&self) -> bool {
        *self == Sdr12drvstval::Typeb
    }
    #[doc = "Driver Type A is selected"]
    #[inline(always)]
    pub fn is_typea(&self) -> bool {
        *self == Sdr12drvstval::Typea
    }
    #[doc = "Driver Type C is selected"]
    #[inline(always)]
    pub fn is_typec(&self) -> bool {
        *self == Sdr12drvstval::Typec
    }
    #[doc = "Driver Type D is selected"]
    #[inline(always)]
    pub fn is_typed(&self) -> bool {
        *self == Sdr12drvstval::Typed
    }
}
impl R {
    #[doc = "Bits 0:9 - SD_CLK Frequency Select Value for High Speed"]
    #[inline(always)]
    pub fn hspsdclkfreqval(&self) -> HspsdclkfreqvalR {
        HspsdclkfreqvalR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Clock Generator Select Value for High Speed"]
    #[inline(always)]
    pub fn hspclkgenval(&self) -> HspclkgenvalR {
        HspclkgenvalR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Driver Strength Select Value for High Speed"]
    #[inline(always)]
    pub fn hspdrvstval(&self) -> HspdrvstvalR {
        HspdrvstvalR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:25 - SD_CLK Frequency Select Value for SDR12"]
    #[inline(always)]
    pub fn sdr12sdclkfreqval(&self) -> Sdr12sdclkfreqvalR {
        Sdr12sdclkfreqvalR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - Clock Generator Select Value for SDR12"]
    #[inline(always)]
    pub fn sdr12clkgenval(&self) -> Sdr12clkgenvalR {
        Sdr12clkgenvalR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Driver Strength Select Value for SDR12"]
    #[inline(always)]
    pub fn sdr12drvstval(&self) -> Sdr12drvstvalR {
        Sdr12drvstvalR::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "Preset Value for High Speed and SDR12 Modes\n\nYou can [`read`](crate::Reg::read) this register and get [`prstval2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Prstval2Spec;
impl crate::RegisterSpec for Prstval2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prstval2::R`](R) reader structure"]
impl crate::Readable for Prstval2Spec {}
#[doc = "`reset()` method sets PRSTVAL2 to value 0"]
impl crate::Resettable for Prstval2Spec {}
