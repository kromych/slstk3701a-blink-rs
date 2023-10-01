#[doc = "Register `APORTREQ` reader"]
pub type R = crate::R<APORTREQ_SPEC>;
#[doc = "Field `APORT0XREQ` reader - 1 If the Bus Connected to APORT0X is Requested"]
pub type APORT0XREQ_R = crate::BitReader;
#[doc = "Field `APORT0YREQ` reader - 1 If the Bus Connected to APORT0Y is Requested"]
pub type APORT0YREQ_R = crate::BitReader;
#[doc = "Field `APORT1XREQ` reader - 1 If the Bus Connected to APORT1X is Requested"]
pub type APORT1XREQ_R = crate::BitReader;
#[doc = "Field `APORT1YREQ` reader - 1 If the Bus Connected to APORT1Y is Requested"]
pub type APORT1YREQ_R = crate::BitReader;
#[doc = "Field `APORT2XREQ` reader - 1 If the Bus Connected to APORT2X is Requested"]
pub type APORT2XREQ_R = crate::BitReader;
#[doc = "Field `APORT2YREQ` reader - 1 If the Bus Connected to APORT2Y is Requested"]
pub type APORT2YREQ_R = crate::BitReader;
#[doc = "Field `APORT3XREQ` reader - 1 If the Bus Connected to APORT3X is Requested"]
pub type APORT3XREQ_R = crate::BitReader;
#[doc = "Field `APORT3YREQ` reader - 1 If the Bus Connected to APORT3Y is Requested"]
pub type APORT3YREQ_R = crate::BitReader;
#[doc = "Field `APORT4XREQ` reader - 1 If the Bus Connected to APORT4X is Requested"]
pub type APORT4XREQ_R = crate::BitReader;
#[doc = "Field `APORT4YREQ` reader - 1 If the Bus Connected to APORT4Y is Requested"]
pub type APORT4YREQ_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 1 If the Bus Connected to APORT0X is Requested"]
    #[inline(always)]
    pub fn aport0xreq(&self) -> APORT0XREQ_R {
        APORT0XREQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1 If the Bus Connected to APORT0Y is Requested"]
    #[inline(always)]
    pub fn aport0yreq(&self) -> APORT0YREQ_R {
        APORT0YREQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1 If the Bus Connected to APORT1X is Requested"]
    #[inline(always)]
    pub fn aport1xreq(&self) -> APORT1XREQ_R {
        APORT1XREQ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1 If the Bus Connected to APORT1Y is Requested"]
    #[inline(always)]
    pub fn aport1yreq(&self) -> APORT1YREQ_R {
        APORT1YREQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1 If the Bus Connected to APORT2X is Requested"]
    #[inline(always)]
    pub fn aport2xreq(&self) -> APORT2XREQ_R {
        APORT2XREQ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1 If the Bus Connected to APORT2Y is Requested"]
    #[inline(always)]
    pub fn aport2yreq(&self) -> APORT2YREQ_R {
        APORT2YREQ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1 If the Bus Connected to APORT3X is Requested"]
    #[inline(always)]
    pub fn aport3xreq(&self) -> APORT3XREQ_R {
        APORT3XREQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1 If the Bus Connected to APORT3Y is Requested"]
    #[inline(always)]
    pub fn aport3yreq(&self) -> APORT3YREQ_R {
        APORT3YREQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1 If the Bus Connected to APORT4X is Requested"]
    #[inline(always)]
    pub fn aport4xreq(&self) -> APORT4XREQ_R {
        APORT4XREQ_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1 If the Bus Connected to APORT4Y is Requested"]
    #[inline(always)]
    pub fn aport4yreq(&self) -> APORT4YREQ_R {
        APORT4YREQ_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "APORT Request Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aportreq::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APORTREQ_SPEC;
impl crate::RegisterSpec for APORTREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aportreq::R`](R) reader structure"]
impl crate::Readable for APORTREQ_SPEC {}
#[doc = "`reset()` method sets APORTREQ to value 0"]
impl crate::Resettable for APORTREQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
