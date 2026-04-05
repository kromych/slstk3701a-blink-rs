#[doc = "Register `ETMPDSR` reader"]
pub type R = crate::R<EtmpdsrSpec>;
#[doc = "Field `ETMUP` reader - ETM Powered Up"]
pub type EtmupR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ETM Powered Up"]
    #[inline(always)]
    pub fn etmup(&self) -> EtmupR {
        EtmupR::new((self.bits & 1) != 0)
    }
}
#[doc = "Device Power-down Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmpdsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtmpdsrSpec;
impl crate::RegisterSpec for EtmpdsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmpdsr::R`](R) reader structure"]
impl crate::Readable for EtmpdsrSpec {}
#[doc = "`reset()` method sets ETMPDSR to value 0x01"]
impl crate::Resettable for EtmpdsrSpec {
    const RESET_VALUE: u32 = 0x01;
}
