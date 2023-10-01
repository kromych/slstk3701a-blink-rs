#[doc = "Register `PRSTVAL6` reader"]
pub type R = crate::R<PRSTVAL6_SPEC>;
#[doc = "Field `SDR104SDCLKFREQVAL` reader - SD_CLK Frequency Select Value for SDR104"]
pub type SDR104SDCLKFREQVAL_R = crate::FieldReader<u16>;
#[doc = "Field `SDR104CLKGENVAL` reader - Clock Generator Select Value for SDR104"]
pub type SDR104CLKGENVAL_R = crate::BitReader;
#[doc = "Field `SDR104DRVSTVAL` reader - Driver Strength Select Value for SDR104"]
pub type SDR104DRVSTVAL_R = crate::FieldReader<SDR104DRVSTVAL_A>;
#[doc = "Driver Strength Select Value for SDR104\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SDR104DRVSTVAL_A {
    #[doc = "0: Driver Type B is selected (Default)"]
    TYPEB = 0,
    #[doc = "1: Driver Type A is selected"]
    TYPEA = 1,
    #[doc = "2: Driver Type C is selected"]
    TYPEC = 2,
    #[doc = "3: Driver Type D is selected"]
    TYPED = 3,
}
impl From<SDR104DRVSTVAL_A> for u8 {
    #[inline(always)]
    fn from(variant: SDR104DRVSTVAL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SDR104DRVSTVAL_A {
    type Ux = u8;
}
impl SDR104DRVSTVAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDR104DRVSTVAL_A {
        match self.bits {
            0 => SDR104DRVSTVAL_A::TYPEB,
            1 => SDR104DRVSTVAL_A::TYPEA,
            2 => SDR104DRVSTVAL_A::TYPEC,
            3 => SDR104DRVSTVAL_A::TYPED,
            _ => unreachable!(),
        }
    }
    #[doc = "Driver Type B is selected (Default)"]
    #[inline(always)]
    pub fn is_typeb(&self) -> bool {
        *self == SDR104DRVSTVAL_A::TYPEB
    }
    #[doc = "Driver Type A is selected"]
    #[inline(always)]
    pub fn is_typea(&self) -> bool {
        *self == SDR104DRVSTVAL_A::TYPEA
    }
    #[doc = "Driver Type C is selected"]
    #[inline(always)]
    pub fn is_typec(&self) -> bool {
        *self == SDR104DRVSTVAL_A::TYPEC
    }
    #[doc = "Driver Type D is selected"]
    #[inline(always)]
    pub fn is_typed(&self) -> bool {
        *self == SDR104DRVSTVAL_A::TYPED
    }
}
#[doc = "Field `DDR50SDCLKFREQVAL` reader - SD_CLK Frequency Select Value for DDR50"]
pub type DDR50SDCLKFREQVAL_R = crate::FieldReader<u16>;
#[doc = "Field `DDR50CLKGENVAL` reader - Clock Generator Select Value for DDR50"]
pub type DDR50CLKGENVAL_R = crate::BitReader;
#[doc = "Field `DDR50DRVSTVAL` reader - Driver Strength Select Value for DDR50"]
pub type DDR50DRVSTVAL_R = crate::FieldReader<DDR50DRVSTVAL_A>;
#[doc = "Driver Strength Select Value for DDR50\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DDR50DRVSTVAL_A {
    #[doc = "0: Driver Type B is selected (Default)"]
    TYPEB = 0,
    #[doc = "1: Driver Type A is selected"]
    TYPEA = 1,
    #[doc = "2: Driver Type C is selected"]
    TYPEC = 2,
    #[doc = "3: Driver Type D is selected"]
    TYPED = 3,
}
impl From<DDR50DRVSTVAL_A> for u8 {
    #[inline(always)]
    fn from(variant: DDR50DRVSTVAL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DDR50DRVSTVAL_A {
    type Ux = u8;
}
impl DDR50DRVSTVAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDR50DRVSTVAL_A {
        match self.bits {
            0 => DDR50DRVSTVAL_A::TYPEB,
            1 => DDR50DRVSTVAL_A::TYPEA,
            2 => DDR50DRVSTVAL_A::TYPEC,
            3 => DDR50DRVSTVAL_A::TYPED,
            _ => unreachable!(),
        }
    }
    #[doc = "Driver Type B is selected (Default)"]
    #[inline(always)]
    pub fn is_typeb(&self) -> bool {
        *self == DDR50DRVSTVAL_A::TYPEB
    }
    #[doc = "Driver Type A is selected"]
    #[inline(always)]
    pub fn is_typea(&self) -> bool {
        *self == DDR50DRVSTVAL_A::TYPEA
    }
    #[doc = "Driver Type C is selected"]
    #[inline(always)]
    pub fn is_typec(&self) -> bool {
        *self == DDR50DRVSTVAL_A::TYPEC
    }
    #[doc = "Driver Type D is selected"]
    #[inline(always)]
    pub fn is_typed(&self) -> bool {
        *self == DDR50DRVSTVAL_A::TYPED
    }
}
impl R {
    #[doc = "Bits 0:9 - SD_CLK Frequency Select Value for SDR104"]
    #[inline(always)]
    pub fn sdr104sdclkfreqval(&self) -> SDR104SDCLKFREQVAL_R {
        SDR104SDCLKFREQVAL_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Clock Generator Select Value for SDR104"]
    #[inline(always)]
    pub fn sdr104clkgenval(&self) -> SDR104CLKGENVAL_R {
        SDR104CLKGENVAL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Driver Strength Select Value for SDR104"]
    #[inline(always)]
    pub fn sdr104drvstval(&self) -> SDR104DRVSTVAL_R {
        SDR104DRVSTVAL_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:25 - SD_CLK Frequency Select Value for DDR50"]
    #[inline(always)]
    pub fn ddr50sdclkfreqval(&self) -> DDR50SDCLKFREQVAL_R {
        DDR50SDCLKFREQVAL_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - Clock Generator Select Value for DDR50"]
    #[inline(always)]
    pub fn ddr50clkgenval(&self) -> DDR50CLKGENVAL_R {
        DDR50CLKGENVAL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Driver Strength Select Value for DDR50"]
    #[inline(always)]
    pub fn ddr50drvstval(&self) -> DDR50DRVSTVAL_R {
        DDR50DRVSTVAL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "Preset Value for SDR104 and DDR50 Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prstval6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRSTVAL6_SPEC;
impl crate::RegisterSpec for PRSTVAL6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prstval6::R`](R) reader structure"]
impl crate::Readable for PRSTVAL6_SPEC {}
#[doc = "`reset()` method sets PRSTVAL6 to value 0"]
impl crate::Resettable for PRSTVAL6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
