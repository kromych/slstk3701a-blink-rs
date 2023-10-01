#[doc = "Register `TXPTPUNICAST` reader"]
pub type R = crate::R<TXPTPUNICAST_SPEC>;
#[doc = "Register `TXPTPUNICAST` writer"]
pub type W = crate::W<TXPTPUNICAST_SPEC>;
#[doc = "Field `ADDR` reader - Unicast IP destination address"]
pub type ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Unicast IP destination address"]
pub type ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Unicast IP destination address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Unicast IP destination address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<TXPTPUNICAST_SPEC, 0> {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PTP TX unicast IP destination address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txptpunicast::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txptpunicast::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXPTPUNICAST_SPEC;
impl crate::RegisterSpec for TXPTPUNICAST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txptpunicast::R`](R) reader structure"]
impl crate::Readable for TXPTPUNICAST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txptpunicast::W`](W) writer structure"]
impl crate::Writable for TXPTPUNICAST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXPTPUNICAST to value 0"]
impl crate::Resettable for TXPTPUNICAST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
