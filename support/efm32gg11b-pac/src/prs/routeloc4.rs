#[doc = "Register `ROUTELOC4` reader"]
pub type R = crate::R<Routeloc4Spec>;
#[doc = "Register `ROUTELOC4` writer"]
pub type W = crate::W<Routeloc4Spec>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch16loc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
}
impl From<Ch16loc> for u8 {
    #[inline(always)]
    fn from(variant: Ch16loc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch16loc {
    type Ux = u8;
}
impl crate::IsEnum for Ch16loc {}
#[doc = "Field `CH16LOC` reader - I/O Location"]
pub type Ch16locR = crate::FieldReader<Ch16loc>;
impl Ch16locR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ch16loc> {
        match self.bits {
            0 => Some(Ch16loc::Loc0),
            1 => Some(Ch16loc::Loc1),
            2 => Some(Ch16loc::Loc2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Ch16loc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Ch16loc::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Ch16loc::Loc2
    }
}
#[doc = "Field `CH16LOC` writer - I/O Location"]
pub type Ch16locW<'a, REG> = crate::FieldWriter<'a, REG, 6, Ch16loc>;
impl<'a, REG> Ch16locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch16loc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch16loc::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch16loc::Loc2)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch17loc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
}
impl From<Ch17loc> for u8 {
    #[inline(always)]
    fn from(variant: Ch17loc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch17loc {
    type Ux = u8;
}
impl crate::IsEnum for Ch17loc {}
#[doc = "Field `CH17LOC` reader - I/O Location"]
pub type Ch17locR = crate::FieldReader<Ch17loc>;
impl Ch17locR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ch17loc> {
        match self.bits {
            0 => Some(Ch17loc::Loc0),
            1 => Some(Ch17loc::Loc1),
            2 => Some(Ch17loc::Loc2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Ch17loc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Ch17loc::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Ch17loc::Loc2
    }
}
#[doc = "Field `CH17LOC` writer - I/O Location"]
pub type Ch17locW<'a, REG> = crate::FieldWriter<'a, REG, 6, Ch17loc>;
impl<'a, REG> Ch17locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch17loc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch17loc::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch17loc::Loc2)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch18loc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
}
impl From<Ch18loc> for u8 {
    #[inline(always)]
    fn from(variant: Ch18loc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch18loc {
    type Ux = u8;
}
impl crate::IsEnum for Ch18loc {}
#[doc = "Field `CH18LOC` reader - I/O Location"]
pub type Ch18locR = crate::FieldReader<Ch18loc>;
impl Ch18locR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ch18loc> {
        match self.bits {
            0 => Some(Ch18loc::Loc0),
            1 => Some(Ch18loc::Loc1),
            2 => Some(Ch18loc::Loc2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Ch18loc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Ch18loc::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Ch18loc::Loc2
    }
}
#[doc = "Field `CH18LOC` writer - I/O Location"]
pub type Ch18locW<'a, REG> = crate::FieldWriter<'a, REG, 6, Ch18loc>;
impl<'a, REG> Ch18locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch18loc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch18loc::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch18loc::Loc2)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch19loc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
}
impl From<Ch19loc> for u8 {
    #[inline(always)]
    fn from(variant: Ch19loc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch19loc {
    type Ux = u8;
}
impl crate::IsEnum for Ch19loc {}
#[doc = "Field `CH19LOC` reader - I/O Location"]
pub type Ch19locR = crate::FieldReader<Ch19loc>;
impl Ch19locR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ch19loc> {
        match self.bits {
            0 => Some(Ch19loc::Loc0),
            1 => Some(Ch19loc::Loc1),
            2 => Some(Ch19loc::Loc2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Ch19loc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Ch19loc::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Ch19loc::Loc2
    }
}
#[doc = "Field `CH19LOC` writer - I/O Location"]
pub type Ch19locW<'a, REG> = crate::FieldWriter<'a, REG, 6, Ch19loc>;
impl<'a, REG> Ch19locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch19loc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch19loc::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch19loc::Loc2)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn ch16loc(&self) -> Ch16locR {
        Ch16locR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn ch17loc(&self) -> Ch17locR {
        Ch17locR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn ch18loc(&self) -> Ch18locR {
        Ch18locR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn ch19loc(&self) -> Ch19locR {
        Ch19locR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn ch16loc(&mut self) -> Ch16locW<'_, Routeloc4Spec> {
        Ch16locW::new(self, 0)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn ch17loc(&mut self) -> Ch17locW<'_, Routeloc4Spec> {
        Ch17locW::new(self, 8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn ch18loc(&mut self) -> Ch18locW<'_, Routeloc4Spec> {
        Ch18locW::new(self, 16)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn ch19loc(&mut self) -> Ch19locW<'_, Routeloc4Spec> {
        Ch19locW::new(self, 24)
    }
}
#[doc = "I/O Routing Location Register\n\nYou can [`read`](crate::Reg::read) this register and get [`routeloc4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routeloc4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Routeloc4Spec;
impl crate::RegisterSpec for Routeloc4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`routeloc4::R`](R) reader structure"]
impl crate::Readable for Routeloc4Spec {}
#[doc = "`write(|w| ..)` method takes [`routeloc4::W`](W) writer structure"]
impl crate::Writable for Routeloc4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ROUTELOC4 to value 0"]
impl crate::Resettable for Routeloc4Spec {}
