#[doc = "Register `ROUTELOC1` reader"]
pub type R = crate::R<ROUTELOC1_SPEC>;
#[doc = "Register `ROUTELOC1` writer"]
pub type W = crate::W<ROUTELOC1_SPEC>;
#[doc = "Field `CLKIN0LOC` reader - I/O Location"]
pub type CLKIN0LOC_R = crate::FieldReader<CLKIN0LOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKIN0LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
    #[doc = "3: Location 3"]
    LOC3 = 3,
    #[doc = "4: Location 4"]
    LOC4 = 4,
    #[doc = "5: Location 5"]
    LOC5 = 5,
    #[doc = "6: Location 6"]
    LOC6 = 6,
    #[doc = "7: Location 7"]
    LOC7 = 7,
}
impl From<CLKIN0LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKIN0LOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLKIN0LOC_A {
    type Ux = u8;
}
impl CLKIN0LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLKIN0LOC_A> {
        match self.bits {
            0 => Some(CLKIN0LOC_A::LOC0),
            1 => Some(CLKIN0LOC_A::LOC1),
            2 => Some(CLKIN0LOC_A::LOC2),
            3 => Some(CLKIN0LOC_A::LOC3),
            4 => Some(CLKIN0LOC_A::LOC4),
            5 => Some(CLKIN0LOC_A::LOC5),
            6 => Some(CLKIN0LOC_A::LOC6),
            7 => Some(CLKIN0LOC_A::LOC7),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CLKIN0LOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CLKIN0LOC_A::LOC1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CLKIN0LOC_A::LOC2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CLKIN0LOC_A::LOC3
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CLKIN0LOC_A::LOC4
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == CLKIN0LOC_A::LOC5
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == CLKIN0LOC_A::LOC6
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == CLKIN0LOC_A::LOC7
    }
}
#[doc = "Field `CLKIN0LOC` writer - I/O Location"]
pub type CLKIN0LOC_W<'a, REG> = crate::FieldWriter<'a, REG, 6, CLKIN0LOC_A>;
impl<'a, REG> CLKIN0LOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CLKIN0LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CLKIN0LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CLKIN0LOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(CLKIN0LOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(CLKIN0LOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(CLKIN0LOC_A::LOC5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut crate::W<REG> {
        self.variant(CLKIN0LOC_A::LOC6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut crate::W<REG> {
        self.variant(CLKIN0LOC_A::LOC7)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn clkin0loc(&self) -> CLKIN0LOC_R {
        CLKIN0LOC_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn clkin0loc(&mut self) -> CLKIN0LOC_W<ROUTELOC1_SPEC> {
        CLKIN0LOC_W::new(self, 0)
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
#[doc = "I/O Routing Location Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routeloc1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routeloc1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
