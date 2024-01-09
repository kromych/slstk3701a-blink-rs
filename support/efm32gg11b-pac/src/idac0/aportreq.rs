#[doc = "Register `APORTREQ` reader"]
pub type R = crate::R<APORTREQ_SPEC>;
#[doc = "Field `APORT1XREQ` reader - 1 If the APORT Bus Connected to APORT1X is Requested"]
pub type APORT1XREQ_R = crate::BitReader;
#[doc = "Field `APORT1YREQ` reader - 1 If the Bus Connected to APORT1Y is Requested"]
pub type APORT1YREQ_R = crate::BitReader;
impl R {
    #[doc = "Bit 2 - 1 If the APORT Bus Connected to APORT1X is Requested"]
    #[inline(always)]
    pub fn aport1xreq(&self) -> APORT1XREQ_R {
        APORT1XREQ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1 If the Bus Connected to APORT1Y is Requested"]
    #[inline(always)]
    pub fn aport1yreq(&self) -> APORT1YREQ_R {
        APORT1YREQ_R::new(((self.bits >> 3) & 1) != 0)
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
    const RESET_VALUE: u32 = 0;
}
