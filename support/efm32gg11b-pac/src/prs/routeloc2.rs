#[doc = "Register `ROUTELOC2` reader"]
pub type R = crate::R<ROUTELOC2_SPEC>;
#[doc = "Register `ROUTELOC2` writer"]
pub type W = crate::W<ROUTELOC2_SPEC>;
#[doc = "Field `CH8LOC` reader - I/O Location"]
pub type CH8LOC_R = crate::FieldReader<CH8LOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH8LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<CH8LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH8LOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH8LOC_A {
    type Ux = u8;
}
impl CH8LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CH8LOC_A> {
        match self.bits {
            0 => Some(CH8LOC_A::LOC0),
            1 => Some(CH8LOC_A::LOC1),
            2 => Some(CH8LOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH8LOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH8LOC_A::LOC1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH8LOC_A::LOC2
    }
}
#[doc = "Field `CH8LOC` writer - I/O Location"]
pub type CH8LOC_W<'a, REG> = crate::FieldWriter<'a, REG, 6, CH8LOC_A>;
impl<'a, REG> CH8LOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CH8LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CH8LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CH8LOC_A::LOC2)
    }
}
#[doc = "Field `CH9LOC` reader - I/O Location"]
pub type CH9LOC_R = crate::FieldReader<CH9LOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH9LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<CH9LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH9LOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH9LOC_A {
    type Ux = u8;
}
impl CH9LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CH9LOC_A> {
        match self.bits {
            0 => Some(CH9LOC_A::LOC0),
            1 => Some(CH9LOC_A::LOC1),
            2 => Some(CH9LOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH9LOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH9LOC_A::LOC1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH9LOC_A::LOC2
    }
}
#[doc = "Field `CH9LOC` writer - I/O Location"]
pub type CH9LOC_W<'a, REG> = crate::FieldWriter<'a, REG, 6, CH9LOC_A>;
impl<'a, REG> CH9LOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CH9LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CH9LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CH9LOC_A::LOC2)
    }
}
#[doc = "Field `CH10LOC` reader - I/O Location"]
pub type CH10LOC_R = crate::FieldReader<CH10LOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH10LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<CH10LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH10LOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH10LOC_A {
    type Ux = u8;
}
impl CH10LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CH10LOC_A> {
        match self.bits {
            0 => Some(CH10LOC_A::LOC0),
            1 => Some(CH10LOC_A::LOC1),
            2 => Some(CH10LOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH10LOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH10LOC_A::LOC1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH10LOC_A::LOC2
    }
}
#[doc = "Field `CH10LOC` writer - I/O Location"]
pub type CH10LOC_W<'a, REG> = crate::FieldWriter<'a, REG, 6, CH10LOC_A>;
impl<'a, REG> CH10LOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CH10LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CH10LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CH10LOC_A::LOC2)
    }
}
#[doc = "Field `CH11LOC` reader - I/O Location"]
pub type CH11LOC_R = crate::FieldReader<CH11LOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH11LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<CH11LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH11LOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH11LOC_A {
    type Ux = u8;
}
impl CH11LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CH11LOC_A> {
        match self.bits {
            0 => Some(CH11LOC_A::LOC0),
            1 => Some(CH11LOC_A::LOC1),
            2 => Some(CH11LOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH11LOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH11LOC_A::LOC1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH11LOC_A::LOC2
    }
}
#[doc = "Field `CH11LOC` writer - I/O Location"]
pub type CH11LOC_W<'a, REG> = crate::FieldWriter<'a, REG, 6, CH11LOC_A>;
impl<'a, REG> CH11LOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CH11LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CH11LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CH11LOC_A::LOC2)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn ch8loc(&self) -> CH8LOC_R {
        CH8LOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn ch9loc(&self) -> CH9LOC_R {
        CH9LOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn ch10loc(&self) -> CH10LOC_R {
        CH10LOC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn ch11loc(&self) -> CH11LOC_R {
        CH11LOC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn ch8loc(&mut self) -> CH8LOC_W<ROUTELOC2_SPEC> {
        CH8LOC_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn ch9loc(&mut self) -> CH9LOC_W<ROUTELOC2_SPEC> {
        CH9LOC_W::new(self, 8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn ch10loc(&mut self) -> CH10LOC_W<ROUTELOC2_SPEC> {
        CH10LOC_W::new(self, 16)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn ch11loc(&mut self) -> CH11LOC_W<ROUTELOC2_SPEC> {
        CH11LOC_W::new(self, 24)
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
#[doc = "I/O Routing Location Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routeloc2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routeloc2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROUTELOC2_SPEC;
impl crate::RegisterSpec for ROUTELOC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`routeloc2::R`](R) reader structure"]
impl crate::Readable for ROUTELOC2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`routeloc2::W`](W) writer structure"]
impl crate::Writable for ROUTELOC2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ROUTELOC2 to value 0"]
impl crate::Resettable for ROUTELOC2_SPEC {
    const RESET_VALUE: u32 = 0;
}
