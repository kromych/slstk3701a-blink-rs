#[doc = "Register `IF1IEN` reader"]
pub type R = crate::R<IF1IEN_SPEC>;
#[doc = "Register `IF1IEN` writer"]
pub type W = crate::W<IF1IEN_SPEC>;
#[doc = "Field `STATUS` reader - STATUS Interrupt Enable"]
pub type STATUS_R = crate::BitReader;
#[doc = "Field `STATUS` writer - STATUS Interrupt Enable"]
pub type STATUS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - STATUS Interrupt Enable"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - STATUS Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn status(&mut self) -> STATUS_W<IF1IEN_SPEC, 0> {
        STATUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Status Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if1ien::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if1ien::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IF1IEN_SPEC;
impl crate::RegisterSpec for IF1IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if1ien::R`](R) reader structure"]
impl crate::Readable for IF1IEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`if1ien::W`](W) writer structure"]
impl crate::Writable for IF1IEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IF1IEN to value 0x01"]
impl crate::Resettable for IF1IEN_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
