#[doc = "Register `SRAMFILL` reader"]
pub type R = crate::R<SRAMFILL_SPEC>;
#[doc = "Field `SRAMFILLINDACREAD` reader - SRAM Fill Level (Indirect Read Partition)"]
pub type SRAMFILLINDACREAD_R = crate::FieldReader<u16>;
#[doc = "Field `SRAMFILLINDACWRITE` reader - SRAM Fill Level (Indirect Write Partition)"]
pub type SRAMFILLINDACWRITE_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - SRAM Fill Level (Indirect Read Partition)"]
    #[inline(always)]
    pub fn sramfillindacread(&self) -> SRAMFILLINDACREAD_R {
        SRAMFILLINDACREAD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - SRAM Fill Level (Indirect Write Partition)"]
    #[inline(always)]
    pub fn sramfillindacwrite(&self) -> SRAMFILLINDACWRITE_R {
        SRAMFILLINDACWRITE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "SRAM Fill Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sramfill::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRAMFILL_SPEC;
impl crate::RegisterSpec for SRAMFILL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sramfill::R`](R) reader structure"]
impl crate::Readable for SRAMFILL_SPEC {}
#[doc = "`reset()` method sets SRAMFILL to value 0"]
impl crate::Resettable for SRAMFILL_SPEC {
    const RESET_VALUE: u32 = 0;
}
