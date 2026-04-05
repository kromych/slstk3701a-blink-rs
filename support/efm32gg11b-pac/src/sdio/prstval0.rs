#[doc = "Register `PRSTVAL0` reader"]
pub type R = crate::R<Prstval0Spec>;
#[doc = "Field `INITSDCLKFREQVAL` reader - SD_CLK Frequency Select Value for Initialization"]
pub type InitsdclkfreqvalR = crate::FieldReader<u16>;
#[doc = "Field `INITCLCKGENVAL` reader - Clock Generator Select Value for Initialization"]
pub type InitclckgenvalR = crate::BitReader;
#[doc = "Driver Strength Select Value for Initialization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Initdrvstval {
    #[doc = "0: Driver Type B is selected (Default)"]
    Typeb = 0,
    #[doc = "1: Driver Type A is selected"]
    Typea = 1,
    #[doc = "2: Driver Type C is selected"]
    Typec = 2,
    #[doc = "3: Driver Type D is selected"]
    Typed = 3,
}
impl From<Initdrvstval> for u8 {
    #[inline(always)]
    fn from(variant: Initdrvstval) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Initdrvstval {
    type Ux = u8;
}
impl crate::IsEnum for Initdrvstval {}
#[doc = "Field `INITDRVSTVAL` reader - Driver Strength Select Value for Initialization"]
pub type InitdrvstvalR = crate::FieldReader<Initdrvstval>;
impl InitdrvstvalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Initdrvstval {
        match self.bits {
            0 => Initdrvstval::Typeb,
            1 => Initdrvstval::Typea,
            2 => Initdrvstval::Typec,
            3 => Initdrvstval::Typed,
            _ => unreachable!(),
        }
    }
    #[doc = "Driver Type B is selected (Default)"]
    #[inline(always)]
    pub fn is_typeb(&self) -> bool {
        *self == Initdrvstval::Typeb
    }
    #[doc = "Driver Type A is selected"]
    #[inline(always)]
    pub fn is_typea(&self) -> bool {
        *self == Initdrvstval::Typea
    }
    #[doc = "Driver Type C is selected"]
    #[inline(always)]
    pub fn is_typec(&self) -> bool {
        *self == Initdrvstval::Typec
    }
    #[doc = "Driver Type D is selected"]
    #[inline(always)]
    pub fn is_typed(&self) -> bool {
        *self == Initdrvstval::Typed
    }
}
#[doc = "Field `DSPSDCLKFREQVAL` reader - SD_CLK Frequency Select Value for Default Speed"]
pub type DspsdclkfreqvalR = crate::FieldReader<u16>;
#[doc = "Field `DSPCLKGENVAL` reader - Clock Generator Select Value for Default Speed"]
pub type DspclkgenvalR = crate::BitReader;
#[doc = "Driver Strength Select Value for Default Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dspdrvstval {
    #[doc = "0: Driver Type B is selected (Default)"]
    Typeb = 0,
    #[doc = "1: Driver Type A is selected"]
    Typea = 1,
    #[doc = "2: Driver Type C is selected"]
    Typec = 2,
    #[doc = "3: Driver Type D is selected"]
    Typed = 3,
}
impl From<Dspdrvstval> for u8 {
    #[inline(always)]
    fn from(variant: Dspdrvstval) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dspdrvstval {
    type Ux = u8;
}
impl crate::IsEnum for Dspdrvstval {}
#[doc = "Field `DSPDRVSTVAL` reader - Driver Strength Select Value for Default Speed"]
pub type DspdrvstvalR = crate::FieldReader<Dspdrvstval>;
impl DspdrvstvalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dspdrvstval {
        match self.bits {
            0 => Dspdrvstval::Typeb,
            1 => Dspdrvstval::Typea,
            2 => Dspdrvstval::Typec,
            3 => Dspdrvstval::Typed,
            _ => unreachable!(),
        }
    }
    #[doc = "Driver Type B is selected (Default)"]
    #[inline(always)]
    pub fn is_typeb(&self) -> bool {
        *self == Dspdrvstval::Typeb
    }
    #[doc = "Driver Type A is selected"]
    #[inline(always)]
    pub fn is_typea(&self) -> bool {
        *self == Dspdrvstval::Typea
    }
    #[doc = "Driver Type C is selected"]
    #[inline(always)]
    pub fn is_typec(&self) -> bool {
        *self == Dspdrvstval::Typec
    }
    #[doc = "Driver Type D is selected"]
    #[inline(always)]
    pub fn is_typed(&self) -> bool {
        *self == Dspdrvstval::Typed
    }
}
impl R {
    #[doc = "Bits 0:9 - SD_CLK Frequency Select Value for Initialization"]
    #[inline(always)]
    pub fn initsdclkfreqval(&self) -> InitsdclkfreqvalR {
        InitsdclkfreqvalR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Clock Generator Select Value for Initialization"]
    #[inline(always)]
    pub fn initclckgenval(&self) -> InitclckgenvalR {
        InitclckgenvalR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Driver Strength Select Value for Initialization"]
    #[inline(always)]
    pub fn initdrvstval(&self) -> InitdrvstvalR {
        InitdrvstvalR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:25 - SD_CLK Frequency Select Value for Default Speed"]
    #[inline(always)]
    pub fn dspsdclkfreqval(&self) -> DspsdclkfreqvalR {
        DspsdclkfreqvalR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - Clock Generator Select Value for Default Speed"]
    #[inline(always)]
    pub fn dspclkgenval(&self) -> DspclkgenvalR {
        DspclkgenvalR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Driver Strength Select Value for Default Speed"]
    #[inline(always)]
    pub fn dspdrvstval(&self) -> DspdrvstvalR {
        DspdrvstvalR::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "Preset Value for Initialization and Default Speed Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`prstval0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Prstval0Spec;
impl crate::RegisterSpec for Prstval0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prstval0::R`](R) reader structure"]
impl crate::Readable for Prstval0Spec {}
#[doc = "`reset()` method sets PRSTVAL0 to value 0"]
impl crate::Resettable for Prstval0Spec {}
