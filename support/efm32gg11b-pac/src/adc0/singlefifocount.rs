#[doc = "Register `SINGLEFIFOCOUNT` reader"]
pub type R = crate::R<SinglefifocountSpec>;
#[doc = "Field `SINGLEDC` reader - Single Data Count"]
pub type SingledcR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Single Data Count"]
    #[inline(always)]
    pub fn singledc(&self) -> SingledcR {
        SingledcR::new((self.bits & 7) as u8)
    }
}
#[doc = "Single FIFO Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`singlefifocount::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SinglefifocountSpec;
impl crate::RegisterSpec for SinglefifocountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`singlefifocount::R`](R) reader structure"]
impl crate::Readable for SinglefifocountSpec {}
#[doc = "`reset()` method sets SINGLEFIFOCOUNT to value 0"]
impl crate::Resettable for SinglefifocountSpec {}
