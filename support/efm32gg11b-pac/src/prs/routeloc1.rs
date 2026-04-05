#[doc = "Register `ROUTELOC1` reader"]
pub type R = crate::R<Routeloc1Spec>;
#[doc = "Register `ROUTELOC1` writer"]
pub type W = crate::W<Routeloc1Spec>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch4loc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
}
impl From<Ch4loc> for u8 {
    #[inline(always)]
    fn from(variant: Ch4loc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch4loc {
    type Ux = u8;
}
impl crate::IsEnum for Ch4loc {}
#[doc = "Field `CH4LOC` reader - I/O Location"]
pub type Ch4locR = crate::FieldReader<Ch4loc>;
impl Ch4locR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ch4loc> {
        match self.bits {
            0 => Some(Ch4loc::Loc0),
            1 => Some(Ch4loc::Loc1),
            2 => Some(Ch4loc::Loc2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Ch4loc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Ch4loc::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Ch4loc::Loc2
    }
}
#[doc = "Field `CH4LOC` writer - I/O Location"]
pub type Ch4locW<'a, REG> = crate::FieldWriter<'a, REG, 6, Ch4loc>;
impl<'a, REG> Ch4locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch4loc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch4loc::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch4loc::Loc2)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch5loc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
}
impl From<Ch5loc> for u8 {
    #[inline(always)]
    fn from(variant: Ch5loc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch5loc {
    type Ux = u8;
}
impl crate::IsEnum for Ch5loc {}
#[doc = "Field `CH5LOC` reader - I/O Location"]
pub type Ch5locR = crate::FieldReader<Ch5loc>;
impl Ch5locR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ch5loc> {
        match self.bits {
            0 => Some(Ch5loc::Loc0),
            1 => Some(Ch5loc::Loc1),
            2 => Some(Ch5loc::Loc2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Ch5loc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Ch5loc::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Ch5loc::Loc2
    }
}
#[doc = "Field `CH5LOC` writer - I/O Location"]
pub type Ch5locW<'a, REG> = crate::FieldWriter<'a, REG, 6, Ch5loc>;
impl<'a, REG> Ch5locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch5loc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch5loc::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch5loc::Loc2)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch6loc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
}
impl From<Ch6loc> for u8 {
    #[inline(always)]
    fn from(variant: Ch6loc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch6loc {
    type Ux = u8;
}
impl crate::IsEnum for Ch6loc {}
#[doc = "Field `CH6LOC` reader - I/O Location"]
pub type Ch6locR = crate::FieldReader<Ch6loc>;
impl Ch6locR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ch6loc> {
        match self.bits {
            0 => Some(Ch6loc::Loc0),
            1 => Some(Ch6loc::Loc1),
            2 => Some(Ch6loc::Loc2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Ch6loc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Ch6loc::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Ch6loc::Loc2
    }
}
#[doc = "Field `CH6LOC` writer - I/O Location"]
pub type Ch6locW<'a, REG> = crate::FieldWriter<'a, REG, 6, Ch6loc>;
impl<'a, REG> Ch6locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch6loc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch6loc::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch6loc::Loc2)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch7loc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
}
impl From<Ch7loc> for u8 {
    #[inline(always)]
    fn from(variant: Ch7loc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch7loc {
    type Ux = u8;
}
impl crate::IsEnum for Ch7loc {}
#[doc = "Field `CH7LOC` reader - I/O Location"]
pub type Ch7locR = crate::FieldReader<Ch7loc>;
impl Ch7locR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ch7loc> {
        match self.bits {
            0 => Some(Ch7loc::Loc0),
            1 => Some(Ch7loc::Loc1),
            2 => Some(Ch7loc::Loc2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Ch7loc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Ch7loc::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Ch7loc::Loc2
    }
}
#[doc = "Field `CH7LOC` writer - I/O Location"]
pub type Ch7locW<'a, REG> = crate::FieldWriter<'a, REG, 6, Ch7loc>;
impl<'a, REG> Ch7locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch7loc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch7loc::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch7loc::Loc2)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn ch4loc(&self) -> Ch4locR {
        Ch4locR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn ch5loc(&self) -> Ch5locR {
        Ch5locR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn ch6loc(&self) -> Ch6locR {
        Ch6locR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn ch7loc(&self) -> Ch7locR {
        Ch7locR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn ch4loc(&mut self) -> Ch4locW<'_, Routeloc1Spec> {
        Ch4locW::new(self, 0)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn ch5loc(&mut self) -> Ch5locW<'_, Routeloc1Spec> {
        Ch5locW::new(self, 8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn ch6loc(&mut self) -> Ch6locW<'_, Routeloc1Spec> {
        Ch6locW::new(self, 16)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn ch7loc(&mut self) -> Ch7locW<'_, Routeloc1Spec> {
        Ch7locW::new(self, 24)
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
