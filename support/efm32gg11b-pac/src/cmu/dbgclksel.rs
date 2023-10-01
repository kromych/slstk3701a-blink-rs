#[doc = "Register `DBGCLKSEL` reader"]
pub type R = crate::R<DBGCLKSEL_SPEC>;
#[doc = "Register `DBGCLKSEL` writer"]
pub type W = crate::W<DBGCLKSEL_SPEC>;
#[doc = "Field `DBG` reader - Debug Trace Clock"]
pub type DBG_R = crate::FieldReader<DBG_A>;
#[doc = "Debug Trace Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DBG_A {
    #[doc = "0: AUXHFRCO is the debug trace clock"]
    AUXHFRCO = 0,
    #[doc = "1: HFCLK is the debug trace clock"]
    HFCLK = 1,
    #[doc = "2: HFRCO divided by 2 is the debug trace clock"]
    HFRCODIV2 = 2,
}
impl From<DBG_A> for u8 {
    #[inline(always)]
    fn from(variant: DBG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DBG_A {
    type Ux = u8;
}
impl DBG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DBG_A> {
        match self.bits {
            0 => Some(DBG_A::AUXHFRCO),
            1 => Some(DBG_A::HFCLK),
            2 => Some(DBG_A::HFRCODIV2),
            _ => None,
        }
    }
    #[doc = "AUXHFRCO is the debug trace clock"]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == DBG_A::AUXHFRCO
    }
    #[doc = "HFCLK is the debug trace clock"]
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        *self == DBG_A::HFCLK
    }
    #[doc = "HFRCO divided by 2 is the debug trace clock"]
    #[inline(always)]
    pub fn is_hfrcodiv2(&self) -> bool {
        *self == DBG_A::HFRCODIV2
    }
}
#[doc = "Field `DBG` writer - Debug Trace Clock"]
pub type DBG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, DBG_A>;
impl<'a, REG, const O: u8> DBG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AUXHFRCO is the debug trace clock"]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_A::AUXHFRCO)
    }
    #[doc = "HFCLK is the debug trace clock"]
    #[inline(always)]
    pub fn hfclk(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_A::HFCLK)
    }
    #[doc = "HFRCO divided by 2 is the debug trace clock"]
    #[inline(always)]
    pub fn hfrcodiv2(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_A::HFRCODIV2)
    }
}
impl R {
    #[doc = "Bits 0:1 - Debug Trace Clock"]
    #[inline(always)]
    pub fn dbg(&self) -> DBG_R {
        DBG_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Debug Trace Clock"]
    #[inline(always)]
    #[must_use]
    pub fn dbg(&mut self) -> DBG_W<DBGCLKSEL_SPEC, 0> {
        DBG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Debug Trace Clock Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgclksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbgclksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBGCLKSEL_SPEC;
impl crate::RegisterSpec for DBGCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgclksel::R`](R) reader structure"]
impl crate::Readable for DBGCLKSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbgclksel::W`](W) writer structure"]
impl crate::Writable for DBGCLKSEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DBGCLKSEL to value 0"]
impl crate::Resettable for DBGCLKSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
