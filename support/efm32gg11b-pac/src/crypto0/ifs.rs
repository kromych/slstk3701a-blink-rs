#[doc = "Register `IFS` writer"]
pub type W = crate::W<IFS_SPEC>;
#[doc = "Field `INSTRDONE` writer - Set INSTRDONE Interrupt Flag"]
pub type INSTRDONE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SEQDONE` writer - Set SEQDONE Interrupt Flag"]
pub type SEQDONE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Set INSTRDONE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn instrdone(&mut self) -> INSTRDONE_W<IFS_SPEC, 0> {
        INSTRDONE_W::new(self)
    }
    #[doc = "Bit 1 - Set SEQDONE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn seqdone(&mut self) -> SEQDONE_W<IFS_SPEC, 1> {
        SEQDONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IFS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
