#[doc = "Register `CACHECONFIG0` reader"]
pub type R = crate::R<Cacheconfig0Spec>;
#[doc = "Register `CACHECONFIG0` writer"]
pub type W = crate::W<Cacheconfig0Spec>;
#[doc = "Instruction Cache Low-Power Level\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cachelplevel {
    #[doc = "0: Base instruction cache functionality."]
    Base = 0,
    #[doc = "1: Advanced buffering mode, where the cache uses the fetch pattern to predict highly accessed data and store it in low-energy memory."]
    Advanced = 1,
    #[doc = "3: Minimum activity mode, which allows the cache to minimize activity in logic that it predicts has a low probability being used. This mode can introduce wait-states into the instruction fetch stream when the cache exits one of its low-activity states. The number of wait-states introduced is small, but users running with 0-wait-state memory and wishing to reduce the variability that the cache might introduce with additional wait-states may wish to lower the cache low-power level. Note, this mode includes the advanced buffering mode functionality."]
    Minactivity = 3,
}
impl From<Cachelplevel> for u8 {
    #[inline(always)]
    fn from(variant: Cachelplevel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cachelplevel {
    type Ux = u8;
}
impl crate::IsEnum for Cachelplevel {}
#[doc = "Field `CACHELPLEVEL` reader - Instruction Cache Low-Power Level"]
pub type CachelplevelR = crate::FieldReader<Cachelplevel>;
impl CachelplevelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cachelplevel> {
        match self.bits {
            0 => Some(Cachelplevel::Base),
            1 => Some(Cachelplevel::Advanced),
            3 => Some(Cachelplevel::Minactivity),
            _ => None,
        }
    }
    #[doc = "Base instruction cache functionality."]
    #[inline(always)]
    pub fn is_base(&self) -> bool {
        *self == Cachelplevel::Base
    }
    #[doc = "Advanced buffering mode, where the cache uses the fetch pattern to predict highly accessed data and store it in low-energy memory."]
    #[inline(always)]
    pub fn is_advanced(&self) -> bool {
        *self == Cachelplevel::Advanced
    }
    #[doc = "Minimum activity mode, which allows the cache to minimize activity in logic that it predicts has a low probability being used. This mode can introduce wait-states into the instruction fetch stream when the cache exits one of its low-activity states. The number of wait-states introduced is small, but users running with 0-wait-state memory and wishing to reduce the variability that the cache might introduce with additional wait-states may wish to lower the cache low-power level. Note, this mode includes the advanced buffering mode functionality."]
    #[inline(always)]
    pub fn is_minactivity(&self) -> bool {
        *self == Cachelplevel::Minactivity
    }
}
#[doc = "Field `CACHELPLEVEL` writer - Instruction Cache Low-Power Level"]
pub type CachelplevelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cachelplevel>;
impl<'a, REG> CachelplevelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Base instruction cache functionality."]
    #[inline(always)]
    pub fn base(self) -> &'a mut crate::W<REG> {
        self.variant(Cachelplevel::Base)
    }
    #[doc = "Advanced buffering mode, where the cache uses the fetch pattern to predict highly accessed data and store it in low-energy memory."]
    #[inline(always)]
    pub fn advanced(self) -> &'a mut crate::W<REG> {
        self.variant(Cachelplevel::Advanced)
    }
    #[doc = "Minimum activity mode, which allows the cache to minimize activity in logic that it predicts has a low probability being used. This mode can introduce wait-states into the instruction fetch stream when the cache exits one of its low-activity states. The number of wait-states introduced is small, but users running with 0-wait-state memory and wishing to reduce the variability that the cache might introduce with additional wait-states may wish to lower the cache low-power level. Note, this mode includes the advanced buffering mode functionality."]
    #[inline(always)]
    pub fn minactivity(self) -> &'a mut crate::W<REG> {
        self.variant(Cachelplevel::Minactivity)
    }
}
impl R {
    #[doc = "Bits 0:1 - Instruction Cache Low-Power Level"]
    #[inline(always)]
    pub fn cachelplevel(&self) -> CachelplevelR {
        CachelplevelR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Instruction Cache Low-Power Level"]
    #[inline(always)]
    pub fn cachelplevel(&mut self) -> CachelplevelW<'_, Cacheconfig0Spec> {
        CachelplevelW::new(self, 0)
    }
}
#[doc = "Cache Configuration Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cacheconfig0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cacheconfig0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cacheconfig0Spec;
impl crate::RegisterSpec for Cacheconfig0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cacheconfig0::R`](R) reader structure"]
impl crate::Readable for Cacheconfig0Spec {}
#[doc = "`write(|w| ..)` method takes [`cacheconfig0::W`](W) writer structure"]
impl crate::Writable for Cacheconfig0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHECONFIG0 to value 0x03"]
impl crate::Resettable for Cacheconfig0Spec {
    const RESET_VALUE: u32 = 0x03;
}
