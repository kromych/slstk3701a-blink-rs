#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Field `OF` reader - Overflow Interrupt Flag"]
pub type OfR = crate::BitReader;
#[doc = "Field `CC0` reader - Channel 0 Interrupt Flag"]
pub type Cc0R = crate::BitReader;
#[doc = "Field `CC1` reader - Channel 1 Interrupt Flag"]
pub type Cc1R = crate::BitReader;
#[doc = "Field `CC2` reader - Channel 2 Interrupt Flag"]
pub type Cc2R = crate::BitReader;
#[doc = "Field `OSCFAIL` reader - Oscillator Failure Interrupt Flag"]
pub type OscfailR = crate::BitReader;
#[doc = "Field `CNTTICK` reader - Main Counter Tick"]
pub type CnttickR = crate::BitReader;
#[doc = "Field `MINTICK` reader - Minute Tick"]
pub type MintickR = crate::BitReader;
#[doc = "Field `HOURTICK` reader - Hour Tick"]
pub type HourtickR = crate::BitReader;
#[doc = "Field `DAYTICK` reader - Day Tick"]
pub type DaytickR = crate::BitReader;
#[doc = "Field `DAYOWOF` reader - Day of Week Overflow"]
pub type DayowofR = crate::BitReader;
#[doc = "Field `MONTHTICK` reader - Month Tick"]
pub type MonthtickR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn of(&self) -> OfR {
        OfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 0 Interrupt Flag"]
    #[inline(always)]
    pub fn cc0(&self) -> Cc0R {
        Cc0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 1 Interrupt Flag"]
    #[inline(always)]
    pub fn cc1(&self) -> Cc1R {
        Cc1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 2 Interrupt Flag"]
    #[inline(always)]
    pub fn cc2(&self) -> Cc2R {
        Cc2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Oscillator Failure Interrupt Flag"]
    #[inline(always)]
    pub fn oscfail(&self) -> OscfailR {
        OscfailR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Main Counter Tick"]
    #[inline(always)]
    pub fn cnttick(&self) -> CnttickR {
        CnttickR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Minute Tick"]
    #[inline(always)]
    pub fn mintick(&self) -> MintickR {
        MintickR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Hour Tick"]
    #[inline(always)]
    pub fn hourtick(&self) -> HourtickR {
        HourtickR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Day Tick"]
    #[inline(always)]
    pub fn daytick(&self) -> DaytickR {
        DaytickR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Day of Week Overflow"]
    #[inline(always)]
    pub fn dayowof(&self) -> DayowofR {
        DayowofR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Month Tick"]
    #[inline(always)]
    pub fn monthtick(&self) -> MonthtickR {
        MonthtickR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "RTCC Interrupt Flags\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfSpec;
impl crate::RegisterSpec for IfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IfSpec {}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IfSpec {}
