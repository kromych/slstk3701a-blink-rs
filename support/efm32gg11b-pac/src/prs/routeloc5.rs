#[doc = "Register `ROUTELOC5` reader"]
pub type R = crate::R<Routeloc5Spec>;
#[doc = "Register `ROUTELOC5` writer"]
pub type W = crate::W<Routeloc5Spec>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch20loc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
}
impl From<Ch20loc> for u8 {
    #[inline(always)]
    fn from(variant: Ch20loc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch20loc {
    type Ux = u8;
}
impl crate::IsEnum for Ch20loc {}
#[doc = "Field `CH20LOC` reader - I/O Location"]
pub type Ch20locR = crate::FieldReader<Ch20loc>;
impl Ch20locR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ch20loc> {
        match self.bits {
            0 => Some(Ch20loc::Loc0),
            1 => Some(Ch20loc::Loc1),
            2 => Some(Ch20loc::Loc2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Ch20loc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Ch20loc::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Ch20loc::Loc2
    }
}
#[doc = "Field `CH20LOC` writer - I/O Location"]
pub type Ch20locW<'a, REG> = crate::FieldWriter<'a, REG, 6, Ch20loc>;
impl<'a, REG> Ch20locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch20loc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch20loc::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch20loc::Loc2)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch21loc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
}
impl From<Ch21loc> for u8 {
    #[inline(always)]
    fn from(variant: Ch21loc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch21loc {
    type Ux = u8;
}
impl crate::IsEnum for Ch21loc {}
#[doc = "Field `CH21LOC` reader - I/O Location"]
pub type Ch21locR = crate::FieldReader<Ch21loc>;
impl Ch21locR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ch21loc> {
        match self.bits {
            0 => Some(Ch21loc::Loc0),
            1 => Some(Ch21loc::Loc1),
            2 => Some(Ch21loc::Loc2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Ch21loc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Ch21loc::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Ch21loc::Loc2
    }
}
#[doc = "Field `CH21LOC` writer - I/O Location"]
pub type Ch21locW<'a, REG> = crate::FieldWriter<'a, REG, 6, Ch21loc>;
impl<'a, REG> Ch21locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch21loc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch21loc::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch21loc::Loc2)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch22loc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
}
impl From<Ch22loc> for u8 {
    #[inline(always)]
    fn from(variant: Ch22loc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch22loc {
    type Ux = u8;
}
impl crate::IsEnum for Ch22loc {}
#[doc = "Field `CH22LOC` reader - I/O Location"]
pub type Ch22locR = crate::FieldReader<Ch22loc>;
impl Ch22locR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ch22loc> {
        match self.bits {
            0 => Some(Ch22loc::Loc0),
            1 => Some(Ch22loc::Loc1),
            2 => Some(Ch22loc::Loc2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Ch22loc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Ch22loc::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Ch22loc::Loc2
    }
}
#[doc = "Field `CH22LOC` writer - I/O Location"]
pub type Ch22locW<'a, REG> = crate::FieldWriter<'a, REG, 6, Ch22loc>;
impl<'a, REG> Ch22locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch22loc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch22loc::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch22loc::Loc2)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch23loc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
}
impl From<Ch23loc> for u8 {
    #[inline(always)]
    fn from(variant: Ch23loc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch23loc {
    type Ux = u8;
}
impl crate::IsEnum for Ch23loc {}
#[doc = "Field `CH23LOC` reader - I/O Location"]
pub type Ch23locR = crate::FieldReader<Ch23loc>;
impl Ch23locR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ch23loc> {
        match self.bits {
            0 => Some(Ch23loc::Loc0),
            1 => Some(Ch23loc::Loc1),
            2 => Some(Ch23loc::Loc2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Ch23loc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Ch23loc::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Ch23loc::Loc2
    }
}
#[doc = "Field `CH23LOC` writer - I/O Location"]
pub type Ch23locW<'a, REG> = crate::FieldWriter<'a, REG, 6, Ch23loc>;
impl<'a, REG> Ch23locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch23loc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch23loc::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch23loc::Loc2)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn ch20loc(&self) -> Ch20locR {
        Ch20locR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn ch21loc(&self) -> Ch21locR {
        Ch21locR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn ch22loc(&self) -> Ch22locR {
        Ch22locR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn ch23loc(&self) -> Ch23locR {
        Ch23locR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn ch20loc(&mut self) -> Ch20locW<'_, Routeloc5Spec> {
        Ch20locW::new(self, 0)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn ch21loc(&mut self) -> Ch21locW<'_, Routeloc5Spec> {
        Ch21locW::new(self, 8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn ch22loc(&mut self) -> Ch22locW<'_, Routeloc5Spec> {
        Ch22locW::new(self, 16)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn ch23loc(&mut self) -> Ch23locW<'_, Routeloc5Spec> {
        Ch23locW::new(self, 24)
    }
}
#[doc = "I/O Routing Location Register\n\nYou can [`read`](crate::Reg::read) this register and get [`routeloc5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routeloc5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Routeloc5Spec;
impl crate::RegisterSpec for Routeloc5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`routeloc5::R`](R) reader structure"]
impl crate::Readable for Routeloc5Spec {}
#[doc = "`write(|w| ..)` method takes [`routeloc5::W`](W) writer structure"]
impl crate::Writable for Routeloc5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ROUTELOC5 to value 0"]
impl crate::Resettable for Routeloc5Spec {}
