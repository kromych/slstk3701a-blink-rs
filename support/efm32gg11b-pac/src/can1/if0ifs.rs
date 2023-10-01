#[doc = "Register `IF0IFS` writer"]
pub type W = crate::W<IF0IFS_SPEC>;
#[doc = "Field `MESSAGE` writer - Set MESSAGE Interrupt Flag"]
pub type MESSAGE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl W {
    #[doc = "Bits 0:31 - Set MESSAGE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn message(&mut self) -> MESSAGE_W<IF0IFS_SPEC, 0> {
        MESSAGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Message Object Interrupt Flag Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if0ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IF0IFS_SPEC;
impl crate::RegisterSpec for IF0IFS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`if0ifs::W`](W) writer structure"]
impl crate::Writable for IF0IFS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IF0IFS to value 0"]
impl crate::Resettable for IF0IFS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
