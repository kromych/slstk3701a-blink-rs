#[doc = "Register `RAMECCADDR` reader"]
pub type R = crate::R<RameccaddrSpec>;
#[doc = "Field `RAMECCADDR` reader - RAM ECC Error Address"]
pub type RameccaddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RAM ECC Error Address"]
    #[inline(always)]
    pub fn rameccaddr(&self) -> RameccaddrR {
        RameccaddrR::new(self.bits)
    }
}
#[doc = "RAM ECC Error Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rameccaddr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RameccaddrSpec;
impl crate::RegisterSpec for RameccaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rameccaddr::R`](R) reader structure"]
impl crate::Readable for RameccaddrSpec {}
#[doc = "`reset()` method sets RAMECCADDR to value 0x2000_0000"]
impl crate::Resettable for RameccaddrSpec {
    const RESET_VALUE: u32 = 0x2000_0000;
}
