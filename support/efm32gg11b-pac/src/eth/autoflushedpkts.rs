#[doc = "Register `AUTOFLUSHEDPKTS` reader"]
pub type R = crate::R<AUTOFLUSHEDPKTS_SPEC>;
#[doc = "Register `AUTOFLUSHEDPKTS` writer"]
pub type W = crate::W<AUTOFLUSHEDPKTS_SPEC>;
#[doc = "Field `COUNT` reader - Flushed RX pkts counter"]
pub type COUNT_R = crate::FieldReader<u16>;
#[doc = "Field `COUNT` writer - Flushed RX pkts counter"]
pub type COUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Flushed RX pkts counter"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Flushed RX pkts counter"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<AUTOFLUSHEDPKTS_SPEC> {
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
#[doc = "Receive DMA Flushed Packets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`autoflushedpkts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`autoflushedpkts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AUTOFLUSHEDPKTS_SPEC;
impl crate::RegisterSpec for AUTOFLUSHEDPKTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`autoflushedpkts::R`](R) reader structure"]
impl crate::Readable for AUTOFLUSHEDPKTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`autoflushedpkts::W`](W) writer structure"]
impl crate::Writable for AUTOFLUSHEDPKTS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUTOFLUSHEDPKTS to value 0"]
impl crate::Resettable for AUTOFLUSHEDPKTS_SPEC {
    const RESET_VALUE: u32 = 0;
}
