#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `SEQRUNNING` reader - AES SEQUENCE Running"]
pub type SeqrunningR = crate::BitReader;
#[doc = "Field `INSTRRUNNING` reader - Action is Active"]
pub type InstrrunningR = crate::BitReader;
#[doc = "Field `DMAACTIVE` reader - DMA Action is Active"]
pub type DmaactiveR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - AES SEQUENCE Running"]
    #[inline(always)]
    pub fn seqrunning(&self) -> SeqrunningR {
        SeqrunningR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Action is Active"]
    #[inline(always)]
    pub fn instrrunning(&self) -> InstrrunningR {
        InstrrunningR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Action is Active"]
    #[inline(always)]
    pub fn dmaactive(&self) -> DmaactiveR {
        DmaactiveR::new(((self.bits >> 2) & 1) != 0)
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
