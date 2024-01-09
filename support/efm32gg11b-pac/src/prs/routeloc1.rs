#[doc = "Register `ROUTELOC1` reader"]
pub type R = crate::R<ROUTELOC1_SPEC>;
#[doc = "Register `ROUTELOC1` writer"]
pub type W = crate::W<ROUTELOC1_SPEC>;
#[doc = "Field `CH4LOC` reader - I/O Location"]
pub type CH4LOC_R = crate::FieldReader<CH4LOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH4LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<CH4LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH4LOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH4LOC_A {
    type Ux = u8;
}
impl CH4LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CH4LOC_A> {
        match self.bits {
            0 => Some(CH4LOC_A::LOC0),
            1 => Some(CH4LOC_A::LOC1),
            2 => Some(CH4LOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH4LOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH4LOC_A::LOC1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH4LOC_A::LOC2
    }
}
#[doc = "Field `CH4LOC` writer - I/O Location"]
pub type CH4LOC_W<'a, REG> = crate::FieldWriter<'a, REG, 6, CH4LOC_A>;
impl<'a, REG> CH4LOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CH4LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CH4LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CH4LOC_A::LOC2)
    }
}
#[doc = "Field `CH5LOC` reader - I/O Location"]
pub type CH5LOC_R = crate::FieldReader<CH5LOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH5LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<CH5LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH5LOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH5LOC_A {
    type Ux = u8;
}
impl CH5LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CH5LOC_A> {
        match self.bits {
            0 => Some(CH5LOC_A::LOC0),
            1 => Some(CH5LOC_A::LOC1),
            2 => Some(CH5LOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH5LOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH5LOC_A::LOC1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH5LOC_A::LOC2
    }
}
#[doc = "Field `CH5LOC` writer - I/O Location"]
pub type CH5LOC_W<'a, REG> = crate::FieldWriter<'a, REG, 6, CH5LOC_A>;
impl<'a, REG> CH5LOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CH5LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CH5LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CH5LOC_A::LOC2)
    }
}
#[doc = "Field `CH6LOC` reader - I/O Location"]
pub type CH6LOC_R = crate::FieldReader<CH6LOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH6LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<CH6LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH6LOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH6LOC_A {
    type Ux = u8;
}
impl CH6LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CH6LOC_A> {
        match self.bits {
            0 => Some(CH6LOC_A::LOC0),
            1 => Some(CH6LOC_A::LOC1),
            2 => Some(CH6LOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH6LOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH6LOC_A::LOC1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH6LOC_A::LOC2
    }
}
#[doc = "Field `CH6LOC` writer - I/O Location"]
pub type CH6LOC_W<'a, REG> = crate::FieldWriter<'a, REG, 6, CH6LOC_A>;
impl<'a, REG> CH6LOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CH6LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CH6LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CH6LOC_A::LOC2)
    }
}
#[doc = "Field `CH7LOC` reader - I/O Location"]
pub type CH7LOC_R = crate::FieldReader<CH7LOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH7LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<CH7LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH7LOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH7LOC_A {
    type Ux = u8;
}
impl CH7LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CH7LOC_A> {
        match self.bits {
            0 => Some(CH7LOC_A::LOC0),
            1 => Some(CH7LOC_A::LOC1),
            2 => Some(CH7LOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH7LOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH7LOC_A::LOC1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH7LOC_A::LOC2
    }
}
#[doc = "Field `CH7LOC` writer - I/O Location"]
pub type CH7LOC_W<'a, REG> = crate::FieldWriter<'a, REG, 6, CH7LOC_A>;
impl<'a, REG> CH7LOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CH7LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CH7LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CH7LOC_A::LOC2)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn ch4loc(&self) -> CH4LOC_R {
        CH4LOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn ch5loc(&self) -> CH5LOC_R {
        CH5LOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn ch6loc(&self) -> CH6LOC_R {
        CH6LOC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn ch7loc(&self) -> CH7LOC_R {
        CH7LOC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn ch4loc(&mut self) -> CH4LOC_W<ROUTELOC1_SPEC> {
        CH4LOC_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn ch5loc(&mut self) -> CH5LOC_W<ROUTELOC1_SPEC> {
        CH5LOC_W::new(self, 8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn ch6loc(&mut self) -> CH6LOC_W<ROUTELOC1_SPEC> {
        CH6LOC_W::new(self, 16)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn ch7loc(&mut self) -> CH7LOC_W<ROUTELOC1_SPEC> {
        CH7LOC_W::new(self, 24)
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
#[doc = "I/O Routing Location Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routeloc1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routeloc1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROUTELOC1_SPEC;
impl crate::RegisterSpec for ROUTELOC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`routeloc1::R`](R) reader structure"]
impl crate::Readable for ROUTELOC1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`routeloc1::W`](W) writer structure"]
impl crate::Writable for ROUTELOC1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ROUTELOC1 to value 0"]
impl crate::Resettable for ROUTELOC1_SPEC {
    const RESET_VALUE: u32 = 0;
}
