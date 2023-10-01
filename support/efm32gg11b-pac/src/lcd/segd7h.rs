#[doc = "Register `SEGD7H` reader"]
pub type R = crate::R<SEGD7H_SPEC>;
#[doc = "Register `SEGD7H` writer"]
pub type W = crate::W<SEGD7H_SPEC>;
#[doc = "Field `SEGD7H` reader - COM3 Segment Data High"]
pub type SEGD7H_R = crate::FieldReader;
#[doc = "Field `SEGD7H` writer - COM3 Segment Data High"]
pub type SEGD7H_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - COM3 Segment Data High"]
    #[inline(always)]
    pub fn segd7h(&self) -> SEGD7H_R {
        SEGD7H_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - COM3 Segment Data High"]
    #[inline(always)]
    #[must_use]
    pub fn segd7h(&mut self) -> SEGD7H_W<SEGD7H_SPEC, 0> {
        SEGD7H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Segment Data High Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`segd7h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`segd7h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEGD7H_SPEC;
impl crate::RegisterSpec for SEGD7H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`segd7h::R`](R) reader structure"]
impl crate::Readable for SEGD7H_SPEC {}
#[doc = "`write(|w| ..)` method takes [`segd7h::W`](W) writer structure"]
impl crate::Writable for SEGD7H_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEGD7H to value 0"]
impl crate::Resettable for SEGD7H_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
