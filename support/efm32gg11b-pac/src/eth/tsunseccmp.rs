#[doc = "Register `TSUNSECCMP` reader"]
pub type R = crate::R<TSUNSECCMP_SPEC>;
#[doc = "Register `TSUNSECCMP` writer"]
pub type W = crate::W<TSUNSECCMP_SPEC>;
#[doc = "Field `COMPVAL` reader - TSU timer comparison value (ns)"]
pub type COMPVAL_R = crate::FieldReader<u32>;
#[doc = "Field `COMPVAL` writer - TSU timer comparison value (ns)"]
pub type COMPVAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 22, O, u32>;
impl R {
    #[doc = "Bits 0:21 - TSU timer comparison value (ns)"]
    #[inline(always)]
    pub fn compval(&self) -> COMPVAL_R {
        COMPVAL_R::new(self.bits & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:21 - TSU timer comparison value (ns)"]
    #[inline(always)]
    #[must_use]
    pub fn compval(&mut self) -> COMPVAL_W<TSUNSECCMP_SPEC, 0> {
        COMPVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "TSU timer comparison value nanoseconds\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsunseccmp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsunseccmp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSUNSECCMP_SPEC;
impl crate::RegisterSpec for TSUNSECCMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsunseccmp::R`](R) reader structure"]
impl crate::Readable for TSUNSECCMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tsunseccmp::W`](W) writer structure"]
impl crate::Writable for TSUNSECCMP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSUNSECCMP to value 0"]
impl crate::Resettable for TSUNSECCMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
