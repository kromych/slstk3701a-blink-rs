#[doc = "Register `ROUTELOC1` reader"]
pub type R = crate::R<Routeloc1Spec>;
#[doc = "Register `ROUTELOC1` writer"]
pub type W = crate::W<Routeloc1Spec>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adloc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
}
impl From<Adloc> for u8 {
    #[inline(always)]
    fn from(variant: Adloc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adloc {
    type Ux = u8;
}
impl crate::IsEnum for Adloc {}
#[doc = "Field `ADLOC` reader - I/O Location"]
pub type AdlocR = crate::FieldReader<Adloc>;
impl AdlocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Adloc> {
        match self.bits {
            0 => Some(Adloc::Loc0),
            1 => Some(Adloc::Loc1),
            2 => Some(Adloc::Loc2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Adloc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Adloc::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Adloc::Loc2
    }
}
#[doc = "Field `ADLOC` writer - I/O Location"]
pub type AdlocW<'a, REG> = crate::FieldWriter<'a, REG, 6, Adloc>;
impl<'a, REG> AdlocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Adloc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Adloc::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Adloc::Loc2)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Aloc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
    #[doc = "3: Location 3"]
    Loc3 = 3,
}
impl From<Aloc> for u8 {
    #[inline(always)]
    fn from(variant: Aloc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Aloc {
    type Ux = u8;
}
impl crate::IsEnum for Aloc {}
#[doc = "Field `ALOC` reader - I/O Location"]
pub type AlocR = crate::FieldReader<Aloc>;
impl AlocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Aloc> {
        match self.bits {
            0 => Some(Aloc::Loc0),
            1 => Some(Aloc::Loc1),
            2 => Some(Aloc::Loc2),
            3 => Some(Aloc::Loc3),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Aloc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Aloc::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Aloc::Loc2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == Aloc::Loc3
    }
}
#[doc = "Field `ALOC` writer - I/O Location"]
pub type AlocW<'a, REG> = crate::FieldWriter<'a, REG, 6, Aloc>;
impl<'a, REG> AlocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Aloc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Aloc::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Aloc::Loc2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(Aloc::Loc3)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rdyloc {
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
}
impl From<Rdyloc> for u8 {
    #[inline(always)]
    fn from(variant: Rdyloc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rdyloc {
    type Ux = u8;
}
impl crate::IsEnum for Rdyloc {}
#[doc = "Field `RDYLOC` reader - I/O Location"]
pub type RdylocR = crate::FieldReader<Rdyloc>;
impl RdylocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rdyloc> {
        match self.bits {
            0 => Some(Rdyloc::Loc0),
            1 => Some(Rdyloc::Loc1),
            2 => Some(Rdyloc::Loc2),
            3 => Some(Rdyloc::Loc3),
            4 => Some(Rdyloc::Loc4),
            5 => Some(Rdyloc::Loc5),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Rdyloc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Rdyloc::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Rdyloc::Loc2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == Rdyloc::Loc3
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == Rdyloc::Loc4
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == Rdyloc::Loc5
    }
}
#[doc = "Field `RDYLOC` writer - I/O Location"]
pub type RdylocW<'a, REG> = crate::FieldWriter<'a, REG, 6, Rdyloc>;
impl<'a, REG> RdylocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Rdyloc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Rdyloc::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Rdyloc::Loc2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(Rdyloc::Loc3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(Rdyloc::Loc4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(Rdyloc::Loc5)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn adloc(&self) -> AdlocR {
        AdlocR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn aloc(&self) -> AlocR {
        AlocR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn rdyloc(&self) -> RdylocR {
        RdylocR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn adloc(&mut self) -> AdlocW<'_, Routeloc1Spec> {
        AdlocW::new(self, 0)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn aloc(&mut self) -> AlocW<'_, Routeloc1Spec> {
        AlocW::new(self, 8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn rdyloc(&mut self) -> RdylocW<'_, Routeloc1Spec> {
        RdylocW::new(self, 16)
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
