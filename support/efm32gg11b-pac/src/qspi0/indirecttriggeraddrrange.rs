#[doc = "Register `INDIRECTTRIGGERADDRRANGE` reader"]
pub type R = crate::R<INDIRECTTRIGGERADDRRANGE_SPEC>;
#[doc = "Register `INDIRECTTRIGGERADDRRANGE` writer"]
pub type W = crate::W<INDIRECTTRIGGERADDRRANGE_SPEC>;
#[doc = "Field `INDRANGEWIDTH` reader - Indirect Trigger Address Width"]
pub type INDRANGEWIDTH_R = crate::FieldReader;
#[doc = "Field `INDRANGEWIDTH` writer - Indirect Trigger Address Width"]
pub type INDRANGEWIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Indirect Trigger Address Width"]
    #[inline(always)]
    pub fn indrangewidth(&self) -> INDRANGEWIDTH_R {
        INDRANGEWIDTH_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Indirect Trigger Address Width"]
    #[inline(always)]
    #[must_use]
    pub fn indrangewidth(&mut self) -> INDRANGEWIDTH_W<INDIRECTTRIGGERADDRRANGE_SPEC> {
        INDRANGEWIDTH_W::new(self, 0)
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
#[doc = "Indirect Trigger Address Range Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indirecttriggeraddrrange::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indirecttriggeraddrrange::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INDIRECTTRIGGERADDRRANGE_SPEC;
impl crate::RegisterSpec for INDIRECTTRIGGERADDRRANGE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`indirecttriggeraddrrange::R`](R) reader structure"]
impl crate::Readable for INDIRECTTRIGGERADDRRANGE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`indirecttriggeraddrrange::W`](W) writer structure"]
impl crate::Writable for INDIRECTTRIGGERADDRRANGE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INDIRECTTRIGGERADDRRANGE to value 0x04"]
impl crate::Resettable for INDIRECTTRIGGERADDRRANGE_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
