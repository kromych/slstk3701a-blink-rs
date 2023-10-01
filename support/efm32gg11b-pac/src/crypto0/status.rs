#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `SEQRUNNING` reader - AES SEQUENCE Running"]
pub type SEQRUNNING_R = crate::BitReader;
#[doc = "Field `INSTRRUNNING` reader - Action is Active"]
pub type INSTRRUNNING_R = crate::BitReader;
#[doc = "Field `DMAACTIVE` reader - DMA Action is Active"]
pub type DMAACTIVE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - AES SEQUENCE Running"]
    #[inline(always)]
    pub fn seqrunning(&self) -> SEQRUNNING_R {
        SEQRUNNING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Action is Active"]
    #[inline(always)]
    pub fn instrrunning(&self) -> INSTRRUNNING_R {
        INSTRRUNNING_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Action is Active"]
    #[inline(always)]
    pub fn dmaactive(&self) -> DMAACTIVE_R {
        DMAACTIVE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
