#[doc = "Register `ROUTELOC0` reader"]
pub type R = crate::R<Routeloc0Spec>;
#[doc = "Register `ROUTELOC0` writer"]
pub type W = crate::W<Routeloc0Spec>;
#[doc = "I/O Location for D0-7 Pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Datloc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
}
impl From<Datloc> for u8 {
    #[inline(always)]
    fn from(variant: Datloc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Datloc {
    type Ux = u8;
}
impl crate::IsEnum for Datloc {}
#[doc = "Field `DATLOC` reader - I/O Location for D0-7 Pins"]
pub type DatlocR = crate::FieldReader<Datloc>;
impl DatlocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Datloc> {
        match self.bits {
            0 => Some(Datloc::Loc0),
            1 => Some(Datloc::Loc1),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Datloc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Datloc::Loc1
    }
}
#[doc = "Field `DATLOC` writer - I/O Location for D0-7 Pins"]
pub type DatlocW<'a, REG> = crate::FieldWriter<'a, REG, 6, Datloc>;
impl<'a, REG> DatlocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Datloc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Datloc::Loc1)
    }
}
#[doc = "I/O Location for CD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cdloc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
    #[doc = "3: Location 3"]
    Loc3 = 3,
}
impl From<Cdloc> for u8 {
    #[inline(always)]
    fn from(variant: Cdloc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cdloc {
    type Ux = u8;
}
impl crate::IsEnum for Cdloc {}
#[doc = "Field `CDLOC` reader - I/O Location for CD"]
pub type CdlocR = crate::FieldReader<Cdloc>;
impl CdlocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cdloc> {
        match self.bits {
            0 => Some(Cdloc::Loc0),
            1 => Some(Cdloc::Loc1),
            2 => Some(Cdloc::Loc2),
            3 => Some(Cdloc::Loc3),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Cdloc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Cdloc::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Cdloc::Loc2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == Cdloc::Loc3
    }
}
#[doc = "Field `CDLOC` writer - I/O Location for CD"]
pub type CdlocW<'a, REG> = crate::FieldWriter<'a, REG, 6, Cdloc>;
impl<'a, REG> CdlocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Cdloc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Cdloc::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Cdloc::Loc2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(Cdloc::Loc3)
    }
}
#[doc = "I/O Location for WP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wploc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
    #[doc = "3: Location 3"]
    Loc3 = 3,
}
impl From<Wploc> for u8 {
    #[inline(always)]
    fn from(variant: Wploc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wploc {
    type Ux = u8;
}
impl crate::IsEnum for Wploc {}
#[doc = "Field `WPLOC` reader - I/O Location for WP"]
pub type WplocR = crate::FieldReader<Wploc>;
impl WplocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wploc> {
        match self.bits {
            0 => Some(Wploc::Loc0),
            1 => Some(Wploc::Loc1),
            2 => Some(Wploc::Loc2),
            3 => Some(Wploc::Loc3),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Wploc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Wploc::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Wploc::Loc2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == Wploc::Loc3
    }
}
#[doc = "Field `WPLOC` writer - I/O Location for WP"]
pub type WplocW<'a, REG> = crate::FieldWriter<'a, REG, 6, Wploc>;
impl<'a, REG> WplocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Wploc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Wploc::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Wploc::Loc2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(Wploc::Loc3)
    }
}
#[doc = "I/O Location for CLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clkloc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
}
impl From<Clkloc> for u8 {
    #[inline(always)]
    fn from(variant: Clkloc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clkloc {
    type Ux = u8;
}
impl crate::IsEnum for Clkloc {}
#[doc = "Field `CLKLOC` reader - I/O Location for CLK"]
pub type ClklocR = crate::FieldReader<Clkloc>;
impl ClklocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clkloc> {
        match self.bits {
            0 => Some(Clkloc::Loc0),
            1 => Some(Clkloc::Loc1),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Clkloc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Clkloc::Loc1
    }
}
#[doc = "Field `CLKLOC` writer - I/O Location for CLK"]
pub type ClklocW<'a, REG> = crate::FieldWriter<'a, REG, 6, Clkloc>;
impl<'a, REG> ClklocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Clkloc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Clkloc::Loc1)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location for D0-7 Pins"]
    #[inline(always)]
    pub fn datloc(&self) -> DatlocR {
        DatlocR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location for CD"]
    #[inline(always)]
    pub fn cdloc(&self) -> CdlocR {
        CdlocR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location for WP"]
    #[inline(always)]
    pub fn wploc(&self) -> WplocR {
        WplocR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - I/O Location for CLK"]
    #[inline(always)]
    pub fn clkloc(&self) -> ClklocR {
        ClklocR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location for D0-7 Pins"]
    #[inline(always)]
    pub fn datloc(&mut self) -> DatlocW<'_, Routeloc0Spec> {
        DatlocW::new(self, 0)
    }
    #[doc = "Bits 8:13 - I/O Location for CD"]
    #[inline(always)]
    pub fn cdloc(&mut self) -> CdlocW<'_, Routeloc0Spec> {
        CdlocW::new(self, 8)
    }
    #[doc = "Bits 16:21 - I/O Location for WP"]
    #[inline(always)]
    pub fn wploc(&mut self) -> WplocW<'_, Routeloc0Spec> {
        WplocW::new(self, 16)
    }
    #[doc = "Bits 24:29 - I/O Location for CLK"]
    #[inline(always)]
    pub fn clkloc(&mut self) -> ClklocW<'_, Routeloc0Spec> {
        ClklocW::new(self, 24)
    }
}
#[doc = "I/O LOCATION Register\n\nYou can [`read`](crate::Reg::read) this register and get [`routeloc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routeloc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
