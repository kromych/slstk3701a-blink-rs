#[doc = "Register `R5VDETCTRL` reader"]
pub type R = crate::R<R5vdetctrlSpec>;
#[doc = "Register `R5VDETCTRL` writer"]
pub type W = crate::W<R5vdetctrlSpec>;
#[doc = "Field `VREGIDETDIS` reader - VREGI Detector Disable"]
pub type VregidetdisR = crate::BitReader;
#[doc = "Field `VREGIDETDIS` writer - VREGI Detector Disable"]
pub type VregidetdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSDETDIS` reader - VBUS Detector Disable"]
pub type VbusdetdisR = crate::BitReader;
#[doc = "Field `VBUSDETDIS` writer - VBUS Detector Disable"]
pub type VbusdetdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREGODETDIS` reader - VREGO Detector Disable"]
pub type VregodetdisR = crate::BitReader;
#[doc = "Field `VREGODETDIS` writer - VREGO Detector Disable"]
pub type VregodetdisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - VREGI Detector Disable"]
    #[inline(always)]
    pub fn vregidetdis(&self) -> VregidetdisR {
        VregidetdisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VBUS Detector Disable"]
    #[inline(always)]
    pub fn vbusdetdis(&self) -> VbusdetdisR {
        VbusdetdisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VREGO Detector Disable"]
    #[inline(always)]
    pub fn vregodetdis(&self) -> VregodetdisR {
        VregodetdisR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VREGI Detector Disable"]
    #[inline(always)]
    pub fn vregidetdis(&mut self) -> VregidetdisW<'_, R5vdetctrlSpec> {
        VregidetdisW::new(self, 0)
    }
    #[doc = "Bit 1 - VBUS Detector Disable"]
    #[inline(always)]
    pub fn vbusdetdis(&mut self) -> VbusdetdisW<'_, R5vdetctrlSpec> {
        VbusdetdisW::new(self, 1)
    }
    #[doc = "Bit 2 - VREGO Detector Disable"]
    #[inline(always)]
    pub fn vregodetdis(&mut self) -> VregodetdisW<'_, R5vdetctrlSpec> {
        VregodetdisW::new(self, 2)
    }
}
#[doc = "5V Detector Enables\n\nYou can [`read`](crate::Reg::read) this register and get [`r5vdetctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5vdetctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R5vdetctrlSpec;
impl crate::RegisterSpec for R5vdetctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r5vdetctrl::R`](R) reader structure"]
impl crate::Readable for R5vdetctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`r5vdetctrl::W`](W) writer structure"]
impl crate::Writable for R5vdetctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R5VDETCTRL to value 0"]
impl crate::Resettable for R5vdetctrlSpec {}
