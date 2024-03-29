#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SYNCBUSY_SPEC>;
#[doc = "Field `CMD` reader - CMD Register Busy"]
pub type CMD_R = crate::BitReader;
impl R {
    #[doc = "Bit 7 - CMD Register Busy"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Synchronization Busy Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncbusy::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYNCBUSY_SPEC;
impl crate::RegisterSpec for SYNCBUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncbusy::R`](R) reader structure"]
impl crate::Readable for SYNCBUSY_SPEC {}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SYNCBUSY_SPEC {
    const RESET_VALUE: u32 = 0;
}
