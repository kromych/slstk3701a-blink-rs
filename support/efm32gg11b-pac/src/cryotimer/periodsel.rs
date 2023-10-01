#[doc = "Register `PERIODSEL` reader"]
pub type R = crate::R<PERIODSEL_SPEC>;
#[doc = "Register `PERIODSEL` writer"]
pub type W = crate::W<PERIODSEL_SPEC>;
#[doc = "Field `PERIODSEL` reader - Interrupts/Wakeup Events Period Setting"]
pub type PERIODSEL_R = crate::FieldReader;
#[doc = "Field `PERIODSEL` writer - Interrupts/Wakeup Events Period Setting"]
pub type PERIODSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Interrupts/Wakeup Events Period Setting"]
    #[inline(always)]
    pub fn periodsel(&self) -> PERIODSEL_R {
        PERIODSEL_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Interrupts/Wakeup Events Period Setting"]
    #[inline(always)]
    #[must_use]
    pub fn periodsel(&mut self) -> PERIODSEL_W<PERIODSEL_SPEC, 0> {
        PERIODSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Duration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`periodsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`periodsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERIODSEL_SPEC;
impl crate::RegisterSpec for PERIODSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periodsel::R`](R) reader structure"]
impl crate::Readable for PERIODSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`periodsel::W`](W) writer structure"]
impl crate::Writable for PERIODSEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERIODSEL to value 0x20"]
impl crate::Resettable for PERIODSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
