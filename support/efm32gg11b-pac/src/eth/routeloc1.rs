#[doc = "Register `ROUTELOC1` reader"]
pub type R = crate::R<Routeloc1Spec>;
#[doc = "Register `ROUTELOC1` writer"]
pub type W = crate::W<Routeloc1Spec>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tsuextclkloc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
    #[doc = "3: Location 3"]
    Loc3 = 3,
}
impl From<Tsuextclkloc> for u8 {
    #[inline(always)]
    fn from(variant: Tsuextclkloc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tsuextclkloc {
    type Ux = u8;
}
impl crate::IsEnum for Tsuextclkloc {}
#[doc = "Field `TSUEXTCLKLOC` reader - I/O Location"]
pub type TsuextclklocR = crate::FieldReader<Tsuextclkloc>;
impl TsuextclklocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tsuextclkloc> {
        match self.bits {
            0 => Some(Tsuextclkloc::Loc0),
            1 => Some(Tsuextclkloc::Loc1),
            2 => Some(Tsuextclkloc::Loc2),
            3 => Some(Tsuextclkloc::Loc3),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Tsuextclkloc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Tsuextclkloc::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Tsuextclkloc::Loc2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == Tsuextclkloc::Loc3
    }
}
#[doc = "Field `TSUEXTCLKLOC` writer - I/O Location"]
pub type TsuextclklocW<'a, REG> = crate::FieldWriter<'a, REG, 6, Tsuextclkloc>;
impl<'a, REG> TsuextclklocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Tsuextclkloc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Tsuextclkloc::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Tsuextclkloc::Loc2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(Tsuextclkloc::Loc3)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tsutmrtogloc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
    #[doc = "3: Location 3"]
    Loc3 = 3,
}
impl From<Tsutmrtogloc> for u8 {
    #[inline(always)]
    fn from(variant: Tsutmrtogloc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tsutmrtogloc {
    type Ux = u8;
}
impl crate::IsEnum for Tsutmrtogloc {}
#[doc = "Field `TSUTMRTOGLOC` reader - I/O Location"]
pub type TsutmrtoglocR = crate::FieldReader<Tsutmrtogloc>;
impl TsutmrtoglocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tsutmrtogloc> {
        match self.bits {
            0 => Some(Tsutmrtogloc::Loc0),
            1 => Some(Tsutmrtogloc::Loc1),
            2 => Some(Tsutmrtogloc::Loc2),
            3 => Some(Tsutmrtogloc::Loc3),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Tsutmrtogloc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Tsutmrtogloc::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Tsutmrtogloc::Loc2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == Tsutmrtogloc::Loc3
    }
}
#[doc = "Field `TSUTMRTOGLOC` writer - I/O Location"]
pub type TsutmrtoglocW<'a, REG> = crate::FieldWriter<'a, REG, 6, Tsutmrtogloc>;
impl<'a, REG> TsutmrtoglocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Tsutmrtogloc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Tsutmrtogloc::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Tsutmrtogloc::Loc2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(Tsutmrtogloc::Loc3)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mdioloc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
    #[doc = "3: Location 3"]
    Loc3 = 3,
}
impl From<Mdioloc> for u8 {
    #[inline(always)]
    fn from(variant: Mdioloc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mdioloc {
    type Ux = u8;
}
impl crate::IsEnum for Mdioloc {}
#[doc = "Field `MDIOLOC` reader - I/O Location"]
pub type MdiolocR = crate::FieldReader<Mdioloc>;
impl MdiolocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mdioloc> {
        match self.bits {
            0 => Some(Mdioloc::Loc0),
            1 => Some(Mdioloc::Loc1),
            2 => Some(Mdioloc::Loc2),
            3 => Some(Mdioloc::Loc3),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Mdioloc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Mdioloc::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Mdioloc::Loc2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == Mdioloc::Loc3
    }
}
#[doc = "Field `MDIOLOC` writer - I/O Location"]
pub type MdiolocW<'a, REG> = crate::FieldWriter<'a, REG, 6, Mdioloc>;
impl<'a, REG> MdiolocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Mdioloc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Mdioloc::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Mdioloc::Loc2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(Mdioloc::Loc3)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rmiiloc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
}
impl From<Rmiiloc> for u8 {
    #[inline(always)]
    fn from(variant: Rmiiloc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rmiiloc {
    type Ux = u8;
}
impl crate::IsEnum for Rmiiloc {}
#[doc = "Field `RMIILOC` reader - I/O Location"]
pub type RmiilocR = crate::FieldReader<Rmiiloc>;
impl RmiilocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rmiiloc> {
        match self.bits {
            0 => Some(Rmiiloc::Loc0),
            1 => Some(Rmiiloc::Loc1),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Rmiiloc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Rmiiloc::Loc1
    }
}
#[doc = "Field `RMIILOC` writer - I/O Location"]
pub type RmiilocW<'a, REG> = crate::FieldWriter<'a, REG, 6, Rmiiloc>;
impl<'a, REG> RmiilocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Rmiiloc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Rmiiloc::Loc1)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn tsuextclkloc(&self) -> TsuextclklocR {
        TsuextclklocR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn tsutmrtogloc(&self) -> TsutmrtoglocR {
        TsutmrtoglocR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn mdioloc(&self) -> MdiolocR {
        MdiolocR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn rmiiloc(&self) -> RmiilocR {
        RmiilocR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn tsuextclkloc(&mut self) -> TsuextclklocW<'_, Routeloc1Spec> {
        TsuextclklocW::new(self, 0)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn tsutmrtogloc(&mut self) -> TsutmrtoglocW<'_, Routeloc1Spec> {
        TsutmrtoglocW::new(self, 8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn mdioloc(&mut self) -> MdiolocW<'_, Routeloc1Spec> {
        MdiolocW::new(self, 16)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn rmiiloc(&mut self) -> RmiilocW<'_, Routeloc1Spec> {
        RmiilocW::new(self, 24)
    }
}
#[doc = "I/O Route Location Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`routeloc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routeloc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Routeloc1Spec;
impl crate::RegisterSpec for Routeloc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`routeloc1::R`](R) reader structure"]
impl crate::Readable for Routeloc1Spec {}
#[doc = "`write(|w| ..)` method takes [`routeloc1::W`](W) writer structure"]
impl crate::Writable for Routeloc1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ROUTELOC1 to value 0"]
impl crate::Resettable for Routeloc1Spec {}
