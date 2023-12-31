#[doc = "Register `KEY` reader"]
pub type R = crate::R<KEY_SPEC>;
#[doc = "Register `KEY` writer"]
pub type W = crate::W<KEY_SPEC>;
#[doc = "Field `KEY` reader - Key Access"]
pub type KEY_R = crate::FieldReader<u32>;
#[doc = "Field `KEY` writer - Key Access"]
pub type KEY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Key Access"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key Access"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<KEY_SPEC, 0> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "KEY Register Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY_SPEC;
impl crate::RegisterSpec for KEY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key::R`](R) reader structure"]
impl crate::Readable for KEY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`key::W`](W) writer structure"]
impl crate::Writable for KEY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEY to value 0"]
impl crate::Resettable for KEY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
