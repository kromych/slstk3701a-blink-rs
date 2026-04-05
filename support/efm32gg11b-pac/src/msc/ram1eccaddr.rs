#[doc = "Register `RAM1ECCADDR` reader"]
pub type R = crate::R<Ram1eccaddrSpec>;
#[doc = "Field `RAM1ECCADDR` reader - RAM1 ECC Error Address"]
pub type Ram1eccaddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RAM1 ECC Error Address"]
    #[inline(always)]
    pub fn ram1eccaddr(&self) -> Ram1eccaddrR {
        Ram1eccaddrR::new(self.bits)
    }
}
#[doc = "RAM1 ECC Error Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ram1eccaddr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ram1eccaddrSpec;
impl crate::RegisterSpec for Ram1eccaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ram1eccaddr::R`](R) reader structure"]
impl crate::Readable for Ram1eccaddrSpec {}
#[doc = "`reset()` method sets RAM1ECCADDR to value 0"]
impl crate::Resettable for Ram1eccaddrSpec {}
