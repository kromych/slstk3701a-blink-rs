#[doc = "Register `SEGD4H` reader"]
pub type R = crate::R<SEGD4H_SPEC>;
#[doc = "Register `SEGD4H` writer"]
pub type W = crate::W<SEGD4H_SPEC>;
#[doc = "Field `SEGD4H` reader - COM0 Segment Data High"]
pub type SEGD4H_R = crate::FieldReader;
#[doc = "Field `SEGD4H` writer - COM0 Segment Data High"]
pub type SEGD4H_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - COM0 Segment Data High"]
    #[inline(always)]
    pub fn segd4h(&self) -> SEGD4H_R {
        SEGD4H_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - COM0 Segment Data High"]
    #[inline(always)]
    #[must_use]
    pub fn segd4h(&mut self) -> SEGD4H_W<SEGD4H_SPEC, 0> {
        SEGD4H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Segment Data High Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`segd4h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`segd4h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEGD4H_SPEC;
impl crate::RegisterSpec for SEGD4H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`segd4h::R`](R) reader structure"]
impl crate::Readable for SEGD4H_SPEC {}
#[doc = "`write(|w| ..)` method takes [`segd4h::W`](W) writer structure"]
impl crate::Writable for SEGD4H_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEGD4H to value 0"]
impl crate::Resettable for SEGD4H_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
