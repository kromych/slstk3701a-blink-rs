#[doc = "Register `IMOD` reader"]
pub type R = crate::R<IMOD_SPEC>;
#[doc = "Register `IMOD` writer"]
pub type W = crate::W<IMOD_SPEC>;
#[doc = "Field `RXINTMOD` reader - Count of 800ns periods before bit 1 is set in the interrupt status register after a frame is received"]
pub type RXINTMOD_R = crate::FieldReader;
#[doc = "Field `RXINTMOD` writer - Count of 800ns periods before bit 1 is set in the interrupt status register after a frame is received"]
pub type RXINTMOD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `TXINTMOD` reader - Count of 800ns periods before bit 7 is set in the interrupt status register after a frame is transmitted"]
pub type TXINTMOD_R = crate::FieldReader;
#[doc = "Field `TXINTMOD` writer - Count of 800ns periods before bit 7 is set in the interrupt status register after a frame is transmitted"]
pub type TXINTMOD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Count of 800ns periods before bit 1 is set in the interrupt status register after a frame is received"]
    #[inline(always)]
    pub fn rxintmod(&self) -> RXINTMOD_R {
        RXINTMOD_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Count of 800ns periods before bit 7 is set in the interrupt status register after a frame is transmitted"]
    #[inline(always)]
    pub fn txintmod(&self) -> TXINTMOD_R {
        TXINTMOD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Count of 800ns periods before bit 1 is set in the interrupt status register after a frame is received"]
    #[inline(always)]
    #[must_use]
    pub fn rxintmod(&mut self) -> RXINTMOD_W<IMOD_SPEC, 0> {
        RXINTMOD_W::new(self)
    }
    #[doc = "Bits 16:23 - Count of 800ns periods before bit 7 is set in the interrupt status register after a frame is transmitted"]
    #[inline(always)]
    #[must_use]
    pub fn txintmod(&mut self) -> TXINTMOD_W<IMOD_SPEC, 16> {
        TXINTMOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt moderation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imod::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imod::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMOD_SPEC;
impl crate::RegisterSpec for IMOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imod::R`](R) reader structure"]
impl crate::Readable for IMOD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`imod::W`](W) writer structure"]
impl crate::Writable for IMOD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMOD to value 0"]
impl crate::Resettable for IMOD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
