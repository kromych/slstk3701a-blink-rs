#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `EN` reader - Watchdog Timer Enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Watchdog Timer Enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEBUGRUN` reader - Debug Mode Run Enable"]
pub type DebugrunR = crate::BitReader;
#[doc = "Field `DEBUGRUN` writer - Debug Mode Run Enable"]
pub type DebugrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM2RUN` reader - Energy Mode 2 Run Enable"]
pub type Em2runR = crate::BitReader;
#[doc = "Field `EM2RUN` writer - Energy Mode 2 Run Enable"]
pub type Em2runW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM3RUN` reader - Energy Mode 3 Run Enable"]
pub type Em3runR = crate::BitReader;
#[doc = "Field `EM3RUN` writer - Energy Mode 3 Run Enable"]
pub type Em3runW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK` reader - Configuration Lock"]
pub type LockR = crate::BitReader;
#[doc = "Field `LOCK` writer - Configuration Lock"]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM4BLOCK` reader - Energy Mode 4 Block"]
pub type Em4blockR = crate::BitReader;
#[doc = "Field `EM4BLOCK` writer - Energy Mode 4 Block"]
pub type Em4blockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWOSCBLOCK` reader - Software Oscillator Disable Block"]
pub type SwoscblockR = crate::BitReader;
#[doc = "Field `SWOSCBLOCK` writer - Software Oscillator Disable Block"]
pub type SwoscblockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERSEL` reader - Watchdog Timeout Period Select"]
pub type PerselR = crate::FieldReader;
#[doc = "Field `PERSEL` writer - Watchdog Timeout Period Select"]
pub type PerselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Watchdog Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clksel {
    #[doc = "0: ULFRCO"]
    Ulfrco = 0,
    #[doc = "1: LFRCO"]
    Lfrco = 1,
    #[doc = "2: LFXO"]
    Lfxo = 2,
    #[doc = "3: HFCORECLK"]
    Hfcoreclk = 3,
}
impl From<Clksel> for u8 {
    #[inline(always)]
    fn from(variant: Clksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clksel {
    type Ux = u8;
}
impl crate::IsEnum for Clksel {}
#[doc = "Field `CLKSEL` reader - Watchdog Clock Select"]
pub type ClkselR = crate::FieldReader<Clksel>;
impl ClkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clksel {
        match self.bits {
            0 => Clksel::Ulfrco,
            1 => Clksel::Lfrco,
            2 => Clksel::Lfxo,
            3 => Clksel::Hfcoreclk,
            _ => unreachable!(),
        }
    }
    #[doc = "ULFRCO"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == Clksel::Ulfrco
    }
    #[doc = "LFRCO"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == Clksel::Lfrco
    }
    #[doc = "LFXO"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == Clksel::Lfxo
    }
    #[doc = "HFCORECLK"]
    #[inline(always)]
    pub fn is_hfcoreclk(&self) -> bool {
        *self == Clksel::Hfcoreclk
    }
}
#[doc = "Field `CLKSEL` writer - Watchdog Clock Select"]
pub type ClkselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Clksel, crate::Safe>;
impl<'a, REG> ClkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ULFRCO"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Ulfrco)
    }
    #[doc = "LFRCO"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Lfrco)
    }
    #[doc = "LFXO"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Lfxo)
    }
    #[doc = "HFCORECLK"]
    #[inline(always)]
    pub fn hfcoreclk(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Hfcoreclk)
    }
}
#[doc = "Field `WARNSEL` reader - Watchdog Timeout Period Select"]
pub type WarnselR = crate::FieldReader;
#[doc = "Field `WARNSEL` writer - Watchdog Timeout Period Select"]
pub type WarnselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WINSEL` reader - Watchdog Illegal Window Select"]
pub type WinselR = crate::FieldReader;
#[doc = "Field `WINSEL` writer - Watchdog Illegal Window Select"]
pub type WinselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CLRSRC` reader - Watchdog Clear Source"]
pub type ClrsrcR = crate::BitReader;
#[doc = "Field `CLRSRC` writer - Watchdog Clear Source"]
pub type ClrsrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDOGRSTDIS` reader - Watchdog Reset Disable"]
pub type WdogrstdisR = crate::BitReader;
#[doc = "Field `WDOGRSTDIS` writer - Watchdog Reset Disable"]
pub type WdogrstdisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Watchdog Timer Enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&self) -> DebugrunR {
        DebugrunR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Energy Mode 2 Run Enable"]
    #[inline(always)]
    pub fn em2run(&self) -> Em2runR {
        Em2runR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Energy Mode 3 Run Enable"]
    #[inline(always)]
    pub fn em3run(&self) -> Em3runR {
        Em3runR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configuration Lock"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Energy Mode 4 Block"]
    #[inline(always)]
    pub fn em4block(&self) -> Em4blockR {
        Em4blockR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Software Oscillator Disable Block"]
    #[inline(always)]
    pub fn swoscblock(&self) -> SwoscblockR {
        SwoscblockR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Watchdog Timeout Period Select"]
    #[inline(always)]
    pub fn persel(&self) -> PerselR {
        PerselR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - Watchdog Clock Select"]
    #[inline(always)]
    pub fn clksel(&self) -> ClkselR {
        ClkselR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Watchdog Timeout Period Select"]
    #[inline(always)]
    pub fn warnsel(&self) -> WarnselR {
        WarnselR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Watchdog Illegal Window Select"]
    #[inline(always)]
    pub fn winsel(&self) -> WinselR {
        WinselR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 30 - Watchdog Clear Source"]
    #[inline(always)]
    pub fn clrsrc(&self) -> ClrsrcR {
        ClrsrcR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Watchdog Reset Disable"]
    #[inline(always)]
    pub fn wdogrstdis(&self) -> WdogrstdisR {
        WdogrstdisR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Timer Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, CtrlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&mut self) -> DebugrunW<'_, CtrlSpec> {
        DebugrunW::new(self, 1)
    }
    #[doc = "Bit 2 - Energy Mode 2 Run Enable"]
    #[inline(always)]
    pub fn em2run(&mut self) -> Em2runW<'_, CtrlSpec> {
        Em2runW::new(self, 2)
    }
    #[doc = "Bit 3 - Energy Mode 3 Run Enable"]
    #[inline(always)]
    pub fn em3run(&mut self) -> Em3runW<'_, CtrlSpec> {
        Em3runW::new(self, 3)
    }
    #[doc = "Bit 4 - Configuration Lock"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<'_, CtrlSpec> {
        LockW::new(self, 4)
    }
    #[doc = "Bit 5 - Energy Mode 4 Block"]
    #[inline(always)]
    pub fn em4block(&mut self) -> Em4blockW<'_, CtrlSpec> {
        Em4blockW::new(self, 5)
    }
    #[doc = "Bit 6 - Software Oscillator Disable Block"]
    #[inline(always)]
    pub fn swoscblock(&mut self) -> SwoscblockW<'_, CtrlSpec> {
        SwoscblockW::new(self, 6)
    }
    #[doc = "Bits 8:11 - Watchdog Timeout Period Select"]
    #[inline(always)]
    pub fn persel(&mut self) -> PerselW<'_, CtrlSpec> {
        PerselW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Watchdog Clock Select"]
    #[inline(always)]
    pub fn clksel(&mut self) -> ClkselW<'_, CtrlSpec> {
        ClkselW::new(self, 12)
    }
    #[doc = "Bits 16:17 - Watchdog Timeout Period Select"]
    #[inline(always)]
    pub fn warnsel(&mut self) -> WarnselW<'_, CtrlSpec> {
        WarnselW::new(self, 16)
    }
    #[doc = "Bits 24:26 - Watchdog Illegal Window Select"]
    #[inline(always)]
    pub fn winsel(&mut self) -> WinselW<'_, CtrlSpec> {
        WinselW::new(self, 24)
    }
    #[doc = "Bit 30 - Watchdog Clear Source"]
    #[inline(always)]
    pub fn clrsrc(&mut self) -> ClrsrcW<'_, CtrlSpec> {
        ClrsrcW::new(self, 30)
    }
    #[doc = "Bit 31 - Watchdog Reset Disable"]
    #[inline(always)]
    pub fn wdogrstdis(&mut self) -> WdogrstdisW<'_, CtrlSpec> {
        WdogrstdisW::new(self, 31)
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
#[doc = "`reset()` method sets CTRL to value 0x0f00"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x0f00;
}
