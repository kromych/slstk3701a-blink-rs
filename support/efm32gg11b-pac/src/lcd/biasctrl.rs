#[doc = "Register `BIASCTRL` reader"]
pub type R = crate::R<BiasctrlSpec>;
#[doc = "Register `BIASCTRL` writer"]
pub type W = crate::W<BiasctrlSpec>;
#[doc = "Field `SPEED` reader - SPEED Adjustment"]
pub type SpeedR = crate::FieldReader;
#[doc = "Field `SPEED` writer - SPEED Adjustment"]
pub type SpeedW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BUFDRV` reader - Buffer Drive Strength"]
pub type BufdrvR = crate::FieldReader;
#[doc = "Field `BUFDRV` writer - Buffer Drive Strength"]
pub type BufdrvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BUFBIAS` reader - Buffer Bias Setting"]
pub type BufbiasR = crate::FieldReader;
#[doc = "Field `BUFBIAS` writer - Buffer Bias Setting"]
pub type BufbiasW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - SPEED Adjustment"]
    #[inline(always)]
    pub fn speed(&self) -> SpeedR {
        SpeedR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:7 - Buffer Drive Strength"]
    #[inline(always)]
    pub fn bufdrv(&self) -> BufdrvR {
        BufdrvR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 10:12 - Buffer Bias Setting"]
    #[inline(always)]
    pub fn bufbias(&self) -> BufbiasR {
        BufbiasR::new(((self.bits >> 10) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SPEED Adjustment"]
    #[inline(always)]
    pub fn speed(&mut self) -> SpeedW<'_, BiasctrlSpec> {
        SpeedW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Buffer Drive Strength"]
    #[inline(always)]
    pub fn bufdrv(&mut self) -> BufdrvW<'_, BiasctrlSpec> {
        BufdrvW::new(self, 4)
    }
    #[doc = "Bits 10:12 - Buffer Bias Setting"]
    #[inline(always)]
    pub fn bufbias(&mut self) -> BufbiasW<'_, BiasctrlSpec> {
        BufbiasW::new(self, 10)
    }
}
#[doc = "Analog BIAS Control\n\nYou can [`read`](crate::Reg::read) this register and get [`biasctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`biasctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BiasctrlSpec;
impl crate::RegisterSpec for BiasctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`biasctrl::R`](R) reader structure"]
impl crate::Readable for BiasctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`biasctrl::W`](W) writer structure"]
impl crate::Writable for BiasctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BIASCTRL to value 0"]
impl crate::Resettable for BiasctrlSpec {}
