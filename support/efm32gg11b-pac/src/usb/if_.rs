#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Field `VBUSDETH` reader - VBUS Detect High Interrupt Flag"]
pub type VbusdethR = crate::BitReader;
#[doc = "Field `VBUSDETL` reader - VBUS Detect Low Interrupt Flag"]
pub type VbusdetlR = crate::BitReader;
#[doc = "Field `ERR` reader - Detection Error Interrupt Flag"]
pub type ErrR = crate::BitReader;
#[doc = "Field `DCD` reader - Data Contact Detection Complete Interrupt Flag"]
pub type DcdR = crate::BitReader;
#[doc = "Field `PD` reader - Primary Detection Complete Interrupt Flag"]
pub type PdR = crate::BitReader;
#[doc = "Field `SD` reader - Secondary Detection Complete Interrupt Flag"]
pub type SdR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - VBUS Detect High Interrupt Flag"]
    #[inline(always)]
    pub fn vbusdeth(&self) -> VbusdethR {
        VbusdethR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VBUS Detect Low Interrupt Flag"]
    #[inline(always)]
    pub fn vbusdetl(&self) -> VbusdetlR {
        VbusdetlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Detection Error Interrupt Flag"]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Data Contact Detection Complete Interrupt Flag"]
    #[inline(always)]
    pub fn dcd(&self) -> DcdR {
        DcdR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Primary Detection Complete Interrupt Flag"]
    #[inline(always)]
    pub fn pd(&self) -> PdR {
        PdR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Secondary Detection Complete Interrupt Flag"]
    #[inline(always)]
    pub fn sd(&self) -> SdR {
        SdR::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfSpec;
impl crate::RegisterSpec for IfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IfSpec {}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IfSpec {}
