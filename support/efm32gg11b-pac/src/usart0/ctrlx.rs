#[doc = "Register `CTRLX` reader"]
pub type R = crate::R<CtrlxSpec>;
#[doc = "Register `CTRLX` writer"]
pub type W = crate::W<CtrlxSpec>;
#[doc = "Field `DBGHALT` reader - Debug Halt"]
pub type DbghaltR = crate::BitReader;
#[doc = "Field `DBGHALT` writer - Debug Halt"]
pub type DbghaltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSINV` reader - CTS Pin Inversion"]
pub type CtsinvR = crate::BitReader;
#[doc = "Field `CTSINV` writer - CTS Pin Inversion"]
pub type CtsinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSEN` reader - CTS Function Enabled"]
pub type CtsenR = crate::BitReader;
#[doc = "Field `CTSEN` writer - CTS Function Enabled"]
pub type CtsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSINV` reader - RTS Pin Inversion"]
pub type RtsinvR = crate::BitReader;
#[doc = "Field `RTSINV` writer - RTS Pin Inversion"]
pub type RtsinvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Debug Halt"]
    #[inline(always)]
    pub fn dbghalt(&self) -> DbghaltR {
        DbghaltR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CTS Pin Inversion"]
    #[inline(always)]
    pub fn ctsinv(&self) -> CtsinvR {
        CtsinvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CTS Function Enabled"]
    #[inline(always)]
    pub fn ctsen(&self) -> CtsenR {
        CtsenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTS Pin Inversion"]
    #[inline(always)]
    pub fn rtsinv(&self) -> RtsinvR {
        RtsinvR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debug Halt"]
    #[inline(always)]
    pub fn dbghalt(&mut self) -> DbghaltW<'_, CtrlxSpec> {
        DbghaltW::new(self, 0)
    }
    #[doc = "Bit 1 - CTS Pin Inversion"]
    #[inline(always)]
    pub fn ctsinv(&mut self) -> CtsinvW<'_, CtrlxSpec> {
        CtsinvW::new(self, 1)
    }
    #[doc = "Bit 2 - CTS Function Enabled"]
    #[inline(always)]
    pub fn ctsen(&mut self) -> CtsenW<'_, CtrlxSpec> {
        CtsenW::new(self, 2)
    }
    #[doc = "Bit 3 - RTS Pin Inversion"]
    #[inline(always)]
    pub fn rtsinv(&mut self) -> RtsinvW<'_, CtrlxSpec> {
        RtsinvW::new(self, 3)
    }
}
#[doc = "Control Register Extended\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlxSpec;
impl crate::RegisterSpec for CtrlxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlx::R`](R) reader structure"]
impl crate::Readable for CtrlxSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlx::W`](W) writer structure"]
impl crate::Writable for CtrlxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLX to value 0"]
impl crate::Resettable for CtrlxSpec {}
