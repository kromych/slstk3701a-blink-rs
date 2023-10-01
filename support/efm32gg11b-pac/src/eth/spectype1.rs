#[doc = "Register `SPECTYPE1` reader"]
pub type R = crate::R<SPECTYPE1_SPEC>;
#[doc = "Register `SPECTYPE1` writer"]
pub type W = crate::W<SPECTYPE1_SPEC>;
#[doc = "Field `MATCH` reader - Type ID match 1"]
pub type MATCH_R = crate::FieldReader<u16>;
#[doc = "Field `MATCH` writer - Type ID match 1"]
pub type MATCH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `ENBCOPY` reader - Enable copying of type ID match 1 matched frames."]
pub type ENBCOPY_R = crate::BitReader;
#[doc = "Field `ENBCOPY` writer - Enable copying of type ID match 1 matched frames."]
pub type ENBCOPY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:15 - Type ID match 1"]
    #[inline(always)]
    pub fn match_(&self) -> MATCH_R {
        MATCH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Enable copying of type ID match 1 matched frames."]
    #[inline(always)]
    pub fn enbcopy(&self) -> ENBCOPY_R {
        ENBCOPY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Type ID match 1"]
    #[inline(always)]
    #[must_use]
    pub fn match_(&mut self) -> MATCH_W<SPECTYPE1_SPEC, 0> {
        MATCH_W::new(self)
    }
    #[doc = "Bit 31 - Enable copying of type ID match 1 matched frames."]
    #[inline(always)]
    #[must_use]
    pub fn enbcopy(&mut self) -> ENBCOPY_W<SPECTYPE1_SPEC, 31> {
        ENBCOPY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Type ID Match 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spectype1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spectype1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPECTYPE1_SPEC;
impl crate::RegisterSpec for SPECTYPE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spectype1::R`](R) reader structure"]
impl crate::Readable for SPECTYPE1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spectype1::W`](W) writer structure"]
impl crate::Writable for SPECTYPE1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPECTYPE1 to value 0"]
impl crate::Resettable for SPECTYPE1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
