#[doc = "Register `SEGD4L` reader"]
pub type R = crate::R<SEGD4L_SPEC>;
#[doc = "Register `SEGD4L` writer"]
pub type W = crate::W<SEGD4L_SPEC>;
#[doc = "Field `SEGD4L` reader - COM4 Segment Data"]
pub type SEGD4L_R = crate::FieldReader<u32>;
#[doc = "Field `SEGD4L` writer - COM4 Segment Data"]
pub type SEGD4L_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - COM4 Segment Data"]
    #[inline(always)]
    pub fn segd4l(&self) -> SEGD4L_R {
        SEGD4L_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - COM4 Segment Data"]
    #[inline(always)]
    #[must_use]
    pub fn segd4l(&mut self) -> SEGD4L_W<SEGD4L_SPEC, 0> {
        SEGD4L_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Segment Data Low Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`segd4l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`segd4l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEGD4L_SPEC;
impl crate::RegisterSpec for SEGD4L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`segd4l::R`](R) reader structure"]
impl crate::Readable for SEGD4L_SPEC {}
#[doc = "`write(|w| ..)` method takes [`segd4l::W`](W) writer structure"]
impl crate::Writable for SEGD4L_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEGD4L to value 0"]
impl crate::Resettable for SEGD4L_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
