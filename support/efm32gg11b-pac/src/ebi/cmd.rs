#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Field `ECCSTART` writer - Error Correction Code Generation Start"]
pub type EccstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCSTOP` writer - Error Correction Code Generation Stop"]
pub type EccstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCCLEAR` writer - Error Correction Code Clear"]
pub type EccclearW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Error Correction Code Generation Start"]
    #[inline(always)]
    pub fn eccstart(&mut self) -> EccstartW<'_, CmdSpec> {
        EccstartW::new(self, 0)
    }
    #[doc = "Bit 1 - Error Correction Code Generation Stop"]
    #[inline(always)]
    pub fn eccstop(&mut self) -> EccstopW<'_, CmdSpec> {
        EccstopW::new(self, 1)
    }
    #[doc = "Bit 2 - Error Correction Code Clear"]
    #[inline(always)]
    pub fn eccclear(&mut self) -> EccclearW<'_, CmdSpec> {
        EccclearW::new(self, 2)
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
