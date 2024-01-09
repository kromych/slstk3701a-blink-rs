#[doc = "Register `SPECTYPE2` reader"]
pub type R = crate::R<SPECTYPE2_SPEC>;
#[doc = "Register `SPECTYPE2` writer"]
pub type W = crate::W<SPECTYPE2_SPEC>;
#[doc = "Field `MATCH` reader - Type ID match 2"]
pub type MATCH_R = crate::FieldReader<u16>;
#[doc = "Field `MATCH` writer - Type ID match 2"]
pub type MATCH_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ENBCOPY` reader - Enable copying of type ID match 2 matched frames."]
pub type ENBCOPY_R = crate::BitReader;
#[doc = "Field `ENBCOPY` writer - Enable copying of type ID match 2 matched frames."]
pub type ENBCOPY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Type ID match 2"]
    #[inline(always)]
    pub fn match_(&self) -> MATCH_R {
        MATCH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Enable copying of type ID match 2 matched frames."]
    #[inline(always)]
    pub fn enbcopy(&self) -> ENBCOPY_R {
        ENBCOPY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Type ID match 2"]
    #[inline(always)]
    #[must_use]
    pub fn match_(&mut self) -> MATCH_W<SPECTYPE2_SPEC> {
        MATCH_W::new(self, 0)
    }
    #[doc = "Bit 31 - Enable copying of type ID match 2 matched frames."]
    #[inline(always)]
    #[must_use]
    pub fn enbcopy(&mut self) -> ENBCOPY_W<SPECTYPE2_SPEC> {
        ENBCOPY_W::new(self, 31)
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
#[doc = "Type ID Match 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spectype2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spectype2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPECTYPE2_SPEC;
impl crate::RegisterSpec for SPECTYPE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spectype2::R`](R) reader structure"]
impl crate::Readable for SPECTYPE2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spectype2::W`](W) writer structure"]
impl crate::Writable for SPECTYPE2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPECTYPE2 to value 0"]
impl crate::Resettable for SPECTYPE2_SPEC {
    const RESET_VALUE: u32 = 0;
}
