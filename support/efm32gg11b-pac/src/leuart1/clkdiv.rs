#[doc = "Register `CLKDIV` reader"]
pub type R = crate::R<CLKDIV_SPEC>;
#[doc = "Register `CLKDIV` writer"]
pub type W = crate::W<CLKDIV_SPEC>;
#[doc = "Field `DIV` reader - Fractional Clock Divider"]
pub type DIV_R = crate::FieldReader<u16>;
#[doc = "Field `DIV` writer - Fractional Clock Divider"]
pub type DIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 14, O, u16>;
impl R {
    #[doc = "Bits 3:16 - Fractional Clock Divider"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 3) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 3:16 - Fractional Clock Divider"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<CLKDIV_SPEC, 3> {
        DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkdiv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkdiv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKDIV_SPEC;
impl crate::RegisterSpec for CLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkdiv::R`](R) reader structure"]
impl crate::Readable for CLKDIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clkdiv::W`](W) writer structure"]
impl crate::Writable for CLKDIV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKDIV to value 0"]
impl crate::Resettable for CLKDIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
