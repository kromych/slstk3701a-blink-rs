#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Field `SINGLE` reader - Single Conversion Complete Interrupt Flag"]
pub type SingleR = crate::BitReader;
#[doc = "Field `SCAN` reader - Scan Conversion Complete Interrupt Flag"]
pub type ScanR = crate::BitReader;
#[doc = "Field `SINGLEOF` reader - Single FIFO Overflow Interrupt Flag"]
pub type SingleofR = crate::BitReader;
#[doc = "Field `SCANOF` reader - Scan FIFO Overflow Interrupt Flag"]
pub type ScanofR = crate::BitReader;
#[doc = "Field `SINGLEUF` reader - Single FIFO Underflow Interrupt Flag"]
pub type SingleufR = crate::BitReader;
#[doc = "Field `SCANUF` reader - Scan FIFO Underflow Interrupt Flag"]
pub type ScanufR = crate::BitReader;
#[doc = "Field `SINGLECMP` reader - Single Result Compare Match Interrupt Flag"]
pub type SinglecmpR = crate::BitReader;
#[doc = "Field `SCANCMP` reader - Scan Result Compare Match Interrupt Flag"]
pub type ScancmpR = crate::BitReader;
#[doc = "Field `VREFOV` reader - VREF Over Voltage Interrupt Flag"]
pub type VrefovR = crate::BitReader;
#[doc = "Field `PROGERR` reader - Programming Error Interrupt Flag"]
pub type ProgerrR = crate::BitReader;
#[doc = "Field `SCANEXTPEND` reader - External Scan Trigger Pending Flag"]
pub type ScanextpendR = crate::BitReader;
#[doc = "Field `SCANPEND` reader - Scan Trigger Pending Flag"]
pub type ScanpendR = crate::BitReader;
#[doc = "Field `PRSTIMEDERR` reader - PRS Timed Mode Error Flag"]
pub type PrstimederrR = crate::BitReader;
#[doc = "Field `EM23ERR` reader - EM23 Entry Error Flag"]
pub type Em23errR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Single Conversion Complete Interrupt Flag"]
    #[inline(always)]
    pub fn single(&self) -> SingleR {
        SingleR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scan Conversion Complete Interrupt Flag"]
    #[inline(always)]
    pub fn scan(&self) -> ScanR {
        ScanR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Single FIFO Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn singleof(&self) -> SingleofR {
        SingleofR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Scan FIFO Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn scanof(&self) -> ScanofR {
        ScanofR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Single FIFO Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn singleuf(&self) -> SingleufR {
        SingleufR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Scan FIFO Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn scanuf(&self) -> ScanufR {
        ScanufR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Single Result Compare Match Interrupt Flag"]
    #[inline(always)]
    pub fn singlecmp(&self) -> SinglecmpR {
        SinglecmpR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Scan Result Compare Match Interrupt Flag"]
    #[inline(always)]
    pub fn scancmp(&self) -> ScancmpR {
        ScancmpR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - VREF Over Voltage Interrupt Flag"]
    #[inline(always)]
    pub fn vrefov(&self) -> VrefovR {
        VrefovR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Programming Error Interrupt Flag"]
    #[inline(always)]
    pub fn progerr(&self) -> ProgerrR {
        ProgerrR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - External Scan Trigger Pending Flag"]
    #[inline(always)]
    pub fn scanextpend(&self) -> ScanextpendR {
        ScanextpendR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Scan Trigger Pending Flag"]
    #[inline(always)]
    pub fn scanpend(&self) -> ScanpendR {
        ScanpendR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PRS Timed Mode Error Flag"]
    #[inline(always)]
    pub fn prstimederr(&self) -> PrstimederrR {
        PrstimederrR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - EM23 Entry Error Flag"]
    #[inline(always)]
    pub fn em23err(&self) -> Em23errR {
        Em23errR::new(((self.bits >> 29) & 1) != 0)
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
