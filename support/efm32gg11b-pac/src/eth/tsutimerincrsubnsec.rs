#[doc = "Register `TSUTIMERINCRSUBNSEC` reader"]
pub type R = crate::R<TSUTIMERINCRSUBNSEC_SPEC>;
#[doc = "Register `TSUTIMERINCRSUBNSEC` writer"]
pub type W = crate::W<TSUTIMERINCRSUBNSEC_SPEC>;
#[doc = "Field `SUBNSINCR` reader - MSB \\[23:8\\] of the subscript-ns value"]
pub type SUBNSINCR_R = crate::FieldReader<u16>;
#[doc = "Field `SUBNSINCR` writer - MSB \\[23:8\\] of the subscript-ns value"]
pub type SUBNSINCR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `SUBNSINCRLSB` reader - LSB \\[7:0\\] of the subscript-ns value"]
pub type SUBNSINCRLSB_R = crate::FieldReader;
#[doc = "Field `SUBNSINCRLSB` writer - LSB \\[7:0\\] of the subscript-ns value"]
pub type SUBNSINCRLSB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:15 - MSB \\[23:8\\] of the subscript-ns value"]
    #[inline(always)]
    pub fn subnsincr(&self) -> SUBNSINCR_R {
        SUBNSINCR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:31 - LSB \\[7:0\\] of the subscript-ns value"]
    #[inline(always)]
    pub fn subnsincrlsb(&self) -> SUBNSINCRLSB_R {
        SUBNSINCRLSB_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - MSB \\[23:8\\] of the subscript-ns value"]
    #[inline(always)]
    #[must_use]
    pub fn subnsincr(&mut self) -> SUBNSINCR_W<TSUTIMERINCRSUBNSEC_SPEC, 0> {
        SUBNSINCR_W::new(self)
    }
    #[doc = "Bits 24:31 - LSB \\[7:0\\] of the subscript-ns value"]
    #[inline(always)]
    #[must_use]
    pub fn subnsincrlsb(&mut self) -> SUBNSINCRLSB_W<TSUTIMERINCRSUBNSEC_SPEC, 24> {
        SUBNSINCRLSB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "1588 Timer Increment Register subscript nsec\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsutimerincrsubnsec::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsutimerincrsubnsec::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSUTIMERINCRSUBNSEC_SPEC;
impl crate::RegisterSpec for TSUTIMERINCRSUBNSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsutimerincrsubnsec::R`](R) reader structure"]
impl crate::Readable for TSUTIMERINCRSUBNSEC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tsutimerincrsubnsec::W`](W) writer structure"]
impl crate::Writable for TSUTIMERINCRSUBNSEC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSUTIMERINCRSUBNSEC to value 0"]
impl crate::Resettable for TSUTIMERINCRSUBNSEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
