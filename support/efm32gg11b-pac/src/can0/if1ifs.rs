#[doc = "Register `IF1IFS` writer"]
pub type W = crate::W<IF1IFS_SPEC>;
#[doc = "Field `STATUS` writer - Set STATUS Interrupt Flag"]
pub type STATUS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Set STATUS Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn status(&mut self) -> STATUS_W<IF1IFS_SPEC, 0> {
        STATUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Message Object Interrupt Flag Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if1ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IF1IFS_SPEC;
impl crate::RegisterSpec for IF1IFS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`if1ifs::W`](W) writer structure"]
impl crate::Writable for IF1IFS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IF1IFS to value 0"]
impl crate::Resettable for IF1IFS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
