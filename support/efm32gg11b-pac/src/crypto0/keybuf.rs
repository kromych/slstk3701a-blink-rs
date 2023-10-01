#[doc = "Register `KEYBUF` reader"]
pub type R = crate::R<KEYBUF_SPEC>;
#[doc = "Register `KEYBUF` writer"]
pub type W = crate::W<KEYBUF_SPEC>;
#[doc = "Field `KEYBUF` reader - Key Buffer Access"]
pub type KEYBUF_R = crate::FieldReader<u32>;
#[doc = "Field `KEYBUF` writer - Key Buffer Access"]
pub type KEYBUF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Key Buffer Access"]
    #[inline(always)]
    pub fn keybuf(&self) -> KEYBUF_R {
        KEYBUF_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key Buffer Access"]
    #[inline(always)]
    #[must_use]
    pub fn keybuf(&mut self) -> KEYBUF_W<KEYBUF_SPEC, 0> {
        KEYBUF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "KEY Buffer Register Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keybuf::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keybuf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEYBUF_SPEC;
impl crate::RegisterSpec for KEYBUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keybuf::R`](R) reader structure"]
impl crate::Readable for KEYBUF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`keybuf::W`](W) writer structure"]
impl crate::Writable for KEYBUF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEYBUF to value 0"]
impl crate::Resettable for KEYBUF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
