#[doc = "Register `VMONAVDDCTRL` reader"]
pub type R = crate::R<VmonavddctrlSpec>;
#[doc = "Register `VMONAVDDCTRL` writer"]
pub type W = crate::W<VmonavddctrlSpec>;
#[doc = "Field `EN` reader - Enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RISEWU` reader - Rise Wakeup"]
pub type RisewuR = crate::BitReader;
#[doc = "Field `RISEWU` writer - Rise Wakeup"]
pub type RisewuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FALLWU` reader - Fall Wakeup"]
pub type FallwuR = crate::BitReader;
#[doc = "Field `FALLWU` writer - Fall Wakeup"]
pub type FallwuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FALLTHRESFINE` reader - Falling Threshold Fine Adjust"]
pub type FallthresfineR = crate::FieldReader;
#[doc = "Field `FALLTHRESFINE` writer - Falling Threshold Fine Adjust"]
pub type FallthresfineW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FALLTHRESCOARSE` reader - Falling Threshold Coarse Adjust"]
pub type FallthrescoarseR = crate::FieldReader;
#[doc = "Field `FALLTHRESCOARSE` writer - Falling Threshold Coarse Adjust"]
pub type FallthrescoarseW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RISETHRESFINE` reader - Rising Threshold Fine Adjust"]
pub type RisethresfineR = crate::FieldReader;
#[doc = "Field `RISETHRESFINE` writer - Rising Threshold Fine Adjust"]
pub type RisethresfineW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RISETHRESCOARSE` reader - Rising Threshold Coarse Adjust"]
pub type RisethrescoarseR = crate::FieldReader;
#[doc = "Field `RISETHRESCOARSE` writer - Rising Threshold Coarse Adjust"]
pub type RisethrescoarseW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Rise Wakeup"]
    #[inline(always)]
    pub fn risewu(&self) -> RisewuR {
        RisewuR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fall Wakeup"]
    #[inline(always)]
    pub fn fallwu(&self) -> FallwuR {
        FallwuR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Falling Threshold Fine Adjust"]
    #[inline(always)]
    pub fn fallthresfine(&self) -> FallthresfineR {
        FallthresfineR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Falling Threshold Coarse Adjust"]
    #[inline(always)]
    pub fn fallthrescoarse(&self) -> FallthrescoarseR {
        FallthrescoarseR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Rising Threshold Fine Adjust"]
    #[inline(always)]
    pub fn risethresfine(&self) -> RisethresfineR {
        RisethresfineR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Rising Threshold Coarse Adjust"]
    #[inline(always)]
    pub fn risethrescoarse(&self) -> RisethrescoarseR {
        RisethrescoarseR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, VmonavddctrlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 2 - Rise Wakeup"]
    #[inline(always)]
    pub fn risewu(&mut self) -> RisewuW<'_, VmonavddctrlSpec> {
        RisewuW::new(self, 2)
    }
    #[doc = "Bit 3 - Fall Wakeup"]
    #[inline(always)]
    pub fn fallwu(&mut self) -> FallwuW<'_, VmonavddctrlSpec> {
        FallwuW::new(self, 3)
    }
    #[doc = "Bits 8:11 - Falling Threshold Fine Adjust"]
    #[inline(always)]
    pub fn fallthresfine(&mut self) -> FallthresfineW<'_, VmonavddctrlSpec> {
        FallthresfineW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Falling Threshold Coarse Adjust"]
    #[inline(always)]
    pub fn fallthrescoarse(&mut self) -> FallthrescoarseW<'_, VmonavddctrlSpec> {
        FallthrescoarseW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Rising Threshold Fine Adjust"]
    #[inline(always)]
    pub fn risethresfine(&mut self) -> RisethresfineW<'_, VmonavddctrlSpec> {
        RisethresfineW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Rising Threshold Coarse Adjust"]
    #[inline(always)]
    pub fn risethrescoarse(&mut self) -> RisethrescoarseW<'_, VmonavddctrlSpec> {
        RisethrescoarseW::new(self, 20)
    }
}
#[doc = "VMON AVDD Channel Control\n\nYou can [`read`](crate::Reg::read) this register and get [`vmonavddctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vmonavddctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VmonavddctrlSpec;
impl crate::RegisterSpec for VmonavddctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vmonavddctrl::R`](R) reader structure"]
impl crate::Readable for VmonavddctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`vmonavddctrl::W`](W) writer structure"]
impl crate::Writable for VmonavddctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VMONAVDDCTRL to value 0"]
impl crate::Resettable for VmonavddctrlSpec {}
