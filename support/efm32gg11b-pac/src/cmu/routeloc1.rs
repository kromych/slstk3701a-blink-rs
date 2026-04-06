#[doc = "Register `ROUTELOC1` reader"]
pub type R = crate::R<Routeloc1Spec>;
#[doc = "Register `ROUTELOC1` writer"]
pub type W = crate::W<Routeloc1Spec>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clkin0loc {
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
    #[doc = "7: Location 7"]
    Loc7 = 7,
}
impl From<Clkin0loc> for u8 {
    #[inline(always)]
    fn from(variant: Clkin0loc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clkin0loc {
    type Ux = u8;
}
impl crate::IsEnum for Clkin0loc {}
#[doc = "Field `CLKIN0LOC` reader - I/O Location"]
pub type Clkin0locR = crate::FieldReader<Clkin0loc>;
impl Clkin0locR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clkin0loc> {
        match self.bits {
            0 => Some(Clkin0loc::Loc0),
            1 => Some(Clkin0loc::Loc1),
            2 => Some(Clkin0loc::Loc2),
            3 => Some(Clkin0loc::Loc3),
            4 => Some(Clkin0loc::Loc4),
            5 => Some(Clkin0loc::Loc5),
            6 => Some(Clkin0loc::Loc6),
            7 => Some(Clkin0loc::Loc7),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Clkin0loc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Clkin0loc::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Clkin0loc::Loc2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == Clkin0loc::Loc3
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == Clkin0loc::Loc4
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == Clkin0loc::Loc5
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == Clkin0loc::Loc6
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == Clkin0loc::Loc7
    }
}
#[doc = "Field `CLKIN0LOC` writer - I/O Location"]
pub type Clkin0locW<'a, REG> = crate::FieldWriter<'a, REG, 6, Clkin0loc>;
impl<'a, REG> Clkin0locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Clkin0loc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Clkin0loc::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Clkin0loc::Loc2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(Clkin0loc::Loc3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(Clkin0loc::Loc4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(Clkin0loc::Loc5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut crate::W<REG> {
        self.variant(Clkin0loc::Loc6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut crate::W<REG> {
        self.variant(Clkin0loc::Loc7)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn clkin0loc(&self) -> Clkin0locR {
        Clkin0locR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn clkin0loc(&mut self) -> Clkin0locW<'_, Routeloc1Spec> {
        Clkin0locW::new(self, 0)
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
