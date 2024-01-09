#[doc = "Register `R5VSYNC` reader"]
pub type R = crate::R<R5VSYNC_SPEC>;
#[doc = "Field `OUTLEVELBUSY` reader - 5V Regulator Voltage Register Transfer Busy"]
pub type OUTLEVELBUSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 5V Regulator Voltage Register Transfer Busy"]
    #[inline(always)]
    pub fn outlevelbusy(&self) -> OUTLEVELBUSY_R {
        OUTLEVELBUSY_R::new((self.bits & 1) != 0)
    }
}
#[doc = "5V Read Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r5vsync::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R5VSYNC_SPEC;
impl crate::RegisterSpec for R5VSYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r5vsync::R`](R) reader structure"]
impl crate::Readable for R5VSYNC_SPEC {}
#[doc = "`reset()` method sets R5VSYNC to value 0"]
impl crate::Resettable for R5VSYNC_SPEC {
    const RESET_VALUE: u32 = 0;
}
