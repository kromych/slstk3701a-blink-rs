#[doc = "Register `ROUTELOC2` reader"]
pub type R = crate::R<Routeloc2Spec>;
#[doc = "Register `ROUTELOC2` writer"]
pub type W = crate::W<Routeloc2Spec>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch8loc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
}
impl From<Ch8loc> for u8 {
    #[inline(always)]
    fn from(variant: Ch8loc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch8loc {
    type Ux = u8;
}
impl crate::IsEnum for Ch8loc {}
#[doc = "Field `CH8LOC` reader - I/O Location"]
pub type Ch8locR = crate::FieldReader<Ch8loc>;
impl Ch8locR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ch8loc> {
        match self.bits {
            0 => Some(Ch8loc::Loc0),
            1 => Some(Ch8loc::Loc1),
            2 => Some(Ch8loc::Loc2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Ch8loc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Ch8loc::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Ch8loc::Loc2
    }
}
#[doc = "Field `CH8LOC` writer - I/O Location"]
pub type Ch8locW<'a, REG> = crate::FieldWriter<'a, REG, 6, Ch8loc>;
impl<'a, REG> Ch8locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch8loc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch8loc::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch8loc::Loc2)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch9loc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
}
impl From<Ch9loc> for u8 {
    #[inline(always)]
    fn from(variant: Ch9loc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch9loc {
    type Ux = u8;
}
impl crate::IsEnum for Ch9loc {}
#[doc = "Field `CH9LOC` reader - I/O Location"]
pub type Ch9locR = crate::FieldReader<Ch9loc>;
impl Ch9locR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ch9loc> {
        match self.bits {
            0 => Some(Ch9loc::Loc0),
            1 => Some(Ch9loc::Loc1),
            2 => Some(Ch9loc::Loc2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Ch9loc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Ch9loc::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Ch9loc::Loc2
    }
}
#[doc = "Field `CH9LOC` writer - I/O Location"]
pub type Ch9locW<'a, REG> = crate::FieldWriter<'a, REG, 6, Ch9loc>;
impl<'a, REG> Ch9locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch9loc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch9loc::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch9loc::Loc2)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch10loc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
}
impl From<Ch10loc> for u8 {
    #[inline(always)]
    fn from(variant: Ch10loc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch10loc {
    type Ux = u8;
}
impl crate::IsEnum for Ch10loc {}
#[doc = "Field `CH10LOC` reader - I/O Location"]
pub type Ch10locR = crate::FieldReader<Ch10loc>;
impl Ch10locR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ch10loc> {
        match self.bits {
            0 => Some(Ch10loc::Loc0),
            1 => Some(Ch10loc::Loc1),
            2 => Some(Ch10loc::Loc2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Ch10loc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Ch10loc::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Ch10loc::Loc2
    }
}
#[doc = "Field `CH10LOC` writer - I/O Location"]
pub type Ch10locW<'a, REG> = crate::FieldWriter<'a, REG, 6, Ch10loc>;
impl<'a, REG> Ch10locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch10loc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch10loc::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch10loc::Loc2)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch11loc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
}
impl From<Ch11loc> for u8 {
    #[inline(always)]
    fn from(variant: Ch11loc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch11loc {
    type Ux = u8;
}
impl crate::IsEnum for Ch11loc {}
#[doc = "Field `CH11LOC` reader - I/O Location"]
pub type Ch11locR = crate::FieldReader<Ch11loc>;
impl Ch11locR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ch11loc> {
        match self.bits {
            0 => Some(Ch11loc::Loc0),
            1 => Some(Ch11loc::Loc1),
            2 => Some(Ch11loc::Loc2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Ch11loc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Ch11loc::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Ch11loc::Loc2
    }
}
#[doc = "Field `CH11LOC` writer - I/O Location"]
pub type Ch11locW<'a, REG> = crate::FieldWriter<'a, REG, 6, Ch11loc>;
impl<'a, REG> Ch11locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch11loc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch11loc::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch11loc::Loc2)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn ch8loc(&self) -> Ch8locR {
        Ch8locR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn ch9loc(&self) -> Ch9locR {
        Ch9locR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn ch10loc(&self) -> Ch10locR {
        Ch10locR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn ch11loc(&self) -> Ch11locR {
        Ch11locR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn ch8loc(&mut self) -> Ch8locW<'_, Routeloc2Spec> {
        Ch8locW::new(self, 0)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn ch9loc(&mut self) -> Ch9locW<'_, Routeloc2Spec> {
        Ch9locW::new(self, 8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn ch10loc(&mut self) -> Ch10locW<'_, Routeloc2Spec> {
        Ch10locW::new(self, 16)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn ch11loc(&mut self) -> Ch11locW<'_, Routeloc2Spec> {
        Ch11locW::new(self, 24)
    }
}
#[doc = "I/O Routing Location Register\n\nYou can [`read`](crate::Reg::read) this register and get [`routeloc2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routeloc2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Routeloc2Spec;
impl crate::RegisterSpec for Routeloc2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`routeloc2::R`](R) reader structure"]
impl crate::Readable for Routeloc2Spec {}
#[doc = "`write(|w| ..)` method takes [`routeloc2::W`](W) writer structure"]
impl crate::Writable for Routeloc2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ROUTELOC2 to value 0"]
impl crate::Resettable for Routeloc2Spec {}
