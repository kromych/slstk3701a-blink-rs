#[doc = "Register `CFGPRESETVAL2` reader"]
pub type R = crate::R<Cfgpresetval2Spec>;
#[doc = "Register `CFGPRESETVAL2` writer"]
pub type W = crate::W<Cfgpresetval2Spec>;
#[doc = "Field `SDR25SDCLKFREQ` reader - SDR25 SD_CLK Frequency"]
pub type Sdr25sdclkfreqR = crate::FieldReader<u16>;
#[doc = "Field `SDR25SDCLKFREQ` writer - SDR25 SD_CLK Frequency"]
pub type Sdr25sdclkfreqW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SDR25CLKGENEN` reader - SDR25 SD_CLK Gen Enable"]
pub type Sdr25clkgenenR = crate::BitReader;
#[doc = "Field `SDR25CLKGENEN` writer - SDR25 SD_CLK Gen Enable"]
pub type Sdr25clkgenenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDR25DRVST` reader - SDR25 SD Drive Strength"]
pub type Sdr25drvstR = crate::FieldReader;
#[doc = "Field `SDR25DRVST` writer - SDR25 SD Drive Strength"]
pub type Sdr25drvstW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SDR50SDCLKFREQ` reader - Preset Value for SDR50 Speed of SD_CLK"]
pub type Sdr50sdclkfreqR = crate::FieldReader<u16>;
#[doc = "Field `SDR50SDCLKFREQ` writer - Preset Value for SDR50 Speed of SD_CLK"]
pub type Sdr50sdclkfreqW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SDR50CLKGENEN` reader - SDR50 Speed Clock Gen Enable"]
pub type Sdr50clkgenenR = crate::BitReader;
#[doc = "Field `SDR50CLKGENEN` writer - SDR50 Speed Clock Gen Enable"]
pub type Sdr50clkgenenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDR50DRVST` reader - SDR50 Speed Drive Strength"]
pub type Sdr50drvstR = crate::FieldReader;
#[doc = "Field `SDR50DRVST` writer - SDR50 Speed Drive Strength"]
pub type Sdr50drvstW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:9 - SDR25 SD_CLK Frequency"]
    #[inline(always)]
    pub fn sdr25sdclkfreq(&self) -> Sdr25sdclkfreqR {
        Sdr25sdclkfreqR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - SDR25 SD_CLK Gen Enable"]
    #[inline(always)]
    pub fn sdr25clkgenen(&self) -> Sdr25clkgenenR {
        Sdr25clkgenenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - SDR25 SD Drive Strength"]
    #[inline(always)]
    pub fn sdr25drvst(&self) -> Sdr25drvstR {
        Sdr25drvstR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 16:25 - Preset Value for SDR50 Speed of SD_CLK"]
    #[inline(always)]
    pub fn sdr50sdclkfreq(&self) -> Sdr50sdclkfreqR {
        Sdr50sdclkfreqR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - SDR50 Speed Clock Gen Enable"]
    #[inline(always)]
    pub fn sdr50clkgenen(&self) -> Sdr50clkgenenR {
        Sdr50clkgenenR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - SDR50 Speed Drive Strength"]
    #[inline(always)]
    pub fn sdr50drvst(&self) -> Sdr50drvstR {
        Sdr50drvstR::new(((self.bits >> 27) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - SDR25 SD_CLK Frequency"]
    #[inline(always)]
    pub fn sdr25sdclkfreq(&mut self) -> Sdr25sdclkfreqW<'_, Cfgpresetval2Spec> {
        Sdr25sdclkfreqW::new(self, 0)
    }
    #[doc = "Bit 10 - SDR25 SD_CLK Gen Enable"]
    #[inline(always)]
    pub fn sdr25clkgenen(&mut self) -> Sdr25clkgenenW<'_, Cfgpresetval2Spec> {
        Sdr25clkgenenW::new(self, 10)
    }
    #[doc = "Bits 11:12 - SDR25 SD Drive Strength"]
    #[inline(always)]
    pub fn sdr25drvst(&mut self) -> Sdr25drvstW<'_, Cfgpresetval2Spec> {
        Sdr25drvstW::new(self, 11)
    }
    #[doc = "Bits 16:25 - Preset Value for SDR50 Speed of SD_CLK"]
    #[inline(always)]
    pub fn sdr50sdclkfreq(&mut self) -> Sdr50sdclkfreqW<'_, Cfgpresetval2Spec> {
        Sdr50sdclkfreqW::new(self, 16)
    }
    #[doc = "Bit 26 - SDR50 Speed Clock Gen Enable"]
    #[inline(always)]
    pub fn sdr50clkgenen(&mut self) -> Sdr50clkgenenW<'_, Cfgpresetval2Spec> {
        Sdr50clkgenenW::new(self, 26)
    }
    #[doc = "Bits 27:28 - SDR50 Speed Drive Strength"]
    #[inline(always)]
    pub fn sdr50drvst(&mut self) -> Sdr50drvstW<'_, Cfgpresetval2Spec> {
        Sdr50drvstW::new(self, 27)
    }
}
#[doc = "Core Configuration Preset Value 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgpresetval2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgpresetval2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfgpresetval2Spec;
impl crate::RegisterSpec for Cfgpresetval2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgpresetval2::R`](R) reader structure"]
impl crate::Readable for Cfgpresetval2Spec {}
#[doc = "`write(|w| ..)` method takes [`cfgpresetval2::W`](W) writer structure"]
impl crate::Writable for Cfgpresetval2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGPRESETVAL2 to value 0"]
impl crate::Resettable for Cfgpresetval2Spec {}
