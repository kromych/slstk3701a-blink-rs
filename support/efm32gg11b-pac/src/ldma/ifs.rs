#[doc = "Register `IFS` writer"]
pub type W = crate::W<IFS_SPEC>;
#[doc = "Field `DONE` writer - Set DONE Interrupt Flag"]
pub type DONE_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `ERROR` writer - Set ERROR Interrupt Flag"]
pub type ERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:23 - Set DONE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DONE_W<IFS_SPEC> {
        DONE_W::new(self, 0)
    }
    #[doc = "Bit 31 - Set ERROR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ERROR_W<IFS_SPEC> {
        ERROR_W::new(self, 31)
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
