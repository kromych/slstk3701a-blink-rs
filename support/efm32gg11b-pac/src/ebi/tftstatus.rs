#[doc = "Register `TFTSTATUS` reader"]
pub type R = crate::R<TftstatusSpec>;
#[doc = "Field `HCNT` reader - Horizontal Count"]
pub type HcntR = crate::FieldReader<u16>;
#[doc = "Field `VCNT` reader - Vertical Count"]
pub type VcntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:10 - Horizontal Count"]
    #[inline(always)]
    pub fn hcnt(&self) -> HcntR {
        HcntR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:30 - Vertical Count"]
    #[inline(always)]
    pub fn vcnt(&self) -> VcntR {
        VcntR::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
#[doc = "TFT Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tftstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TftstatusSpec;
impl crate::RegisterSpec for TftstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tftstatus::R`](R) reader structure"]
impl crate::Readable for TftstatusSpec {}
#[doc = "`reset()` method sets TFTSTATUS to value 0"]
impl crate::Resettable for TftstatusSpec {}
