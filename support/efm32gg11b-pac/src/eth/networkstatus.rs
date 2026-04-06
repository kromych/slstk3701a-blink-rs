#[doc = "Register `NETWORKSTATUS` reader"]
pub type R = crate::R<NetworkstatusSpec>;
#[doc = "Field `MDIOIN` reader - Returns status of the mdio_in pin."]
pub type MdioinR = crate::BitReader;
#[doc = "Field `MANDONE` reader - The PHY management logic is idle (i.e. has completed)."]
pub type MandoneR = crate::BitReader;
#[doc = "Field `PFCNEGOTIATE` reader - Set when PFC Priority Based Pause has been negotiated."]
pub type PfcnegotiateR = crate::BitReader;
#[doc = "Field `LPIINDICATE` reader - LPI Indication"]
pub type LpiindicateR = crate::BitReader;
impl R {
    #[doc = "Bit 1 - Returns status of the mdio_in pin."]
    #[inline(always)]
    pub fn mdioin(&self) -> MdioinR {
        MdioinR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The PHY management logic is idle (i.e. has completed)."]
    #[inline(always)]
    pub fn mandone(&self) -> MandoneR {
        MandoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Set when PFC Priority Based Pause has been negotiated."]
    #[inline(always)]
    pub fn pfcnegotiate(&self) -> PfcnegotiateR {
        PfcnegotiateR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LPI Indication"]
    #[inline(always)]
    pub fn lpiindicate(&self) -> LpiindicateR {
        LpiindicateR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Network status register\n\nYou can [`read`](crate::Reg::read) this register and get [`networkstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NetworkstatusSpec;
impl crate::RegisterSpec for NetworkstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`networkstatus::R`](R) reader structure"]
impl crate::Readable for NetworkstatusSpec {}
#[doc = "`reset()` method sets NETWORKSTATUS to value 0x04"]
impl crate::Resettable for NetworkstatusSpec {
    const RESET_VALUE: u32 = 0x04;
}
