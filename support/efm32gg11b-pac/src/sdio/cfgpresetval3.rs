#[doc = "Register `CFGPRESETVAL3` reader"]
pub type R = crate::R<Cfgpresetval3Spec>;
#[doc = "Register `CFGPRESETVAL3` writer"]
pub type W = crate::W<Cfgpresetval3Spec>;
#[doc = "Field `SDR104SDCLKFREQ` reader - SDR104 SD_CLK Frequency"]
pub type Sdr104sdclkfreqR = crate::FieldReader<u16>;
#[doc = "Field `SDR104SDCLKFREQ` writer - SDR104 SD_CLK Frequency"]
pub type Sdr104sdclkfreqW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SDR104CLKGENEN` reader - SDR104 SD_CLK Gen Enable"]
pub type Sdr104clkgenenR = crate::BitReader;
#[doc = "Field `SDR104CLKGENEN` writer - SDR104 SD_CLK Gen Enable"]
pub type Sdr104clkgenenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDR104DRVST` reader - SDR104 SD Drive Strength"]
pub type Sdr104drvstR = crate::FieldReader;
#[doc = "Field `SDR104DRVST` writer - SDR104 SD Drive Strength"]
pub type Sdr104drvstW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DDR50SDCLKFREQ` reader - Preset Value for DDR50 Speed of SD_CLK"]
pub type Ddr50sdclkfreqR = crate::FieldReader<u16>;
#[doc = "Field `DDR50SDCLKFREQ` writer - Preset Value for DDR50 Speed of SD_CLK"]
pub type Ddr50sdclkfreqW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DDR50CLKGENEN` reader - DDR50 Speed Clock Gen Enable"]
pub type Ddr50clkgenenR = crate::BitReader;
#[doc = "Field `DDR50CLKGENEN` writer - DDR50 Speed Clock Gen Enable"]
pub type Ddr50clkgenenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR50DRVST` reader - DDR50 Speed Drive Strength"]
pub type Ddr50drvstR = crate::FieldReader;
#[doc = "Field `DDR50DRVST` writer - DDR50 Speed Drive Strength"]
pub type Ddr50drvstW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:9 - SDR104 SD_CLK Frequency"]
    #[inline(always)]
    pub fn sdr104sdclkfreq(&self) -> Sdr104sdclkfreqR {
        Sdr104sdclkfreqR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - SDR104 SD_CLK Gen Enable"]
    #[inline(always)]
    pub fn sdr104clkgenen(&self) -> Sdr104clkgenenR {
        Sdr104clkgenenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - SDR104 SD Drive Strength"]
    #[inline(always)]
    pub fn sdr104drvst(&self) -> Sdr104drvstR {
        Sdr104drvstR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 16:25 - Preset Value for DDR50 Speed of SD_CLK"]
    #[inline(always)]
    pub fn ddr50sdclkfreq(&self) -> Ddr50sdclkfreqR {
        Ddr50sdclkfreqR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - DDR50 Speed Clock Gen Enable"]
    #[inline(always)]
    pub fn ddr50clkgenen(&self) -> Ddr50clkgenenR {
        Ddr50clkgenenR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - DDR50 Speed Drive Strength"]
    #[inline(always)]
    pub fn ddr50drvst(&self) -> Ddr50drvstR {
        Ddr50drvstR::new(((self.bits >> 27) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - SDR104 SD_CLK Frequency"]
    #[inline(always)]
    pub fn sdr104sdclkfreq(&mut self) -> Sdr104sdclkfreqW<'_, Cfgpresetval3Spec> {
        Sdr104sdclkfreqW::new(self, 0)
    }
    #[doc = "Bit 10 - SDR104 SD_CLK Gen Enable"]
    #[inline(always)]
    pub fn sdr104clkgenen(&mut self) -> Sdr104clkgenenW<'_, Cfgpresetval3Spec> {
        Sdr104clkgenenW::new(self, 10)
    }
    #[doc = "Bits 11:12 - SDR104 SD Drive Strength"]
    #[inline(always)]
    pub fn sdr104drvst(&mut self) -> Sdr104drvstW<'_, Cfgpresetval3Spec> {
        Sdr104drvstW::new(self, 11)
    }
    #[doc = "Bits 16:25 - Preset Value for DDR50 Speed of SD_CLK"]
    #[inline(always)]
    pub fn ddr50sdclkfreq(&mut self) -> Ddr50sdclkfreqW<'_, Cfgpresetval3Spec> {
        Ddr50sdclkfreqW::new(self, 16)
    }
    #[doc = "Bit 26 - DDR50 Speed Clock Gen Enable"]
    #[inline(always)]
    pub fn ddr50clkgenen(&mut self) -> Ddr50clkgenenW<'_, Cfgpresetval3Spec> {
        Ddr50clkgenenW::new(self, 26)
    }
    #[doc = "Bits 27:28 - DDR50 Speed Drive Strength"]
    #[inline(always)]
    pub fn ddr50drvst(&mut self) -> Ddr50drvstW<'_, Cfgpresetval3Spec> {
        Ddr50drvstW::new(self, 27)
    }
}
#[doc = "Core Configuration Preset Value 3\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgpresetval3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgpresetval3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfgpresetval3Spec;
impl crate::RegisterSpec for Cfgpresetval3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgpresetval3::R`](R) reader structure"]
impl crate::Readable for Cfgpresetval3Spec {}
#[doc = "`write(|w| ..)` method takes [`cfgpresetval3::W`](W) writer structure"]
impl crate::Writable for Cfgpresetval3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGPRESETVAL3 to value 0"]
impl crate::Resettable for Cfgpresetval3Spec {}
