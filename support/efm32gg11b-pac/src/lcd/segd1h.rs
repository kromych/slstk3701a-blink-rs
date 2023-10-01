#[doc = "Register `SEGD1H` reader"]
pub type R = crate::R<SEGD1H_SPEC>;
#[doc = "Register `SEGD1H` writer"]
pub type W = crate::W<SEGD1H_SPEC>;
#[doc = "Field `SEGD1H` reader - COM1 Segment Data High"]
pub type SEGD1H_R = crate::FieldReader;
#[doc = "Field `SEGD1H` writer - COM1 Segment Data High"]
pub type SEGD1H_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - COM1 Segment Data High"]
    #[inline(always)]
    pub fn segd1h(&self) -> SEGD1H_R {
        SEGD1H_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - COM1 Segment Data High"]
    #[inline(always)]
    #[must_use]
    pub fn segd1h(&mut self) -> SEGD1H_W<SEGD1H_SPEC, 0> {
        SEGD1H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Segment Data High Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`segd1h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`segd1h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEGD1H_SPEC;
impl crate::RegisterSpec for SEGD1H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`segd1h::R`](R) reader structure"]
impl crate::Readable for SEGD1H_SPEC {}
#[doc = "`write(|w| ..)` method takes [`segd1h::W`](W) writer structure"]
impl crate::Writable for SEGD1H_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEGD1H to value 0"]
impl crate::Resettable for SEGD1H_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
