#[doc = "Register `IF` reader"]
pub type R = crate::R<IF_SPEC>;
#[doc = "Field `CMP` reader - Digital Comparator Interrupt Flag"]
pub type CMP_R = crate::BitReader;
#[doc = "Field `CONV` reader - Conversion Done Interrupt Flag"]
pub type CONV_R = crate::BitReader;
#[doc = "Field `EOS` reader - End of Scan Interrupt Flag."]
pub type EOS_R = crate::BitReader;
#[doc = "Field `DMAOF` reader - DMA Overflow Interrupt Flag."]
pub type DMAOF_R = crate::BitReader;
#[doc = "Field `APORTCONFLICT` reader - APORT Conflict Interrupt Flag"]
pub type APORTCONFLICT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Digital Comparator Interrupt Flag"]
    #[inline(always)]
    pub fn cmp(&self) -> CMP_R {
        CMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Conversion Done Interrupt Flag"]
    #[inline(always)]
    pub fn conv(&self) -> CONV_R {
        CONV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of Scan Interrupt Flag."]
    #[inline(always)]
    pub fn eos(&self) -> EOS_R {
        EOS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Overflow Interrupt Flag."]
    #[inline(always)]
    pub fn dmaof(&self) -> DMAOF_R {
        DMAOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - APORT Conflict Interrupt Flag"]
    #[inline(always)]
    pub fn aportconflict(&self) -> APORTCONFLICT_R {
        APORTCONFLICT_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Interrupt Flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if_::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IF_SPEC;
impl crate::RegisterSpec for IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IF_SPEC {}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IF_SPEC {
    const RESET_VALUE: u32 = 0;
}
