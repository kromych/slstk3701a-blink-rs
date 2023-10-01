#[doc = "Register `TXLPI` reader"]
pub type R = crate::R<TXLPI_SPEC>;
#[doc = "Register `TXLPI` writer"]
pub type W = crate::W<TXLPI_SPEC>;
#[doc = "Field `COUNT` reader - Count of LPI transmitions"]
pub type COUNT_R = crate::FieldReader<u16>;
#[doc = "Field `COUNT` writer - Count of LPI transmitions"]
pub type COUNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Count of LPI transmitions"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Count of LPI transmitions"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<TXLPI_SPEC, 0> {
        COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transmit LPI transitions\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txlpi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txlpi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXLPI_SPEC;
impl crate::RegisterSpec for TXLPI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txlpi::R`](R) reader structure"]
impl crate::Readable for TXLPI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txlpi::W`](W) writer structure"]
impl crate::Writable for TXLPI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXLPI to value 0"]
impl crate::Resettable for TXLPI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}