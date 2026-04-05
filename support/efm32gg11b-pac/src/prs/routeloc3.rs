#[doc = "Register `ROUTELOC3` reader"]
pub type R = crate::R<Routeloc3Spec>;
#[doc = "Register `ROUTELOC3` writer"]
pub type W = crate::W<Routeloc3Spec>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch12loc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
}
impl From<Ch12loc> for u8 {
    #[inline(always)]
    fn from(variant: Ch12loc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch12loc {
    type Ux = u8;
}
impl crate::IsEnum for Ch12loc {}
#[doc = "Field `CH12LOC` reader - I/O Location"]
pub type Ch12locR = crate::FieldReader<Ch12loc>;
impl Ch12locR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ch12loc> {
        match self.bits {
            0 => Some(Ch12loc::Loc0),
            1 => Some(Ch12loc::Loc1),
            2 => Some(Ch12loc::Loc2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Ch12loc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Ch12loc::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Ch12loc::Loc2
    }
}
#[doc = "Field `CH12LOC` writer - I/O Location"]
pub type Ch12locW<'a, REG> = crate::FieldWriter<'a, REG, 6, Ch12loc>;
impl<'a, REG> Ch12locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch12loc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch12loc::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch12loc::Loc2)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch13loc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
}
impl From<Ch13loc> for u8 {
    #[inline(always)]
    fn from(variant: Ch13loc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch13loc {
    type Ux = u8;
}
impl crate::IsEnum for Ch13loc {}
#[doc = "Field `CH13LOC` reader - I/O Location"]
pub type Ch13locR = crate::FieldReader<Ch13loc>;
impl Ch13locR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ch13loc> {
        match self.bits {
            0 => Some(Ch13loc::Loc0),
            1 => Some(Ch13loc::Loc1),
            2 => Some(Ch13loc::Loc2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Ch13loc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Ch13loc::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Ch13loc::Loc2
    }
}
#[doc = "Field `CH13LOC` writer - I/O Location"]
pub type Ch13locW<'a, REG> = crate::FieldWriter<'a, REG, 6, Ch13loc>;
impl<'a, REG> Ch13locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch13loc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch13loc::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch13loc::Loc2)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch14loc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
}
impl From<Ch14loc> for u8 {
    #[inline(always)]
    fn from(variant: Ch14loc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch14loc {
    type Ux = u8;
}
impl crate::IsEnum for Ch14loc {}
#[doc = "Field `CH14LOC` reader - I/O Location"]
pub type Ch14locR = crate::FieldReader<Ch14loc>;
impl Ch14locR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ch14loc> {
        match self.bits {
            0 => Some(Ch14loc::Loc0),
            1 => Some(Ch14loc::Loc1),
            2 => Some(Ch14loc::Loc2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Ch14loc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Ch14loc::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Ch14loc::Loc2
    }
}
#[doc = "Field `CH14LOC` writer - I/O Location"]
pub type Ch14locW<'a, REG> = crate::FieldWriter<'a, REG, 6, Ch14loc>;
impl<'a, REG> Ch14locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch14loc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch14loc::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch14loc::Loc2)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch15loc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
}
impl From<Ch15loc> for u8 {
    #[inline(always)]
    fn from(variant: Ch15loc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch15loc {
    type Ux = u8;
}
impl crate::IsEnum for Ch15loc {}
#[doc = "Field `CH15LOC` reader - I/O Location"]
pub type Ch15locR = crate::FieldReader<Ch15loc>;
impl Ch15locR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ch15loc> {
        match self.bits {
            0 => Some(Ch15loc::Loc0),
            1 => Some(Ch15loc::Loc1),
            2 => Some(Ch15loc::Loc2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Ch15loc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Ch15loc::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Ch15loc::Loc2
    }
}
#[doc = "Field `CH15LOC` writer - I/O Location"]
pub type Ch15locW<'a, REG> = crate::FieldWriter<'a, REG, 6, Ch15loc>;
impl<'a, REG> Ch15locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch15loc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch15loc::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch15loc::Loc2)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn ch12loc(&self) -> Ch12locR {
        Ch12locR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn ch13loc(&self) -> Ch13locR {
        Ch13locR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn ch14loc(&self) -> Ch14locR {
        Ch14locR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn ch15loc(&self) -> Ch15locR {
        Ch15locR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn ch12loc(&mut self) -> Ch12locW<'_, Routeloc3Spec> {
        Ch12locW::new(self, 0)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn ch13loc(&mut self) -> Ch13locW<'_, Routeloc3Spec> {
        Ch13locW::new(self, 8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn ch14loc(&mut self) -> Ch14locW<'_, Routeloc3Spec> {
        Ch14locW::new(self, 16)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn ch15loc(&mut self) -> Ch15locW<'_, Routeloc3Spec> {
        Ch15locW::new(self, 24)
    }
}
#[doc = "I/O Routing Location Register\n\nYou can [`read`](crate::Reg::read) this register and get [`routeloc3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routeloc3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Routeloc3Spec;
impl crate::RegisterSpec for Routeloc3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`routeloc3::R`](R) reader structure"]
impl crate::Readable for Routeloc3Spec {}
#[doc = "`write(|w| ..)` method takes [`routeloc3::W`](W) writer structure"]
impl crate::Writable for Routeloc3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ROUTELOC3 to value 0"]
impl crate::Resettable for Routeloc3Spec {}
