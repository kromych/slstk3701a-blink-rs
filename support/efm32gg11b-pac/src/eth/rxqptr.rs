#[doc = "Register `RXQPTR` reader"]
pub type R = crate::R<RXQPTR_SPEC>;
#[doc = "Register `RXQPTR` writer"]
pub type W = crate::W<RXQPTR_SPEC>;
#[doc = "Field `DMARXQPTR` reader - Receive buffer queue base address"]
pub type DMARXQPTR_R = crate::FieldReader<u32>;
#[doc = "Field `DMARXQPTR` writer - Receive buffer queue base address"]
pub type DMARXQPTR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 30, O, u32>;
impl R {
    #[doc = "Bits 2:31 - Receive buffer queue base address"]
    #[inline(always)]
    pub fn dmarxqptr(&self) -> DMARXQPTR_R {
        DMARXQPTR_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Receive buffer queue base address"]
    #[inline(always)]
    #[must_use]
    pub fn dmarxqptr(&mut self) -> DMARXQPTR_W<RXQPTR_SPEC, 2> {
        DMARXQPTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Start address of the receive buffer queue\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxqptr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxqptr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXQPTR_SPEC;
impl crate::RegisterSpec for RXQPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxqptr::R`](R) reader structure"]
impl crate::Readable for RXQPTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxqptr::W`](W) writer structure"]
impl crate::Writable for RXQPTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXQPTR to value 0"]
impl crate::Resettable for RXQPTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
