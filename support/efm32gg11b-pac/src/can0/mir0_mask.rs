#[doc = "Register `MIR0_MASK` reader"]
pub type R = crate::R<MIR0_MASK_SPEC>;
#[doc = "Register `MIR0_MASK` writer"]
pub type W = crate::W<MIR0_MASK_SPEC>;
#[doc = "Field `MASK` reader - Identifier Mask"]
pub type MASK_R = crate::FieldReader<u32>;
#[doc = "Field `MASK` writer - Identifier Mask"]
pub type MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
#[doc = "Field `MDIR` reader - Mask Message Direction"]
pub type MDIR_R = crate::BitReader;
#[doc = "Field `MDIR` writer - Mask Message Direction"]
pub type MDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MXTD` reader - Mask Extended Identifier"]
pub type MXTD_R = crate::BitReader;
#[doc = "Field `MXTD` writer - Mask Extended Identifier"]
pub type MXTD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:28 - Identifier Mask"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(self.bits & 0x1fff_ffff)
    }
    #[doc = "Bit 30 - Mask Message Direction"]
    #[inline(always)]
    pub fn mdir(&self) -> MDIR_R {
        MDIR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Mask Extended Identifier"]
    #[inline(always)]
    pub fn mxtd(&self) -> MXTD_R {
        MXTD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:28 - Identifier Mask"]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MASK_W<MIR0_MASK_SPEC> {
        MASK_W::new(self, 0)
    }
    #[doc = "Bit 30 - Mask Message Direction"]
    #[inline(always)]
    #[must_use]
    pub fn mdir(&mut self) -> MDIR_W<MIR0_MASK_SPEC> {
        MDIR_W::new(self, 30)
    }
    #[doc = "Bit 31 - Mask Extended Identifier"]
    #[inline(always)]
    #[must_use]
    pub fn mxtd(&mut self) -> MXTD_W<MIR0_MASK_SPEC> {
        MXTD_W::new(self, 31)
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
#[doc = "Interface Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mir0_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mir0_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MIR0_MASK_SPEC;
impl crate::RegisterSpec for MIR0_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mir0_mask::R`](R) reader structure"]
impl crate::Readable for MIR0_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mir0_mask::W`](W) writer structure"]
impl crate::Writable for MIR0_MASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MIR0_MASK to value 0xdfff_ffff"]
impl crate::Resettable for MIR0_MASK_SPEC {
    const RESET_VALUE: u32 = 0xdfff_ffff;
}
