#[doc = "Register `NOOFPOLLSBEFEXP` reader"]
pub type R = crate::R<NOOFPOLLSBEFEXP_SPEC>;
#[doc = "Register `NOOFPOLLSBEFEXP` writer"]
pub type W = crate::W<NOOFPOLLSBEFEXP_SPEC>;
#[doc = "Field `NOOFPOLLSBEFEXP` reader - Number of Polls Cycles Before Expiration"]
pub type NOOFPOLLSBEFEXP_R = crate::FieldReader<u32>;
#[doc = "Field `NOOFPOLLSBEFEXP` writer - Number of Polls Cycles Before Expiration"]
pub type NOOFPOLLSBEFEXP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of Polls Cycles Before Expiration"]
    #[inline(always)]
    pub fn noofpollsbefexp(&self) -> NOOFPOLLSBEFEXP_R {
        NOOFPOLLSBEFEXP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of Polls Cycles Before Expiration"]
    #[inline(always)]
    #[must_use]
    pub fn noofpollsbefexp(&mut self) -> NOOFPOLLSBEFEXP_W<NOOFPOLLSBEFEXP_SPEC, 0> {
        NOOFPOLLSBEFEXP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Polling Expiration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`noofpollsbefexp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`noofpollsbefexp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NOOFPOLLSBEFEXP_SPEC;
impl crate::RegisterSpec for NOOFPOLLSBEFEXP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`noofpollsbefexp::R`](R) reader structure"]
impl crate::Readable for NOOFPOLLSBEFEXP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`noofpollsbefexp::W`](W) writer structure"]
impl crate::Writable for NOOFPOLLSBEFEXP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NOOFPOLLSBEFEXP to value 0xffff_ffff"]
impl crate::Resettable for NOOFPOLLSBEFEXP_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
