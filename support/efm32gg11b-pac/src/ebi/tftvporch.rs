#[doc = "Register `TFTVPORCH` reader"]
pub type R = crate::R<TftvporchSpec>;
#[doc = "Register `TFTVPORCH` writer"]
pub type W = crate::W<TftvporchSpec>;
#[doc = "Field `VSYNC` reader - Vertical Synchronization Pulse Width"]
pub type VsyncR = crate::FieldReader;
#[doc = "Field `VSYNC` writer - Vertical Synchronization Pulse Width"]
pub type VsyncW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `VFPORCH` reader - Vertical Front Porch Size"]
pub type VfporchR = crate::FieldReader<u16>;
#[doc = "Field `VFPORCH` writer - Vertical Front Porch Size"]
pub type VfporchW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `VBPORCH` reader - Vertical Back Porch Size"]
pub type VbporchR = crate::FieldReader<u16>;
#[doc = "Field `VBPORCH` writer - Vertical Back Porch Size"]
pub type VbporchW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:6 - Vertical Synchronization Pulse Width"]
    #[inline(always)]
    pub fn vsync(&self) -> VsyncR {
        VsyncR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:19 - Vertical Front Porch Size"]
    #[inline(always)]
    pub fn vfporch(&self) -> VfporchR {
        VfporchR::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bits 20:31 - Vertical Back Porch Size"]
    #[inline(always)]
    pub fn vbporch(&self) -> VbporchR {
        VbporchR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:6 - Vertical Synchronization Pulse Width"]
    #[inline(always)]
    pub fn vsync(&mut self) -> VsyncW<'_, TftvporchSpec> {
        VsyncW::new(self, 0)
    }
    #[doc = "Bits 8:19 - Vertical Front Porch Size"]
    #[inline(always)]
    pub fn vfporch(&mut self) -> VfporchW<'_, TftvporchSpec> {
        VfporchW::new(self, 8)
    }
    #[doc = "Bits 20:31 - Vertical Back Porch Size"]
    #[inline(always)]
    pub fn vbporch(&mut self) -> VbporchW<'_, TftvporchSpec> {
        VbporchW::new(self, 20)
    }
}
#[doc = "TFT Vertical Porch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tftvporch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tftvporch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TftvporchSpec;
impl crate::RegisterSpec for TftvporchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tftvporch::R`](R) reader structure"]
impl crate::Readable for TftvporchSpec {}
#[doc = "`write(|w| ..)` method takes [`tftvporch::W`](W) writer structure"]
impl crate::Writable for TftvporchSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TFTVPORCH to value 0"]
impl crate::Resettable for TftvporchSpec {}
