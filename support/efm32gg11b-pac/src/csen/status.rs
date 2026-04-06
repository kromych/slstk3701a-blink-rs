#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `CSENBUSY` reader - Busy Flag"]
pub type CsenbusyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Busy Flag"]
    #[inline(always)]
    pub fn csenbusy(&self) -> CsenbusyR {
        CsenbusyR::new((self.bits & 1) != 0)
    }
}
#[doc = "Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
