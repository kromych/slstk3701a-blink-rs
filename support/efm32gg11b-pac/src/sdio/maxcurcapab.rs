#[doc = "Register `MAXCURCAPAB` reader"]
pub type R = crate::R<MaxcurcapabSpec>;
#[doc = "Field `MAXCUR3P3VAL` reader - Maximum Current for 3.3V"]
pub type Maxcur3p3valR = crate::FieldReader;
#[doc = "Field `MAXCUR3P0VAL` reader - Maximum Current for 3.0V"]
pub type Maxcur3p0valR = crate::FieldReader;
#[doc = "Field `MAXCUR1P8VAL` reader - Maximum Current for 1.8V"]
pub type Maxcur1p8valR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Maximum Current for 3.3V"]
    #[inline(always)]
    pub fn maxcur3p3val(&self) -> Maxcur3p3valR {
        Maxcur3p3valR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Maximum Current for 3.0V"]
    #[inline(always)]
    pub fn maxcur3p0val(&self) -> Maxcur3p0valR {
        Maxcur3p0valR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Maximum Current for 1.8V"]
    #[inline(always)]
    pub fn maxcur1p8val(&self) -> Maxcur1p8valR {
        Maxcur1p8valR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Maximum Current Capabilities Register\n\nYou can [`read`](crate::Reg::read) this register and get [`maxcurcapab::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaxcurcapabSpec;
impl crate::RegisterSpec for MaxcurcapabSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maxcurcapab::R`](R) reader structure"]
impl crate::Readable for MaxcurcapabSpec {}
#[doc = "`reset()` method sets MAXCURCAPAB to value 0"]
impl crate::Resettable for MaxcurcapabSpec {}
