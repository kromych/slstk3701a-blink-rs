#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `INIT` reader - Initialize"]
pub type InitR = crate::BitReader;
#[doc = "Field `INIT` writer - Initialize"]
pub type InitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE` reader - Module Interrupt Enable"]
pub type IeR = crate::BitReader;
#[doc = "Field `IE` writer - Module Interrupt Enable"]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIE` reader - Status Change Interrupt Enable"]
pub type SieR = crate::BitReader;
#[doc = "Field `SIE` writer - Status Change Interrupt Enable"]
pub type SieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIE` reader - Error Interrupt Enable"]
pub type EieR = crate::BitReader;
#[doc = "Field `EIE` writer - Error Interrupt Enable"]
pub type EieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAR` reader - Disable Automatic Retransmission"]
pub type DarR = crate::BitReader;
#[doc = "Field `DAR` writer - Disable Automatic Retransmission"]
pub type DarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCE` reader - Configuration Change Enable"]
pub type CceR = crate::BitReader;
#[doc = "Field `CCE` writer - Configuration Change Enable"]
pub type CceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEST` reader - Test Mode Enable Write"]
pub type TestR = crate::BitReader;
#[doc = "Field `TEST` writer - Test Mode Enable Write"]
pub type TestW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Initialize"]
    #[inline(always)]
    pub fn init(&self) -> InitR {
        InitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Module Interrupt Enable"]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Status Change Interrupt Enable"]
    #[inline(always)]
    pub fn sie(&self) -> SieR {
        SieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Error Interrupt Enable"]
    #[inline(always)]
    pub fn eie(&self) -> EieR {
        EieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Disable Automatic Retransmission"]
    #[inline(always)]
    pub fn dar(&self) -> DarR {
        DarR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configuration Change Enable"]
    #[inline(always)]
    pub fn cce(&self) -> CceR {
        CceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Test Mode Enable Write"]
    #[inline(always)]
    pub fn test(&self) -> TestR {
        TestR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Initialize"]
    #[inline(always)]
    pub fn init(&mut self) -> InitW<'_, CtrlSpec> {
        InitW::new(self, 0)
    }
    #[doc = "Bit 1 - Module Interrupt Enable"]
    #[inline(always)]
    pub fn ie(&mut self) -> IeW<'_, CtrlSpec> {
        IeW::new(self, 1)
    }
    #[doc = "Bit 2 - Status Change Interrupt Enable"]
    #[inline(always)]
    pub fn sie(&mut self) -> SieW<'_, CtrlSpec> {
        SieW::new(self, 2)
    }
    #[doc = "Bit 3 - Error Interrupt Enable"]
    #[inline(always)]
    pub fn eie(&mut self) -> EieW<'_, CtrlSpec> {
        EieW::new(self, 3)
    }
    #[doc = "Bit 5 - Disable Automatic Retransmission"]
    #[inline(always)]
    pub fn dar(&mut self) -> DarW<'_, CtrlSpec> {
        DarW::new(self, 5)
    }
    #[doc = "Bit 6 - Configuration Change Enable"]
    #[inline(always)]
    pub fn cce(&mut self) -> CceW<'_, CtrlSpec> {
        CceW::new(self, 6)
    }
    #[doc = "Bit 7 - Test Mode Enable Write"]
    #[inline(always)]
    pub fn test(&mut self) -> TestW<'_, CtrlSpec> {
        TestW::new(self, 7)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0x01"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x01;
}
