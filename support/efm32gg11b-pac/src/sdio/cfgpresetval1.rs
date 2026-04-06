#[doc = "Register `CFGPRESETVAL1` reader"]
pub type R = crate::R<Cfgpresetval1Spec>;
#[doc = "Register `CFGPRESETVAL1` writer"]
pub type W = crate::W<Cfgpresetval1Spec>;
#[doc = "Field `HSPSDCLKFREQ` reader - High Speed SD_CLK Frequency"]
pub type HspsdclkfreqR = crate::FieldReader<u16>;
#[doc = "Field `HSPSDCLKFREQ` writer - High Speed SD_CLK Frequency"]
pub type HspsdclkfreqW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `HSPCLKGENEN` reader - High Speed SD_CLK Gen Enable"]
pub type HspclkgenenR = crate::BitReader;
#[doc = "Field `HSPCLKGENEN` writer - High Speed SD_CLK Gen Enable"]
pub type HspclkgenenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSPDRVST` reader - High Speed SD Drive Strength"]
pub type HspdrvstR = crate::FieldReader;
#[doc = "Field `HSPDRVST` writer - High Speed SD Drive Strength"]
pub type HspdrvstW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SDR12SDCLKFREQ` reader - Preset Value for SDR12 Speed of SD_CLK"]
pub type Sdr12sdclkfreqR = crate::FieldReader<u16>;
#[doc = "Field `SDR12SDCLKFREQ` writer - Preset Value for SDR12 Speed of SD_CLK"]
pub type Sdr12sdclkfreqW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SDR12CLKGENEN` reader - SDR12 Speed Clock Gen Enable"]
pub type Sdr12clkgenenR = crate::BitReader;
#[doc = "Field `SDR12CLKGENEN` writer - SDR12 Speed Clock Gen Enable"]
pub type Sdr12clkgenenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDR12DRVST` reader - SDR12 Speed Drive Strength"]
pub type Sdr12drvstR = crate::FieldReader;
#[doc = "Field `SDR12DRVST` writer - SDR12 Speed Drive Strength"]
pub type Sdr12drvstW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:9 - High Speed SD_CLK Frequency"]
    #[inline(always)]
    pub fn hspsdclkfreq(&self) -> HspsdclkfreqR {
        HspsdclkfreqR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - High Speed SD_CLK Gen Enable"]
    #[inline(always)]
    pub fn hspclkgenen(&self) -> HspclkgenenR {
        HspclkgenenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - High Speed SD Drive Strength"]
    #[inline(always)]
    pub fn hspdrvst(&self) -> HspdrvstR {
        HspdrvstR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 16:25 - Preset Value for SDR12 Speed of SD_CLK"]
    #[inline(always)]
    pub fn sdr12sdclkfreq(&self) -> Sdr12sdclkfreqR {
        Sdr12sdclkfreqR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - SDR12 Speed Clock Gen Enable"]
    #[inline(always)]
    pub fn sdr12clkgenen(&self) -> Sdr12clkgenenR {
        Sdr12clkgenenR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - SDR12 Speed Drive Strength"]
    #[inline(always)]
    pub fn sdr12drvst(&self) -> Sdr12drvstR {
        Sdr12drvstR::new(((self.bits >> 27) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - High Speed SD_CLK Frequency"]
    #[inline(always)]
    pub fn hspsdclkfreq(&mut self) -> HspsdclkfreqW<'_, Cfgpresetval1Spec> {
        HspsdclkfreqW::new(self, 0)
    }
    #[doc = "Bit 10 - High Speed SD_CLK Gen Enable"]
    #[inline(always)]
    pub fn hspclkgenen(&mut self) -> HspclkgenenW<'_, Cfgpresetval1Spec> {
        HspclkgenenW::new(self, 10)
    }
    #[doc = "Bits 11:12 - High Speed SD Drive Strength"]
    #[inline(always)]
    pub fn hspdrvst(&mut self) -> HspdrvstW<'_, Cfgpresetval1Spec> {
        HspdrvstW::new(self, 11)
    }
    #[doc = "Bits 16:25 - Preset Value for SDR12 Speed of SD_CLK"]
    #[inline(always)]
    pub fn sdr12sdclkfreq(&mut self) -> Sdr12sdclkfreqW<'_, Cfgpresetval1Spec> {
        Sdr12sdclkfreqW::new(self, 16)
    }
    #[doc = "Bit 26 - SDR12 Speed Clock Gen Enable"]
    #[inline(always)]
    pub fn sdr12clkgenen(&mut self) -> Sdr12clkgenenW<'_, Cfgpresetval1Spec> {
        Sdr12clkgenenW::new(self, 26)
    }
    #[doc = "Bits 27:28 - SDR12 Speed Drive Strength"]
    #[inline(always)]
    pub fn sdr12drvst(&mut self) -> Sdr12drvstW<'_, Cfgpresetval1Spec> {
        Sdr12drvstW::new(self, 27)
    }
}
#[doc = "Core Configuration Preset Value 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgpresetval1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgpresetval1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfgpresetval1Spec;
impl crate::RegisterSpec for Cfgpresetval1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgpresetval1::R`](R) reader structure"]
impl crate::Readable for Cfgpresetval1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfgpresetval1::W`](W) writer structure"]
impl crate::Writable for Cfgpresetval1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGPRESETVAL1 to value 0"]
impl crate::Resettable for Cfgpresetval1Spec {}
