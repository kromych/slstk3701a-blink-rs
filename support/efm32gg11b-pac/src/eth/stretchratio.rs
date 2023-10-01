#[doc = "Register `STRETCHRATIO` reader"]
pub type R = crate::R<STRETCHRATIO_SPEC>;
#[doc = "Register `STRETCHRATIO` writer"]
pub type W = crate::W<STRETCHRATIO_SPEC>;
#[doc = "Field `IPGSTRETCH` reader - IPG Stretch"]
pub type IPGSTRETCH_R = crate::FieldReader<u16>;
#[doc = "Field `IPGSTRETCH` writer - IPG Stretch"]
pub type IPGSTRETCH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - IPG Stretch"]
    #[inline(always)]
    pub fn ipgstretch(&self) -> IPGSTRETCH_R {
        IPGSTRETCH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IPG Stretch"]
    #[inline(always)]
    #[must_use]
    pub fn ipgstretch(&mut self) -> IPGSTRETCH_W<STRETCHRATIO_SPEC, 0> {
        IPGSTRETCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "IPG stretch register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stretchratio::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stretchratio::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STRETCHRATIO_SPEC;
impl crate::RegisterSpec for STRETCHRATIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stretchratio::R`](R) reader structure"]
impl crate::Readable for STRETCHRATIO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stretchratio::W`](W) writer structure"]
impl crate::Writable for STRETCHRATIO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STRETCHRATIO to value 0"]
impl crate::Resettable for STRETCHRATIO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
