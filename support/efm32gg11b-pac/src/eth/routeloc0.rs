#[doc = "Register `ROUTELOC0` reader"]
pub type R = crate::R<Routeloc0Spec>;
#[doc = "Register `ROUTELOC0` writer"]
pub type W = crate::W<Routeloc0Spec>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Miitxloc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
}
impl From<Miitxloc> for u8 {
    #[inline(always)]
    fn from(variant: Miitxloc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Miitxloc {
    type Ux = u8;
}
impl crate::IsEnum for Miitxloc {}
#[doc = "Field `MIITXLOC` reader - I/O Location"]
pub type MiitxlocR = crate::FieldReader<Miitxloc>;
impl MiitxlocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Miitxloc> {
        match self.bits {
            0 => Some(Miitxloc::Loc0),
            1 => Some(Miitxloc::Loc1),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Miitxloc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Miitxloc::Loc1
    }
}
#[doc = "Field `MIITXLOC` writer - I/O Location"]
pub type MiitxlocW<'a, REG> = crate::FieldWriter<'a, REG, 6, Miitxloc>;
impl<'a, REG> MiitxlocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Miitxloc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Miitxloc::Loc1)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Miirxloc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
}
impl From<Miirxloc> for u8 {
    #[inline(always)]
    fn from(variant: Miirxloc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Miirxloc {
    type Ux = u8;
}
impl crate::IsEnum for Miirxloc {}
#[doc = "Field `MIIRXLOC` reader - I/O Location"]
pub type MiirxlocR = crate::FieldReader<Miirxloc>;
impl MiirxlocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Miirxloc> {
        match self.bits {
            0 => Some(Miirxloc::Loc0),
            1 => Some(Miirxloc::Loc1),
            2 => Some(Miirxloc::Loc2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Miirxloc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Miirxloc::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Miirxloc::Loc2
    }
}
#[doc = "Field `MIIRXLOC` writer - I/O Location"]
pub type MiirxlocW<'a, REG> = crate::FieldWriter<'a, REG, 6, Miirxloc>;
impl<'a, REG> MiirxlocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Miirxloc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Miirxloc::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Miirxloc::Loc2)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Miicrsloc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
}
impl From<Miicrsloc> for u8 {
    #[inline(always)]
    fn from(variant: Miicrsloc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Miicrsloc {
    type Ux = u8;
}
impl crate::IsEnum for Miicrsloc {}
#[doc = "Field `MIICRSLOC` reader - I/O Location"]
pub type MiicrslocR = crate::FieldReader<Miicrsloc>;
impl MiicrslocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Miicrsloc> {
        match self.bits {
            0 => Some(Miicrsloc::Loc0),
            1 => Some(Miicrsloc::Loc1),
            2 => Some(Miicrsloc::Loc2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Miicrsloc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Miicrsloc::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Miicrsloc::Loc2
    }
}
#[doc = "Field `MIICRSLOC` writer - I/O Location"]
pub type MiicrslocW<'a, REG> = crate::FieldWriter<'a, REG, 6, Miicrsloc>;
impl<'a, REG> MiicrslocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Miicrsloc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Miicrsloc::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Miicrsloc::Loc2)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Miicolloc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
}
impl From<Miicolloc> for u8 {
    #[inline(always)]
    fn from(variant: Miicolloc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Miicolloc {
    type Ux = u8;
}
impl crate::IsEnum for Miicolloc {}
#[doc = "Field `MIICOLLOC` reader - I/O Location"]
pub type MiicollocR = crate::FieldReader<Miicolloc>;
impl MiicollocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Miicolloc> {
        match self.bits {
            0 => Some(Miicolloc::Loc0),
            1 => Some(Miicolloc::Loc1),
            2 => Some(Miicolloc::Loc2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Miicolloc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Miicolloc::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Miicolloc::Loc2
    }
}
#[doc = "Field `MIICOLLOC` writer - I/O Location"]
pub type MiicollocW<'a, REG> = crate::FieldWriter<'a, REG, 6, Miicolloc>;
impl<'a, REG> MiicollocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Miicolloc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Miicolloc::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Miicolloc::Loc2)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn miitxloc(&self) -> MiitxlocR {
        MiitxlocR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn miirxloc(&self) -> MiirxlocR {
        MiirxlocR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn miicrsloc(&self) -> MiicrslocR {
        MiicrslocR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn miicolloc(&self) -> MiicollocR {
        MiicollocR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn miitxloc(&mut self) -> MiitxlocW<'_, Routeloc0Spec> {
        MiitxlocW::new(self, 0)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn miirxloc(&mut self) -> MiirxlocW<'_, Routeloc0Spec> {
        MiirxlocW::new(self, 8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn miicrsloc(&mut self) -> MiicrslocW<'_, Routeloc0Spec> {
        MiicrslocW::new(self, 16)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn miicolloc(&mut self) -> MiicollocW<'_, Routeloc0Spec> {
        MiicollocW::new(self, 24)
    }
}
#[doc = "I/O Route Location Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`routeloc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routeloc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Routeloc0Spec;
impl crate::RegisterSpec for Routeloc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`routeloc0::R`](R) reader structure"]
impl crate::Readable for Routeloc0Spec {}
#[doc = "`write(|w| ..)` method takes [`routeloc0::W`](W) writer structure"]
impl crate::Writable for Routeloc0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ROUTELOC0 to value 0"]
impl crate::Resettable for Routeloc0Spec {}
