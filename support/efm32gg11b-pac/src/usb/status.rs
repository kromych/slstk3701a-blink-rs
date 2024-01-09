#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `VBUSDETH` reader - VBUS Detect High"]
pub type VBUSDETH_R = crate::BitReader;
#[doc = "Field `LEMACTIVE` reader - Low Energy Mode Active"]
pub type LEMACTIVE_R = crate::BitReader;
#[doc = "Field `DCDTO` reader - Data Contact Detection Timeout"]
pub type DCDTO_R = crate::BitReader;
#[doc = "Field `SDP` reader - Standard Downstream Port Detected"]
pub type SDP_R = crate::BitReader;
#[doc = "Field `CDP` reader - Charging Downstream Port Detected"]
pub type CDP_R = crate::BitReader;
#[doc = "Field `DCP` reader - Dedicated Charging Port Detected"]
pub type DCP_R = crate::BitReader;
#[doc = "Field `ACAFS` reader - ACA Full Speed TypeB Device"]
pub type ACAFS_R = crate::BitReader;
#[doc = "Field `ACALS` reader - ACA Low Speed TypeB Device"]
pub type ACALS_R = crate::BitReader;
#[doc = "Field `USBCDBUSY` reader - USB Charger Detect Busy"]
pub type USBCDBUSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - VBUS Detect High"]
    #[inline(always)]
    pub fn vbusdeth(&self) -> VBUSDETH_R {
        VBUSDETH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Low Energy Mode Active"]
    #[inline(always)]
    pub fn lemactive(&self) -> LEMACTIVE_R {
        LEMACTIVE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Data Contact Detection Timeout"]
    #[inline(always)]
    pub fn dcdto(&self) -> DCDTO_R {
        DCDTO_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Standard Downstream Port Detected"]
    #[inline(always)]
    pub fn sdp(&self) -> SDP_R {
        SDP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Charging Downstream Port Detected"]
    #[inline(always)]
    pub fn cdp(&self) -> CDP_R {
        CDP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Dedicated Charging Port Detected"]
    #[inline(always)]
    pub fn dcp(&self) -> DCP_R {
        DCP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ACA Full Speed TypeB Device"]
    #[inline(always)]
    pub fn acafs(&self) -> ACAFS_R {
        ACAFS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ACA Low Speed TypeB Device"]
    #[inline(always)]
    pub fn acals(&self) -> ACALS_R {
        ACALS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - USB Charger Detect Busy"]
    #[inline(always)]
    pub fn usbcdbusy(&self) -> USBCDBUSY_R {
        USBCDBUSY_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "System Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
