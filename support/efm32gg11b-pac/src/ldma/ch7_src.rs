#[doc = "Register `CH7_SRC` reader"]
pub type R = crate::R<CH7_SRC_SPEC>;
#[doc = "Register `CH7_SRC` writer"]
pub type W = crate::W<CH7_SRC_SPEC>;
#[doc = "Field `SRCADDR` reader - Source Data Address"]
pub type SRCADDR_R = crate::FieldReader<u32>;
#[doc = "Field `SRCADDR` writer - Source Data Address"]
pub type SRCADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Source Data Address"]
    #[inline(always)]
    pub fn srcaddr(&self) -> SRCADDR_R {
        SRCADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source Data Address"]
    #[inline(always)]
    #[must_use]
    pub fn srcaddr(&mut self) -> SRCADDR_W<CH7_SRC_SPEC, 0> {
        SRCADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7_src::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7_src::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH7_SRC_SPEC;
impl crate::RegisterSpec for CH7_SRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch7_src::R`](R) reader structure"]
impl crate::Readable for CH7_SRC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch7_src::W`](W) writer structure"]
impl crate::Writable for CH7_SRC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH7_SRC to value 0"]
impl crate::Resettable for CH7_SRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
