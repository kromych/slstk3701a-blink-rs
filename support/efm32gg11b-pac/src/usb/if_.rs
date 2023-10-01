#[doc = "Register `IF` reader"]
pub type R = crate::R<IF_SPEC>;
#[doc = "Field `VBUSDETH` reader - VBUS Detect High Interrupt Flag"]
pub type VBUSDETH_R = crate::BitReader;
#[doc = "Field `VBUSDETL` reader - VBUS Detect Low Interrupt Flag"]
pub type VBUSDETL_R = crate::BitReader;
#[doc = "Field `ERR` reader - Detection Error Interrupt Flag"]
pub type ERR_R = crate::BitReader;
#[doc = "Field `DCD` reader - Data Contact Detection Complete Interrupt Flag"]
pub type DCD_R = crate::BitReader;
#[doc = "Field `PD` reader - Primary Detection Complete Interrupt Flag"]
pub type PD_R = crate::BitReader;
#[doc = "Field `SD` reader - Secondary Detection Complete Interrupt Flag"]
pub type SD_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - VBUS Detect High Interrupt Flag"]
    #[inline(always)]
    pub fn vbusdeth(&self) -> VBUSDETH_R {
        VBUSDETH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VBUS Detect Low Interrupt Flag"]
    #[inline(always)]
    pub fn vbusdetl(&self) -> VBUSDETL_R {
        VBUSDETL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Detection Error Interrupt Flag"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Data Contact Detection Complete Interrupt Flag"]
    #[inline(always)]
    pub fn dcd(&self) -> DCD_R {
        DCD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Primary Detection Complete Interrupt Flag"]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Secondary Detection Complete Interrupt Flag"]
    #[inline(always)]
    pub fn sd(&self) -> SD_R {
        SD_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if_::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IF_SPEC;
impl crate::RegisterSpec for IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IF_SPEC {}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
