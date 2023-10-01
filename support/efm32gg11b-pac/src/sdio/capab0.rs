#[doc = "Register `CAPAB0` reader"]
pub type R = crate::R<CAPAB0_SPEC>;
#[doc = "Field `TMOUTCLKFREQ` reader - Timeout Clock Frequency"]
pub type TMOUTCLKFREQ_R = crate::FieldReader;
#[doc = "Field `TMOUTCLKUNIT` reader - Timeout Clock Unit"]
pub type TMOUTCLKUNIT_R = crate::BitReader;
#[doc = "Field `BASECLKFREQSD` reader - Base Clock Frequency for SD_CLK"]
pub type BASECLKFREQSD_R = crate::FieldReader;
#[doc = "Field `MAXBLOCKLEN` reader - Maximum Block Length"]
pub type MAXBLOCKLEN_R = crate::FieldReader;
#[doc = "Field `EXTMEDIABUSSUP` reader - Extended Media Bus Support"]
pub type EXTMEDIABUSSUP_R = crate::BitReader;
#[doc = "Field `ADMA2SUP` reader - ADMA2 Support"]
pub type ADMA2SUP_R = crate::BitReader;
#[doc = "Field `HSSUP` reader - High Speed Support"]
pub type HSSUP_R = crate::BitReader;
#[doc = "Field `SDMASUP` reader - SDMA Support"]
pub type SDMASUP_R = crate::BitReader;
#[doc = "Field `SUSRESSUP` reader - Suspend / Resume Support"]
pub type SUSRESSUP_R = crate::BitReader;
#[doc = "Field `VOLTSUP3P3V` reader - Voltage Support 3.3V"]
pub type VOLTSUP3P3V_R = crate::BitReader;
#[doc = "Field `VOLTSUP3P0V` reader - Voltage Support 3.0V"]
pub type VOLTSUP3P0V_R = crate::BitReader;
#[doc = "Field `VOLTSUP1P8V` reader - Voltage Support 1.8V"]
pub type VOLTSUP1P8V_R = crate::BitReader;
#[doc = "Field `SYSBUS64BSUP` reader - System Bus 64-bit Support"]
pub type SYSBUS64BSUP_R = crate::BitReader;
#[doc = "Field `ASYNCINTSUP` reader - Asynchronous Interrupt Support"]
pub type ASYNCINTSUP_R = crate::BitReader;
#[doc = "Field `IFSLOTTYPE` reader - Interface Card Slot Type"]
pub type IFSLOTTYPE_R = crate::FieldReader<IFSLOTTYPE_A>;
#[doc = "Interface Card Slot Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IFSLOTTYPE_A {
    #[doc = "0: Removable Card Slot"]
    REMOVABLE = 0,
    #[doc = "1: Only one non-removable device is conected to a SD bus slot"]
    EMBEDDED = 1,
    #[doc = "2: Can be set if Host controller supports Shared Bus CTRL register"]
    SHARED = 2,
}
impl From<IFSLOTTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: IFSLOTTYPE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IFSLOTTYPE_A {
    type Ux = u8;
}
impl IFSLOTTYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IFSLOTTYPE_A> {
        match self.bits {
            0 => Some(IFSLOTTYPE_A::REMOVABLE),
            1 => Some(IFSLOTTYPE_A::EMBEDDED),
            2 => Some(IFSLOTTYPE_A::SHARED),
            _ => None,
        }
    }
    #[doc = "Removable Card Slot"]
    #[inline(always)]
    pub fn is_removable(&self) -> bool {
        *self == IFSLOTTYPE_A::REMOVABLE
    }
    #[doc = "Only one non-removable device is conected to a SD bus slot"]
    #[inline(always)]
    pub fn is_embedded(&self) -> bool {
        *self == IFSLOTTYPE_A::EMBEDDED
    }
    #[doc = "Can be set if Host controller supports Shared Bus CTRL register"]
    #[inline(always)]
    pub fn is_shared(&self) -> bool {
        *self == IFSLOTTYPE_A::SHARED
    }
}
impl R {
    #[doc = "Bits 0:5 - Timeout Clock Frequency"]
    #[inline(always)]
    pub fn tmoutclkfreq(&self) -> TMOUTCLKFREQ_R {
        TMOUTCLKFREQ_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Timeout Clock Unit"]
    #[inline(always)]
    pub fn tmoutclkunit(&self) -> TMOUTCLKUNIT_R {
        TMOUTCLKUNIT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Base Clock Frequency for SD_CLK"]
    #[inline(always)]
    pub fn baseclkfreqsd(&self) -> BASECLKFREQSD_R {
        BASECLKFREQSD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Maximum Block Length"]
    #[inline(always)]
    pub fn maxblocklen(&self) -> MAXBLOCKLEN_R {
        MAXBLOCKLEN_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Extended Media Bus Support"]
    #[inline(always)]
    pub fn extmediabussup(&self) -> EXTMEDIABUSSUP_R {
        EXTMEDIABUSSUP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ADMA2 Support"]
    #[inline(always)]
    pub fn adma2sup(&self) -> ADMA2SUP_R {
        ADMA2SUP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - High Speed Support"]
    #[inline(always)]
    pub fn hssup(&self) -> HSSUP_R {
        HSSUP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SDMA Support"]
    #[inline(always)]
    pub fn sdmasup(&self) -> SDMASUP_R {
        SDMASUP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Suspend / Resume Support"]
    #[inline(always)]
    pub fn susressup(&self) -> SUSRESSUP_R {
        SUSRESSUP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Voltage Support 3.3V"]
    #[inline(always)]
    pub fn voltsup3p3v(&self) -> VOLTSUP3P3V_R {
        VOLTSUP3P3V_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Voltage Support 3.0V"]
    #[inline(always)]
    pub fn voltsup3p0v(&self) -> VOLTSUP3P0V_R {
        VOLTSUP3P0V_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Voltage Support 1.8V"]
    #[inline(always)]
    pub fn voltsup1p8v(&self) -> VOLTSUP1P8V_R {
        VOLTSUP1P8V_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - System Bus 64-bit Support"]
    #[inline(always)]
    pub fn sysbus64bsup(&self) -> SYSBUS64BSUP_R {
        SYSBUS64BSUP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Asynchronous Interrupt Support"]
    #[inline(always)]
    pub fn asyncintsup(&self) -> ASYNCINTSUP_R {
        ASYNCINTSUP_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Interface Card Slot Type"]
    #[inline(always)]
    pub fn ifslottype(&self) -> IFSLOTTYPE_R {
        IFSLOTTYPE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "Capabilities Register to Hold Bits 31~0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capab0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAPAB0_SPEC;
impl crate::RegisterSpec for CAPAB0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`capab0::R`](R) reader structure"]
impl crate::Readable for CAPAB0_SPEC {}
#[doc = "`reset()` method sets CAPAB0 to value 0"]
impl crate::Resettable for CAPAB0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
