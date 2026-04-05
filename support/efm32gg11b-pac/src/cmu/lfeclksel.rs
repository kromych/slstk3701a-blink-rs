#[doc = "Register `LFECLKSEL` reader"]
pub type R = crate::R<LfeclkselSpec>;
#[doc = "Register `LFECLKSEL` writer"]
pub type W = crate::W<LfeclkselSpec>;
#[doc = "Clock Select for LFE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lfe {
    #[doc = "0: LFECLK is disabled"]
    Disabled = 0,
    #[doc = "1: LFRCO selected as LFECLK"]
    Lfrco = 1,
    #[doc = "2: LFXO selected as LFECLK"]
    Lfxo = 2,
    #[doc = "4: ULFRCO selected as LFECLK"]
    Ulfrco = 4,
}
impl From<Lfe> for u8 {
    #[inline(always)]
    fn from(variant: Lfe) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lfe {
    type Ux = u8;
}
impl crate::IsEnum for Lfe {}
#[doc = "Field `LFE` reader - Clock Select for LFE"]
pub type LfeR = crate::FieldReader<Lfe>;
impl LfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lfe> {
        match self.bits {
            0 => Some(Lfe::Disabled),
            1 => Some(Lfe::Lfrco),
            2 => Some(Lfe::Lfxo),
            4 => Some(Lfe::Ulfrco),
            _ => None,
        }
    }
    #[doc = "LFECLK is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lfe::Disabled
    }
    #[doc = "LFRCO selected as LFECLK"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == Lfe::Lfrco
    }
    #[doc = "LFXO selected as LFECLK"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == Lfe::Lfxo
    }
    #[doc = "ULFRCO selected as LFECLK"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == Lfe::Ulfrco
    }
}
#[doc = "Field `LFE` writer - Clock Select for LFE"]
pub type LfeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Lfe>;
impl<'a, REG> LfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LFECLK is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lfe::Disabled)
    }
    #[doc = "LFRCO selected as LFECLK"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Lfe::Lfrco)
    }
    #[doc = "LFXO selected as LFECLK"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Lfe::Lfxo)
    }
    #[doc = "ULFRCO selected as LFECLK"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Lfe::Ulfrco)
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Select for LFE"]
    #[inline(always)]
    pub fn lfe(&self) -> LfeR {
        LfeR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Select for LFE"]
    #[inline(always)]
    pub fn lfe(&mut self) -> LfeW<'_, LfeclkselSpec> {
        LfeW::new(self, 0)
    }
}
#[doc = "Low Frequency E Clock Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfeclksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfeclksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfeclkselSpec;
impl crate::RegisterSpec for LfeclkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfeclksel::R`](R) reader structure"]
impl crate::Readable for LfeclkselSpec {}
#[doc = "`write(|w| ..)` method takes [`lfeclksel::W`](W) writer structure"]
impl crate::Writable for LfeclkselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFECLKSEL to value 0"]
impl crate::Resettable for LfeclkselSpec {}
