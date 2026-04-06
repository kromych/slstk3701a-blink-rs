#[doc = "Register `CC2_CCVP` reader"]
pub type R = crate::R<Cc2CcvpSpec>;
#[doc = "Field `CCVP` reader - CC Channel Value Peek"]
pub type CcvpR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - CC Channel Value Peek"]
    #[inline(always)]
    pub fn ccvp(&self) -> CcvpR {
        CcvpR::new(self.bits)
    }
}
#[doc = "CC Channel Value Peek Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc2_ccvp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cc2CcvpSpec;
impl crate::RegisterSpec for Cc2CcvpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc2_ccvp::R`](R) reader structure"]
impl crate::Readable for Cc2CcvpSpec {}
#[doc = "`reset()` method sets CC2_CCVP to value 0"]
impl crate::Resettable for Cc2CcvpSpec {}
