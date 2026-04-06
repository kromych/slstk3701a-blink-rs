#[doc = "Register `MIR1_MASK` reader"]
pub type R = crate::R<Mir1MaskSpec>;
#[doc = "Register `MIR1_MASK` writer"]
pub type W = crate::W<Mir1MaskSpec>;
#[doc = "Field `MASK` reader - Identifier Mask"]
pub type MaskR = crate::FieldReader<u32>;
#[doc = "Field `MASK` writer - Identifier Mask"]
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
#[doc = "Field `MDIR` reader - Mask Message Direction"]
pub type MdirR = crate::BitReader;
#[doc = "Field `MDIR` writer - Mask Message Direction"]
pub type MdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MXTD` reader - Mask Extended Identifier"]
pub type MxtdR = crate::BitReader;
#[doc = "Field `MXTD` writer - Mask Extended Identifier"]
pub type MxtdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:28 - Identifier Mask"]
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new(self.bits & 0x1fff_ffff)
    }
    #[doc = "Bit 30 - Mask Message Direction"]
    #[inline(always)]
    pub fn mdir(&self) -> MdirR {
        MdirR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Mask Extended Identifier"]
    #[inline(always)]
    pub fn mxtd(&self) -> MxtdR {
        MxtdR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:28 - Identifier Mask"]
    #[inline(always)]
    pub fn mask(&mut self) -> MaskW<'_, Mir1MaskSpec> {
        MaskW::new(self, 0)
    }
    #[doc = "Bit 30 - Mask Message Direction"]
    #[inline(always)]
    pub fn mdir(&mut self) -> MdirW<'_, Mir1MaskSpec> {
        MdirW::new(self, 30)
    }
    #[doc = "Bit 31 - Mask Extended Identifier"]
    #[inline(always)]
    pub fn mxtd(&mut self) -> MxtdW<'_, Mir1MaskSpec> {
        MxtdW::new(self, 31)
    }
}
#[doc = "Interface Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mir1_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mir1_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mir1MaskSpec;
impl crate::RegisterSpec for Mir1MaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mir1_mask::R`](R) reader structure"]
impl crate::Readable for Mir1MaskSpec {}
#[doc = "`write(|w| ..)` method takes [`mir1_mask::W`](W) writer structure"]
impl crate::Writable for Mir1MaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MIR1_MASK to value 0xdfff_ffff"]
impl crate::Resettable for Mir1MaskSpec {
    const RESET_VALUE: u32 = 0xdfff_ffff;
}
