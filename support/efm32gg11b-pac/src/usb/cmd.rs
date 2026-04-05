#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Field `STARTCD` writer - Start Charger Detection Enabled"]
pub type StartcdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPCD` writer - Start Charger Detection in Progress"]
pub type StopcdW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Start Charger Detection Enabled"]
    #[inline(always)]
    pub fn startcd(&mut self) -> StartcdW<'_, CmdSpec> {
        StartcdW::new(self, 0)
    }
    #[doc = "Bit 1 - Start Charger Detection in Progress"]
    #[inline(always)]
    pub fn stopcd(&mut self) -> StopcdW<'_, CmdSpec> {
        StopcdW::new(self, 1)
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
