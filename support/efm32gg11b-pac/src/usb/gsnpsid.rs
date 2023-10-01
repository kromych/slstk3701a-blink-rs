#[doc = "Register `GSNPSID` reader"]
pub type R = crate::R<GSNPSID_SPEC>;
#[doc = "Field `SYNOPSYSID` reader - "]
pub type SYNOPSYSID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn synopsysid(&self) -> SYNOPSYSID_R {
        SYNOPSYSID_R::new(self.bits)
    }
}
#[doc = "Synopsys ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gsnpsid::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GSNPSID_SPEC;
impl crate::RegisterSpec for GSNPSID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsnpsid::R`](R) reader structure"]
impl crate::Readable for GSNPSID_SPEC {}
#[doc = "`reset()` method sets GSNPSID to value 0x4f54_330a"]
impl crate::Resettable for GSNPSID_SPEC {
    const RESET_VALUE: Self::Ux = 0x4f54_330a;
}
