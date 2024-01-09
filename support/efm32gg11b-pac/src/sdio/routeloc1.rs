#[doc = "Register `ROUTELOC1` reader"]
pub type R = crate::R<ROUTELOC1_SPEC>;
#[doc = "Register `ROUTELOC1` writer"]
pub type W = crate::W<ROUTELOC1_SPEC>;
#[doc = "Field `CMDLOC` reader - I/O Location for CMD Pin"]
pub type CMDLOC_R = crate::FieldReader<CMDLOC_A>;
#[doc = "I/O Location for CMD Pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMDLOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
}
impl From<CMDLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CMDLOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMDLOC_A {
    type Ux = u8;
}
impl CMDLOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CMDLOC_A> {
        match self.bits {
            0 => Some(CMDLOC_A::LOC0),
            1 => Some(CMDLOC_A::LOC1),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CMDLOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CMDLOC_A::LOC1
    }
}
#[doc = "Field `CMDLOC` writer - I/O Location for CMD Pin"]
pub type CMDLOC_W<'a, REG> = crate::FieldWriter<'a, REG, 6, CMDLOC_A>;
impl<'a, REG> CMDLOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CMDLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CMDLOC_A::LOC1)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location for CMD Pin"]
    #[inline(always)]
    pub fn cmdloc(&self) -> CMDLOC_R {
        CMDLOC_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location for CMD Pin"]
    #[inline(always)]
    #[must_use]
    pub fn cmdloc(&mut self) -> CMDLOC_W<ROUTELOC1_SPEC> {
        CMDLOC_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "I/O LOCATION Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routeloc1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routeloc1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROUTELOC1_SPEC;
impl crate::RegisterSpec for ROUTELOC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`routeloc1::R`](R) reader structure"]
impl crate::Readable for ROUTELOC1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`routeloc1::W`](W) writer structure"]
impl crate::Writable for ROUTELOC1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ROUTELOC1 to value 0"]
impl crate::Resettable for ROUTELOC1_SPEC {
    const RESET_VALUE: u32 = 0;
}
