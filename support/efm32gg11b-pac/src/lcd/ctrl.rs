#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `EN` reader - LCD Enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - LCD Enable"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDCTRL` reader - Update Data Control"]
pub type UDCTRL_R = crate::FieldReader<UDCTRL_A>;
#[doc = "Update Data Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UDCTRL_A {
    #[doc = "0: The data transfer is controlled by SW. Transfer is performed as soon as possible"]
    REGULAR = 0,
    #[doc = "1: The data transfer is done at the next event triggered by the Frame Counter"]
    FCEVENT = 1,
    #[doc = "2: The data transfer is done continuously at every LCD frame start"]
    FRAMESTART = 2,
}
impl From<UDCTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: UDCTRL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UDCTRL_A {
    type Ux = u8;
}
impl UDCTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<UDCTRL_A> {
        match self.bits {
            0 => Some(UDCTRL_A::REGULAR),
            1 => Some(UDCTRL_A::FCEVENT),
            2 => Some(UDCTRL_A::FRAMESTART),
            _ => None,
        }
    }
    #[doc = "The data transfer is controlled by SW. Transfer is performed as soon as possible"]
    #[inline(always)]
    pub fn is_regular(&self) -> bool {
        *self == UDCTRL_A::REGULAR
    }
    #[doc = "The data transfer is done at the next event triggered by the Frame Counter"]
    #[inline(always)]
    pub fn is_fcevent(&self) -> bool {
        *self == UDCTRL_A::FCEVENT
    }
    #[doc = "The data transfer is done continuously at every LCD frame start"]
    #[inline(always)]
    pub fn is_framestart(&self) -> bool {
        *self == UDCTRL_A::FRAMESTART
    }
}
#[doc = "Field `UDCTRL` writer - Update Data Control"]
pub type UDCTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, UDCTRL_A>;
impl<'a, REG> UDCTRL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The data transfer is controlled by SW. Transfer is performed as soon as possible"]
    #[inline(always)]
    pub fn regular(self) -> &'a mut crate::W<REG> {
        self.variant(UDCTRL_A::REGULAR)
    }
    #[doc = "The data transfer is done at the next event triggered by the Frame Counter"]
    #[inline(always)]
    pub fn fcevent(self) -> &'a mut crate::W<REG> {
        self.variant(UDCTRL_A::FCEVENT)
    }
    #[doc = "The data transfer is done continuously at every LCD frame start"]
    #[inline(always)]
    pub fn framestart(self) -> &'a mut crate::W<REG> {
        self.variant(UDCTRL_A::FRAMESTART)
    }
}
#[doc = "Field `DSC` reader - Direct Segment Control"]
pub type DSC_R = crate::BitReader;
#[doc = "Field `DSC` writer - Direct Segment Control"]
pub type DSC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LCD Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Update Data Control"]
    #[inline(always)]
    pub fn udctrl(&self) -> UDCTRL_R {
        UDCTRL_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 23 - Direct Segment Control"]
    #[inline(always)]
    pub fn dsc(&self) -> DSC_R {
        DSC_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CTRL_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Update Data Control"]
    #[inline(always)]
    #[must_use]
    pub fn udctrl(&mut self) -> UDCTRL_W<CTRL_SPEC> {
        UDCTRL_W::new(self, 1)
    }
    #[doc = "Bit 23 - Direct Segment Control"]
    #[inline(always)]
    #[must_use]
    pub fn dsc(&mut self) -> DSC_W<CTRL_SPEC> {
        DSC_W::new(self, 23)
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
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
