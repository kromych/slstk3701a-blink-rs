#[doc = "Register `CMPTHR` reader"]
pub type R = crate::R<CMPTHR_SPEC>;
#[doc = "Register `CMPTHR` writer"]
pub type W = crate::W<CMPTHR_SPEC>;
#[doc = "Field `CMPTHR` reader - Comparator Threshold."]
pub type CMPTHR_R = crate::FieldReader<u16>;
#[doc = "Field `CMPTHR` writer - Comparator Threshold."]
pub type CMPTHR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Comparator Threshold."]
    #[inline(always)]
    pub fn cmpthr(&self) -> CMPTHR_R {
        CMPTHR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Comparator Threshold."]
    #[inline(always)]
    #[must_use]
    pub fn cmpthr(&mut self) -> CMPTHR_W<CMPTHR_SPEC> {
        CMPTHR_W::new(self, 0)
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
#[doc = "Comparator Threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpthr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpthr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMPTHR_SPEC;
impl crate::RegisterSpec for CMPTHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpthr::R`](R) reader structure"]
impl crate::Readable for CMPTHR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmpthr::W`](W) writer structure"]
impl crate::Writable for CMPTHR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMPTHR to value 0"]
impl crate::Resettable for CMPTHR_SPEC {
    const RESET_VALUE: u32 = 0;
}
