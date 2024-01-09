#[doc = "Register `NETWORKSTATUS` reader"]
pub type R = crate::R<NETWORKSTATUS_SPEC>;
#[doc = "Field `MDIOIN` reader - Returns status of the mdio_in pin."]
pub type MDIOIN_R = crate::BitReader;
#[doc = "Field `MANDONE` reader - The PHY management logic is idle (i.e. has completed)."]
pub type MANDONE_R = crate::BitReader;
#[doc = "Field `PFCNEGOTIATE` reader - Set when PFC Priority Based Pause has been negotiated."]
pub type PFCNEGOTIATE_R = crate::BitReader;
#[doc = "Field `LPIINDICATE` reader - LPI Indication"]
pub type LPIINDICATE_R = crate::BitReader;
impl R {
    #[doc = "Bit 1 - Returns status of the mdio_in pin."]
    #[inline(always)]
    pub fn mdioin(&self) -> MDIOIN_R {
        MDIOIN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The PHY management logic is idle (i.e. has completed)."]
    #[inline(always)]
    pub fn mandone(&self) -> MANDONE_R {
        MANDONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Set when PFC Priority Based Pause has been negotiated."]
    #[inline(always)]
    pub fn pfcnegotiate(&self) -> PFCNEGOTIATE_R {
        PFCNEGOTIATE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LPI Indication"]
    #[inline(always)]
    pub fn lpiindicate(&self) -> LPIINDICATE_R {
        LPIINDICATE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Network status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`networkstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NETWORKSTATUS_SPEC;
impl crate::RegisterSpec for NETWORKSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`networkstatus::R`](R) reader structure"]
impl crate::Readable for NETWORKSTATUS_SPEC {}
#[doc = "`reset()` method sets NETWORKSTATUS to value 0x04"]
impl crate::Resettable for NETWORKSTATUS_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
