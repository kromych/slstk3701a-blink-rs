#[doc = "Register `CMPTHR` reader"]
pub type R = crate::R<CMPTHR_SPEC>;
#[doc = "Register `CMPTHR` writer"]
pub type W = crate::W<CMPTHR_SPEC>;
#[doc = "Field `ADLT` reader - Less Than Compare Threshold"]
pub type ADLT_R = crate::FieldReader<u16>;
#[doc = "Field `ADLT` writer - Less Than Compare Threshold"]
pub type ADLT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `ADGT` reader - Greater Than Compare Threshold"]
pub type ADGT_R = crate::FieldReader<u16>;
#[doc = "Field `ADGT` writer - Greater Than Compare Threshold"]
pub type ADGT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Less Than Compare Threshold"]
    #[inline(always)]
    pub fn adlt(&self) -> ADLT_R {
        ADLT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Greater Than Compare Threshold"]
    #[inline(always)]
    pub fn adgt(&self) -> ADGT_R {
        ADGT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Less Than Compare Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn adlt(&mut self) -> ADLT_W<CMPTHR_SPEC, 0> {
        ADLT_W::new(self)
    }
    #[doc = "Bits 16:31 - Greater Than Compare Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn adgt(&mut self) -> ADGT_W<CMPTHR_SPEC, 16> {
        ADGT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Compare Threshold Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpthr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpthr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMPTHR_SPEC;
impl crate::RegisterSpec for CMPTHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpthr::R`](R) reader structure"]
impl crate::Readable for CMPTHR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmpthr::W`](W) writer structure"]
impl crate::Writable for CMPTHR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPTHR to value 0"]
impl crate::Resettable for CMPTHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
