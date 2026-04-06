#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `VBUSDETH` reader - VBUS Detect High"]
pub type VbusdethR = crate::BitReader;
#[doc = "Field `LEMACTIVE` reader - Low Energy Mode Active"]
pub type LemactiveR = crate::BitReader;
#[doc = "Field `DCDTO` reader - Data Contact Detection Timeout"]
pub type DcdtoR = crate::BitReader;
#[doc = "Field `SDP` reader - Standard Downstream Port Detected"]
pub type SdpR = crate::BitReader;
#[doc = "Field `CDP` reader - Charging Downstream Port Detected"]
pub type CdpR = crate::BitReader;
#[doc = "Field `DCP` reader - Dedicated Charging Port Detected"]
pub type DcpR = crate::BitReader;
#[doc = "Field `ACAFS` reader - ACA Full Speed TypeB Device"]
pub type AcafsR = crate::BitReader;
#[doc = "Field `ACALS` reader - ACA Low Speed TypeB Device"]
pub type AcalsR = crate::BitReader;
#[doc = "Field `USBCDBUSY` reader - USB Charger Detect Busy"]
pub type UsbcdbusyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - VBUS Detect High"]
    #[inline(always)]
    pub fn vbusdeth(&self) -> VbusdethR {
        VbusdethR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Low Energy Mode Active"]
    #[inline(always)]
    pub fn lemactive(&self) -> LemactiveR {
        LemactiveR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Data Contact Detection Timeout"]
    #[inline(always)]
    pub fn dcdto(&self) -> DcdtoR {
        DcdtoR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Standard Downstream Port Detected"]
    #[inline(always)]
    pub fn sdp(&self) -> SdpR {
        SdpR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Charging Downstream Port Detected"]
    #[inline(always)]
    pub fn cdp(&self) -> CdpR {
        CdpR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Dedicated Charging Port Detected"]
    #[inline(always)]
    pub fn dcp(&self) -> DcpR {
        DcpR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ACA Full Speed TypeB Device"]
    #[inline(always)]
    pub fn acafs(&self) -> AcafsR {
        AcafsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ACA Low Speed TypeB Device"]
    #[inline(always)]
    pub fn acals(&self) -> AcalsR {
        AcalsR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - USB Charger Detect Busy"]
    #[inline(always)]
    pub fn usbcdbusy(&self) -> UsbcdbusyR {
        UsbcdbusyR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "System Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
