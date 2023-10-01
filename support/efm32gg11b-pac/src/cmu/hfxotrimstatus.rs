#[doc = "Register `HFXOTRIMSTATUS` reader"]
pub type R = crate::R<HFXOTRIMSTATUS_SPEC>;
#[doc = "Field `IBTRIMXOCORE` reader - Value of IBTRIMXOCORE Found By Automatic HFXO Peak Detection Algorithm"]
pub type IBTRIMXOCORE_R = crate::FieldReader<u16>;
#[doc = "Field `IBTRIMXOCOREMON` reader - Value of IBTRIMXOCORE Found By Automatic HFXO Peak Detection Algorithm or Peak Monitoring Algorithm (completion of Either Algorithm Will Cause an Update of IBTRIMXOCOREMON)"]
pub type IBTRIMXOCOREMON_R = crate::FieldReader<u16>;
#[doc = "Field `VALID` reader - Peak Detection Algorithm Found a Value for IBTRIMXOCORE"]
pub type VALID_R = crate::BitReader;
#[doc = "Field `MONVALID` reader - Peak Detection Algorithm or Peak Monitoring Algorithm Found a Value for IBTRIMXOCOREMON"]
pub type MONVALID_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:10 - Value of IBTRIMXOCORE Found By Automatic HFXO Peak Detection Algorithm"]
    #[inline(always)]
    pub fn ibtrimxocore(&self) -> IBTRIMXOCORE_R {
        IBTRIMXOCORE_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Value of IBTRIMXOCORE Found By Automatic HFXO Peak Detection Algorithm or Peak Monitoring Algorithm (completion of Either Algorithm Will Cause an Update of IBTRIMXOCOREMON)"]
    #[inline(always)]
    pub fn ibtrimxocoremon(&self) -> IBTRIMXOCOREMON_R {
        IBTRIMXOCOREMON_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 30 - Peak Detection Algorithm Found a Value for IBTRIMXOCORE"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Peak Detection Algorithm or Peak Monitoring Algorithm Found a Value for IBTRIMXOCOREMON"]
    #[inline(always)]
    pub fn monvalid(&self) -> MONVALID_R {
        MONVALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "HFXO Trim Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfxotrimstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFXOTRIMSTATUS_SPEC;
impl crate::RegisterSpec for HFXOTRIMSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfxotrimstatus::R`](R) reader structure"]
impl crate::Readable for HFXOTRIMSTATUS_SPEC {}
#[doc = "`reset()` method sets HFXOTRIMSTATUS to value 0"]
impl crate::Resettable for HFXOTRIMSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
