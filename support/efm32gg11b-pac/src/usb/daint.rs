#[doc = "Register `DAINT` reader"]
pub type R = crate::R<DAINT_SPEC>;
#[doc = "Field `INEPINT0` reader - IN Endpoint 0 Interrupt Bit"]
pub type INEPINT0_R = crate::BitReader;
#[doc = "Field `INEPINT1` reader - IN Endpoint 1 Interrupt Bit"]
pub type INEPINT1_R = crate::BitReader;
#[doc = "Field `INEPINT2` reader - IN Endpoint 2 Interrupt Bit"]
pub type INEPINT2_R = crate::BitReader;
#[doc = "Field `INEPINT3` reader - IN Endpoint 3 Interrupt Bit"]
pub type INEPINT3_R = crate::BitReader;
#[doc = "Field `INEPINT4` reader - IN Endpoint 4 Interrupt Bit"]
pub type INEPINT4_R = crate::BitReader;
#[doc = "Field `INEPINT5` reader - IN Endpoint 5 Interrupt Bit"]
pub type INEPINT5_R = crate::BitReader;
#[doc = "Field `INEPINT6` reader - IN Endpoint 6 Interrupt Bit"]
pub type INEPINT6_R = crate::BitReader;
#[doc = "Field `OUTEPINT0` reader - OUT Endpoint 0 Interrupt Bit"]
pub type OUTEPINT0_R = crate::BitReader;
#[doc = "Field `OUTEPINT1` reader - OUT Endpoint 1 Interrupt Bit"]
pub type OUTEPINT1_R = crate::BitReader;
#[doc = "Field `OUTEPINT2` reader - OUT Endpoint 2 Interrupt Bit"]
pub type OUTEPINT2_R = crate::BitReader;
#[doc = "Field `OUTEPINT3` reader - OUT Endpoint 3 Interrupt Bit"]
pub type OUTEPINT3_R = crate::BitReader;
#[doc = "Field `OUTEPINT4` reader - OUT Endpoint 4 Interrupt Bit"]
pub type OUTEPINT4_R = crate::BitReader;
#[doc = "Field `OUTEPINT5` reader - OUT Endpoint 5 Interrupt Bit"]
pub type OUTEPINT5_R = crate::BitReader;
#[doc = "Field `OUTEPINT6` reader - OUT Endpoint 6 Interrupt Bit"]
pub type OUTEPINT6_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - IN Endpoint 0 Interrupt Bit"]
    #[inline(always)]
    pub fn inepint0(&self) -> INEPINT0_R {
        INEPINT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IN Endpoint 1 Interrupt Bit"]
    #[inline(always)]
    pub fn inepint1(&self) -> INEPINT1_R {
        INEPINT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IN Endpoint 2 Interrupt Bit"]
    #[inline(always)]
    pub fn inepint2(&self) -> INEPINT2_R {
        INEPINT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IN Endpoint 3 Interrupt Bit"]
    #[inline(always)]
    pub fn inepint3(&self) -> INEPINT3_R {
        INEPINT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IN Endpoint 4 Interrupt Bit"]
    #[inline(always)]
    pub fn inepint4(&self) -> INEPINT4_R {
        INEPINT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IN Endpoint 5 Interrupt Bit"]
    #[inline(always)]
    pub fn inepint5(&self) -> INEPINT5_R {
        INEPINT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IN Endpoint 6 Interrupt Bit"]
    #[inline(always)]
    pub fn inepint6(&self) -> INEPINT6_R {
        INEPINT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - OUT Endpoint 0 Interrupt Bit"]
    #[inline(always)]
    pub fn outepint0(&self) -> OUTEPINT0_R {
        OUTEPINT0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - OUT Endpoint 1 Interrupt Bit"]
    #[inline(always)]
    pub fn outepint1(&self) -> OUTEPINT1_R {
        OUTEPINT1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - OUT Endpoint 2 Interrupt Bit"]
    #[inline(always)]
    pub fn outepint2(&self) -> OUTEPINT2_R {
        OUTEPINT2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OUT Endpoint 3 Interrupt Bit"]
    #[inline(always)]
    pub fn outepint3(&self) -> OUTEPINT3_R {
        OUTEPINT3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - OUT Endpoint 4 Interrupt Bit"]
    #[inline(always)]
    pub fn outepint4(&self) -> OUTEPINT4_R {
        OUTEPINT4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - OUT Endpoint 5 Interrupt Bit"]
    #[inline(always)]
    pub fn outepint5(&self) -> OUTEPINT5_R {
        OUTEPINT5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - OUT Endpoint 6 Interrupt Bit"]
    #[inline(always)]
    pub fn outepint6(&self) -> OUTEPINT6_R {
        OUTEPINT6_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[doc = "Device All Endpoints Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daint::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAINT_SPEC;
impl crate::RegisterSpec for DAINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`daint::R`](R) reader structure"]
impl crate::Readable for DAINT_SPEC {}
#[doc = "`reset()` method sets DAINT to value 0"]
impl crate::Resettable for DAINT_SPEC {
    const RESET_VALUE: u32 = 0;
}
