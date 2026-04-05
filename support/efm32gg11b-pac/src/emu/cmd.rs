#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Field `EM4UNLATCH` writer - EM4 Unlatch"]
pub type Em4unlatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM01VSCALE0` writer - EM01 Voltage Scale Command to Scale to Voltage Scale Level 0"]
pub type Em01vscale0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM01VSCALE2` writer - EM01 Voltage Scale Command to Scale to Voltage Scale Level 2"]
pub type Em01vscale2W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - EM4 Unlatch"]
    #[inline(always)]
    pub fn em4unlatch(&mut self) -> Em4unlatchW<'_, CmdSpec> {
        Em4unlatchW::new(self, 0)
    }
    #[doc = "Bit 4 - EM01 Voltage Scale Command to Scale to Voltage Scale Level 0"]
    #[inline(always)]
    pub fn em01vscale0(&mut self) -> Em01vscale0W<'_, CmdSpec> {
        Em01vscale0W::new(self, 4)
    }
    #[doc = "Bit 6 - EM01 Voltage Scale Command to Scale to Voltage Scale Level 2"]
    #[inline(always)]
    pub fn em01vscale2(&mut self) -> Em01vscale2W<'_, CmdSpec> {
        Em01vscale2W::new(self, 6)
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
