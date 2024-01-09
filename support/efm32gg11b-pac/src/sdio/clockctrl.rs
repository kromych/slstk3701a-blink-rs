#[doc = "Register `CLOCKCTRL` reader"]
pub type R = crate::R<CLOCKCTRL_SPEC>;
#[doc = "Register `CLOCKCTRL` writer"]
pub type W = crate::W<CLOCKCTRL_SPEC>;
#[doc = "Field `INTCLKEN` reader - Internal Clock Enable"]
pub type INTCLKEN_R = crate::BitReader;
#[doc = "Field `INTCLKEN` writer - Internal Clock Enable"]
pub type INTCLKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTCLKSTABLE` reader - Internal Clock Stable"]
pub type INTCLKSTABLE_R = crate::BitReader;
#[doc = "Field `SDCLKEN` reader - SDIO_CLK Pin Clock Enable"]
pub type SDCLKEN_R = crate::BitReader;
#[doc = "Field `SDCLKEN` writer - SDIO_CLK Pin Clock Enable"]
pub type SDCLKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKGENSEL` reader - Clock Generator Select"]
pub type CLKGENSEL_R = crate::BitReader;
#[doc = "Field `CLKGENSEL` writer - Clock Generator Select"]
pub type CLKGENSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPPSDCLKFRE` reader - Upper Bits of SD_CLK Frequency Select"]
pub type UPPSDCLKFRE_R = crate::FieldReader;
#[doc = "Field `UPPSDCLKFRE` writer - Upper Bits of SD_CLK Frequency Select"]
pub type UPPSDCLKFRE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SDCLKFREQSEL` reader - SD_CLK Frequency Select"]
pub type SDCLKFREQSEL_R = crate::FieldReader<SDCLKFREQSEL_A>;
#[doc = "SD_CLK Frequency Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SDCLKFREQSEL_A {
    #[doc = "0: `0`"]
    NODIVISION = 0,
}
impl From<SDCLKFREQSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SDCLKFREQSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SDCLKFREQSEL_A {
    type Ux = u8;
}
impl SDCLKFREQSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SDCLKFREQSEL_A> {
        match self.bits {
            0 => Some(SDCLKFREQSEL_A::NODIVISION),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_nodivision(&self) -> bool {
        *self == SDCLKFREQSEL_A::NODIVISION
    }
}
#[doc = "Field `SDCLKFREQSEL` writer - SD_CLK Frequency Select"]
pub type SDCLKFREQSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8, SDCLKFREQSEL_A>;
impl<'a, REG> SDCLKFREQSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn nodivision(self) -> &'a mut crate::W<REG> {
        self.variant(SDCLKFREQSEL_A::NODIVISION)
    }
}
#[doc = "Field `DATTOUTCNTVAL` reader - Data Timeout Counter Value"]
pub type DATTOUTCNTVAL_R = crate::FieldReader;
#[doc = "Field `DATTOUTCNTVAL` writer - Data Timeout Counter Value"]
pub type DATTOUTCNTVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SFTRSTA` reader - Software Reset for All"]
pub type SFTRSTA_R = crate::BitReader;
#[doc = "Field `SFTRSTA` writer - Software Reset for All"]
pub type SFTRSTA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFTRSTCMD` reader - Software Reset for CMD Line"]
pub type SFTRSTCMD_R = crate::BitReader;
#[doc = "Field `SFTRSTCMD` writer - Software Reset for CMD Line"]
pub type SFTRSTCMD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFTRSTDAT` reader - Software Reset for DAT Line"]
pub type SFTRSTDAT_R = crate::BitReader;
#[doc = "Field `SFTRSTDAT` writer - Software Reset for DAT Line"]
pub type SFTRSTDAT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Internal Clock Enable"]
    #[inline(always)]
    pub fn intclken(&self) -> INTCLKEN_R {
        INTCLKEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal Clock Stable"]
    #[inline(always)]
    pub fn intclkstable(&self) -> INTCLKSTABLE_R {
        INTCLKSTABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SDIO_CLK Pin Clock Enable"]
    #[inline(always)]
    pub fn sdclken(&self) -> SDCLKEN_R {
        SDCLKEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock Generator Select"]
    #[inline(always)]
    pub fn clkgensel(&self) -> CLKGENSEL_R {
        CLKGENSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Upper Bits of SD_CLK Frequency Select"]
    #[inline(always)]
    pub fn uppsdclkfre(&self) -> UPPSDCLKFRE_R {
        UPPSDCLKFRE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - SD_CLK Frequency Select"]
    #[inline(always)]
    pub fn sdclkfreqsel(&self) -> SDCLKFREQSEL_R {
        SDCLKFREQSEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Data Timeout Counter Value"]
    #[inline(always)]
    pub fn dattoutcntval(&self) -> DATTOUTCNTVAL_R {
        DATTOUTCNTVAL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Software Reset for All"]
    #[inline(always)]
    pub fn sftrsta(&self) -> SFTRSTA_R {
        SFTRSTA_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Software Reset for CMD Line"]
    #[inline(always)]
    pub fn sftrstcmd(&self) -> SFTRSTCMD_R {
        SFTRSTCMD_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Software Reset for DAT Line"]
    #[inline(always)]
    pub fn sftrstdat(&self) -> SFTRSTDAT_R {
        SFTRSTDAT_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn intclken(&mut self) -> INTCLKEN_W<CLOCKCTRL_SPEC> {
        INTCLKEN_W::new(self, 0)
    }
    #[doc = "Bit 2 - SDIO_CLK Pin Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdclken(&mut self) -> SDCLKEN_W<CLOCKCTRL_SPEC> {
        SDCLKEN_W::new(self, 2)
    }
    #[doc = "Bit 5 - Clock Generator Select"]
    #[inline(always)]
    #[must_use]
    pub fn clkgensel(&mut self) -> CLKGENSEL_W<CLOCKCTRL_SPEC> {
        CLKGENSEL_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Upper Bits of SD_CLK Frequency Select"]
    #[inline(always)]
    #[must_use]
    pub fn uppsdclkfre(&mut self) -> UPPSDCLKFRE_W<CLOCKCTRL_SPEC> {
        UPPSDCLKFRE_W::new(self, 6)
    }
    #[doc = "Bits 8:15 - SD_CLK Frequency Select"]
    #[inline(always)]
    #[must_use]
    pub fn sdclkfreqsel(&mut self) -> SDCLKFREQSEL_W<CLOCKCTRL_SPEC> {
        SDCLKFREQSEL_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Data Timeout Counter Value"]
    #[inline(always)]
    #[must_use]
    pub fn dattoutcntval(&mut self) -> DATTOUTCNTVAL_W<CLOCKCTRL_SPEC> {
        DATTOUTCNTVAL_W::new(self, 16)
    }
    #[doc = "Bit 24 - Software Reset for All"]
    #[inline(always)]
    #[must_use]
    pub fn sftrsta(&mut self) -> SFTRSTA_W<CLOCKCTRL_SPEC> {
        SFTRSTA_W::new(self, 24)
    }
    #[doc = "Bit 25 - Software Reset for CMD Line"]
    #[inline(always)]
    #[must_use]
    pub fn sftrstcmd(&mut self) -> SFTRSTCMD_W<CLOCKCTRL_SPEC> {
        SFTRSTCMD_W::new(self, 25)
    }
    #[doc = "Bit 26 - Software Reset for DAT Line"]
    #[inline(always)]
    #[must_use]
    pub fn sftrstdat(&mut self) -> SFTRSTDAT_W<CLOCKCTRL_SPEC> {
        SFTRSTDAT_W::new(self, 26)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Clock Control, Timeout Control and Software Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clockctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clockctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLOCKCTRL_SPEC;
impl crate::RegisterSpec for CLOCKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clockctrl::R`](R) reader structure"]
impl crate::Readable for CLOCKCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clockctrl::W`](W) writer structure"]
impl crate::Writable for CLOCKCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLOCKCTRL to value 0"]
impl crate::Resettable for CLOCKCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
