#[doc = "Register `TSUMSBSECCMP` reader"]
pub type R = crate::R<TSUMSBSECCMP_SPEC>;
#[doc = "Register `TSUMSBSECCMP` writer"]
pub type W = crate::W<TSUMSBSECCMP_SPEC>;
#[doc = "Field `COMPVAL` reader - TSU timer comparison value (s)"]
pub type COMPVAL_R = crate::FieldReader<u16>;
#[doc = "Field `COMPVAL` writer - TSU timer comparison value (s)"]
pub type COMPVAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - TSU timer comparison value (s)"]
    #[inline(always)]
    pub fn compval(&self) -> COMPVAL_R {
        COMPVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TSU timer comparison value (s)"]
    #[inline(always)]
    #[must_use]
    pub fn compval(&mut self) -> COMPVAL_W<TSUMSBSECCMP_SPEC, 0> {
        COMPVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "TSU timer comparison value seconds \\[47:32\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsumsbseccmp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsumsbseccmp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSUMSBSECCMP_SPEC;
impl crate::RegisterSpec for TSUMSBSECCMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsumsbseccmp::R`](R) reader structure"]
impl crate::Readable for TSUMSBSECCMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tsumsbseccmp::W`](W) writer structure"]
impl crate::Writable for TSUMSBSECCMP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSUMSBSECCMP to value 0"]
impl crate::Resettable for TSUMSBSECCMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
