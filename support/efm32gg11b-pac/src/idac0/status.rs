#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `CURSTABLE` reader - IDAC Output Current Stable"]
pub type CurstableR = crate::BitReader;
#[doc = "Field `APORTCONFLICT` reader - APORT Conflict Output"]
pub type AportconflictR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - IDAC Output Current Stable"]
    #[inline(always)]
    pub fn curstable(&self) -> CurstableR {
        CurstableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - APORT Conflict Output"]
    #[inline(always)]
    pub fn aportconflict(&self) -> AportconflictR {
        AportconflictR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
