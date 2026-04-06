#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `SINGLEACT` reader - Single Channel Conversion Active"]
pub type SingleactR = crate::BitReader;
#[doc = "Field `SCANACT` reader - Scan Conversion Active"]
pub type ScanactR = crate::BitReader;
#[doc = "Field `SCANPENDING` reader - Scan Conversion Pending"]
pub type ScanpendingR = crate::BitReader;
#[doc = "Field `SINGLEREFWARM` reader - Single Channel Reference Warmed Up"]
pub type SinglerefwarmR = crate::BitReader;
#[doc = "Field `SCANREFWARM` reader - Scan Reference Warmed Up"]
pub type ScanrefwarmR = crate::BitReader;
#[doc = "Programming Error Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Progerr {
    #[doc = "1: `1`"]
    Busconf = 1,
    #[doc = "2: `10`"]
    Negselconf = 2,
}
impl From<Progerr> for u8 {
    #[inline(always)]
    fn from(variant: Progerr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Progerr {
    type Ux = u8;
}
impl crate::IsEnum for Progerr {}
#[doc = "Field `PROGERR` reader - Programming Error Status"]
pub type ProgerrR = crate::FieldReader<Progerr>;
impl ProgerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Progerr> {
        match self.bits {
            1 => Some(Progerr::Busconf),
            2 => Some(Progerr::Negselconf),
            _ => None,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_busconf(&self) -> bool {
        *self == Progerr::Busconf
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_negselconf(&self) -> bool {
        *self == Progerr::Negselconf
    }
}
#[doc = "Field `WARM` reader - ADC Warmed Up"]
pub type WarmR = crate::BitReader;
#[doc = "Field `SINGLEDV` reader - Single Channel Data Valid"]
pub type SingledvR = crate::BitReader;
#[doc = "Field `SCANDV` reader - Scan Data Valid"]
pub type ScandvR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Single Channel Conversion Active"]
    #[inline(always)]
    pub fn singleact(&self) -> SingleactR {
        SingleactR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scan Conversion Active"]
    #[inline(always)]
    pub fn scanact(&self) -> ScanactR {
        ScanactR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Scan Conversion Pending"]
    #[inline(always)]
    pub fn scanpending(&self) -> ScanpendingR {
        ScanpendingR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Single Channel Reference Warmed Up"]
    #[inline(always)]
    pub fn singlerefwarm(&self) -> SinglerefwarmR {
        SinglerefwarmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Scan Reference Warmed Up"]
    #[inline(always)]
    pub fn scanrefwarm(&self) -> ScanrefwarmR {
        ScanrefwarmR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Programming Error Status"]
    #[inline(always)]
    pub fn progerr(&self) -> ProgerrR {
        ProgerrR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - ADC Warmed Up"]
    #[inline(always)]
    pub fn warm(&self) -> WarmR {
        WarmR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Single Channel Data Valid"]
    #[inline(always)]
    pub fn singledv(&self) -> SingledvR {
        SingledvR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Scan Data Valid"]
    #[inline(always)]
    pub fn scandv(&self) -> ScandvR {
        ScandvR::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
