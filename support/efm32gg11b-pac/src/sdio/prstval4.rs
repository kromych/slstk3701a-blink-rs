#[doc = "Register `PRSTVAL4` reader"]
pub type R = crate::R<PRSTVAL4_SPEC>;
#[doc = "Field `SDR25SDCLKFREQVAL` reader - SD_CLK Frequency Select Value for SDR25"]
pub type SDR25SDCLKFREQVAL_R = crate::FieldReader<u16>;
#[doc = "Field `SDR25CLKGENVAL` reader - Clock Generator Select Value for SDR25"]
pub type SDR25CLKGENVAL_R = crate::BitReader;
#[doc = "Field `SDR25DRVSTVAL` reader - Driver Strength Select Value for SDR25"]
pub type SDR25DRVSTVAL_R = crate::FieldReader<SDR25DRVSTVAL_A>;
#[doc = "Driver Strength Select Value for SDR25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SDR25DRVSTVAL_A {
    #[doc = "0: Driver Type B is selected (Default)"]
    TYPEB = 0,
    #[doc = "1: Driver Type A is selected"]
    TYPEA = 1,
    #[doc = "2: Driver Type C is selected"]
    TYPEC = 2,
    #[doc = "3: Driver Type D is selected"]
    TYPED = 3,
}
impl From<SDR25DRVSTVAL_A> for u8 {
    #[inline(always)]
    fn from(variant: SDR25DRVSTVAL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SDR25DRVSTVAL_A {
    type Ux = u8;
}
impl SDR25DRVSTVAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDR25DRVSTVAL_A {
        match self.bits {
            0 => SDR25DRVSTVAL_A::TYPEB,
            1 => SDR25DRVSTVAL_A::TYPEA,
            2 => SDR25DRVSTVAL_A::TYPEC,
            3 => SDR25DRVSTVAL_A::TYPED,
            _ => unreachable!(),
        }
    }
    #[doc = "Driver Type B is selected (Default)"]
    #[inline(always)]
    pub fn is_typeb(&self) -> bool {
        *self == SDR25DRVSTVAL_A::TYPEB
    }
    #[doc = "Driver Type A is selected"]
    #[inline(always)]
    pub fn is_typea(&self) -> bool {
        *self == SDR25DRVSTVAL_A::TYPEA
    }
    #[doc = "Driver Type C is selected"]
    #[inline(always)]
    pub fn is_typec(&self) -> bool {
        *self == SDR25DRVSTVAL_A::TYPEC
    }
    #[doc = "Driver Type D is selected"]
    #[inline(always)]
    pub fn is_typed(&self) -> bool {
        *self == SDR25DRVSTVAL_A::TYPED
    }
}
#[doc = "Field `SDR50SDCLKFREQVAL` reader - SD_CLK Frequency Select Value for SDR50"]
pub type SDR50SDCLKFREQVAL_R = crate::FieldReader<u16>;
#[doc = "Field `SDR50CLCKGENVAL` reader - Clock Generator Select Value for SDR50"]
pub type SDR50CLCKGENVAL_R = crate::BitReader;
#[doc = "Field `SDR50DRVSTVAL` reader - Driver Strength Select Value for SDR50"]
pub type SDR50DRVSTVAL_R = crate::FieldReader<SDR50DRVSTVAL_A>;
#[doc = "Driver Strength Select Value for SDR50\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SDR50DRVSTVAL_A {
    #[doc = "0: Driver Type B is selected (Default)"]
    TYPEB = 0,
    #[doc = "1: Driver Type A is selected"]
    TYPEA = 1,
    #[doc = "2: Driver Type C is selected"]
    TYPEC = 2,
    #[doc = "3: Driver Type D is selected"]
    TYPED = 3,
}
impl From<SDR50DRVSTVAL_A> for u8 {
    #[inline(always)]
    fn from(variant: SDR50DRVSTVAL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SDR50DRVSTVAL_A {
    type Ux = u8;
}
impl SDR50DRVSTVAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDR50DRVSTVAL_A {
        match self.bits {
            0 => SDR50DRVSTVAL_A::TYPEB,
            1 => SDR50DRVSTVAL_A::TYPEA,
            2 => SDR50DRVSTVAL_A::TYPEC,
            3 => SDR50DRVSTVAL_A::TYPED,
            _ => unreachable!(),
        }
    }
    #[doc = "Driver Type B is selected (Default)"]
    #[inline(always)]
    pub fn is_typeb(&self) -> bool {
        *self == SDR50DRVSTVAL_A::TYPEB
    }
    #[doc = "Driver Type A is selected"]
    #[inline(always)]
    pub fn is_typea(&self) -> bool {
        *self == SDR50DRVSTVAL_A::TYPEA
    }
    #[doc = "Driver Type C is selected"]
    #[inline(always)]
    pub fn is_typec(&self) -> bool {
        *self == SDR50DRVSTVAL_A::TYPEC
    }
    #[doc = "Driver Type D is selected"]
    #[inline(always)]
    pub fn is_typed(&self) -> bool {
        *self == SDR50DRVSTVAL_A::TYPED
    }
}
impl R {
    #[doc = "Bits 0:9 - SD_CLK Frequency Select Value for SDR25"]
    #[inline(always)]
    pub fn sdr25sdclkfreqval(&self) -> SDR25SDCLKFREQVAL_R {
        SDR25SDCLKFREQVAL_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Clock Generator Select Value for SDR25"]
    #[inline(always)]
    pub fn sdr25clkgenval(&self) -> SDR25CLKGENVAL_R {
        SDR25CLKGENVAL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Driver Strength Select Value for SDR25"]
    #[inline(always)]
    pub fn sdr25drvstval(&self) -> SDR25DRVSTVAL_R {
        SDR25DRVSTVAL_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:25 - SD_CLK Frequency Select Value for SDR50"]
    #[inline(always)]
    pub fn sdr50sdclkfreqval(&self) -> SDR50SDCLKFREQVAL_R {
        SDR50SDCLKFREQVAL_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - Clock Generator Select Value for SDR50"]
    #[inline(always)]
    pub fn sdr50clckgenval(&self) -> SDR50CLCKGENVAL_R {
        SDR50CLCKGENVAL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Driver Strength Select Value for SDR50"]
    #[inline(always)]
    pub fn sdr50drvstval(&self) -> SDR50DRVSTVAL_R {
        SDR50DRVSTVAL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "Preset Value for SDR25 and SDR50 Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prstval4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRSTVAL4_SPEC;
impl crate::RegisterSpec for PRSTVAL4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prstval4::R`](R) reader structure"]
impl crate::Readable for PRSTVAL4_SPEC {}
#[doc = "`reset()` method sets PRSTVAL4 to value 0"]
impl crate::Resettable for PRSTVAL4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
