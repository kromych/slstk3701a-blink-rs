#[doc = "Register `TXQPTR` reader"]
pub type R = crate::R<TXQPTR_SPEC>;
#[doc = "Register `TXQPTR` writer"]
pub type W = crate::W<TXQPTR_SPEC>;
#[doc = "Field `DMATXQPTR` reader - Transmit buffer queue base address"]
pub type DMATXQPTR_R = crate::FieldReader<u32>;
#[doc = "Field `DMATXQPTR` writer - Transmit buffer queue base address"]
pub type DMATXQPTR_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Transmit buffer queue base address"]
    #[inline(always)]
    pub fn dmatxqptr(&self) -> DMATXQPTR_R {
        DMATXQPTR_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Transmit buffer queue base address"]
    #[inline(always)]
    #[must_use]
    pub fn dmatxqptr(&mut self) -> DMATXQPTR_W<TXQPTR_SPEC> {
        DMATXQPTR_W::new(self, 2)
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
#[doc = "Start address of the transmit buffer queue\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txqptr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txqptr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXQPTR_SPEC;
impl crate::RegisterSpec for TXQPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txqptr::R`](R) reader structure"]
impl crate::Readable for TXQPTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txqptr::W`](W) writer structure"]
impl crate::Writable for TXQPTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXQPTR to value 0"]
impl crate::Resettable for TXQPTR_SPEC {
    const RESET_VALUE: u32 = 0;
}
