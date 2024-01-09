#[doc = "Register `CHBUSY` reader"]
pub type R = crate::R<CHBUSY_SPEC>;
#[doc = "Field `BUSY` reader - Channels Busy"]
pub type BUSY_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Channels Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "DMA Channel Busy Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chbusy::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHBUSY_SPEC;
impl crate::RegisterSpec for CHBUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chbusy::R`](R) reader structure"]
impl crate::Readable for CHBUSY_SPEC {}
#[doc = "`reset()` method sets CHBUSY to value 0"]
impl crate::Resettable for CHBUSY_SPEC {
    const RESET_VALUE: u32 = 0;
}
