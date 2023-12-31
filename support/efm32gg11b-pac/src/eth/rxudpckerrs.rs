#[doc = "Register `RXUDPCKERRS` reader"]
pub type R = crate::R<RXUDPCKERRS_SPEC>;
#[doc = "Register `RXUDPCKERRS` writer"]
pub type W = crate::W<RXUDPCKERRS_SPEC>;
#[doc = "Field `COUNT` reader - UDP checksum errors"]
pub type COUNT_R = crate::FieldReader;
#[doc = "Field `COUNT` writer - UDP checksum errors"]
pub type COUNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - UDP checksum errors"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - UDP checksum errors"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<RXUDPCKERRS_SPEC, 0> {
        COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "UDP Checksum Errors\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxudpckerrs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxudpckerrs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXUDPCKERRS_SPEC;
impl crate::RegisterSpec for RXUDPCKERRS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxudpckerrs::R`](R) reader structure"]
impl crate::Readable for RXUDPCKERRS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxudpckerrs::W`](W) writer structure"]
impl crate::Writable for RXUDPCKERRS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXUDPCKERRS to value 0"]
impl crate::Resettable for RXUDPCKERRS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
