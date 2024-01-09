#[doc = "Register `CFGPRESETVAL0` reader"]
pub type R = crate::R<CFGPRESETVAL0_SPEC>;
#[doc = "Register `CFGPRESETVAL0` writer"]
pub type W = crate::W<CFGPRESETVAL0_SPEC>;
#[doc = "Field `INITSDCLKFREQ` reader - Initial SD_CLK Frequency"]
pub type INITSDCLKFREQ_R = crate::FieldReader<u16>;
#[doc = "Field `INITSDCLKFREQ` writer - Initial SD_CLK Frequency"]
pub type INITSDCLKFREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `INITCLKGENEN` reader - Initial Clock Gen Enable"]
pub type INITCLKGENEN_R = crate::BitReader;
#[doc = "Field `INITCLKGENEN` writer - Initial Clock Gen Enable"]
pub type INITCLKGENEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INITDRVST` reader - Initial Drive Strength"]
pub type INITDRVST_R = crate::FieldReader;
#[doc = "Field `INITDRVST` writer - Initial Drive Strength"]
pub type INITDRVST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DSPSDCLKFREQ` reader - Preset Value for Default Speed of SD_CLK"]
pub type DSPSDCLKFREQ_R = crate::FieldReader<u16>;
#[doc = "Field `DSPSDCLKFREQ` writer - Preset Value for Default Speed of SD_CLK"]
pub type DSPSDCLKFREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DSPCLKGENEN` reader - Default Speed Clock Gen Enable"]
pub type DSPCLKGENEN_R = crate::BitReader;
#[doc = "Field `DSPCLKGENEN` writer - Default Speed Clock Gen Enable"]
pub type DSPCLKGENEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSPDRVST` reader - Default Speed Drive Strength"]
pub type DSPDRVST_R = crate::FieldReader;
#[doc = "Field `DSPDRVST` writer - Default Speed Drive Strength"]
pub type DSPDRVST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:9 - Initial SD_CLK Frequency"]
    #[inline(always)]
    pub fn initsdclkfreq(&self) -> INITSDCLKFREQ_R {
        INITSDCLKFREQ_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Initial Clock Gen Enable"]
    #[inline(always)]
    pub fn initclkgenen(&self) -> INITCLKGENEN_R {
        INITCLKGENEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Initial Drive Strength"]
    #[inline(always)]
    pub fn initdrvst(&self) -> INITDRVST_R {
        INITDRVST_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 16:25 - Preset Value for Default Speed of SD_CLK"]
    #[inline(always)]
    pub fn dspsdclkfreq(&self) -> DSPSDCLKFREQ_R {
        DSPSDCLKFREQ_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - Default Speed Clock Gen Enable"]
    #[inline(always)]
    pub fn dspclkgenen(&self) -> DSPCLKGENEN_R {
        DSPCLKGENEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - Default Speed Drive Strength"]
    #[inline(always)]
    pub fn dspdrvst(&self) -> DSPDRVST_R {
        DSPDRVST_R::new(((self.bits >> 27) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Initial SD_CLK Frequency"]
    #[inline(always)]
    #[must_use]
    pub fn initsdclkfreq(&mut self) -> INITSDCLKFREQ_W<CFGPRESETVAL0_SPEC> {
        INITSDCLKFREQ_W::new(self, 0)
    }
    #[doc = "Bit 10 - Initial Clock Gen Enable"]
    #[inline(always)]
    #[must_use]
    pub fn initclkgenen(&mut self) -> INITCLKGENEN_W<CFGPRESETVAL0_SPEC> {
        INITCLKGENEN_W::new(self, 10)
    }
    #[doc = "Bits 11:12 - Initial Drive Strength"]
    #[inline(always)]
    #[must_use]
    pub fn initdrvst(&mut self) -> INITDRVST_W<CFGPRESETVAL0_SPEC> {
        INITDRVST_W::new(self, 11)
    }
    #[doc = "Bits 16:25 - Preset Value for Default Speed of SD_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn dspsdclkfreq(&mut self) -> DSPSDCLKFREQ_W<CFGPRESETVAL0_SPEC> {
        DSPSDCLKFREQ_W::new(self, 16)
    }
    #[doc = "Bit 26 - Default Speed Clock Gen Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dspclkgenen(&mut self) -> DSPCLKGENEN_W<CFGPRESETVAL0_SPEC> {
        DSPCLKGENEN_W::new(self, 26)
    }
    #[doc = "Bits 27:28 - Default Speed Drive Strength"]
    #[inline(always)]
    #[must_use]
    pub fn dspdrvst(&mut self) -> DSPDRVST_W<CFGPRESETVAL0_SPEC> {
        DSPDRVST_W::new(self, 27)
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
#[doc = "Core Configuration Preset Value 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgpresetval0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgpresetval0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGPRESETVAL0_SPEC;
impl crate::RegisterSpec for CFGPRESETVAL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgpresetval0::R`](R) reader structure"]
impl crate::Readable for CFGPRESETVAL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfgpresetval0::W`](W) writer structure"]
impl crate::Writable for CFGPRESETVAL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGPRESETVAL0 to value 0"]
impl crate::Resettable for CFGPRESETVAL0_SPEC {
    const RESET_VALUE: u32 = 0;
}
