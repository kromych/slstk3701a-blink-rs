#[doc = "Register `TSUSECCMP` reader"]
pub type R = crate::R<TSUSECCMP_SPEC>;
#[doc = "Register `TSUSECCMP` writer"]
pub type W = crate::W<TSUSECCMP_SPEC>;
#[doc = "Field `COMPVAL` reader - TSU timer comparison value (s)"]
pub type COMPVAL_R = crate::FieldReader<u32>;
#[doc = "Field `COMPVAL` writer - TSU timer comparison value (s)"]
pub type COMPVAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - TSU timer comparison value (s)"]
    #[inline(always)]
    pub fn compval(&self) -> COMPVAL_R {
        COMPVAL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TSU timer comparison value (s)"]
    #[inline(always)]
    #[must_use]
    pub fn compval(&mut self) -> COMPVAL_W<TSUSECCMP_SPEC, 0> {
        COMPVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "TSU timer comparison value seconds \\[31:0\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsuseccmp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsuseccmp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSUSECCMP_SPEC;
impl crate::RegisterSpec for TSUSECCMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsuseccmp::R`](R) reader structure"]
impl crate::Readable for TSUSECCMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tsuseccmp::W`](W) writer structure"]
impl crate::Writable for TSUSECCMP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSUSECCMP to value 0"]
impl crate::Resettable for TSUSECCMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
