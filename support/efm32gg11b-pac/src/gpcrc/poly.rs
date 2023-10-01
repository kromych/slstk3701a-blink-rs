#[doc = "Register `POLY` reader"]
pub type R = crate::R<POLY_SPEC>;
#[doc = "Register `POLY` writer"]
pub type W = crate::W<POLY_SPEC>;
#[doc = "Field `POLY` reader - CRC Polynomial Value"]
pub type POLY_R = crate::FieldReader<u16>;
#[doc = "Field `POLY` writer - CRC Polynomial Value"]
pub type POLY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - CRC Polynomial Value"]
    #[inline(always)]
    pub fn poly(&self) -> POLY_R {
        POLY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC Polynomial Value"]
    #[inline(always)]
    #[must_use]
    pub fn poly(&mut self) -> POLY_W<POLY_SPEC, 0> {
        POLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "CRC Polynomial Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`poly::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`poly::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POLY_SPEC;
impl crate::RegisterSpec for POLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`poly::R`](R) reader structure"]
impl crate::Readable for POLY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`poly::W`](W) writer structure"]
impl crate::Writable for POLY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets POLY to value 0"]
impl crate::Resettable for POLY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
