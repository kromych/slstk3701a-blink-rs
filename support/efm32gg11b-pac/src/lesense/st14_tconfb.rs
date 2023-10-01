#[doc = "Register `ST14_TCONFB` reader"]
pub type R = crate::R<ST14_TCONFB_SPEC>;
#[doc = "Register `ST14_TCONFB` writer"]
pub type W = crate::W<ST14_TCONFB_SPEC>;
#[doc = "Field `COMP` reader - Sensor Compare Value"]
pub type COMP_R = crate::FieldReader;
#[doc = "Field `COMP` writer - Sensor Compare Value"]
pub type COMP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `MASK` reader - Sensor Mask"]
pub type MASK_R = crate::FieldReader;
#[doc = "Field `MASK` writer - Sensor Mask"]
pub type MASK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `NEXTSTATE` reader - Next State Index"]
pub type NEXTSTATE_R = crate::FieldReader;
#[doc = "Field `NEXTSTATE` writer - Next State Index"]
pub type NEXTSTATE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `SETIF` reader - Set Interrupt Flag"]
pub type SETIF_R = crate::BitReader;
#[doc = "Field `SETIF` writer - Set Interrupt Flag"]
pub type SETIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PRSACT` reader - Configure Transition Action"]
pub type PRSACT_R = crate::FieldReader;
#[doc = "Field `PRSACT` writer - Configure Transition Action"]
pub type PRSACT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:3 - Sensor Compare Value"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Sensor Mask"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Next State Index"]
    #[inline(always)]
    pub fn nextstate(&self) -> NEXTSTATE_R {
        NEXTSTATE_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Set Interrupt Flag"]
    #[inline(always)]
    pub fn setif(&self) -> SETIF_R {
        SETIF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Configure Transition Action"]
    #[inline(always)]
    pub fn prsact(&self) -> PRSACT_R {
        PRSACT_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Sensor Compare Value"]
    #[inline(always)]
    #[must_use]
    pub fn comp(&mut self) -> COMP_W<ST14_TCONFB_SPEC, 0> {
        COMP_W::new(self)
    }
    #[doc = "Bits 4:7 - Sensor Mask"]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MASK_W<ST14_TCONFB_SPEC, 4> {
        MASK_W::new(self)
    }
    #[doc = "Bits 8:12 - Next State Index"]
    #[inline(always)]
    #[must_use]
    pub fn nextstate(&mut self) -> NEXTSTATE_W<ST14_TCONFB_SPEC, 8> {
        NEXTSTATE_W::new(self)
    }
    #[doc = "Bit 15 - Set Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn setif(&mut self) -> SETIF_W<ST14_TCONFB_SPEC, 15> {
        SETIF_W::new(self)
    }
    #[doc = "Bits 16:18 - Configure Transition Action"]
    #[inline(always)]
    #[must_use]
    pub fn prsact(&mut self) -> PRSACT_W<ST14_TCONFB_SPEC, 16> {
        PRSACT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "State Transition Configuration B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st14_tconfb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st14_tconfb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ST14_TCONFB_SPEC;
impl crate::RegisterSpec for ST14_TCONFB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st14_tconfb::R`](R) reader structure"]
impl crate::Readable for ST14_TCONFB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`st14_tconfb::W`](W) writer structure"]
impl crate::Writable for ST14_TCONFB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ST14_TCONFB to value 0"]
impl crate::Resettable for ST14_TCONFB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
