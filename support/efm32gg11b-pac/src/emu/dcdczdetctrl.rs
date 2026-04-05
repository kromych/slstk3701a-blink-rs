#[doc = "Register `DCDCZDETCTRL` reader"]
pub type R = crate::R<DcdczdetctrlSpec>;
#[doc = "Register `DCDCZDETCTRL` writer"]
pub type W = crate::W<DcdczdetctrlSpec>;
#[doc = "Field `ZDETILIMSEL` reader - Reverse Current Limit Level Selection for Zero Detector"]
pub type ZdetilimselR = crate::FieldReader;
#[doc = "Field `ZDETILIMSEL` writer - Reverse Current Limit Level Selection for Zero Detector"]
pub type ZdetilimselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ZDETBLANKDLY` reader - Reserved for internal use. Do not change."]
pub type ZdetblankdlyR = crate::FieldReader;
#[doc = "Field `ZDETBLANKDLY` writer - Reserved for internal use. Do not change."]
pub type ZdetblankdlyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 4:6 - Reverse Current Limit Level Selection for Zero Detector"]
    #[inline(always)]
    pub fn zdetilimsel(&self) -> ZdetilimselR {
        ZdetilimselR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9 - Reserved for internal use. Do not change."]
    #[inline(always)]
    pub fn zdetblankdly(&self) -> ZdetblankdlyR {
        ZdetblankdlyR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - Reverse Current Limit Level Selection for Zero Detector"]
    #[inline(always)]
    pub fn zdetilimsel(&mut self) -> ZdetilimselW<'_, DcdczdetctrlSpec> {
        ZdetilimselW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Reserved for internal use. Do not change."]
    #[inline(always)]
    pub fn zdetblankdly(&mut self) -> ZdetblankdlyW<'_, DcdczdetctrlSpec> {
        ZdetblankdlyW::new(self, 8)
    }
}
#[doc = "DCDC Power Train NFET Zero Current Detector Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdczdetctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdczdetctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcdczdetctrlSpec;
impl crate::RegisterSpec for DcdczdetctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdczdetctrl::R`](R) reader structure"]
impl crate::Readable for DcdczdetctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dcdczdetctrl::W`](W) writer structure"]
impl crate::Writable for DcdczdetctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCDCZDETCTRL to value 0x0150"]
impl crate::Resettable for DcdczdetctrlSpec {
    const RESET_VALUE: u32 = 0x0150;
}
