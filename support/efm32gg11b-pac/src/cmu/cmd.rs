#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Field `CALSTART` writer - Calibration Start"]
pub type CalstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALSTOP` writer - Calibration Stop"]
pub type CalstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXOPEAKDETSTART` writer - HFXO Peak Detection Start"]
pub type HfxopeakdetstartW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Calibration Start"]
    #[inline(always)]
    pub fn calstart(&mut self) -> CalstartW<'_, CmdSpec> {
        CalstartW::new(self, 0)
    }
    #[doc = "Bit 1 - Calibration Stop"]
    #[inline(always)]
    pub fn calstop(&mut self) -> CalstopW<'_, CmdSpec> {
        CalstopW::new(self, 1)
    }
    #[doc = "Bit 4 - HFXO Peak Detection Start"]
    #[inline(always)]
    pub fn hfxopeakdetstart(&mut self) -> HfxopeakdetstartW<'_, CmdSpec> {
        HfxopeakdetstartW::new(self, 4)
    }
}
#[doc = "Command Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdSpec;
impl crate::RegisterSpec for CmdSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CmdSpec {}
