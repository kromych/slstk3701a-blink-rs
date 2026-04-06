#[doc = "Register `LFBCLKSEL` reader"]
pub type R = crate::R<LfbclkselSpec>;
#[doc = "Register `LFBCLKSEL` writer"]
pub type W = crate::W<LfbclkselSpec>;
#[doc = "Clock Select for LFB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lfb {
    #[doc = "0: LFBCLK is disabled"]
    Disabled = 0,
    #[doc = "1: LFRCO selected as LFBCLK"]
    Lfrco = 1,
    #[doc = "2: LFXO selected as LFBCLK"]
    Lfxo = 2,
    #[doc = "3: HFCLK divided by two/four is selected as LFBCLK"]
    Hfclkle = 3,
    #[doc = "4: ULFRCO selected as LFBCLK"]
    Ulfrco = 4,
}
impl From<Lfb> for u8 {
    #[inline(always)]
    fn from(variant: Lfb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lfb {
    type Ux = u8;
}
impl crate::IsEnum for Lfb {}
#[doc = "Field `LFB` reader - Clock Select for LFB"]
pub type LfbR = crate::FieldReader<Lfb>;
impl LfbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lfb> {
        match self.bits {
            0 => Some(Lfb::Disabled),
            1 => Some(Lfb::Lfrco),
            2 => Some(Lfb::Lfxo),
            3 => Some(Lfb::Hfclkle),
            4 => Some(Lfb::Ulfrco),
            _ => None,
        }
    }
    #[doc = "LFBCLK is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lfb::Disabled
    }
    #[doc = "LFRCO selected as LFBCLK"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == Lfb::Lfrco
    }
    #[doc = "LFXO selected as LFBCLK"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == Lfb::Lfxo
    }
    #[doc = "HFCLK divided by two/four is selected as LFBCLK"]
    #[inline(always)]
    pub fn is_hfclkle(&self) -> bool {
        *self == Lfb::Hfclkle
    }
    #[doc = "ULFRCO selected as LFBCLK"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == Lfb::Ulfrco
    }
}
#[doc = "Field `LFB` writer - Clock Select for LFB"]
pub type LfbW<'a, REG> = crate::FieldWriter<'a, REG, 3, Lfb>;
impl<'a, REG> LfbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LFBCLK is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lfb::Disabled)
    }
    #[doc = "LFRCO selected as LFBCLK"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Lfb::Lfrco)
    }
    #[doc = "LFXO selected as LFBCLK"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Lfb::Lfxo)
    }
    #[doc = "HFCLK divided by two/four is selected as LFBCLK"]
    #[inline(always)]
    pub fn hfclkle(self) -> &'a mut crate::W<REG> {
        self.variant(Lfb::Hfclkle)
    }
    #[doc = "ULFRCO selected as LFBCLK"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Lfb::Ulfrco)
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Select for LFB"]
    #[inline(always)]
    pub fn lfb(&self) -> LfbR {
        LfbR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Select for LFB"]
    #[inline(always)]
    pub fn lfb(&mut self) -> LfbW<'_, LfbclkselSpec> {
        LfbW::new(self, 0)
    }
}
#[doc = "Low Frequency B Clock Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfbclksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfbclksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfbclkselSpec;
impl crate::RegisterSpec for LfbclkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfbclksel::R`](R) reader structure"]
impl crate::Readable for LfbclkselSpec {}
#[doc = "`write(|w| ..)` method takes [`lfbclksel::W`](W) writer structure"]
impl crate::Writable for LfbclkselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFBCLKSEL to value 0"]
impl crate::Resettable for LfbclkselSpec {}
