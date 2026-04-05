#[doc = "Register `APORTREQ` reader"]
pub type R = crate::R<AportreqSpec>;
#[doc = "Field `APORT1XREQ` reader - 1 If the Bus Connected to APORT2X is Requested"]
pub type Aport1xreqR = crate::BitReader;
#[doc = "Field `APORT1YREQ` reader - 1 If the Bus Connected to APORT1X is Requested"]
pub type Aport1yreqR = crate::BitReader;
#[doc = "Field `APORT2XREQ` reader - 1 If the Bus Connected to APORT2X is Requested"]
pub type Aport2xreqR = crate::BitReader;
#[doc = "Field `APORT2YREQ` reader - 1 If the Bus Connected to APORT2Y is Requested"]
pub type Aport2yreqR = crate::BitReader;
#[doc = "Field `APORT3XREQ` reader - 1 If the Bus Connected to APORT3X is Requested"]
pub type Aport3xreqR = crate::BitReader;
#[doc = "Field `APORT3YREQ` reader - 1 If the Bus Connected to APORT3Y is Requested"]
pub type Aport3yreqR = crate::BitReader;
#[doc = "Field `APORT4XREQ` reader - 1 If the Bus Connected to APORT4X is Requested"]
pub type Aport4xreqR = crate::BitReader;
#[doc = "Field `APORT4YREQ` reader - 1 If the Bus Connected to APORT4Y is Requested"]
pub type Aport4yreqR = crate::BitReader;
impl R {
    #[doc = "Bit 2 - 1 If the Bus Connected to APORT2X is Requested"]
    #[inline(always)]
    pub fn aport1xreq(&self) -> Aport1xreqR {
        Aport1xreqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1 If the Bus Connected to APORT1X is Requested"]
    #[inline(always)]
    pub fn aport1yreq(&self) -> Aport1yreqR {
        Aport1yreqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1 If the Bus Connected to APORT2X is Requested"]
    #[inline(always)]
    pub fn aport2xreq(&self) -> Aport2xreqR {
        Aport2xreqR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1 If the Bus Connected to APORT2Y is Requested"]
    #[inline(always)]
    pub fn aport2yreq(&self) -> Aport2yreqR {
        Aport2yreqR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1 If the Bus Connected to APORT3X is Requested"]
    #[inline(always)]
    pub fn aport3xreq(&self) -> Aport3xreqR {
        Aport3xreqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1 If the Bus Connected to APORT3Y is Requested"]
    #[inline(always)]
    pub fn aport3yreq(&self) -> Aport3yreqR {
        Aport3yreqR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1 If the Bus Connected to APORT4X is Requested"]
    #[inline(always)]
    pub fn aport4xreq(&self) -> Aport4xreqR {
        Aport4xreqR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1 If the Bus Connected to APORT4Y is Requested"]
    #[inline(always)]
    pub fn aport4yreq(&self) -> Aport4yreqR {
        Aport4yreqR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "APORT Request Status\n\nYou can [`read`](crate::Reg::read) this register and get [`aportreq::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AportreqSpec;
impl crate::RegisterSpec for AportreqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aportreq::R`](R) reader structure"]
impl crate::Readable for AportreqSpec {}
#[doc = "`reset()` method sets APORTREQ to value 0"]
impl crate::Resettable for AportreqSpec {}
