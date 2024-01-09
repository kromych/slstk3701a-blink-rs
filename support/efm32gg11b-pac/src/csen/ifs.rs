#[doc = "Register `IFS` writer"]
pub type W = crate::W<IFS_SPEC>;
#[doc = "Field `CMP` writer - Set CMP Interrupt Flag"]
pub type CMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONV` writer - Set CONV Interrupt Flag"]
pub type CONV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOS` writer - Set EOS Interrupt Flag"]
pub type EOS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAOF` writer - Set DMAOF Interrupt Flag"]
pub type DMAOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APORTCONFLICT` writer - Set APORTCONFLICT Interrupt Flag"]
pub type APORTCONFLICT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set CMP Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmp(&mut self) -> CMP_W<IFS_SPEC> {
        CMP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set CONV Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn conv(&mut self) -> CONV_W<IFS_SPEC> {
        CONV_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set EOS Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn eos(&mut self) -> EOS_W<IFS_SPEC> {
        EOS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set DMAOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dmaof(&mut self) -> DMAOF_W<IFS_SPEC> {
        DMAOF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set APORTCONFLICT Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn aportconflict(&mut self) -> APORTCONFLICT_W<IFS_SPEC> {
        APORTCONFLICT_W::new(self, 4)
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
#[doc = "Interrupt Flag Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFS_SPEC;
impl crate::RegisterSpec for IFS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifs::W`](W) writer structure"]
impl crate::Writable for IFS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IFS_SPEC {
    const RESET_VALUE: u32 = 0;
}
