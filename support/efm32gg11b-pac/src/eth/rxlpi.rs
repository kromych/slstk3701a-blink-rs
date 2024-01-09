#[doc = "Register `RXLPI` reader"]
pub type R = crate::R<RXLPI_SPEC>;
#[doc = "Register `RXLPI` writer"]
pub type W = crate::W<RXLPI_SPEC>;
#[doc = "Field `COUNT` reader - Count of RX LPI transitions"]
pub type COUNT_R = crate::FieldReader<u16>;
#[doc = "Field `COUNT` writer - Count of RX LPI transitions"]
pub type COUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Count of RX LPI transitions"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Count of RX LPI transitions"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<RXLPI_SPEC> {
        COUNT_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Received LPI transitions\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxlpi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxlpi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXLPI_SPEC;
impl crate::RegisterSpec for RXLPI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxlpi::R`](R) reader structure"]
impl crate::Readable for RXLPI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxlpi::W`](W) writer structure"]
impl crate::Writable for RXLPI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXLPI to value 0"]
impl crate::Resettable for RXLPI_SPEC {
    const RESET_VALUE: u32 = 0;
}
