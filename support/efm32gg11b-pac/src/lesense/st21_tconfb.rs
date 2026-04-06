#[doc = "Register `ST21_TCONFB` reader"]
pub type R = crate::R<St21TconfbSpec>;
#[doc = "Register `ST21_TCONFB` writer"]
pub type W = crate::W<St21TconfbSpec>;
#[doc = "Field `COMP` reader - Sensor Compare Value"]
pub type CompR = crate::FieldReader;
#[doc = "Field `COMP` writer - Sensor Compare Value"]
pub type CompW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MASK` reader - Sensor Mask"]
pub type MaskR = crate::FieldReader;
#[doc = "Field `MASK` writer - Sensor Mask"]
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NEXTSTATE` reader - Next State Index"]
pub type NextstateR = crate::FieldReader;
#[doc = "Field `NEXTSTATE` writer - Next State Index"]
pub type NextstateW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SETIF` reader - Set Interrupt Flag"]
pub type SetifR = crate::BitReader;
#[doc = "Field `SETIF` writer - Set Interrupt Flag"]
pub type SetifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRSACT` reader - Configure Transition Action"]
pub type PrsactR = crate::FieldReader;
#[doc = "Field `PRSACT` writer - Configure Transition Action"]
pub type PrsactW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - Sensor Compare Value"]
    #[inline(always)]
    pub fn comp(&self) -> CompR {
        CompR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Sensor Mask"]
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Next State Index"]
    #[inline(always)]
    pub fn nextstate(&self) -> NextstateR {
        NextstateR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Set Interrupt Flag"]
    #[inline(always)]
    pub fn setif(&self) -> SetifR {
        SetifR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Configure Transition Action"]
    #[inline(always)]
    pub fn prsact(&self) -> PrsactR {
        PrsactR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Sensor Compare Value"]
    #[inline(always)]
    pub fn comp(&mut self) -> CompW<'_, St21TconfbSpec> {
        CompW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Sensor Mask"]
    #[inline(always)]
    pub fn mask(&mut self) -> MaskW<'_, St21TconfbSpec> {
        MaskW::new(self, 4)
    }
    #[doc = "Bits 8:12 - Next State Index"]
    #[inline(always)]
    pub fn nextstate(&mut self) -> NextstateW<'_, St21TconfbSpec> {
        NextstateW::new(self, 8)
    }
    #[doc = "Bit 15 - Set Interrupt Flag"]
    #[inline(always)]
    pub fn setif(&mut self) -> SetifW<'_, St21TconfbSpec> {
        SetifW::new(self, 15)
    }
    #[doc = "Bits 16:18 - Configure Transition Action"]
    #[inline(always)]
    pub fn prsact(&mut self) -> PrsactW<'_, St21TconfbSpec> {
        PrsactW::new(self, 16)
    }
}
#[doc = "State Transition Configuration B\n\nYou can [`read`](crate::Reg::read) this register and get [`st21_tconfb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st21_tconfb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct St21TconfbSpec;
impl crate::RegisterSpec for St21TconfbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st21_tconfb::R`](R) reader structure"]
impl crate::Readable for St21TconfbSpec {}
#[doc = "`write(|w| ..)` method takes [`st21_tconfb::W`](W) writer structure"]
impl crate::Writable for St21TconfbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ST21_TCONFB to value 0"]
impl crate::Resettable for St21TconfbSpec {}
