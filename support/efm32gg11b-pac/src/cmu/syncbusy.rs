#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SYNCBUSY_SPEC>;
#[doc = "Field `LFACLKEN0` reader - Low Frequency a Clock Enable 0 Busy"]
pub type LFACLKEN0_R = crate::BitReader;
#[doc = "Field `LFAPRESC0` reader - Low Frequency a Prescaler 0 Busy"]
pub type LFAPRESC0_R = crate::BitReader;
#[doc = "Field `LFBCLKEN0` reader - Low Frequency B Clock Enable 0 Busy"]
pub type LFBCLKEN0_R = crate::BitReader;
#[doc = "Field `LFBPRESC0` reader - Low Frequency B Prescaler 0 Busy"]
pub type LFBPRESC0_R = crate::BitReader;
#[doc = "Field `LFCCLKEN0` reader - Low Frequency C Clock Enable 0 Busy"]
pub type LFCCLKEN0_R = crate::BitReader;
#[doc = "Field `LFECLKEN0` reader - Low Frequency E Clock Enable 0 Busy"]
pub type LFECLKEN0_R = crate::BitReader;
#[doc = "Field `LFEPRESC0` reader - Low Frequency E Prescaler 0 Busy"]
pub type LFEPRESC0_R = crate::BitReader;
#[doc = "Field `HFRCOBSY` reader - HFRCO Busy"]
pub type HFRCOBSY_R = crate::BitReader;
#[doc = "Field `AUXHFRCOBSY` reader - AUXHFRCO Busy"]
pub type AUXHFRCOBSY_R = crate::BitReader;
#[doc = "Field `LFRCOBSY` reader - LFRCO Busy"]
pub type LFRCOBSY_R = crate::BitReader;
#[doc = "Field `LFRCOVREFBSY` reader - LFRCO VREF Busy"]
pub type LFRCOVREFBSY_R = crate::BitReader;
#[doc = "Field `HFXOBSY` reader - HFXO Busy"]
pub type HFXOBSY_R = crate::BitReader;
#[doc = "Field `LFXOBSY` reader - LFXO Busy"]
pub type LFXOBSY_R = crate::BitReader;
#[doc = "Field `USHFRCOBSY` reader - USHFRCO Busy"]
pub type USHFRCOBSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Low Frequency a Clock Enable 0 Busy"]
    #[inline(always)]
    pub fn lfaclken0(&self) -> LFACLKEN0_R {
        LFACLKEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Low Frequency a Prescaler 0 Busy"]
    #[inline(always)]
    pub fn lfapresc0(&self) -> LFAPRESC0_R {
        LFAPRESC0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Low Frequency B Clock Enable 0 Busy"]
    #[inline(always)]
    pub fn lfbclken0(&self) -> LFBCLKEN0_R {
        LFBCLKEN0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Low Frequency B Prescaler 0 Busy"]
    #[inline(always)]
    pub fn lfbpresc0(&self) -> LFBPRESC0_R {
        LFBPRESC0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Low Frequency C Clock Enable 0 Busy"]
    #[inline(always)]
    pub fn lfcclken0(&self) -> LFCCLKEN0_R {
        LFCCLKEN0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Low Frequency E Clock Enable 0 Busy"]
    #[inline(always)]
    pub fn lfeclken0(&self) -> LFECLKEN0_R {
        LFECLKEN0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Low Frequency E Prescaler 0 Busy"]
    #[inline(always)]
    pub fn lfepresc0(&self) -> LFEPRESC0_R {
        LFEPRESC0_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - HFRCO Busy"]
    #[inline(always)]
    pub fn hfrcobsy(&self) -> HFRCOBSY_R {
        HFRCOBSY_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - AUXHFRCO Busy"]
    #[inline(always)]
    pub fn auxhfrcobsy(&self) -> AUXHFRCOBSY_R {
        AUXHFRCOBSY_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - LFRCO Busy"]
    #[inline(always)]
    pub fn lfrcobsy(&self) -> LFRCOBSY_R {
        LFRCOBSY_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - LFRCO VREF Busy"]
    #[inline(always)]
    pub fn lfrcovrefbsy(&self) -> LFRCOVREFBSY_R {
        LFRCOVREFBSY_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - HFXO Busy"]
    #[inline(always)]
    pub fn hfxobsy(&self) -> HFXOBSY_R {
        HFXOBSY_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - LFXO Busy"]
    #[inline(always)]
    pub fn lfxobsy(&self) -> LFXOBSY_R {
        LFXOBSY_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - USHFRCO Busy"]
    #[inline(always)]
    pub fn ushfrcobsy(&self) -> USHFRCOBSY_R {
        USHFRCOBSY_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[doc = "Synchronization Busy Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncbusy::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYNCBUSY_SPEC;
impl crate::RegisterSpec for SYNCBUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncbusy::R`](R) reader structure"]
impl crate::Readable for SYNCBUSY_SPEC {}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SYNCBUSY_SPEC {
    const RESET_VALUE: u32 = 0;
}
