#[doc = "Register `LFACLKSEL` reader"]
pub type R = crate::R<LfaclkselSpec>;
#[doc = "Register `LFACLKSEL` writer"]
pub type W = crate::W<LfaclkselSpec>;
#[doc = "Clock Select for LFA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lfa {
    #[doc = "0: LFACLK is disabled"]
    Disabled = 0,
    #[doc = "1: LFRCO selected as LFACLK"]
    Lfrco = 1,
    #[doc = "2: LFXO selected as LFACLK"]
    Lfxo = 2,
    #[doc = "4: ULFRCO selected as LFACLK"]
    Ulfrco = 4,
}
impl From<Lfa> for u8 {
    #[inline(always)]
    fn from(variant: Lfa) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lfa {
    type Ux = u8;
}
impl crate::IsEnum for Lfa {}
#[doc = "Field `LFA` reader - Clock Select for LFA"]
pub type LfaR = crate::FieldReader<Lfa>;
impl LfaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lfa> {
        match self.bits {
            0 => Some(Lfa::Disabled),
            1 => Some(Lfa::Lfrco),
            2 => Some(Lfa::Lfxo),
            4 => Some(Lfa::Ulfrco),
            _ => None,
        }
    }
    #[doc = "LFACLK is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lfa::Disabled
    }
    #[doc = "LFRCO selected as LFACLK"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == Lfa::Lfrco
    }
    #[doc = "LFXO selected as LFACLK"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == Lfa::Lfxo
    }
    #[doc = "ULFRCO selected as LFACLK"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == Lfa::Ulfrco
    }
}
#[doc = "Field `LFA` writer - Clock Select for LFA"]
pub type LfaW<'a, REG> = crate::FieldWriter<'a, REG, 3, Lfa>;
impl<'a, REG> LfaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LFACLK is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lfa::Disabled)
    }
    #[doc = "LFRCO selected as LFACLK"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Lfa::Lfrco)
    }
    #[doc = "LFXO selected as LFACLK"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Lfa::Lfxo)
    }
    #[doc = "ULFRCO selected as LFACLK"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Lfa::Ulfrco)
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Select for LFA"]
    #[inline(always)]
    pub fn lfa(&self) -> LfaR {
        LfaR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Select for LFA"]
    #[inline(always)]
    pub fn lfa(&mut self) -> LfaW<'_, LfaclkselSpec> {
        LfaW::new(self, 0)
    }
}
#[doc = "Low Frequency A Clock Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfaclksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfaclksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfaclkselSpec;
impl crate::RegisterSpec for LfaclkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfaclksel::R`](R) reader structure"]
impl crate::Readable for LfaclkselSpec {}
#[doc = "`write(|w| ..)` method takes [`lfaclksel::W`](W) writer structure"]
impl crate::Writable for LfaclkselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFACLKSEL to value 0"]
impl crate::Resettable for LfaclkselSpec {}
