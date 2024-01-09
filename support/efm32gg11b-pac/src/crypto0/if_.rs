#[doc = "Register `IF` reader"]
pub type R = crate::R<IF_SPEC>;
#[doc = "Field `INSTRDONE` reader - Instruction Done"]
pub type INSTRDONE_R = crate::BitReader;
#[doc = "Field `SEQDONE` reader - Sequence Done"]
pub type SEQDONE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Instruction Done"]
    #[inline(always)]
    pub fn instrdone(&self) -> INSTRDONE_R {
        INSTRDONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sequence Done"]
    #[inline(always)]
    pub fn seqdone(&self) -> SEQDONE_R {
        SEQDONE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "AES Interrupt Flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if_::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
