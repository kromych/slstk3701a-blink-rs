#[doc = "Register `ST8_TCONFB` reader"]
pub type R = crate::R<ST8_TCONFB_SPEC>;
#[doc = "Register `ST8_TCONFB` writer"]
pub type W = crate::W<ST8_TCONFB_SPEC>;
#[doc = "Field `COMP` reader - Sensor Compare Value"]
pub type COMP_R = crate::FieldReader;
#[doc = "Field `COMP` writer - Sensor Compare Value"]
pub type COMP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MASK` reader - Sensor Mask"]
pub type MASK_R = crate::FieldReader;
#[doc = "Field `MASK` writer - Sensor Mask"]
pub type MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NEXTSTATE` reader - Next State Index"]
pub type NEXTSTATE_R = crate::FieldReader;
#[doc = "Field `NEXTSTATE` writer - Next State Index"]
pub type NEXTSTATE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SETIF` reader - Set Interrupt Flag"]
pub type SETIF_R = crate::BitReader;
#[doc = "Field `SETIF` writer - Set Interrupt Flag"]
pub type SETIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRSACT` reader - Configure Transition Action"]
pub type PRSACT_R = crate::FieldReader;
#[doc = "Field `PRSACT` writer - Configure Transition Action"]
pub type PRSACT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
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
    pub fn comp(&mut self) -> COMP_W<ST8_TCONFB_SPEC> {
        COMP_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Sensor Mask"]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MASK_W<ST8_TCONFB_SPEC> {
        MASK_W::new(self, 4)
    }
    #[doc = "Bits 8:12 - Next State Index"]
    #[inline(always)]
    #[must_use]
    pub fn nextstate(&mut self) -> NEXTSTATE_W<ST8_TCONFB_SPEC> {
        NEXTSTATE_W::new(self, 8)
    }
    #[doc = "Bit 15 - Set Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn setif(&mut self) -> SETIF_W<ST8_TCONFB_SPEC> {
        SETIF_W::new(self, 15)
    }
    #[doc = "Bits 16:18 - Configure Transition Action"]
    #[inline(always)]
    #[must_use]
    pub fn prsact(&mut self) -> PRSACT_W<ST8_TCONFB_SPEC> {
        PRSACT_W::new(self, 16)
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
#[doc = "State Transition Configuration B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st8_tconfb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st8_tconfb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ST8_TCONFB_SPEC;
impl crate::RegisterSpec for ST8_TCONFB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st8_tconfb::R`](R) reader structure"]
impl crate::Readable for ST8_TCONFB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`st8_tconfb::W`](W) writer structure"]
impl crate::Writable for ST8_TCONFB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ST8_TCONFB to value 0"]
impl crate::Resettable for ST8_TCONFB_SPEC {
    const RESET_VALUE: u32 = 0;
}
