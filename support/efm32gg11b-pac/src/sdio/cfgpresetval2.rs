#[doc = "Register `CFGPRESETVAL2` reader"]
pub type R = crate::R<CFGPRESETVAL2_SPEC>;
#[doc = "Register `CFGPRESETVAL2` writer"]
pub type W = crate::W<CFGPRESETVAL2_SPEC>;
#[doc = "Field `SDR25SDCLKFREQ` reader - SDR25 SD_CLK Frequency"]
pub type SDR25SDCLKFREQ_R = crate::FieldReader<u16>;
#[doc = "Field `SDR25SDCLKFREQ` writer - SDR25 SD_CLK Frequency"]
pub type SDR25SDCLKFREQ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `SDR25CLKGENEN` reader - SDR25 SD_CLK Gen Enable"]
pub type SDR25CLKGENEN_R = crate::BitReader;
#[doc = "Field `SDR25CLKGENEN` writer - SDR25 SD_CLK Gen Enable"]
pub type SDR25CLKGENEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SDR25DRVST` reader - SDR25 SD Drive Strength"]
pub type SDR25DRVST_R = crate::FieldReader;
#[doc = "Field `SDR25DRVST` writer - SDR25 SD Drive Strength"]
pub type SDR25DRVST_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `SDR50SDCLKFREQ` reader - Preset Value for SDR50 Speed of SD_CLK"]
pub type SDR50SDCLKFREQ_R = crate::FieldReader<u16>;
#[doc = "Field `SDR50SDCLKFREQ` writer - Preset Value for SDR50 Speed of SD_CLK"]
pub type SDR50SDCLKFREQ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `SDR50CLKGENEN` reader - SDR50 Speed Clock Gen Enable"]
pub type SDR50CLKGENEN_R = crate::BitReader;
#[doc = "Field `SDR50CLKGENEN` writer - SDR50 Speed Clock Gen Enable"]
pub type SDR50CLKGENEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SDR50DRVST` reader - SDR50 Speed Drive Strength"]
pub type SDR50DRVST_R = crate::FieldReader;
#[doc = "Field `SDR50DRVST` writer - SDR50 Speed Drive Strength"]
pub type SDR50DRVST_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:9 - SDR25 SD_CLK Frequency"]
    #[inline(always)]
    pub fn sdr25sdclkfreq(&self) -> SDR25SDCLKFREQ_R {
        SDR25SDCLKFREQ_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - SDR25 SD_CLK Gen Enable"]
    #[inline(always)]
    pub fn sdr25clkgenen(&self) -> SDR25CLKGENEN_R {
        SDR25CLKGENEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - SDR25 SD Drive Strength"]
    #[inline(always)]
    pub fn sdr25drvst(&self) -> SDR25DRVST_R {
        SDR25DRVST_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 16:25 - Preset Value for SDR50 Speed of SD_CLK"]
    #[inline(always)]
    pub fn sdr50sdclkfreq(&self) -> SDR50SDCLKFREQ_R {
        SDR50SDCLKFREQ_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - SDR50 Speed Clock Gen Enable"]
    #[inline(always)]
    pub fn sdr50clkgenen(&self) -> SDR50CLKGENEN_R {
        SDR50CLKGENEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - SDR50 Speed Drive Strength"]
    #[inline(always)]
    pub fn sdr50drvst(&self) -> SDR50DRVST_R {
        SDR50DRVST_R::new(((self.bits >> 27) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - SDR25 SD_CLK Frequency"]
    #[inline(always)]
    #[must_use]
    pub fn sdr25sdclkfreq(&mut self) -> SDR25SDCLKFREQ_W<CFGPRESETVAL2_SPEC, 0> {
        SDR25SDCLKFREQ_W::new(self)
    }
    #[doc = "Bit 10 - SDR25 SD_CLK Gen Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdr25clkgenen(&mut self) -> SDR25CLKGENEN_W<CFGPRESETVAL2_SPEC, 10> {
        SDR25CLKGENEN_W::new(self)
    }
    #[doc = "Bits 11:12 - SDR25 SD Drive Strength"]
    #[inline(always)]
    #[must_use]
    pub fn sdr25drvst(&mut self) -> SDR25DRVST_W<CFGPRESETVAL2_SPEC, 11> {
        SDR25DRVST_W::new(self)
    }
    #[doc = "Bits 16:25 - Preset Value for SDR50 Speed of SD_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn sdr50sdclkfreq(&mut self) -> SDR50SDCLKFREQ_W<CFGPRESETVAL2_SPEC, 16> {
        SDR50SDCLKFREQ_W::new(self)
    }
    #[doc = "Bit 26 - SDR50 Speed Clock Gen Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdr50clkgenen(&mut self) -> SDR50CLKGENEN_W<CFGPRESETVAL2_SPEC, 26> {
        SDR50CLKGENEN_W::new(self)
    }
    #[doc = "Bits 27:28 - SDR50 Speed Drive Strength"]
    #[inline(always)]
    #[must_use]
    pub fn sdr50drvst(&mut self) -> SDR50DRVST_W<CFGPRESETVAL2_SPEC, 27> {
        SDR50DRVST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Core Configuration Preset Value 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgpresetval2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgpresetval2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGPRESETVAL2_SPEC;
impl crate::RegisterSpec for CFGPRESETVAL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgpresetval2::R`](R) reader structure"]
impl crate::Readable for CFGPRESETVAL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfgpresetval2::W`](W) writer structure"]
impl crate::Writable for CFGPRESETVAL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGPRESETVAL2 to value 0"]
impl crate::Resettable for CFGPRESETVAL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
