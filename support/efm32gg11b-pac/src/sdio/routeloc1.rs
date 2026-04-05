#[doc = "Register `ROUTELOC1` reader"]
pub type R = crate::R<Routeloc1Spec>;
#[doc = "Register `ROUTELOC1` writer"]
pub type W = crate::W<Routeloc1Spec>;
#[doc = "I/O Location for CMD Pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmdloc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
}
impl From<Cmdloc> for u8 {
    #[inline(always)]
    fn from(variant: Cmdloc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmdloc {
    type Ux = u8;
}
impl crate::IsEnum for Cmdloc {}
#[doc = "Field `CMDLOC` reader - I/O Location for CMD Pin"]
pub type CmdlocR = crate::FieldReader<Cmdloc>;
impl CmdlocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cmdloc> {
        match self.bits {
            0 => Some(Cmdloc::Loc0),
            1 => Some(Cmdloc::Loc1),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Cmdloc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Cmdloc::Loc1
    }
}
#[doc = "Field `CMDLOC` writer - I/O Location for CMD Pin"]
pub type CmdlocW<'a, REG> = crate::FieldWriter<'a, REG, 6, Cmdloc>;
impl<'a, REG> CmdlocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdloc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdloc::Loc1)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location for CMD Pin"]
    #[inline(always)]
    pub fn cmdloc(&self) -> CmdlocR {
        CmdlocR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location for CMD Pin"]
    #[inline(always)]
    pub fn cmdloc(&mut self) -> CmdlocW<'_, Routeloc1Spec> {
        CmdlocW::new(self, 0)
    }
}
#[doc = "I/O LOCATION Register\n\nYou can [`read`](crate::Reg::read) this register and get [`routeloc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routeloc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
