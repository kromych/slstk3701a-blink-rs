#[doc = "Register `CLOCKCTRL` reader"]
pub type R = crate::R<ClockctrlSpec>;
#[doc = "Register `CLOCKCTRL` writer"]
pub type W = crate::W<ClockctrlSpec>;
#[doc = "Field `INTCLKEN` reader - Internal Clock Enable"]
pub type IntclkenR = crate::BitReader;
#[doc = "Field `INTCLKEN` writer - Internal Clock Enable"]
pub type IntclkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTCLKSTABLE` reader - Internal Clock Stable"]
pub type IntclkstableR = crate::BitReader;
#[doc = "Field `SDCLKEN` reader - SDIO_CLK Pin Clock Enable"]
pub type SdclkenR = crate::BitReader;
#[doc = "Field `SDCLKEN` writer - SDIO_CLK Pin Clock Enable"]
pub type SdclkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKGENSEL` reader - Clock Generator Select"]
pub type ClkgenselR = crate::BitReader;
#[doc = "Field `CLKGENSEL` writer - Clock Generator Select"]
pub type ClkgenselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPPSDCLKFRE` reader - Upper Bits of SD_CLK Frequency Select"]
pub type UppsdclkfreR = crate::FieldReader;
#[doc = "Field `UPPSDCLKFRE` writer - Upper Bits of SD_CLK Frequency Select"]
pub type UppsdclkfreW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "SD_CLK Frequency Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sdclkfreqsel {
    #[doc = "0: `0`"]
    Nodivision = 0,
}
impl From<Sdclkfreqsel> for u8 {
    #[inline(always)]
    fn from(variant: Sdclkfreqsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sdclkfreqsel {
    type Ux = u8;
}
impl crate::IsEnum for Sdclkfreqsel {}
#[doc = "Field `SDCLKFREQSEL` reader - SD_CLK Frequency Select"]
pub type SdclkfreqselR = crate::FieldReader<Sdclkfreqsel>;
impl SdclkfreqselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sdclkfreqsel> {
        match self.bits {
            0 => Some(Sdclkfreqsel::Nodivision),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_nodivision(&self) -> bool {
        *self == Sdclkfreqsel::Nodivision
    }
}
#[doc = "Field `SDCLKFREQSEL` writer - SD_CLK Frequency Select"]
pub type SdclkfreqselW<'a, REG> = crate::FieldWriter<'a, REG, 8, Sdclkfreqsel>;
impl<'a, REG> SdclkfreqselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn nodivision(self) -> &'a mut crate::W<REG> {
        self.variant(Sdclkfreqsel::Nodivision)
    }
}
#[doc = "Field `DATTOUTCNTVAL` reader - Data Timeout Counter Value"]
pub type DattoutcntvalR = crate::FieldReader;
#[doc = "Field `DATTOUTCNTVAL` writer - Data Timeout Counter Value"]
pub type DattoutcntvalW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SFTRSTA` reader - Software Reset for All"]
pub type SftrstaR = crate::BitReader;
#[doc = "Field `SFTRSTA` writer - Software Reset for All"]
pub type SftrstaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFTRSTCMD` reader - Software Reset for CMD Line"]
pub type SftrstcmdR = crate::BitReader;
#[doc = "Field `SFTRSTCMD` writer - Software Reset for CMD Line"]
pub type SftrstcmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFTRSTDAT` reader - Software Reset for DAT Line"]
pub type SftrstdatR = crate::BitReader;
#[doc = "Field `SFTRSTDAT` writer - Software Reset for DAT Line"]
pub type SftrstdatW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Internal Clock Enable"]
    #[inline(always)]
    pub fn intclken(&self) -> IntclkenR {
        IntclkenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal Clock Stable"]
    #[inline(always)]
    pub fn intclkstable(&self) -> IntclkstableR {
        IntclkstableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SDIO_CLK Pin Clock Enable"]
    #[inline(always)]
    pub fn sdclken(&self) -> SdclkenR {
        SdclkenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock Generator Select"]
    #[inline(always)]
    pub fn clkgensel(&self) -> ClkgenselR {
        ClkgenselR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Upper Bits of SD_CLK Frequency Select"]
    #[inline(always)]
    pub fn uppsdclkfre(&self) -> UppsdclkfreR {
        UppsdclkfreR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - SD_CLK Frequency Select"]
    #[inline(always)]
    pub fn sdclkfreqsel(&self) -> SdclkfreqselR {
        SdclkfreqselR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Data Timeout Counter Value"]
    #[inline(always)]
    pub fn dattoutcntval(&self) -> DattoutcntvalR {
        DattoutcntvalR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Software Reset for All"]
    #[inline(always)]
    pub fn sftrsta(&self) -> SftrstaR {
        SftrstaR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Software Reset for CMD Line"]
    #[inline(always)]
    pub fn sftrstcmd(&self) -> SftrstcmdR {
        SftrstcmdR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Software Reset for DAT Line"]
    #[inline(always)]
    pub fn sftrstdat(&self) -> SftrstdatR {
        SftrstdatR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal Clock Enable"]
    #[inline(always)]
    pub fn intclken(&mut self) -> IntclkenW<'_, ClockctrlSpec> {
        IntclkenW::new(self, 0)
    }
    #[doc = "Bit 2 - SDIO_CLK Pin Clock Enable"]
    #[inline(always)]
    pub fn sdclken(&mut self) -> SdclkenW<'_, ClockctrlSpec> {
        SdclkenW::new(self, 2)
    }
    #[doc = "Bit 5 - Clock Generator Select"]
    #[inline(always)]
    pub fn clkgensel(&mut self) -> ClkgenselW<'_, ClockctrlSpec> {
        ClkgenselW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Upper Bits of SD_CLK Frequency Select"]
    #[inline(always)]
    pub fn uppsdclkfre(&mut self) -> UppsdclkfreW<'_, ClockctrlSpec> {
        UppsdclkfreW::new(self, 6)
    }
    #[doc = "Bits 8:15 - SD_CLK Frequency Select"]
    #[inline(always)]
    pub fn sdclkfreqsel(&mut self) -> SdclkfreqselW<'_, ClockctrlSpec> {
        SdclkfreqselW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Data Timeout Counter Value"]
    #[inline(always)]
    pub fn dattoutcntval(&mut self) -> DattoutcntvalW<'_, ClockctrlSpec> {
        DattoutcntvalW::new(self, 16)
    }
    #[doc = "Bit 24 - Software Reset for All"]
    #[inline(always)]
    pub fn sftrsta(&mut self) -> SftrstaW<'_, ClockctrlSpec> {
        SftrstaW::new(self, 24)
    }
    #[doc = "Bit 25 - Software Reset for CMD Line"]
    #[inline(always)]
    pub fn sftrstcmd(&mut self) -> SftrstcmdW<'_, ClockctrlSpec> {
        SftrstcmdW::new(self, 25)
    }
    #[doc = "Bit 26 - Software Reset for DAT Line"]
    #[inline(always)]
    pub fn sftrstdat(&mut self) -> SftrstdatW<'_, ClockctrlSpec> {
        SftrstdatW::new(self, 26)
    }
}
#[doc = "Clock Control, Timeout Control and Software Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clockctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clockctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClockctrlSpec;
impl crate::RegisterSpec for ClockctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clockctrl::R`](R) reader structure"]
impl crate::Readable for ClockctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`clockctrl::W`](W) writer structure"]
impl crate::Writable for ClockctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLOCKCTRL to value 0"]
impl crate::Resettable for ClockctrlSpec {}
