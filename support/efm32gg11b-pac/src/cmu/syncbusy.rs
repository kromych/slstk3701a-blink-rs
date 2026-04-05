#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SyncbusySpec>;
#[doc = "Field `LFACLKEN0` reader - Low Frequency a Clock Enable 0 Busy"]
pub type Lfaclken0R = crate::BitReader;
#[doc = "Field `LFAPRESC0` reader - Low Frequency a Prescaler 0 Busy"]
pub type Lfapresc0R = crate::BitReader;
#[doc = "Field `LFBCLKEN0` reader - Low Frequency B Clock Enable 0 Busy"]
pub type Lfbclken0R = crate::BitReader;
#[doc = "Field `LFBPRESC0` reader - Low Frequency B Prescaler 0 Busy"]
pub type Lfbpresc0R = crate::BitReader;
#[doc = "Field `LFCCLKEN0` reader - Low Frequency C Clock Enable 0 Busy"]
pub type Lfcclken0R = crate::BitReader;
#[doc = "Field `LFECLKEN0` reader - Low Frequency E Clock Enable 0 Busy"]
pub type Lfeclken0R = crate::BitReader;
#[doc = "Field `LFEPRESC0` reader - Low Frequency E Prescaler 0 Busy"]
pub type Lfepresc0R = crate::BitReader;
#[doc = "Field `HFRCOBSY` reader - HFRCO Busy"]
pub type HfrcobsyR = crate::BitReader;
#[doc = "Field `AUXHFRCOBSY` reader - AUXHFRCO Busy"]
pub type AuxhfrcobsyR = crate::BitReader;
#[doc = "Field `LFRCOBSY` reader - LFRCO Busy"]
pub type LfrcobsyR = crate::BitReader;
#[doc = "Field `LFRCOVREFBSY` reader - LFRCO VREF Busy"]
pub type LfrcovrefbsyR = crate::BitReader;
#[doc = "Field `HFXOBSY` reader - HFXO Busy"]
pub type HfxobsyR = crate::BitReader;
#[doc = "Field `LFXOBSY` reader - LFXO Busy"]
pub type LfxobsyR = crate::BitReader;
#[doc = "Field `USHFRCOBSY` reader - USHFRCO Busy"]
pub type UshfrcobsyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Low Frequency a Clock Enable 0 Busy"]
    #[inline(always)]
    pub fn lfaclken0(&self) -> Lfaclken0R {
        Lfaclken0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Low Frequency a Prescaler 0 Busy"]
    #[inline(always)]
    pub fn lfapresc0(&self) -> Lfapresc0R {
        Lfapresc0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Low Frequency B Clock Enable 0 Busy"]
    #[inline(always)]
    pub fn lfbclken0(&self) -> Lfbclken0R {
        Lfbclken0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Low Frequency B Prescaler 0 Busy"]
    #[inline(always)]
    pub fn lfbpresc0(&self) -> Lfbpresc0R {
        Lfbpresc0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Low Frequency C Clock Enable 0 Busy"]
    #[inline(always)]
    pub fn lfcclken0(&self) -> Lfcclken0R {
        Lfcclken0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Low Frequency E Clock Enable 0 Busy"]
    #[inline(always)]
    pub fn lfeclken0(&self) -> Lfeclken0R {
        Lfeclken0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Low Frequency E Prescaler 0 Busy"]
    #[inline(always)]
    pub fn lfepresc0(&self) -> Lfepresc0R {
        Lfepresc0R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - HFRCO Busy"]
    #[inline(always)]
    pub fn hfrcobsy(&self) -> HfrcobsyR {
        HfrcobsyR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - AUXHFRCO Busy"]
    #[inline(always)]
    pub fn auxhfrcobsy(&self) -> AuxhfrcobsyR {
        AuxhfrcobsyR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - LFRCO Busy"]
    #[inline(always)]
    pub fn lfrcobsy(&self) -> LfrcobsyR {
        LfrcobsyR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - LFRCO VREF Busy"]
    #[inline(always)]
    pub fn lfrcovrefbsy(&self) -> LfrcovrefbsyR {
        LfrcovrefbsyR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - HFXO Busy"]
    #[inline(always)]
    pub fn hfxobsy(&self) -> HfxobsyR {
        HfxobsyR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - LFXO Busy"]
    #[inline(always)]
    pub fn lfxobsy(&self) -> LfxobsyR {
        LfxobsyR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - USHFRCO Busy"]
    #[inline(always)]
    pub fn ushfrcobsy(&self) -> UshfrcobsyR {
        UshfrcobsyR::new(((self.bits >> 30) & 1) != 0)
    }
}
#[doc = "Synchronization Busy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncbusySpec;
impl crate::RegisterSpec for SyncbusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncbusy::R`](R) reader structure"]
impl crate::Readable for SyncbusySpec {}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SyncbusySpec {}
