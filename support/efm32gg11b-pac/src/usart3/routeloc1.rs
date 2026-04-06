#[doc = "Register `ROUTELOC1` reader"]
pub type R = crate::R<Routeloc1Spec>;
#[doc = "Register `ROUTELOC1` writer"]
pub type W = crate::W<Routeloc1Spec>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctsloc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
    #[doc = "3: Location 3"]
    Loc3 = 3,
    #[doc = "4: Location 4"]
    Loc4 = 4,
    #[doc = "5: Location 5"]
    Loc5 = 5,
    #[doc = "6: Location 6"]
    Loc6 = 6,
}
impl From<Ctsloc> for u8 {
    #[inline(always)]
    fn from(variant: Ctsloc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctsloc {
    type Ux = u8;
}
impl crate::IsEnum for Ctsloc {}
#[doc = "Field `CTSLOC` reader - I/O Location"]
pub type CtslocR = crate::FieldReader<Ctsloc>;
impl CtslocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ctsloc> {
        match self.bits {
            0 => Some(Ctsloc::Loc0),
            1 => Some(Ctsloc::Loc1),
            2 => Some(Ctsloc::Loc2),
            3 => Some(Ctsloc::Loc3),
            4 => Some(Ctsloc::Loc4),
            5 => Some(Ctsloc::Loc5),
            6 => Some(Ctsloc::Loc6),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Ctsloc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Ctsloc::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Ctsloc::Loc2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == Ctsloc::Loc3
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == Ctsloc::Loc4
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == Ctsloc::Loc5
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == Ctsloc::Loc6
    }
}
#[doc = "Field `CTSLOC` writer - I/O Location"]
pub type CtslocW<'a, REG> = crate::FieldWriter<'a, REG, 6, Ctsloc>;
impl<'a, REG> CtslocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsloc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsloc::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsloc::Loc2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsloc::Loc3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsloc::Loc4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsloc::Loc5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsloc::Loc6)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rtsloc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
    #[doc = "3: Location 3"]
    Loc3 = 3,
    #[doc = "4: Location 4"]
    Loc4 = 4,
    #[doc = "5: Location 5"]
    Loc5 = 5,
    #[doc = "6: Location 6"]
    Loc6 = 6,
}
impl From<Rtsloc> for u8 {
    #[inline(always)]
    fn from(variant: Rtsloc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rtsloc {
    type Ux = u8;
}
impl crate::IsEnum for Rtsloc {}
#[doc = "Field `RTSLOC` reader - I/O Location"]
pub type RtslocR = crate::FieldReader<Rtsloc>;
impl RtslocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rtsloc> {
        match self.bits {
            0 => Some(Rtsloc::Loc0),
            1 => Some(Rtsloc::Loc1),
            2 => Some(Rtsloc::Loc2),
            3 => Some(Rtsloc::Loc3),
            4 => Some(Rtsloc::Loc4),
            5 => Some(Rtsloc::Loc5),
            6 => Some(Rtsloc::Loc6),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Rtsloc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Rtsloc::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Rtsloc::Loc2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == Rtsloc::Loc3
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == Rtsloc::Loc4
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == Rtsloc::Loc5
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == Rtsloc::Loc6
    }
}
#[doc = "Field `RTSLOC` writer - I/O Location"]
pub type RtslocW<'a, REG> = crate::FieldWriter<'a, REG, 6, Rtsloc>;
impl<'a, REG> RtslocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsloc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsloc::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsloc::Loc2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsloc::Loc3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsloc::Loc4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsloc::Loc5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsloc::Loc6)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn ctsloc(&self) -> CtslocR {
        CtslocR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn rtsloc(&self) -> RtslocR {
        RtslocR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn ctsloc(&mut self) -> CtslocW<'_, Routeloc1Spec> {
        CtslocW::new(self, 0)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn rtsloc(&mut self) -> RtslocW<'_, Routeloc1Spec> {
        RtslocW::new(self, 8)
    }
}
#[doc = "I/O Routing Location Register\n\nYou can [`read`](crate::Reg::read) this register and get [`routeloc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routeloc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
