#[doc = "Register `CFGPRESETVAL0` reader"]
pub type R = crate::R<Cfgpresetval0Spec>;
#[doc = "Register `CFGPRESETVAL0` writer"]
pub type W = crate::W<Cfgpresetval0Spec>;
#[doc = "Field `INITSDCLKFREQ` reader - Initial SD_CLK Frequency"]
pub type InitsdclkfreqR = crate::FieldReader<u16>;
#[doc = "Field `INITSDCLKFREQ` writer - Initial SD_CLK Frequency"]
pub type InitsdclkfreqW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `INITCLKGENEN` reader - Initial Clock Gen Enable"]
pub type InitclkgenenR = crate::BitReader;
#[doc = "Field `INITCLKGENEN` writer - Initial Clock Gen Enable"]
pub type InitclkgenenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INITDRVST` reader - Initial Drive Strength"]
pub type InitdrvstR = crate::FieldReader;
#[doc = "Field `INITDRVST` writer - Initial Drive Strength"]
pub type InitdrvstW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DSPSDCLKFREQ` reader - Preset Value for Default Speed of SD_CLK"]
pub type DspsdclkfreqR = crate::FieldReader<u16>;
#[doc = "Field `DSPSDCLKFREQ` writer - Preset Value for Default Speed of SD_CLK"]
pub type DspsdclkfreqW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DSPCLKGENEN` reader - Default Speed Clock Gen Enable"]
pub type DspclkgenenR = crate::BitReader;
#[doc = "Field `DSPCLKGENEN` writer - Default Speed Clock Gen Enable"]
pub type DspclkgenenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSPDRVST` reader - Default Speed Drive Strength"]
pub type DspdrvstR = crate::FieldReader;
#[doc = "Field `DSPDRVST` writer - Default Speed Drive Strength"]
pub type DspdrvstW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:9 - Initial SD_CLK Frequency"]
    #[inline(always)]
    pub fn initsdclkfreq(&self) -> InitsdclkfreqR {
        InitsdclkfreqR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Initial Clock Gen Enable"]
    #[inline(always)]
    pub fn initclkgenen(&self) -> InitclkgenenR {
        InitclkgenenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Initial Drive Strength"]
    #[inline(always)]
    pub fn initdrvst(&self) -> InitdrvstR {
        InitdrvstR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 16:25 - Preset Value for Default Speed of SD_CLK"]
    #[inline(always)]
    pub fn dspsdclkfreq(&self) -> DspsdclkfreqR {
        DspsdclkfreqR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - Default Speed Clock Gen Enable"]
    #[inline(always)]
    pub fn dspclkgenen(&self) -> DspclkgenenR {
        DspclkgenenR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - Default Speed Drive Strength"]
    #[inline(always)]
    pub fn dspdrvst(&self) -> DspdrvstR {
        DspdrvstR::new(((self.bits >> 27) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Initial SD_CLK Frequency"]
    #[inline(always)]
    pub fn initsdclkfreq(&mut self) -> InitsdclkfreqW<'_, Cfgpresetval0Spec> {
        InitsdclkfreqW::new(self, 0)
    }
    #[doc = "Bit 10 - Initial Clock Gen Enable"]
    #[inline(always)]
    pub fn initclkgenen(&mut self) -> InitclkgenenW<'_, Cfgpresetval0Spec> {
        InitclkgenenW::new(self, 10)
    }
    #[doc = "Bits 11:12 - Initial Drive Strength"]
    #[inline(always)]
    pub fn initdrvst(&mut self) -> InitdrvstW<'_, Cfgpresetval0Spec> {
        InitdrvstW::new(self, 11)
    }
    #[doc = "Bits 16:25 - Preset Value for Default Speed of SD_CLK"]
    #[inline(always)]
    pub fn dspsdclkfreq(&mut self) -> DspsdclkfreqW<'_, Cfgpresetval0Spec> {
        DspsdclkfreqW::new(self, 16)
    }
    #[doc = "Bit 26 - Default Speed Clock Gen Enable"]
    #[inline(always)]
    pub fn dspclkgenen(&mut self) -> DspclkgenenW<'_, Cfgpresetval0Spec> {
        DspclkgenenW::new(self, 26)
    }
    #[doc = "Bits 27:28 - Default Speed Drive Strength"]
    #[inline(always)]
    pub fn dspdrvst(&mut self) -> DspdrvstW<'_, Cfgpresetval0Spec> {
        DspdrvstW::new(self, 27)
    }
}
#[doc = "Core Configuration Preset Value 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgpresetval0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgpresetval0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfgpresetval0Spec;
impl crate::RegisterSpec for Cfgpresetval0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgpresetval0::R`](R) reader structure"]
impl crate::Readable for Cfgpresetval0Spec {}
#[doc = "`write(|w| ..)` method takes [`cfgpresetval0::W`](W) writer structure"]
impl crate::Writable for Cfgpresetval0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGPRESETVAL0 to value 0"]
impl crate::Resettable for Cfgpresetval0Spec {}
