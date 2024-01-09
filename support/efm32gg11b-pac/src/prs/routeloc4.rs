#[doc = "Register `ROUTELOC4` reader"]
pub type R = crate::R<ROUTELOC4_SPEC>;
#[doc = "Register `ROUTELOC4` writer"]
pub type W = crate::W<ROUTELOC4_SPEC>;
#[doc = "Field `CH16LOC` reader - I/O Location"]
pub type CH16LOC_R = crate::FieldReader<CH16LOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH16LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<CH16LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH16LOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH16LOC_A {
    type Ux = u8;
}
impl CH16LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CH16LOC_A> {
        match self.bits {
            0 => Some(CH16LOC_A::LOC0),
            1 => Some(CH16LOC_A::LOC1),
            2 => Some(CH16LOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH16LOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH16LOC_A::LOC1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH16LOC_A::LOC2
    }
}
#[doc = "Field `CH16LOC` writer - I/O Location"]
pub type CH16LOC_W<'a, REG> = crate::FieldWriter<'a, REG, 6, CH16LOC_A>;
impl<'a, REG> CH16LOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CH16LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CH16LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CH16LOC_A::LOC2)
    }
}
#[doc = "Field `CH17LOC` reader - I/O Location"]
pub type CH17LOC_R = crate::FieldReader<CH17LOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH17LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<CH17LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH17LOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH17LOC_A {
    type Ux = u8;
}
impl CH17LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CH17LOC_A> {
        match self.bits {
            0 => Some(CH17LOC_A::LOC0),
            1 => Some(CH17LOC_A::LOC1),
            2 => Some(CH17LOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH17LOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH17LOC_A::LOC1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH17LOC_A::LOC2
    }
}
#[doc = "Field `CH17LOC` writer - I/O Location"]
pub type CH17LOC_W<'a, REG> = crate::FieldWriter<'a, REG, 6, CH17LOC_A>;
impl<'a, REG> CH17LOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CH17LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CH17LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CH17LOC_A::LOC2)
    }
}
#[doc = "Field `CH18LOC` reader - I/O Location"]
pub type CH18LOC_R = crate::FieldReader<CH18LOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH18LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<CH18LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH18LOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH18LOC_A {
    type Ux = u8;
}
impl CH18LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CH18LOC_A> {
        match self.bits {
            0 => Some(CH18LOC_A::LOC0),
            1 => Some(CH18LOC_A::LOC1),
            2 => Some(CH18LOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH18LOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH18LOC_A::LOC1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH18LOC_A::LOC2
    }
}
#[doc = "Field `CH18LOC` writer - I/O Location"]
pub type CH18LOC_W<'a, REG> = crate::FieldWriter<'a, REG, 6, CH18LOC_A>;
impl<'a, REG> CH18LOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CH18LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CH18LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CH18LOC_A::LOC2)
    }
}
#[doc = "Field `CH19LOC` reader - I/O Location"]
pub type CH19LOC_R = crate::FieldReader<CH19LOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH19LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<CH19LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH19LOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH19LOC_A {
    type Ux = u8;
}
impl CH19LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CH19LOC_A> {
        match self.bits {
            0 => Some(CH19LOC_A::LOC0),
            1 => Some(CH19LOC_A::LOC1),
            2 => Some(CH19LOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH19LOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH19LOC_A::LOC1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH19LOC_A::LOC2
    }
}
#[doc = "Field `CH19LOC` writer - I/O Location"]
pub type CH19LOC_W<'a, REG> = crate::FieldWriter<'a, REG, 6, CH19LOC_A>;
impl<'a, REG> CH19LOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CH19LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CH19LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CH19LOC_A::LOC2)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn ch16loc(&self) -> CH16LOC_R {
        CH16LOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn ch17loc(&self) -> CH17LOC_R {
        CH17LOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn ch18loc(&self) -> CH18LOC_R {
        CH18LOC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn ch19loc(&self) -> CH19LOC_R {
        CH19LOC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn ch16loc(&mut self) -> CH16LOC_W<ROUTELOC4_SPEC> {
        CH16LOC_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn ch17loc(&mut self) -> CH17LOC_W<ROUTELOC4_SPEC> {
        CH17LOC_W::new(self, 8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn ch18loc(&mut self) -> CH18LOC_W<ROUTELOC4_SPEC> {
        CH18LOC_W::new(self, 16)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn ch19loc(&mut self) -> CH19LOC_W<ROUTELOC4_SPEC> {
        CH19LOC_W::new(self, 24)
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
#[doc = "I/O Routing Location Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routeloc4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routeloc4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROUTELOC4_SPEC;
impl crate::RegisterSpec for ROUTELOC4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`routeloc4::R`](R) reader structure"]
impl crate::Readable for ROUTELOC4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`routeloc4::W`](W) writer structure"]
impl crate::Writable for ROUTELOC4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ROUTELOC4 to value 0"]
impl crate::Resettable for ROUTELOC4_SPEC {
    const RESET_VALUE: u32 = 0;
}
