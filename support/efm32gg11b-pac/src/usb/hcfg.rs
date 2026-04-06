#[doc = "Register `HCFG` reader"]
pub type R = crate::R<HcfgSpec>;
#[doc = "Register `HCFG` writer"]
pub type W = crate::W<HcfgSpec>;
#[doc = "FS/LS PHY Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fslspclksel {
    #[doc = "1: Internal PHY clock is running at 48 MHz (undivided)."]
    Div1 = 1,
    #[doc = "2: Internal PHY clock is running at 6 MHz (48 MHz divided by 8)."]
    Div8 = 2,
}
impl From<Fslspclksel> for u8 {
    #[inline(always)]
    fn from(variant: Fslspclksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fslspclksel {
    type Ux = u8;
}
impl crate::IsEnum for Fslspclksel {}
#[doc = "Field `FSLSPCLKSEL` reader - FS/LS PHY Clock Select"]
pub type FslspclkselR = crate::FieldReader<Fslspclksel>;
impl FslspclkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fslspclksel> {
        match self.bits {
            1 => Some(Fslspclksel::Div1),
            2 => Some(Fslspclksel::Div8),
            _ => None,
        }
    }
    #[doc = "Internal PHY clock is running at 48 MHz (undivided)."]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Fslspclksel::Div1
    }
    #[doc = "Internal PHY clock is running at 6 MHz (48 MHz divided by 8)."]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Fslspclksel::Div8
    }
}
#[doc = "Field `FSLSPCLKSEL` writer - FS/LS PHY Clock Select"]
pub type FslspclkselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Fslspclksel>;
impl<'a, REG> FslspclkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal PHY clock is running at 48 MHz (undivided)."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Fslspclksel::Div1)
    }
    #[doc = "Internal PHY clock is running at 6 MHz (48 MHz divided by 8)."]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Fslspclksel::Div8)
    }
}
#[doc = "Field `FSLSSUPP` reader - FS- and LS-Only Support"]
pub type FslssuppR = crate::BitReader;
#[doc = "Field `FSLSSUPP` writer - FS- and LS-Only Support"]
pub type FslssuppW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENA32KHZS` reader - Enable 32 kHz Suspend Mode"]
pub type Ena32khzsR = crate::BitReader;
#[doc = "Field `ENA32KHZS` writer - Enable 32 kHz Suspend Mode"]
pub type Ena32khzsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESVALID` reader - Resume Validation Period"]
pub type ResvalidR = crate::FieldReader;
#[doc = "Field `RESVALID` writer - Resume Validation Period"]
pub type ResvalidW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MODECHTIMEN` reader - Mode Change Time"]
pub type ModechtimenR = crate::BitReader;
#[doc = "Field `MODECHTIMEN` writer - Mode Change Time"]
pub type ModechtimenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - FS/LS PHY Clock Select"]
    #[inline(always)]
    pub fn fslspclksel(&self) -> FslspclkselR {
        FslspclkselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - FS- and LS-Only Support"]
    #[inline(always)]
    pub fn fslssupp(&self) -> FslssuppR {
        FslssuppR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable 32 kHz Suspend Mode"]
    #[inline(always)]
    pub fn ena32khzs(&self) -> Ena32khzsR {
        Ena32khzsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Resume Validation Period"]
    #[inline(always)]
    pub fn resvalid(&self) -> ResvalidR {
        ResvalidR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - Mode Change Time"]
    #[inline(always)]
    pub fn modechtimen(&self) -> ModechtimenR {
        ModechtimenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - FS/LS PHY Clock Select"]
    #[inline(always)]
    pub fn fslspclksel(&mut self) -> FslspclkselW<'_, HcfgSpec> {
        FslspclkselW::new(self, 0)
    }
    #[doc = "Bit 2 - FS- and LS-Only Support"]
    #[inline(always)]
    pub fn fslssupp(&mut self) -> FslssuppW<'_, HcfgSpec> {
        FslssuppW::new(self, 2)
    }
    #[doc = "Bit 7 - Enable 32 kHz Suspend Mode"]
    #[inline(always)]
    pub fn ena32khzs(&mut self) -> Ena32khzsW<'_, HcfgSpec> {
        Ena32khzsW::new(self, 7)
    }
    #[doc = "Bits 8:15 - Resume Validation Period"]
    #[inline(always)]
    pub fn resvalid(&mut self) -> ResvalidW<'_, HcfgSpec> {
        ResvalidW::new(self, 8)
    }
    #[doc = "Bit 31 - Mode Change Time"]
    #[inline(always)]
    pub fn modechtimen(&mut self) -> ModechtimenW<'_, HcfgSpec> {
        ModechtimenW::new(self, 31)
    }
}
#[doc = "Host Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcfgSpec;
impl crate::RegisterSpec for HcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcfg::R`](R) reader structure"]
impl crate::Readable for HcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`hcfg::W`](W) writer structure"]
impl crate::Writable for HcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCFG to value 0x0200"]
impl crate::Resettable for HcfgSpec {
    const RESET_VALUE: u32 = 0x0200;
}
