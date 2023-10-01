#[doc = "Register `INDAHBADDRTRIGGER` reader"]
pub type R = crate::R<INDAHBADDRTRIGGER_SPEC>;
#[doc = "Register `INDAHBADDRTRIGGER` writer"]
pub type W = crate::W<INDAHBADDRTRIGGER_SPEC>;
#[doc = "Field `ADDR` reader - Indirect Address Trigger Register"]
pub type ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Indirect Address Trigger Register"]
pub type ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Indirect Address Trigger Register"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Indirect Address Trigger Register"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<INDAHBADDRTRIGGER_SPEC, 0> {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Indirect Address Trigger Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indahbaddrtrigger::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indahbaddrtrigger::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INDAHBADDRTRIGGER_SPEC;
impl crate::RegisterSpec for INDAHBADDRTRIGGER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`indahbaddrtrigger::R`](R) reader structure"]
impl crate::Readable for INDAHBADDRTRIGGER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`indahbaddrtrigger::W`](W) writer structure"]
impl crate::Writable for INDAHBADDRTRIGGER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INDAHBADDRTRIGGER to value 0"]
impl crate::Resettable for INDAHBADDRTRIGGER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
