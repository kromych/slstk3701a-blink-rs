#[doc = "Register `HFCLKSTATUS` reader"]
pub type R = crate::R<HfclkstatusSpec>;
#[doc = "HFCLK Selected\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Selected {
    #[doc = "1: HFRCO is selected as HFCLK clock source"]
    Hfrco = 1,
    #[doc = "2: HFXO is selected as HFCLK clock source"]
    Hfxo = 2,
    #[doc = "3: LFRCO is selected as HFCLK clock source"]
    Lfrco = 3,
    #[doc = "4: LFXO is selected as HFCLK clock source"]
    Lfxo = 4,
    #[doc = "5: HFRCO divided by 2 is selected as HFCLK clock source"]
    Hfrcodiv2 = 5,
    #[doc = "6: USHFRCO is selected as HFCLK clock source"]
    Ushfrco = 6,
    #[doc = "7: CLKIN0 is selected as HFCLK clock source"]
    Clkin0 = 7,
}
impl From<Selected> for u8 {
    #[inline(always)]
    fn from(variant: Selected) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Selected {
    type Ux = u8;
}
impl crate::IsEnum for Selected {}
#[doc = "Field `SELECTED` reader - HFCLK Selected"]
pub type SelectedR = crate::FieldReader<Selected>;
impl SelectedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Selected> {
        match self.bits {
            1 => Some(Selected::Hfrco),
            2 => Some(Selected::Hfxo),
            3 => Some(Selected::Lfrco),
            4 => Some(Selected::Lfxo),
            5 => Some(Selected::Hfrcodiv2),
            6 => Some(Selected::Ushfrco),
            7 => Some(Selected::Clkin0),
            _ => None,
        }
    }
    #[doc = "HFRCO is selected as HFCLK clock source"]
    #[inline(always)]
    pub fn is_hfrco(&self) -> bool {
        *self == Selected::Hfrco
    }
    #[doc = "HFXO is selected as HFCLK clock source"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == Selected::Hfxo
    }
    #[doc = "LFRCO is selected as HFCLK clock source"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == Selected::Lfrco
    }
    #[doc = "LFXO is selected as HFCLK clock source"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == Selected::Lfxo
    }
    #[doc = "HFRCO divided by 2 is selected as HFCLK clock source"]
    #[inline(always)]
    pub fn is_hfrcodiv2(&self) -> bool {
        *self == Selected::Hfrcodiv2
    }
    #[doc = "USHFRCO is selected as HFCLK clock source"]
    #[inline(always)]
    pub fn is_ushfrco(&self) -> bool {
        *self == Selected::Ushfrco
    }
    #[doc = "CLKIN0 is selected as HFCLK clock source"]
    #[inline(always)]
    pub fn is_clkin0(&self) -> bool {
        *self == Selected::Clkin0
    }
}
impl R {
    #[doc = "Bits 0:2 - HFCLK Selected"]
    #[inline(always)]
    pub fn selected(&self) -> SelectedR {
        SelectedR::new((self.bits & 7) as u8)
    }
}
#[doc = "HFCLK Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hfclkstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfclkstatusSpec;
impl crate::RegisterSpec for HfclkstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfclkstatus::R`](R) reader structure"]
impl crate::Readable for HfclkstatusSpec {}
#[doc = "`reset()` method sets HFCLKSTATUS to value 0x01"]
impl crate::Resettable for HfclkstatusSpec {
    const RESET_VALUE: u32 = 0x01;
}
