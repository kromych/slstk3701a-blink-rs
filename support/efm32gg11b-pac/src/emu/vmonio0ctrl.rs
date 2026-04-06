#[doc = "Register `VMONIO0CTRL` reader"]
pub type R = crate::R<Vmonio0ctrlSpec>;
#[doc = "Register `VMONIO0CTRL` writer"]
pub type W = crate::W<Vmonio0ctrlSpec>;
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
#[doc = "Field `RETDIS` reader - EM4 IO0 Retention Disable"]
pub type RetdisR = crate::BitReader;
#[doc = "Field `RETDIS` writer - EM4 IO0 Retention Disable"]
pub type RetdisW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 4 - EM4 IO0 Retention Disable"]
    #[inline(always)]
    pub fn retdis(&self) -> RetdisR {
        RetdisR::new(((self.bits >> 4) & 1) != 0)
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
    pub fn en(&mut self) -> EnW<'_, Vmonio0ctrlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 2 - Rise Wakeup"]
    #[inline(always)]
    pub fn risewu(&mut self) -> RisewuW<'_, Vmonio0ctrlSpec> {
        RisewuW::new(self, 2)
    }
    #[doc = "Bit 3 - Fall Wakeup"]
    #[inline(always)]
    pub fn fallwu(&mut self) -> FallwuW<'_, Vmonio0ctrlSpec> {
        FallwuW::new(self, 3)
    }
    #[doc = "Bit 4 - EM4 IO0 Retention Disable"]
    #[inline(always)]
    pub fn retdis(&mut self) -> RetdisW<'_, Vmonio0ctrlSpec> {
        RetdisW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Threshold Fine Adjust"]
    #[inline(always)]
    pub fn thresfine(&mut self) -> ThresfineW<'_, Vmonio0ctrlSpec> {
        ThresfineW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Threshold Coarse Adjust"]
    #[inline(always)]
    pub fn threscoarse(&mut self) -> ThrescoarseW<'_, Vmonio0ctrlSpec> {
        ThrescoarseW::new(self, 12)
    }
}
#[doc = "VMON IOVDD0 Channel Control\n\nYou can [`read`](crate::Reg::read) this register and get [`vmonio0ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vmonio0ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vmonio0ctrlSpec;
impl crate::RegisterSpec for Vmonio0ctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vmonio0ctrl::R`](R) reader structure"]
impl crate::Readable for Vmonio0ctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`vmonio0ctrl::W`](W) writer structure"]
impl crate::Writable for Vmonio0ctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VMONIO0CTRL to value 0"]
impl crate::Resettable for Vmonio0ctrlSpec {}
