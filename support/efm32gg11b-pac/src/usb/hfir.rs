#[doc = "Register `HFIR` reader"]
pub type R = crate::R<HfirSpec>;
#[doc = "Register `HFIR` writer"]
pub type W = crate::W<HfirSpec>;
#[doc = "Field `FRINT` reader - Frame Interval"]
pub type FrintR = crate::FieldReader<u16>;
#[doc = "Field `FRINT` writer - Frame Interval"]
pub type FrintW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `HFIRRLDCTRL` reader - Reload Control"]
pub type HfirrldctrlR = crate::BitReader;
#[doc = "Field `HFIRRLDCTRL` writer - Reload Control"]
pub type HfirrldctrlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Frame Interval"]
    #[inline(always)]
    pub fn frint(&self) -> FrintR {
        FrintR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Reload Control"]
    #[inline(always)]
    pub fn hfirrldctrl(&self) -> HfirrldctrlR {
        HfirrldctrlR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Frame Interval"]
    #[inline(always)]
    pub fn frint(&mut self) -> FrintW<'_, HfirSpec> {
        FrintW::new(self, 0)
    }
    #[doc = "Bit 16 - Reload Control"]
    #[inline(always)]
    pub fn hfirrldctrl(&mut self) -> HfirrldctrlW<'_, HfirSpec> {
        HfirrldctrlW::new(self, 16)
    }
}
#[doc = "Host Frame Interval Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hfir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfirSpec;
impl crate::RegisterSpec for HfirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfir::R`](R) reader structure"]
impl crate::Readable for HfirSpec {}
#[doc = "`write(|w| ..)` method takes [`hfir::W`](W) writer structure"]
impl crate::Writable for HfirSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HFIR to value 0xea60"]
impl crate::Resettable for HfirSpec {
    const RESET_VALUE: u32 = 0xea60;
}
