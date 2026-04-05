#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Field `CMP` reader - Digital Comparator Interrupt Flag"]
pub type CmpR = crate::BitReader;
#[doc = "Field `CONV` reader - Conversion Done Interrupt Flag"]
pub type ConvR = crate::BitReader;
#[doc = "Field `EOS` reader - End of Scan Interrupt Flag."]
pub type EosR = crate::BitReader;
#[doc = "Field `DMAOF` reader - DMA Overflow Interrupt Flag."]
pub type DmaofR = crate::BitReader;
#[doc = "Field `APORTCONFLICT` reader - APORT Conflict Interrupt Flag"]
pub type AportconflictR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Digital Comparator Interrupt Flag"]
    #[inline(always)]
    pub fn cmp(&self) -> CmpR {
        CmpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Conversion Done Interrupt Flag"]
    #[inline(always)]
    pub fn conv(&self) -> ConvR {
        ConvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of Scan Interrupt Flag."]
    #[inline(always)]
    pub fn eos(&self) -> EosR {
        EosR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Overflow Interrupt Flag."]
    #[inline(always)]
    pub fn dmaof(&self) -> DmaofR {
        DmaofR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - APORT Conflict Interrupt Flag"]
    #[inline(always)]
    pub fn aportconflict(&self) -> AportconflictR {
        AportconflictR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfSpec;
impl crate::RegisterSpec for IfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IfSpec {}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IfSpec {}
