#[doc = "Register `ROUTELOC0` reader"]
pub type R = crate::R<Routeloc0Spec>;
#[doc = "Register `ROUTELOC0` writer"]
pub type W = crate::W<Routeloc0Spec>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Qspiloc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
}
impl From<Qspiloc> for u8 {
    #[inline(always)]
    fn from(variant: Qspiloc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Qspiloc {
    type Ux = u8;
}
impl crate::IsEnum for Qspiloc {}
#[doc = "Field `QSPILOC` reader - I/O Location"]
pub type QspilocR = crate::FieldReader<Qspiloc>;
impl QspilocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Qspiloc> {
        match self.bits {
            0 => Some(Qspiloc::Loc0),
            1 => Some(Qspiloc::Loc1),
            2 => Some(Qspiloc::Loc2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Qspiloc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Qspiloc::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Qspiloc::Loc2
    }
}
#[doc = "Field `QSPILOC` writer - I/O Location"]
pub type QspilocW<'a, REG> = crate::FieldWriter<'a, REG, 6, Qspiloc>;
impl<'a, REG> QspilocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Qspiloc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Qspiloc::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Qspiloc::Loc2)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn qspiloc(&self) -> QspilocR {
        QspilocR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn qspiloc(&mut self) -> QspilocW<'_, Routeloc0Spec> {
        QspilocW::new(self, 0)
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
