#[doc = "Register `CFGPRESETVAL3` reader"]
pub type R = crate::R<CFGPRESETVAL3_SPEC>;
#[doc = "Register `CFGPRESETVAL3` writer"]
pub type W = crate::W<CFGPRESETVAL3_SPEC>;
#[doc = "Field `SDR104SDCLKFREQ` reader - SDR104 SD_CLK Frequency"]
pub type SDR104SDCLKFREQ_R = crate::FieldReader<u16>;
#[doc = "Field `SDR104SDCLKFREQ` writer - SDR104 SD_CLK Frequency"]
pub type SDR104SDCLKFREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SDR104CLKGENEN` reader - SDR104 SD_CLK Gen Enable"]
pub type SDR104CLKGENEN_R = crate::BitReader;
#[doc = "Field `SDR104CLKGENEN` writer - SDR104 SD_CLK Gen Enable"]
pub type SDR104CLKGENEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDR104DRVST` reader - SDR104 SD Drive Strength"]
pub type SDR104DRVST_R = crate::FieldReader;
#[doc = "Field `SDR104DRVST` writer - SDR104 SD Drive Strength"]
pub type SDR104DRVST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DDR50SDCLKFREQ` reader - Preset Value for DDR50 Speed of SD_CLK"]
pub type DDR50SDCLKFREQ_R = crate::FieldReader<u16>;
#[doc = "Field `DDR50SDCLKFREQ` writer - Preset Value for DDR50 Speed of SD_CLK"]
pub type DDR50SDCLKFREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DDR50CLKGENEN` reader - DDR50 Speed Clock Gen Enable"]
pub type DDR50CLKGENEN_R = crate::BitReader;
#[doc = "Field `DDR50CLKGENEN` writer - DDR50 Speed Clock Gen Enable"]
pub type DDR50CLKGENEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR50DRVST` reader - DDR50 Speed Drive Strength"]
pub type DDR50DRVST_R = crate::FieldReader;
#[doc = "Field `DDR50DRVST` writer - DDR50 Speed Drive Strength"]
pub type DDR50DRVST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:9 - SDR104 SD_CLK Frequency"]
    #[inline(always)]
    pub fn sdr104sdclkfreq(&self) -> SDR104SDCLKFREQ_R {
        SDR104SDCLKFREQ_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - SDR104 SD_CLK Gen Enable"]
    #[inline(always)]
    pub fn sdr104clkgenen(&self) -> SDR104CLKGENEN_R {
        SDR104CLKGENEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - SDR104 SD Drive Strength"]
    #[inline(always)]
    pub fn sdr104drvst(&self) -> SDR104DRVST_R {
        SDR104DRVST_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 16:25 - Preset Value for DDR50 Speed of SD_CLK"]
    #[inline(always)]
    pub fn ddr50sdclkfreq(&self) -> DDR50SDCLKFREQ_R {
        DDR50SDCLKFREQ_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - DDR50 Speed Clock Gen Enable"]
    #[inline(always)]
    pub fn ddr50clkgenen(&self) -> DDR50CLKGENEN_R {
        DDR50CLKGENEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - DDR50 Speed Drive Strength"]
    #[inline(always)]
    pub fn ddr50drvst(&self) -> DDR50DRVST_R {
        DDR50DRVST_R::new(((self.bits >> 27) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - SDR104 SD_CLK Frequency"]
    #[inline(always)]
    #[must_use]
    pub fn sdr104sdclkfreq(&mut self) -> SDR104SDCLKFREQ_W<CFGPRESETVAL3_SPEC> {
        SDR104SDCLKFREQ_W::new(self, 0)
    }
    #[doc = "Bit 10 - SDR104 SD_CLK Gen Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdr104clkgenen(&mut self) -> SDR104CLKGENEN_W<CFGPRESETVAL3_SPEC> {
        SDR104CLKGENEN_W::new(self, 10)
    }
    #[doc = "Bits 11:12 - SDR104 SD Drive Strength"]
    #[inline(always)]
    #[must_use]
    pub fn sdr104drvst(&mut self) -> SDR104DRVST_W<CFGPRESETVAL3_SPEC> {
        SDR104DRVST_W::new(self, 11)
    }
    #[doc = "Bits 16:25 - Preset Value for DDR50 Speed of SD_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn ddr50sdclkfreq(&mut self) -> DDR50SDCLKFREQ_W<CFGPRESETVAL3_SPEC> {
        DDR50SDCLKFREQ_W::new(self, 16)
    }
    #[doc = "Bit 26 - DDR50 Speed Clock Gen Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ddr50clkgenen(&mut self) -> DDR50CLKGENEN_W<CFGPRESETVAL3_SPEC> {
        DDR50CLKGENEN_W::new(self, 26)
    }
    #[doc = "Bits 27:28 - DDR50 Speed Drive Strength"]
    #[inline(always)]
    #[must_use]
    pub fn ddr50drvst(&mut self) -> DDR50DRVST_W<CFGPRESETVAL3_SPEC> {
        DDR50DRVST_W::new(self, 27)
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
#[doc = "Core Configuration Preset Value 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgpresetval3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgpresetval3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGPRESETVAL3_SPEC;
impl crate::RegisterSpec for CFGPRESETVAL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgpresetval3::R`](R) reader structure"]
impl crate::Readable for CFGPRESETVAL3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfgpresetval3::W`](W) writer structure"]
impl crate::Writable for CFGPRESETVAL3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGPRESETVAL3 to value 0"]
impl crate::Resettable for CFGPRESETVAL3_SPEC {
    const RESET_VALUE: u32 = 0;
}
