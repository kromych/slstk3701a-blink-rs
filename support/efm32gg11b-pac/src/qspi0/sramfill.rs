#[doc = "Register `SRAMFILL` reader"]
pub type R = crate::R<SramfillSpec>;
#[doc = "Field `SRAMFILLINDACREAD` reader - SRAM Fill Level (Indirect Read Partition)"]
pub type SramfillindacreadR = crate::FieldReader<u16>;
#[doc = "Field `SRAMFILLINDACWRITE` reader - SRAM Fill Level (Indirect Write Partition)"]
pub type SramfillindacwriteR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - SRAM Fill Level (Indirect Read Partition)"]
    #[inline(always)]
    pub fn sramfillindacread(&self) -> SramfillindacreadR {
        SramfillindacreadR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - SRAM Fill Level (Indirect Write Partition)"]
    #[inline(always)]
    pub fn sramfillindacwrite(&self) -> SramfillindacwriteR {
        SramfillindacwriteR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "SRAM Fill Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sramfill::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SramfillSpec;
impl crate::RegisterSpec for SramfillSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sramfill::R`](R) reader structure"]
impl crate::Readable for SramfillSpec {}
#[doc = "`reset()` method sets SRAMFILL to value 0"]
impl crate::Resettable for SramfillSpec {}
