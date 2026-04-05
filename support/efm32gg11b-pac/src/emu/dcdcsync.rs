#[doc = "Register `DCDCSYNC` reader"]
pub type R = crate::R<DcdcsyncSpec>;
#[doc = "Field `DCDCCTRLBUSY` reader - DCDC CTRL Register Transfer Busy"]
pub type DcdcctrlbusyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DCDC CTRL Register Transfer Busy"]
    #[inline(always)]
    pub fn dcdcctrlbusy(&self) -> DcdcctrlbusyR {
        DcdcctrlbusyR::new((self.bits & 1) != 0)
    }
}
#[doc = "DCDC Read Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdcsync::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcdcsyncSpec;
impl crate::RegisterSpec for DcdcsyncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdcsync::R`](R) reader structure"]
impl crate::Readable for DcdcsyncSpec {}
#[doc = "`reset()` method sets DCDCSYNC to value 0"]
impl crate::Resettable for DcdcsyncSpec {}
