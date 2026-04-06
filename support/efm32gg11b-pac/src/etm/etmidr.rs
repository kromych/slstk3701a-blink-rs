#[doc = "Register `ETMIDR` reader"]
pub type R = crate::R<EtmidrSpec>;
#[doc = "Field `IMPVER` reader - Implementation Revision"]
pub type ImpverR = crate::FieldReader;
#[doc = "Field `ETMMINVER` reader - Minor ETM Architecture Version"]
pub type EtmminverR = crate::FieldReader;
#[doc = "Field `ETMMAJVER` reader - Major ETM Architecture Version"]
pub type EtmmajverR = crate::FieldReader;
#[doc = "Field `PROCFAM` reader - Implementer Code"]
pub type ProcfamR = crate::FieldReader;
#[doc = "Field `LPCF` reader - Load PC First"]
pub type LpcfR = crate::BitReader;
#[doc = "Field `THUMBT` reader - 32-bit Thumb Instruction Tracing"]
pub type ThumbtR = crate::BitReader;
#[doc = "Field `SECEXT` reader - Security Extension Support"]
pub type SecextR = crate::BitReader;
#[doc = "Field `BPE` reader - Branch Packet Encoding"]
pub type BpeR = crate::BitReader;
#[doc = "Field `IMPCODE` reader - Implementer Code"]
pub type ImpcodeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Implementation Revision"]
    #[inline(always)]
    pub fn impver(&self) -> ImpverR {
        ImpverR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Minor ETM Architecture Version"]
    #[inline(always)]
    pub fn etmminver(&self) -> EtmminverR {
        EtmminverR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Major ETM Architecture Version"]
    #[inline(always)]
    pub fn etmmajver(&self) -> EtmmajverR {
        EtmmajverR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Implementer Code"]
    #[inline(always)]
    pub fn procfam(&self) -> ProcfamR {
        ProcfamR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Load PC First"]
    #[inline(always)]
    pub fn lpcf(&self) -> LpcfR {
        LpcfR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - 32-bit Thumb Instruction Tracing"]
    #[inline(always)]
    pub fn thumbt(&self) -> ThumbtR {
        ThumbtR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Security Extension Support"]
    #[inline(always)]
    pub fn secext(&self) -> SecextR {
        SecextR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Branch Packet Encoding"]
    #[inline(always)]
    pub fn bpe(&self) -> BpeR {
        BpeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Implementer Code"]
    #[inline(always)]
    pub fn impcode(&self) -> ImpcodeR {
        ImpcodeR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtmidrSpec;
impl crate::RegisterSpec for EtmidrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmidr::R`](R) reader structure"]
impl crate::Readable for EtmidrSpec {}
#[doc = "`reset()` method sets ETMIDR to value 0x4114_f253"]
impl crate::Resettable for EtmidrSpec {
    const RESET_VALUE: u32 = 0x4114_f253;
}
