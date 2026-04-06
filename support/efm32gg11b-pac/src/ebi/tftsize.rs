#[doc = "Register `TFTSIZE` reader"]
pub type R = crate::R<TftsizeSpec>;
#[doc = "Register `TFTSIZE` writer"]
pub type W = crate::W<TftsizeSpec>;
#[doc = "Field `HSZ` reader - Horizontal Size (excluding Porches)"]
pub type HszR = crate::FieldReader<u16>;
#[doc = "Field `HSZ` writer - Horizontal Size (excluding Porches)"]
pub type HszW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `VSZ` reader - Vertical Size (excluding Porches)"]
pub type VszR = crate::FieldReader<u16>;
#[doc = "Field `VSZ` writer - Vertical Size (excluding Porches)"]
pub type VszW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Horizontal Size (excluding Porches)"]
    #[inline(always)]
    pub fn hsz(&self) -> HszR {
        HszR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Vertical Size (excluding Porches)"]
    #[inline(always)]
    pub fn vsz(&self) -> VszR {
        VszR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Horizontal Size (excluding Porches)"]
    #[inline(always)]
    pub fn hsz(&mut self) -> HszW<'_, TftsizeSpec> {
        HszW::new(self, 0)
    }
    #[doc = "Bits 16:25 - Vertical Size (excluding Porches)"]
    #[inline(always)]
    pub fn vsz(&mut self) -> VszW<'_, TftsizeSpec> {
        VszW::new(self, 16)
    }
}
#[doc = "TFT Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tftsize::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tftsize::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TftsizeSpec;
impl crate::RegisterSpec for TftsizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tftsize::R`](R) reader structure"]
impl crate::Readable for TftsizeSpec {}
#[doc = "`write(|w| ..)` method takes [`tftsize::W`](W) writer structure"]
impl crate::Writable for TftsizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TFTSIZE to value 0"]
impl crate::Resettable for TftsizeSpec {}
