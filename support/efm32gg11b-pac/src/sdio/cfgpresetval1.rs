#[doc = "Register `CFGPRESETVAL1` reader"]
pub type R = crate::R<CFGPRESETVAL1_SPEC>;
#[doc = "Register `CFGPRESETVAL1` writer"]
pub type W = crate::W<CFGPRESETVAL1_SPEC>;
#[doc = "Field `HSPSDCLKFREQ` reader - High Speed SD_CLK Frequency"]
pub type HSPSDCLKFREQ_R = crate::FieldReader<u16>;
#[doc = "Field `HSPSDCLKFREQ` writer - High Speed SD_CLK Frequency"]
pub type HSPSDCLKFREQ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `HSPCLKGENEN` reader - High Speed SD_CLK Gen Enable"]
pub type HSPCLKGENEN_R = crate::BitReader;
#[doc = "Field `HSPCLKGENEN` writer - High Speed SD_CLK Gen Enable"]
pub type HSPCLKGENEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSPDRVST` reader - High Speed SD Drive Strength"]
pub type HSPDRVST_R = crate::FieldReader;
#[doc = "Field `HSPDRVST` writer - High Speed SD Drive Strength"]
pub type HSPDRVST_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `SDR12SDCLKFREQ` reader - Preset Value for SDR12 Speed of SD_CLK"]
pub type SDR12SDCLKFREQ_R = crate::FieldReader<u16>;
#[doc = "Field `SDR12SDCLKFREQ` writer - Preset Value for SDR12 Speed of SD_CLK"]
pub type SDR12SDCLKFREQ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `SDR12CLKGENEN` reader - SDR12 Speed Clock Gen Enable"]
pub type SDR12CLKGENEN_R = crate::BitReader;
#[doc = "Field `SDR12CLKGENEN` writer - SDR12 Speed Clock Gen Enable"]
pub type SDR12CLKGENEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SDR12DRVST` reader - SDR12 Speed Drive Strength"]
pub type SDR12DRVST_R = crate::FieldReader;
#[doc = "Field `SDR12DRVST` writer - SDR12 Speed Drive Strength"]
pub type SDR12DRVST_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:9 - High Speed SD_CLK Frequency"]
    #[inline(always)]
    pub fn hspsdclkfreq(&self) -> HSPSDCLKFREQ_R {
        HSPSDCLKFREQ_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - High Speed SD_CLK Gen Enable"]
    #[inline(always)]
    pub fn hspclkgenen(&self) -> HSPCLKGENEN_R {
        HSPCLKGENEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - High Speed SD Drive Strength"]
    #[inline(always)]
    pub fn hspdrvst(&self) -> HSPDRVST_R {
        HSPDRVST_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 16:25 - Preset Value for SDR12 Speed of SD_CLK"]
    #[inline(always)]
    pub fn sdr12sdclkfreq(&self) -> SDR12SDCLKFREQ_R {
        SDR12SDCLKFREQ_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - SDR12 Speed Clock Gen Enable"]
    #[inline(always)]
    pub fn sdr12clkgenen(&self) -> SDR12CLKGENEN_R {
        SDR12CLKGENEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - SDR12 Speed Drive Strength"]
    #[inline(always)]
    pub fn sdr12drvst(&self) -> SDR12DRVST_R {
        SDR12DRVST_R::new(((self.bits >> 27) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - High Speed SD_CLK Frequency"]
    #[inline(always)]
    #[must_use]
    pub fn hspsdclkfreq(&mut self) -> HSPSDCLKFREQ_W<CFGPRESETVAL1_SPEC, 0> {
        HSPSDCLKFREQ_W::new(self)
    }
    #[doc = "Bit 10 - High Speed SD_CLK Gen Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hspclkgenen(&mut self) -> HSPCLKGENEN_W<CFGPRESETVAL1_SPEC, 10> {
        HSPCLKGENEN_W::new(self)
    }
    #[doc = "Bits 11:12 - High Speed SD Drive Strength"]
    #[inline(always)]
    #[must_use]
    pub fn hspdrvst(&mut self) -> HSPDRVST_W<CFGPRESETVAL1_SPEC, 11> {
        HSPDRVST_W::new(self)
    }
    #[doc = "Bits 16:25 - Preset Value for SDR12 Speed of SD_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn sdr12sdclkfreq(&mut self) -> SDR12SDCLKFREQ_W<CFGPRESETVAL1_SPEC, 16> {
        SDR12SDCLKFREQ_W::new(self)
    }
    #[doc = "Bit 26 - SDR12 Speed Clock Gen Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdr12clkgenen(&mut self) -> SDR12CLKGENEN_W<CFGPRESETVAL1_SPEC, 26> {
        SDR12CLKGENEN_W::new(self)
    }
    #[doc = "Bits 27:28 - SDR12 Speed Drive Strength"]
    #[inline(always)]
    #[must_use]
    pub fn sdr12drvst(&mut self) -> SDR12DRVST_W<CFGPRESETVAL1_SPEC, 27> {
        SDR12DRVST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Core Configuration Preset Value 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgpresetval1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgpresetval1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGPRESETVAL1_SPEC;
impl crate::RegisterSpec for CFGPRESETVAL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgpresetval1::R`](R) reader structure"]
impl crate::Readable for CFGPRESETVAL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfgpresetval1::W`](W) writer structure"]
impl crate::Writable for CFGPRESETVAL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGPRESETVAL1 to value 0"]
impl crate::Resettable for CFGPRESETVAL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
