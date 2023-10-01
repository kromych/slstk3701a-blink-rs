#[doc = "Register `HFCLKSEL` writer"]
pub type W = crate::W<HFCLKSEL_SPEC>;
#[doc = "HFCLK Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HF_AW {
    #[doc = "1: Select HFRCO as HFCLK"]
    HFRCO = 1,
    #[doc = "2: Select HFXO as HFCLK"]
    HFXO = 2,
    #[doc = "3: Select LFRCO as HFCLK"]
    LFRCO = 3,
    #[doc = "4: Select LFXO as HFCLK"]
    LFXO = 4,
    #[doc = "5: Select HFRCO divided by 2 as HFCLK"]
    HFRCODIV2 = 5,
    #[doc = "6: Select USHFRCO as HFCLK"]
    USHFRCO = 6,
    #[doc = "7: Select CLKIN0 as HFCLK"]
    CLKIN0 = 7,
}
impl From<HF_AW> for u8 {
    #[inline(always)]
    fn from(variant: HF_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HF_AW {
    type Ux = u8;
}
#[doc = "Field `HF` writer - HFCLK Select"]
pub type HF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, HF_AW>;
impl<'a, REG, const O: u8> HF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select HFRCO as HFCLK"]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut crate::W<REG> {
        self.variant(HF_AW::HFRCO)
    }
    #[doc = "Select HFXO as HFCLK"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(HF_AW::HFXO)
    }
    #[doc = "Select LFRCO as HFCLK"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(HF_AW::LFRCO)
    }
    #[doc = "Select LFXO as HFCLK"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(HF_AW::LFXO)
    }
    #[doc = "Select HFRCO divided by 2 as HFCLK"]
    #[inline(always)]
    pub fn hfrcodiv2(self) -> &'a mut crate::W<REG> {
        self.variant(HF_AW::HFRCODIV2)
    }
    #[doc = "Select USHFRCO as HFCLK"]
    #[inline(always)]
    pub fn ushfrco(self) -> &'a mut crate::W<REG> {
        self.variant(HF_AW::USHFRCO)
    }
    #[doc = "Select CLKIN0 as HFCLK"]
    #[inline(always)]
    pub fn clkin0(self) -> &'a mut crate::W<REG> {
        self.variant(HF_AW::CLKIN0)
    }
}
impl W {
    #[doc = "Bits 0:2 - HFCLK Select"]
    #[inline(always)]
    #[must_use]
    pub fn hf(&mut self) -> HF_W<HFCLKSEL_SPEC, 0> {
        HF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "High Frequency Clock Select Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfclksel::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFCLKSEL_SPEC;
impl crate::RegisterSpec for HFCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hfclksel::W`](W) writer structure"]
impl crate::Writable for HFCLKSEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HFCLKSEL to value 0"]
impl crate::Resettable for HFCLKSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
