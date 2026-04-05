#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Field `ERASE` reader - Erase Done Interrupt Read Flag"]
pub type EraseR = crate::BitReader;
#[doc = "Field `WRITE` reader - Write Done Interrupt Read Flag"]
pub type WriteR = crate::BitReader;
#[doc = "Field `CHOF` reader - Cache Hits Overflow Interrupt Flag"]
pub type ChofR = crate::BitReader;
#[doc = "Field `CMOF` reader - Cache Misses Overflow Interrupt Flag"]
pub type CmofR = crate::BitReader;
#[doc = "Field `PWRUPF` reader - Flash Power Up Sequence Complete Flag"]
pub type PwrupfR = crate::BitReader;
#[doc = "Field `ICACHERR` reader - ICache RAM Parity Error Flag"]
pub type IcacherrR = crate::BitReader;
#[doc = "Field `WDATAOV` reader - Flash Controller Write Buffer Overflow"]
pub type WdataovR = crate::BitReader;
#[doc = "Field `LVEWRITE` reader - Flash LVE Write Error Flag"]
pub type LvewriteR = crate::BitReader;
#[doc = "Field `RAMERR1B` reader - RAM 1-bit ECC Error Interrupt Flag"]
pub type Ramerr1bR = crate::BitReader;
#[doc = "Field `RAMERR2B` reader - RAM 2-bit ECC Error Interrupt Flag"]
pub type Ramerr2bR = crate::BitReader;
#[doc = "Field `RAM1ERR1B` reader - RAM1 1-bit ECC Error Interrupt Flag"]
pub type Ram1err1bR = crate::BitReader;
#[doc = "Field `RAM1ERR2B` reader - RAM1 2-bit ECC Error Interrupt Flag"]
pub type Ram1err2bR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Erase Done Interrupt Read Flag"]
    #[inline(always)]
    pub fn erase(&self) -> EraseR {
        EraseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write Done Interrupt Read Flag"]
    #[inline(always)]
    pub fn write(&self) -> WriteR {
        WriteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Cache Hits Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn chof(&self) -> ChofR {
        ChofR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Cache Misses Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn cmof(&self) -> CmofR {
        CmofR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Flash Power Up Sequence Complete Flag"]
    #[inline(always)]
    pub fn pwrupf(&self) -> PwrupfR {
        PwrupfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ICache RAM Parity Error Flag"]
    #[inline(always)]
    pub fn icacherr(&self) -> IcacherrR {
        IcacherrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Flash Controller Write Buffer Overflow"]
    #[inline(always)]
    pub fn wdataov(&self) -> WdataovR {
        WdataovR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash LVE Write Error Flag"]
    #[inline(always)]
    pub fn lvewrite(&self) -> LvewriteR {
        LvewriteR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - RAM 1-bit ECC Error Interrupt Flag"]
    #[inline(always)]
    pub fn ramerr1b(&self) -> Ramerr1bR {
        Ramerr1bR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RAM 2-bit ECC Error Interrupt Flag"]
    #[inline(always)]
    pub fn ramerr2b(&self) -> Ramerr2bR {
        Ramerr2bR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RAM1 1-bit ECC Error Interrupt Flag"]
    #[inline(always)]
    pub fn ram1err1b(&self) -> Ram1err1bR {
        Ram1err1bR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - RAM1 2-bit ECC Error Interrupt Flag"]
    #[inline(always)]
    pub fn ram1err2b(&self) -> Ram1err2bR {
        Ram1err2bR::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfSpec;
impl crate::RegisterSpec for IfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IfSpec {}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IfSpec {}
