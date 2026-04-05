#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `CMP` reader - CMP Interrupt Enable"]
pub type CmpR = crate::BitReader;
#[doc = "Field `CMP` writer - CMP Interrupt Enable"]
pub type CmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONV` reader - CONV Interrupt Enable"]
pub type ConvR = crate::BitReader;
#[doc = "Field `CONV` writer - CONV Interrupt Enable"]
pub type ConvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOS` reader - EOS Interrupt Enable"]
pub type EosR = crate::BitReader;
#[doc = "Field `EOS` writer - EOS Interrupt Enable"]
pub type EosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAOF` reader - DMAOF Interrupt Enable"]
pub type DmaofR = crate::BitReader;
#[doc = "Field `DMAOF` writer - DMAOF Interrupt Enable"]
pub type DmaofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APORTCONFLICT` reader - APORTCONFLICT Interrupt Enable"]
pub type AportconflictR = crate::BitReader;
#[doc = "Field `APORTCONFLICT` writer - APORTCONFLICT Interrupt Enable"]
pub type AportconflictW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CMP Interrupt Enable"]
    #[inline(always)]
    pub fn cmp(&self) -> CmpR {
        CmpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CONV Interrupt Enable"]
    #[inline(always)]
    pub fn conv(&self) -> ConvR {
        ConvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EOS Interrupt Enable"]
    #[inline(always)]
    pub fn eos(&self) -> EosR {
        EosR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMAOF Interrupt Enable"]
    #[inline(always)]
    pub fn dmaof(&self) -> DmaofR {
        DmaofR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    pub fn aportconflict(&self) -> AportconflictR {
        AportconflictR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CMP Interrupt Enable"]
    #[inline(always)]
    pub fn cmp(&mut self) -> CmpW<'_, IenSpec> {
        CmpW::new(self, 0)
    }
    #[doc = "Bit 1 - CONV Interrupt Enable"]
    #[inline(always)]
    pub fn conv(&mut self) -> ConvW<'_, IenSpec> {
        ConvW::new(self, 1)
    }
    #[doc = "Bit 2 - EOS Interrupt Enable"]
    #[inline(always)]
    pub fn eos(&mut self) -> EosW<'_, IenSpec> {
        EosW::new(self, 2)
    }
    #[doc = "Bit 3 - DMAOF Interrupt Enable"]
    #[inline(always)]
    pub fn dmaof(&mut self) -> DmaofW<'_, IenSpec> {
        DmaofW::new(self, 3)
    }
    #[doc = "Bit 4 - APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    pub fn aportconflict(&mut self) -> AportconflictW<'_, IenSpec> {
        AportconflictW::new(self, 4)
    }
}
#[doc = "Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IenSpec;
impl crate::RegisterSpec for IenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IenSpec {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IenSpec {}
