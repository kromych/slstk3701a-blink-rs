#[doc = "Register `DBGCLKSEL` reader"]
pub type R = crate::R<DbgclkselSpec>;
#[doc = "Register `DBGCLKSEL` writer"]
pub type W = crate::W<DbgclkselSpec>;
#[doc = "Debug Trace Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dbg {
    #[doc = "0: AUXHFRCO is the debug trace clock"]
    Auxhfrco = 0,
    #[doc = "1: HFCLK is the debug trace clock"]
    Hfclk = 1,
    #[doc = "2: HFRCO divided by 2 is the debug trace clock"]
    Hfrcodiv2 = 2,
}
impl From<Dbg> for u8 {
    #[inline(always)]
    fn from(variant: Dbg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dbg {
    type Ux = u8;
}
impl crate::IsEnum for Dbg {}
#[doc = "Field `DBG` reader - Debug Trace Clock"]
pub type DbgR = crate::FieldReader<Dbg>;
impl DbgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dbg> {
        match self.bits {
            0 => Some(Dbg::Auxhfrco),
            1 => Some(Dbg::Hfclk),
            2 => Some(Dbg::Hfrcodiv2),
            _ => None,
        }
    }
    #[doc = "AUXHFRCO is the debug trace clock"]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == Dbg::Auxhfrco
    }
    #[doc = "HFCLK is the debug trace clock"]
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        *self == Dbg::Hfclk
    }
    #[doc = "HFRCO divided by 2 is the debug trace clock"]
    #[inline(always)]
    pub fn is_hfrcodiv2(&self) -> bool {
        *self == Dbg::Hfrcodiv2
    }
}
#[doc = "Field `DBG` writer - Debug Trace Clock"]
pub type DbgW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dbg>;
impl<'a, REG> DbgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AUXHFRCO is the debug trace clock"]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Dbg::Auxhfrco)
    }
    #[doc = "HFCLK is the debug trace clock"]
    #[inline(always)]
    pub fn hfclk(self) -> &'a mut crate::W<REG> {
        self.variant(Dbg::Hfclk)
    }
    #[doc = "HFRCO divided by 2 is the debug trace clock"]
    #[inline(always)]
    pub fn hfrcodiv2(self) -> &'a mut crate::W<REG> {
        self.variant(Dbg::Hfrcodiv2)
    }
}
impl R {
    #[doc = "Bits 0:1 - Debug Trace Clock"]
    #[inline(always)]
    pub fn dbg(&self) -> DbgR {
        DbgR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Debug Trace Clock"]
    #[inline(always)]
    pub fn dbg(&mut self) -> DbgW<'_, DbgclkselSpec> {
        DbgW::new(self, 0)
    }
}
#[doc = "Debug Trace Clock Select\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgclksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgclksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgclkselSpec;
impl crate::RegisterSpec for DbgclkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgclksel::R`](R) reader structure"]
impl crate::Readable for DbgclkselSpec {}
#[doc = "`write(|w| ..)` method takes [`dbgclksel::W`](W) writer structure"]
impl crate::Writable for DbgclkselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBGCLKSEL to value 0"]
impl crate::Resettable for DbgclkselSpec {}
