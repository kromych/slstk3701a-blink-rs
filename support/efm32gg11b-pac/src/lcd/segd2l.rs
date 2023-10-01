#[doc = "Register `SEGD2L` reader"]
pub type R = crate::R<SEGD2L_SPEC>;
#[doc = "Register `SEGD2L` writer"]
pub type W = crate::W<SEGD2L_SPEC>;
#[doc = "Field `SEGD2L` reader - COM2 Segment Data Low"]
pub type SEGD2L_R = crate::FieldReader<u32>;
#[doc = "Field `SEGD2L` writer - COM2 Segment Data Low"]
pub type SEGD2L_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - COM2 Segment Data Low"]
    #[inline(always)]
    pub fn segd2l(&self) -> SEGD2L_R {
        SEGD2L_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - COM2 Segment Data Low"]
    #[inline(always)]
    #[must_use]
    pub fn segd2l(&mut self) -> SEGD2L_W<SEGD2L_SPEC, 0> {
        SEGD2L_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Segment Data Low Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`segd2l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`segd2l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEGD2L_SPEC;
impl crate::RegisterSpec for SEGD2L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`segd2l::R`](R) reader structure"]
impl crate::Readable for SEGD2L_SPEC {}
#[doc = "`write(|w| ..)` method takes [`segd2l::W`](W) writer structure"]
impl crate::Writable for SEGD2L_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEGD2L to value 0"]
impl crate::Resettable for SEGD2L_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
