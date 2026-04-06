#[doc = "Register `LFCCLKSEL` reader"]
pub type R = crate::R<LfcclkselSpec>;
#[doc = "Register `LFCCLKSEL` writer"]
pub type W = crate::W<LfcclkselSpec>;
#[doc = "Clock Select for LFC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lfc {
    #[doc = "0: LFCCLK is disabled"]
    Disabled = 0,
    #[doc = "1: LFRCO selected as LFCCLK"]
    Lfrco = 1,
    #[doc = "2: LFXO selected as LFCCLK"]
    Lfxo = 2,
    #[doc = "4: ULFRCO selected as LFCCLK"]
    Ulfrco = 4,
}
impl From<Lfc> for u8 {
    #[inline(always)]
    fn from(variant: Lfc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lfc {
    type Ux = u8;
}
impl crate::IsEnum for Lfc {}
#[doc = "Field `LFC` reader - Clock Select for LFC"]
pub type LfcR = crate::FieldReader<Lfc>;
impl LfcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lfc> {
        match self.bits {
            0 => Some(Lfc::Disabled),
            1 => Some(Lfc::Lfrco),
            2 => Some(Lfc::Lfxo),
            4 => Some(Lfc::Ulfrco),
            _ => None,
        }
    }
    #[doc = "LFCCLK is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lfc::Disabled
    }
    #[doc = "LFRCO selected as LFCCLK"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == Lfc::Lfrco
    }
    #[doc = "LFXO selected as LFCCLK"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == Lfc::Lfxo
    }
    #[doc = "ULFRCO selected as LFCCLK"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == Lfc::Ulfrco
    }
}
#[doc = "Field `LFC` writer - Clock Select for LFC"]
pub type LfcW<'a, REG> = crate::FieldWriter<'a, REG, 3, Lfc>;
impl<'a, REG> LfcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LFCCLK is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lfc::Disabled)
    }
    #[doc = "LFRCO selected as LFCCLK"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Lfc::Lfrco)
    }
    #[doc = "LFXO selected as LFCCLK"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Lfc::Lfxo)
    }
    #[doc = "ULFRCO selected as LFCCLK"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Lfc::Ulfrco)
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Select for LFC"]
    #[inline(always)]
    pub fn lfc(&self) -> LfcR {
        LfcR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Select for LFC"]
    #[inline(always)]
    pub fn lfc(&mut self) -> LfcW<'_, LfcclkselSpec> {
        LfcW::new(self, 0)
    }
}
#[doc = "Low Frequency C Clock Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfcclksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfcclksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfcclkselSpec;
impl crate::RegisterSpec for LfcclkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfcclksel::R`](R) reader structure"]
impl crate::Readable for LfcclkselSpec {}
#[doc = "`write(|w| ..)` method takes [`lfcclksel::W`](W) writer structure"]
impl crate::Writable for LfcclkselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFCCLKSEL to value 0"]
impl crate::Resettable for LfcclkselSpec {}
