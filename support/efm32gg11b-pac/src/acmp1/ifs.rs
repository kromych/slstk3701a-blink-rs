#[doc = "Register `IFS` writer"]
pub type W = crate::W<IFS_SPEC>;
#[doc = "Field `EDGE` writer - Set EDGE Interrupt Flag"]
pub type EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WARMUP` writer - Set WARMUP Interrupt Flag"]
pub type WARMUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APORTCONFLICT` writer - Set APORTCONFLICT Interrupt Flag"]
pub type APORTCONFLICT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set EDGE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn edge(&mut self) -> EDGE_W<IFS_SPEC> {
        EDGE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set WARMUP Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn warmup(&mut self) -> WARMUP_W<IFS_SPEC> {
        WARMUP_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set APORTCONFLICT Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn aportconflict(&mut self) -> APORTCONFLICT_W<IFS_SPEC> {
        APORTCONFLICT_W::new(self, 2)
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
#[doc = "Interrupt Flag Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
