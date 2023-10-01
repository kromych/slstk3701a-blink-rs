#[doc = "Register `RET6_REG` reader"]
pub type R = crate::R<RET6_REG_SPEC>;
#[doc = "Register `RET6_REG` writer"]
pub type W = crate::W<RET6_REG_SPEC>;
#[doc = "Field `REG` reader - General Purpose Retention Register"]
pub type REG_R = crate::FieldReader<u32>;
#[doc = "Field `REG` writer - General Purpose Retention Register"]
pub type REG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - General Purpose Retention Register"]
    #[inline(always)]
    pub fn reg(&self) -> REG_R {
        REG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - General Purpose Retention Register"]
    #[inline(always)]
    #[must_use]
    pub fn reg(&mut self) -> REG_W<RET6_REG_SPEC, 0> {
        REG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Retention Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ret6_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ret6_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RET6_REG_SPEC;
impl crate::RegisterSpec for RET6_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ret6_reg::R`](R) reader structure"]
impl crate::Readable for RET6_REG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ret6_reg::W`](W) writer structure"]
impl crate::Writable for RET6_REG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RET6_REG to value 0"]
impl crate::Resettable for RET6_REG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
