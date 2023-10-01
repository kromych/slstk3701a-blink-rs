#[doc = "Register `CC3_CCVB` reader"]
pub type R = crate::R<CC3_CCVB_SPEC>;
#[doc = "Register `CC3_CCVB` writer"]
pub type W = crate::W<CC3_CCVB_SPEC>;
#[doc = "Field `CCVB` reader - CC Channel Value Buffer"]
pub type CCVB_R = crate::FieldReader<u32>;
#[doc = "Field `CCVB` writer - CC Channel Value Buffer"]
pub type CCVB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - CC Channel Value Buffer"]
    #[inline(always)]
    pub fn ccvb(&self) -> CCVB_R {
        CCVB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CC Channel Value Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn ccvb(&mut self) -> CCVB_W<CC3_CCVB_SPEC, 0> {
        CCVB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "CC Channel Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc3_ccvb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc3_ccvb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CC3_CCVB_SPEC;
impl crate::RegisterSpec for CC3_CCVB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc3_ccvb::R`](R) reader structure"]
impl crate::Readable for CC3_CCVB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cc3_ccvb::W`](W) writer structure"]
impl crate::Writable for CC3_CCVB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CC3_CCVB to value 0"]
impl crate::Resettable for CC3_CCVB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
