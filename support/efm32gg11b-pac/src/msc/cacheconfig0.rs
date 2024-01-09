#[doc = "Register `CACHECONFIG0` reader"]
pub type R = crate::R<CACHECONFIG0_SPEC>;
#[doc = "Register `CACHECONFIG0` writer"]
pub type W = crate::W<CACHECONFIG0_SPEC>;
#[doc = "Field `CACHELPLEVEL` reader - Instruction Cache Low-Power Level"]
pub type CACHELPLEVEL_R = crate::FieldReader<CACHELPLEVEL_A>;
#[doc = "Instruction Cache Low-Power Level\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CACHELPLEVEL_A {
    #[doc = "0: Base instruction cache functionality."]
    BASE = 0,
    #[doc = "1: Advanced buffering mode, where the cache uses the fetch pattern to predict highly accessed data and store it in low-energy memory."]
    ADVANCED = 1,
    #[doc = "3: Minimum activity mode, which allows the cache to minimize activity in logic that it predicts has a low probability being used. This mode can introduce wait-states into the instruction fetch stream when the cache exits one of its low-activity states. The number of wait-states introduced is small, but users running with 0-wait-state memory and wishing to reduce the variability that the cache might introduce with additional wait-states may wish to lower the cache low-power level. Note, this mode includes the advanced buffering mode functionality."]
    MINACTIVITY = 3,
}
impl From<CACHELPLEVEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CACHELPLEVEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CACHELPLEVEL_A {
    type Ux = u8;
}
impl CACHELPLEVEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CACHELPLEVEL_A> {
        match self.bits {
            0 => Some(CACHELPLEVEL_A::BASE),
            1 => Some(CACHELPLEVEL_A::ADVANCED),
            3 => Some(CACHELPLEVEL_A::MINACTIVITY),
            _ => None,
        }
    }
    #[doc = "Base instruction cache functionality."]
    #[inline(always)]
    pub fn is_base(&self) -> bool {
        *self == CACHELPLEVEL_A::BASE
    }
    #[doc = "Advanced buffering mode, where the cache uses the fetch pattern to predict highly accessed data and store it in low-energy memory."]
    #[inline(always)]
    pub fn is_advanced(&self) -> bool {
        *self == CACHELPLEVEL_A::ADVANCED
    }
    #[doc = "Minimum activity mode, which allows the cache to minimize activity in logic that it predicts has a low probability being used. This mode can introduce wait-states into the instruction fetch stream when the cache exits one of its low-activity states. The number of wait-states introduced is small, but users running with 0-wait-state memory and wishing to reduce the variability that the cache might introduce with additional wait-states may wish to lower the cache low-power level. Note, this mode includes the advanced buffering mode functionality."]
    #[inline(always)]
    pub fn is_minactivity(&self) -> bool {
        *self == CACHELPLEVEL_A::MINACTIVITY
    }
}
#[doc = "Field `CACHELPLEVEL` writer - Instruction Cache Low-Power Level"]
pub type CACHELPLEVEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CACHELPLEVEL_A>;
impl<'a, REG> CACHELPLEVEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Base instruction cache functionality."]
    #[inline(always)]
    pub fn base(self) -> &'a mut crate::W<REG> {
        self.variant(CACHELPLEVEL_A::BASE)
    }
    #[doc = "Advanced buffering mode, where the cache uses the fetch pattern to predict highly accessed data and store it in low-energy memory."]
    #[inline(always)]
    pub fn advanced(self) -> &'a mut crate::W<REG> {
        self.variant(CACHELPLEVEL_A::ADVANCED)
    }
    #[doc = "Minimum activity mode, which allows the cache to minimize activity in logic that it predicts has a low probability being used. This mode can introduce wait-states into the instruction fetch stream when the cache exits one of its low-activity states. The number of wait-states introduced is small, but users running with 0-wait-state memory and wishing to reduce the variability that the cache might introduce with additional wait-states may wish to lower the cache low-power level. Note, this mode includes the advanced buffering mode functionality."]
    #[inline(always)]
    pub fn minactivity(self) -> &'a mut crate::W<REG> {
        self.variant(CACHELPLEVEL_A::MINACTIVITY)
    }
}
impl R {
    #[doc = "Bits 0:1 - Instruction Cache Low-Power Level"]
    #[inline(always)]
    pub fn cachelplevel(&self) -> CACHELPLEVEL_R {
        CACHELPLEVEL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Instruction Cache Low-Power Level"]
    #[inline(always)]
    #[must_use]
    pub fn cachelplevel(&mut self) -> CACHELPLEVEL_W<CACHECONFIG0_SPEC> {
        CACHELPLEVEL_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Cache Configuration Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cacheconfig0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cacheconfig0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHECONFIG0_SPEC;
impl crate::RegisterSpec for CACHECONFIG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cacheconfig0::R`](R) reader structure"]
impl crate::Readable for CACHECONFIG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cacheconfig0::W`](W) writer structure"]
impl crate::Writable for CACHECONFIG0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CACHECONFIG0 to value 0x03"]
impl crate::Resettable for CACHECONFIG0_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
