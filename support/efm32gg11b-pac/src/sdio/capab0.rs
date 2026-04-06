#[doc = "Register `CAPAB0` reader"]
pub type R = crate::R<Capab0Spec>;
#[doc = "Field `TMOUTCLKFREQ` reader - Timeout Clock Frequency"]
pub type TmoutclkfreqR = crate::FieldReader;
#[doc = "Field `TMOUTCLKUNIT` reader - Timeout Clock Unit"]
pub type TmoutclkunitR = crate::BitReader;
#[doc = "Field `BASECLKFREQSD` reader - Base Clock Frequency for SD_CLK"]
pub type BaseclkfreqsdR = crate::FieldReader;
#[doc = "Field `MAXBLOCKLEN` reader - Maximum Block Length"]
pub type MaxblocklenR = crate::FieldReader;
#[doc = "Field `EXTMEDIABUSSUP` reader - Extended Media Bus Support"]
pub type ExtmediabussupR = crate::BitReader;
#[doc = "Field `ADMA2SUP` reader - ADMA2 Support"]
pub type Adma2supR = crate::BitReader;
#[doc = "Field `HSSUP` reader - High Speed Support"]
pub type HssupR = crate::BitReader;
#[doc = "Field `SDMASUP` reader - SDMA Support"]
pub type SdmasupR = crate::BitReader;
#[doc = "Field `SUSRESSUP` reader - Suspend / Resume Support"]
pub type SusressupR = crate::BitReader;
#[doc = "Field `VOLTSUP3P3V` reader - Voltage Support 3.3V"]
pub type Voltsup3p3vR = crate::BitReader;
#[doc = "Field `VOLTSUP3P0V` reader - Voltage Support 3.0V"]
pub type Voltsup3p0vR = crate::BitReader;
#[doc = "Field `VOLTSUP1P8V` reader - Voltage Support 1.8V"]
pub type Voltsup1p8vR = crate::BitReader;
#[doc = "Field `SYSBUS64BSUP` reader - System Bus 64-bit Support"]
pub type Sysbus64bsupR = crate::BitReader;
#[doc = "Field `ASYNCINTSUP` reader - Asynchronous Interrupt Support"]
pub type AsyncintsupR = crate::BitReader;
#[doc = "Interface Card Slot Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ifslottype {
    #[doc = "0: Removable Card Slot"]
    Removable = 0,
    #[doc = "1: Only one non-removable device is conected to a SD bus slot"]
    Embedded = 1,
    #[doc = "2: Can be set if Host controller supports Shared Bus CTRL register"]
    Shared = 2,
}
impl From<Ifslottype> for u8 {
    #[inline(always)]
    fn from(variant: Ifslottype) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ifslottype {
    type Ux = u8;
}
impl crate::IsEnum for Ifslottype {}
#[doc = "Field `IFSLOTTYPE` reader - Interface Card Slot Type"]
pub type IfslottypeR = crate::FieldReader<Ifslottype>;
impl IfslottypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ifslottype> {
        match self.bits {
            0 => Some(Ifslottype::Removable),
            1 => Some(Ifslottype::Embedded),
            2 => Some(Ifslottype::Shared),
            _ => None,
        }
    }
    #[doc = "Removable Card Slot"]
    #[inline(always)]
    pub fn is_removable(&self) -> bool {
        *self == Ifslottype::Removable
    }
    #[doc = "Only one non-removable device is conected to a SD bus slot"]
    #[inline(always)]
    pub fn is_embedded(&self) -> bool {
        *self == Ifslottype::Embedded
    }
    #[doc = "Can be set if Host controller supports Shared Bus CTRL register"]
    #[inline(always)]
    pub fn is_shared(&self) -> bool {
        *self == Ifslottype::Shared
    }
}
impl R {
    #[doc = "Bits 0:5 - Timeout Clock Frequency"]
    #[inline(always)]
    pub fn tmoutclkfreq(&self) -> TmoutclkfreqR {
        TmoutclkfreqR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Timeout Clock Unit"]
    #[inline(always)]
    pub fn tmoutclkunit(&self) -> TmoutclkunitR {
        TmoutclkunitR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Base Clock Frequency for SD_CLK"]
    #[inline(always)]
    pub fn baseclkfreqsd(&self) -> BaseclkfreqsdR {
        BaseclkfreqsdR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Maximum Block Length"]
    #[inline(always)]
    pub fn maxblocklen(&self) -> MaxblocklenR {
        MaxblocklenR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Extended Media Bus Support"]
    #[inline(always)]
    pub fn extmediabussup(&self) -> ExtmediabussupR {
        ExtmediabussupR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ADMA2 Support"]
    #[inline(always)]
    pub fn adma2sup(&self) -> Adma2supR {
        Adma2supR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - High Speed Support"]
    #[inline(always)]
    pub fn hssup(&self) -> HssupR {
        HssupR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SDMA Support"]
    #[inline(always)]
    pub fn sdmasup(&self) -> SdmasupR {
        SdmasupR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Suspend / Resume Support"]
    #[inline(always)]
    pub fn susressup(&self) -> SusressupR {
        SusressupR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Voltage Support 3.3V"]
    #[inline(always)]
    pub fn voltsup3p3v(&self) -> Voltsup3p3vR {
        Voltsup3p3vR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Voltage Support 3.0V"]
    #[inline(always)]
    pub fn voltsup3p0v(&self) -> Voltsup3p0vR {
        Voltsup3p0vR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Voltage Support 1.8V"]
    #[inline(always)]
    pub fn voltsup1p8v(&self) -> Voltsup1p8vR {
        Voltsup1p8vR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - System Bus 64-bit Support"]
    #[inline(always)]
    pub fn sysbus64bsup(&self) -> Sysbus64bsupR {
        Sysbus64bsupR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Asynchronous Interrupt Support"]
    #[inline(always)]
    pub fn asyncintsup(&self) -> AsyncintsupR {
        AsyncintsupR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Interface Card Slot Type"]
    #[inline(always)]
    pub fn ifslottype(&self) -> IfslottypeR {
        IfslottypeR::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "Capabilities Register to Hold Bits 31~0\n\nYou can [`read`](crate::Reg::read) this register and get [`capab0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Capab0Spec;
impl crate::RegisterSpec for Capab0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`capab0::R`](R) reader structure"]
impl crate::Readable for Capab0Spec {}
#[doc = "`reset()` method sets CAPAB0 to value 0"]
impl crate::Resettable for Capab0Spec {}
