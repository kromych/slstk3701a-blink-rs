#[doc = "Register `ERRCNT` reader"]
pub type R = crate::R<ErrcntSpec>;
#[doc = "Field `TEC` reader - Transmit Error Counter"]
pub type TecR = crate::FieldReader;
#[doc = "Field `REC` reader - Receive Error Counter"]
pub type RecR = crate::FieldReader;
#[doc = "Field `RECERRP` reader - Receive Error Passive"]
pub type RecerrpR = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - Transmit Error Counter"]
    #[inline(always)]
    pub fn tec(&self) -> TecR {
        TecR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - Receive Error Counter"]
    #[inline(always)]
    pub fn rec(&self) -> RecR {
        RecR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Receive Error Passive"]
    #[inline(always)]
    pub fn recerrp(&self) -> RecerrpR {
        RecerrpR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Error Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`errcnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrcntSpec;
impl crate::RegisterSpec for ErrcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errcnt::R`](R) reader structure"]
impl crate::Readable for ErrcntSpec {}
#[doc = "`reset()` method sets ERRCNT to value 0"]
impl crate::Resettable for ErrcntSpec {}
