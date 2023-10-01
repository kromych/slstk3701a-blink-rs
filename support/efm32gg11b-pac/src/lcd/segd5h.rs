#[doc = "Register `SEGD5H` reader"]
pub type R = crate::R<SEGD5H_SPEC>;
#[doc = "Register `SEGD5H` writer"]
pub type W = crate::W<SEGD5H_SPEC>;
#[doc = "Field `SEGD5H` reader - COM1 Segment Data High"]
pub type SEGD5H_R = crate::FieldReader;
#[doc = "Field `SEGD5H` writer - COM1 Segment Data High"]
pub type SEGD5H_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - COM1 Segment Data High"]
    #[inline(always)]
    pub fn segd5h(&self) -> SEGD5H_R {
        SEGD5H_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - COM1 Segment Data High"]
    #[inline(always)]
    #[must_use]
    pub fn segd5h(&mut self) -> SEGD5H_W<SEGD5H_SPEC, 0> {
        SEGD5H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Segment Data High Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`segd5h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`segd5h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEGD5H_SPEC;
impl crate::RegisterSpec for SEGD5H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`segd5h::R`](R) reader structure"]
impl crate::Readable for SEGD5H_SPEC {}
#[doc = "`write(|w| ..)` method takes [`segd5h::W`](W) writer structure"]
impl crate::Writable for SEGD5H_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEGD5H to value 0"]
impl crate::Resettable for SEGD5H_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
