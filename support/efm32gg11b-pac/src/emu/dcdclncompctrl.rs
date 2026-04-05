#[doc = "Register `DCDCLNCOMPCTRL` reader"]
pub type R = crate::R<DcdclncompctrlSpec>;
#[doc = "Register `DCDCLNCOMPCTRL` writer"]
pub type W = crate::W<DcdclncompctrlSpec>;
#[doc = "Field `COMPENR1` reader - Low Noise Mode Compensator R1 Trim Value"]
pub type Compenr1R = crate::FieldReader;
#[doc = "Field `COMPENR1` writer - Low Noise Mode Compensator R1 Trim Value"]
pub type Compenr1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `COMPENR2` reader - Low Noise Mode Compensator R2 Trim Value"]
pub type Compenr2R = crate::FieldReader;
#[doc = "Field `COMPENR2` writer - Low Noise Mode Compensator R2 Trim Value"]
pub type Compenr2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `COMPENR3` reader - Low Noise Mode Compensator R3 Trim Value"]
pub type Compenr3R = crate::FieldReader;
#[doc = "Field `COMPENR3` writer - Low Noise Mode Compensator R3 Trim Value"]
pub type Compenr3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `COMPENC1` reader - Low Noise Mode Compensator C1 Trim Value"]
pub type Compenc1R = crate::FieldReader;
#[doc = "Field `COMPENC1` writer - Low Noise Mode Compensator C1 Trim Value"]
pub type Compenc1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMPENC2` reader - Low Noise Mode Compensator C2 Trim Value"]
pub type Compenc2R = crate::FieldReader;
#[doc = "Field `COMPENC2` writer - Low Noise Mode Compensator C2 Trim Value"]
pub type Compenc2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `COMPENC3` reader - Low Noise Mode Compensator C3 Trim Value"]
pub type Compenc3R = crate::FieldReader;
#[doc = "Field `COMPENC3` writer - Low Noise Mode Compensator C3 Trim Value"]
pub type Compenc3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - Low Noise Mode Compensator R1 Trim Value"]
    #[inline(always)]
    pub fn compenr1(&self) -> Compenr1R {
        Compenr1R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:8 - Low Noise Mode Compensator R2 Trim Value"]
    #[inline(always)]
    pub fn compenr2(&self) -> Compenr2R {
        Compenr2R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 12:15 - Low Noise Mode Compensator R3 Trim Value"]
    #[inline(always)]
    pub fn compenr3(&self) -> Compenr3R {
        Compenr3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Low Noise Mode Compensator C1 Trim Value"]
    #[inline(always)]
    pub fn compenc1(&self) -> Compenc1R {
        Compenc1R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Low Noise Mode Compensator C2 Trim Value"]
    #[inline(always)]
    pub fn compenc2(&self) -> Compenc2R {
        Compenc2R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:31 - Low Noise Mode Compensator C3 Trim Value"]
    #[inline(always)]
    pub fn compenc3(&self) -> Compenc3R {
        Compenc3R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Low Noise Mode Compensator R1 Trim Value"]
    #[inline(always)]
    pub fn compenr1(&mut self) -> Compenr1W<'_, DcdclncompctrlSpec> {
        Compenr1W::new(self, 0)
    }
    #[doc = "Bits 4:8 - Low Noise Mode Compensator R2 Trim Value"]
    #[inline(always)]
    pub fn compenr2(&mut self) -> Compenr2W<'_, DcdclncompctrlSpec> {
        Compenr2W::new(self, 4)
    }
    #[doc = "Bits 12:15 - Low Noise Mode Compensator R3 Trim Value"]
    #[inline(always)]
    pub fn compenr3(&mut self) -> Compenr3W<'_, DcdclncompctrlSpec> {
        Compenr3W::new(self, 12)
    }
    #[doc = "Bits 20:21 - Low Noise Mode Compensator C1 Trim Value"]
    #[inline(always)]
    pub fn compenc1(&mut self) -> Compenc1W<'_, DcdclncompctrlSpec> {
        Compenc1W::new(self, 20)
    }
    #[doc = "Bits 24:26 - Low Noise Mode Compensator C2 Trim Value"]
    #[inline(always)]
    pub fn compenc2(&mut self) -> Compenc2W<'_, DcdclncompctrlSpec> {
        Compenc2W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Low Noise Mode Compensator C3 Trim Value"]
    #[inline(always)]
    pub fn compenc3(&mut self) -> Compenc3W<'_, DcdclncompctrlSpec> {
        Compenc3W::new(self, 28)
    }
}
#[doc = "DCDC Low Noise Compensator Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdclncompctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdclncompctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcdclncompctrlSpec;
impl crate::RegisterSpec for DcdclncompctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdclncompctrl::R`](R) reader structure"]
impl crate::Readable for DcdclncompctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dcdclncompctrl::W`](W) writer structure"]
impl crate::Writable for DcdclncompctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCDCLNCOMPCTRL to value 0x5720_4077"]
impl crate::Resettable for DcdclncompctrlSpec {
    const RESET_VALUE: u32 = 0x5720_4077;
}
