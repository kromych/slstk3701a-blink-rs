#[doc = "Register `GSNPSID` reader"]
pub type R = crate::R<GsnpsidSpec>;
#[doc = "Field `SYNOPSYSID` reader - "]
pub type SynopsysidR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn synopsysid(&self) -> SynopsysidR {
        SynopsysidR::new(self.bits)
    }
}
#[doc = "Synopsys ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gsnpsid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GsnpsidSpec;
impl crate::RegisterSpec for GsnpsidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsnpsid::R`](R) reader structure"]
impl crate::Readable for GsnpsidSpec {}
#[doc = "`reset()` method sets GSNPSID to value 0x4f54_330a"]
impl crate::Resettable for GsnpsidSpec {
    const RESET_VALUE: u32 = 0x4f54_330a;
}
