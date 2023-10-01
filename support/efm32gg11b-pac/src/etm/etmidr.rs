#[doc = "Register `ETMIDR` reader"]
pub type R = crate::R<ETMIDR_SPEC>;
#[doc = "Field `IMPVER` reader - Implementation Revision"]
pub type IMPVER_R = crate::FieldReader;
#[doc = "Field `ETMMINVER` reader - Minor ETM Architecture Version"]
pub type ETMMINVER_R = crate::FieldReader;
#[doc = "Field `ETMMAJVER` reader - Major ETM Architecture Version"]
pub type ETMMAJVER_R = crate::FieldReader;
#[doc = "Field `PROCFAM` reader - Implementer Code"]
pub type PROCFAM_R = crate::FieldReader;
#[doc = "Field `LPCF` reader - Load PC First"]
pub type LPCF_R = crate::BitReader;
#[doc = "Field `THUMBT` reader - 32-bit Thumb Instruction Tracing"]
pub type THUMBT_R = crate::BitReader;
#[doc = "Field `SECEXT` reader - Security Extension Support"]
pub type SECEXT_R = crate::BitReader;
#[doc = "Field `BPE` reader - Branch Packet Encoding"]
pub type BPE_R = crate::BitReader;
#[doc = "Field `IMPCODE` reader - Implementer Code"]
pub type IMPCODE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Implementation Revision"]
    #[inline(always)]
    pub fn impver(&self) -> IMPVER_R {
        IMPVER_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Minor ETM Architecture Version"]
    #[inline(always)]
    pub fn etmminver(&self) -> ETMMINVER_R {
        ETMMINVER_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Major ETM Architecture Version"]
    #[inline(always)]
    pub fn etmmajver(&self) -> ETMMAJVER_R {
        ETMMAJVER_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Implementer Code"]
    #[inline(always)]
    pub fn procfam(&self) -> PROCFAM_R {
        PROCFAM_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Load PC First"]
    #[inline(always)]
    pub fn lpcf(&self) -> LPCF_R {
        LPCF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - 32-bit Thumb Instruction Tracing"]
    #[inline(always)]
    pub fn thumbt(&self) -> THUMBT_R {
        THUMBT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Security Extension Support"]
    #[inline(always)]
    pub fn secext(&self) -> SECEXT_R {
        SECEXT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Branch Packet Encoding"]
    #[inline(always)]
    pub fn bpe(&self) -> BPE_R {
        BPE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Implementer Code"]
    #[inline(always)]
    pub fn impcode(&self) -> IMPCODE_R {
        IMPCODE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETMIDR_SPEC;
impl crate::RegisterSpec for ETMIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmidr::R`](R) reader structure"]
impl crate::Readable for ETMIDR_SPEC {}
#[doc = "`reset()` method sets ETMIDR to value 0x4114_f253"]
impl crate::Resettable for ETMIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x4114_f253;
}
