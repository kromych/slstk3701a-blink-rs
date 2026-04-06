#[doc = "Register `VMONALTAVDDCTRL` reader"]
pub type R = crate::R<VmonaltavddctrlSpec>;
#[doc = "Register `VMONALTAVDDCTRL` writer"]
pub type W = crate::W<VmonaltavddctrlSpec>;
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
#[doc = "Field `THRESFINE` reader - Threshold Fine Adjust"]
pub type ThresfineR = crate::FieldReader;
#[doc = "Field `THRESFINE` writer - Threshold Fine Adjust"]
pub type ThresfineW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `THRESCOARSE` reader - Threshold Coarse Adjust"]
pub type ThrescoarseR = crate::FieldReader;
#[doc = "Field `THRESCOARSE` writer - Threshold Coarse Adjust"]
pub type ThrescoarseW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
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
    #[doc = "Bits 8:11 - Threshold Fine Adjust"]
    #[inline(always)]
    pub fn thresfine(&self) -> ThresfineR {
        ThresfineR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Threshold Coarse Adjust"]
    #[inline(always)]
    pub fn threscoarse(&self) -> ThrescoarseR {
        ThrescoarseR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, VmonaltavddctrlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 2 - Rise Wakeup"]
    #[inline(always)]
    pub fn risewu(&mut self) -> RisewuW<'_, VmonaltavddctrlSpec> {
        RisewuW::new(self, 2)
    }
    #[doc = "Bit 3 - Fall Wakeup"]
    #[inline(always)]
    pub fn fallwu(&mut self) -> FallwuW<'_, VmonaltavddctrlSpec> {
        FallwuW::new(self, 3)
    }
    #[doc = "Bits 8:11 - Threshold Fine Adjust"]
    #[inline(always)]
    pub fn thresfine(&mut self) -> ThresfineW<'_, VmonaltavddctrlSpec> {
        ThresfineW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Threshold Coarse Adjust"]
    #[inline(always)]
    pub fn threscoarse(&mut self) -> ThrescoarseW<'_, VmonaltavddctrlSpec> {
        ThrescoarseW::new(self, 12)
    }
}
#[doc = "Alternate VMON AVDD Channel Control\n\nYou can [`read`](crate::Reg::read) this register and get [`vmonaltavddctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vmonaltavddctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VmonaltavddctrlSpec;
impl crate::RegisterSpec for VmonaltavddctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vmonaltavddctrl::R`](R) reader structure"]
impl crate::Readable for VmonaltavddctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`vmonaltavddctrl::W`](W) writer structure"]
impl crate::Writable for VmonaltavddctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VMONALTAVDDCTRL to value 0"]
impl crate::Resettable for VmonaltavddctrlSpec {}
