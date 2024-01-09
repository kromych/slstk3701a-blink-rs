#[doc = "Register `MAXCURCAPAB` reader"]
pub type R = crate::R<MAXCURCAPAB_SPEC>;
#[doc = "Field `MAXCUR3P3VAL` reader - Maximum Current for 3.3V"]
pub type MAXCUR3P3VAL_R = crate::FieldReader;
#[doc = "Field `MAXCUR3P0VAL` reader - Maximum Current for 3.0V"]
pub type MAXCUR3P0VAL_R = crate::FieldReader;
#[doc = "Field `MAXCUR1P8VAL` reader - Maximum Current for 1.8V"]
pub type MAXCUR1P8VAL_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Maximum Current for 3.3V"]
    #[inline(always)]
    pub fn maxcur3p3val(&self) -> MAXCUR3P3VAL_R {
        MAXCUR3P3VAL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Maximum Current for 3.0V"]
    #[inline(always)]
    pub fn maxcur3p0val(&self) -> MAXCUR3P0VAL_R {
        MAXCUR3P0VAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Maximum Current for 1.8V"]
    #[inline(always)]
    pub fn maxcur1p8val(&self) -> MAXCUR1P8VAL_R {
        MAXCUR1P8VAL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Maximum Current Capabilities Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maxcurcapab::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAXCURCAPAB_SPEC;
impl crate::RegisterSpec for MAXCURCAPAB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maxcurcapab::R`](R) reader structure"]
impl crate::Readable for MAXCURCAPAB_SPEC {}
#[doc = "`reset()` method sets MAXCURCAPAB to value 0"]
impl crate::Resettable for MAXCURCAPAB_SPEC {
    const RESET_VALUE: u32 = 0;
}
