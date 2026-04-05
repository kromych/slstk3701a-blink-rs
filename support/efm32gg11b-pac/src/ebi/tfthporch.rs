#[doc = "Register `TFTHPORCH` reader"]
pub type R = crate::R<TfthporchSpec>;
#[doc = "Register `TFTHPORCH` writer"]
pub type W = crate::W<TfthporchSpec>;
#[doc = "Field `HSYNC` reader - Horizontal Synchronization Pulse Width"]
pub type HsyncR = crate::FieldReader;
#[doc = "Field `HSYNC` writer - Horizontal Synchronization Pulse Width"]
pub type HsyncW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `HFPORCH` reader - Horizontal Front Porch Size"]
pub type HfporchR = crate::FieldReader;
#[doc = "Field `HFPORCH` writer - Horizontal Front Porch Size"]
pub type HfporchW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HBPORCH` reader - Horizontal Back Porch Size"]
pub type HbporchR = crate::FieldReader;
#[doc = "Field `HBPORCH` writer - Horizontal Back Porch Size"]
pub type HbporchW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HSYNCSTART` reader - HSYNC Start Delay"]
pub type HsyncstartR = crate::FieldReader;
#[doc = "Field `HSYNCSTART` writer - HSYNC Start Delay"]
pub type HsyncstartW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6 - Horizontal Synchronization Pulse Width"]
    #[inline(always)]
    pub fn hsync(&self) -> HsyncR {
        HsyncR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - Horizontal Front Porch Size"]
    #[inline(always)]
    pub fn hfporch(&self) -> HfporchR {
        HfporchR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 18:25 - Horizontal Back Porch Size"]
    #[inline(always)]
    pub fn hbporch(&self) -> HbporchR {
        HbporchR::new(((self.bits >> 18) & 0xff) as u8)
    }
    #[doc = "Bits 28:29 - HSYNC Start Delay"]
    #[inline(always)]
    pub fn hsyncstart(&self) -> HsyncstartR {
        HsyncstartR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Horizontal Synchronization Pulse Width"]
    #[inline(always)]
    pub fn hsync(&mut self) -> HsyncW<'_, TfthporchSpec> {
        HsyncW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Horizontal Front Porch Size"]
    #[inline(always)]
    pub fn hfporch(&mut self) -> HfporchW<'_, TfthporchSpec> {
        HfporchW::new(self, 8)
    }
    #[doc = "Bits 18:25 - Horizontal Back Porch Size"]
    #[inline(always)]
    pub fn hbporch(&mut self) -> HbporchW<'_, TfthporchSpec> {
        HbporchW::new(self, 18)
    }
    #[doc = "Bits 28:29 - HSYNC Start Delay"]
    #[inline(always)]
    pub fn hsyncstart(&mut self) -> HsyncstartW<'_, TfthporchSpec> {
        HsyncstartW::new(self, 28)
    }
}
#[doc = "TFT Horizontal Porch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tfthporch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tfthporch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TfthporchSpec;
impl crate::RegisterSpec for TfthporchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tfthporch::R`](R) reader structure"]
impl crate::Readable for TfthporchSpec {}
#[doc = "`write(|w| ..)` method takes [`tfthporch::W`](W) writer structure"]
impl crate::Writable for TfthporchSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TFTHPORCH to value 0"]
impl crate::Resettable for TfthporchSpec {}
