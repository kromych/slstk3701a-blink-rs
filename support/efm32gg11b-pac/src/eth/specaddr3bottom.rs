#[doc = "Register `SPECADDR3BOTTOM` reader"]
pub type R = crate::R<SPECADDR3BOTTOM_SPEC>;
#[doc = "Register `SPECADDR3BOTTOM` writer"]
pub type W = crate::W<SPECADDR3BOTTOM_SPEC>;
#[doc = "Field `ADDR` reader - Least significant 32 bits of the destination address"]
pub type ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Least significant 32 bits of the destination address"]
pub type ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Least significant 32 bits of the destination address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Least significant 32 bits of the destination address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<SPECADDR3BOTTOM_SPEC, 0> {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Specific Address 3 Bottom\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`specaddr3bottom::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`specaddr3bottom::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPECADDR3BOTTOM_SPEC;
impl crate::RegisterSpec for SPECADDR3BOTTOM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`specaddr3bottom::R`](R) reader structure"]
impl crate::Readable for SPECADDR3BOTTOM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`specaddr3bottom::W`](W) writer structure"]
impl crate::Writable for SPECADDR3BOTTOM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPECADDR3BOTTOM to value 0"]
impl crate::Resettable for SPECADDR3BOTTOM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
