#[doc = "Register `ERRCNT` reader"]
pub type R = crate::R<ERRCNT_SPEC>;
#[doc = "Field `TEC` reader - Transmit Error Counter"]
pub type TEC_R = crate::FieldReader;
#[doc = "Field `REC` reader - Receive Error Counter"]
pub type REC_R = crate::FieldReader;
#[doc = "Field `RECERRP` reader - Receive Error Passive"]
pub type RECERRP_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - Transmit Error Counter"]
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - Receive Error Counter"]
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Receive Error Passive"]
    #[inline(always)]
    pub fn recerrp(&self) -> RECERRP_R {
        RECERRP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Error Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`errcnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ERRCNT_SPEC;
impl crate::RegisterSpec for ERRCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errcnt::R`](R) reader structure"]
impl crate::Readable for ERRCNT_SPEC {}
#[doc = "`reset()` method sets ERRCNT to value 0"]
impl crate::Resettable for ERRCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
