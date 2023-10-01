#[doc = "Register `ROUTELOC0` reader"]
pub type R = crate::R<ROUTELOC0_SPEC>;
#[doc = "Register `ROUTELOC0` writer"]
pub type W = crate::W<ROUTELOC0_SPEC>;
#[doc = "Field `MIITXLOC` reader - I/O Location"]
pub type MIITXLOC_R = crate::FieldReader<MIITXLOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MIITXLOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
}
impl From<MIITXLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: MIITXLOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MIITXLOC_A {
    type Ux = u8;
}
impl MIITXLOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MIITXLOC_A> {
        match self.bits {
            0 => Some(MIITXLOC_A::LOC0),
            1 => Some(MIITXLOC_A::LOC1),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == MIITXLOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == MIITXLOC_A::LOC1
    }
}
#[doc = "Field `MIITXLOC` writer - I/O Location"]
pub type MIITXLOC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O, MIITXLOC_A>;
impl<'a, REG, const O: u8> MIITXLOC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(MIITXLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(MIITXLOC_A::LOC1)
    }
}
#[doc = "Field `MIIRXLOC` reader - I/O Location"]
pub type MIIRXLOC_R = crate::FieldReader<MIIRXLOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MIIRXLOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<MIIRXLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: MIIRXLOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MIIRXLOC_A {
    type Ux = u8;
}
impl MIIRXLOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MIIRXLOC_A> {
        match self.bits {
            0 => Some(MIIRXLOC_A::LOC0),
            1 => Some(MIIRXLOC_A::LOC1),
            2 => Some(MIIRXLOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == MIIRXLOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == MIIRXLOC_A::LOC1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == MIIRXLOC_A::LOC2
    }
}
#[doc = "Field `MIIRXLOC` writer - I/O Location"]
pub type MIIRXLOC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O, MIIRXLOC_A>;
impl<'a, REG, const O: u8> MIIRXLOC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(MIIRXLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(MIIRXLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(MIIRXLOC_A::LOC2)
    }
}
#[doc = "Field `MIICRSLOC` reader - I/O Location"]
pub type MIICRSLOC_R = crate::FieldReader<MIICRSLOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MIICRSLOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<MIICRSLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: MIICRSLOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MIICRSLOC_A {
    type Ux = u8;
}
impl MIICRSLOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MIICRSLOC_A> {
        match self.bits {
            0 => Some(MIICRSLOC_A::LOC0),
            1 => Some(MIICRSLOC_A::LOC1),
            2 => Some(MIICRSLOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == MIICRSLOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == MIICRSLOC_A::LOC1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == MIICRSLOC_A::LOC2
    }
}
#[doc = "Field `MIICRSLOC` writer - I/O Location"]
pub type MIICRSLOC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O, MIICRSLOC_A>;
impl<'a, REG, const O: u8> MIICRSLOC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(MIICRSLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(MIICRSLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(MIICRSLOC_A::LOC2)
    }
}
#[doc = "Field `MIICOLLOC` reader - I/O Location"]
pub type MIICOLLOC_R = crate::FieldReader<MIICOLLOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MIICOLLOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<MIICOLLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: MIICOLLOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MIICOLLOC_A {
    type Ux = u8;
}
impl MIICOLLOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MIICOLLOC_A> {
        match self.bits {
            0 => Some(MIICOLLOC_A::LOC0),
            1 => Some(MIICOLLOC_A::LOC1),
            2 => Some(MIICOLLOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == MIICOLLOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == MIICOLLOC_A::LOC1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == MIICOLLOC_A::LOC2
    }
}
#[doc = "Field `MIICOLLOC` writer - I/O Location"]
pub type MIICOLLOC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O, MIICOLLOC_A>;
impl<'a, REG, const O: u8> MIICOLLOC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(MIICOLLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(MIICOLLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(MIICOLLOC_A::LOC2)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn miitxloc(&self) -> MIITXLOC_R {
        MIITXLOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn miirxloc(&self) -> MIIRXLOC_R {
        MIIRXLOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn miicrsloc(&self) -> MIICRSLOC_R {
        MIICRSLOC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn miicolloc(&self) -> MIICOLLOC_R {
        MIICOLLOC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn miitxloc(&mut self) -> MIITXLOC_W<ROUTELOC0_SPEC, 0> {
        MIITXLOC_W::new(self)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn miirxloc(&mut self) -> MIIRXLOC_W<ROUTELOC0_SPEC, 8> {
        MIIRXLOC_W::new(self)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn miicrsloc(&mut self) -> MIICRSLOC_W<ROUTELOC0_SPEC, 16> {
        MIICRSLOC_W::new(self)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn miicolloc(&mut self) -> MIICOLLOC_W<ROUTELOC0_SPEC, 24> {
        MIICOLLOC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "I/O Route Location Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routeloc0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routeloc0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROUTELOC0_SPEC;
impl crate::RegisterSpec for ROUTELOC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`routeloc0::R`](R) reader structure"]
impl crate::Readable for ROUTELOC0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`routeloc0::W`](W) writer structure"]
impl crate::Writable for ROUTELOC0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ROUTELOC0 to value 0"]
impl crate::Resettable for ROUTELOC0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
