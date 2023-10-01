#[doc = "Register `GOTGINT` reader"]
pub type R = crate::R<GOTGINT_SPEC>;
#[doc = "Register `GOTGINT` writer"]
pub type W = crate::W<GOTGINT_SPEC>;
#[doc = "Field `SESENDDET` reader - Session End Detected"]
pub type SESENDDET_R = crate::BitReader;
#[doc = "Field `SESENDDET` writer - Session End Detected"]
pub type SESENDDET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SESREQSUCSTSCHNG` reader - Session Request Success Status Change"]
pub type SESREQSUCSTSCHNG_R = crate::BitReader;
#[doc = "Field `SESREQSUCSTSCHNG` writer - Session Request Success Status Change"]
pub type SESREQSUCSTSCHNG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSTNEGSUCSTSCHNG` reader - Host Negotiation Success Status Change"]
pub type HSTNEGSUCSTSCHNG_R = crate::BitReader;
#[doc = "Field `HSTNEGSUCSTSCHNG` writer - Host Negotiation Success Status Change"]
pub type HSTNEGSUCSTSCHNG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSTNEGDET` reader - Host Negotiation Detected"]
pub type HSTNEGDET_R = crate::BitReader;
#[doc = "Field `HSTNEGDET` writer - Host Negotiation Detected"]
pub type HSTNEGDET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADEVTOUTCHG` reader - A-Device Timeout Change"]
pub type ADEVTOUTCHG_R = crate::BitReader;
#[doc = "Field `ADEVTOUTCHG` writer - A-Device Timeout Change"]
pub type ADEVTOUTCHG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DBNCEDONE` reader - Debounce Done"]
pub type DBNCEDONE_R = crate::BitReader;
#[doc = "Field `DBNCEDONE` writer - Debounce Done"]
pub type DBNCEDONE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 2 - Session End Detected"]
    #[inline(always)]
    pub fn sesenddet(&self) -> SESENDDET_R {
        SESENDDET_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Session Request Success Status Change"]
    #[inline(always)]
    pub fn sesreqsucstschng(&self) -> SESREQSUCSTSCHNG_R {
        SESREQSUCSTSCHNG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Host Negotiation Success Status Change"]
    #[inline(always)]
    pub fn hstnegsucstschng(&self) -> HSTNEGSUCSTSCHNG_R {
        HSTNEGSUCSTSCHNG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 17 - Host Negotiation Detected"]
    #[inline(always)]
    pub fn hstnegdet(&self) -> HSTNEGDET_R {
        HSTNEGDET_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - A-Device Timeout Change"]
    #[inline(always)]
    pub fn adevtoutchg(&self) -> ADEVTOUTCHG_R {
        ADEVTOUTCHG_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Debounce Done"]
    #[inline(always)]
    pub fn dbncedone(&self) -> DBNCEDONE_R {
        DBNCEDONE_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Session End Detected"]
    #[inline(always)]
    #[must_use]
    pub fn sesenddet(&mut self) -> SESENDDET_W<GOTGINT_SPEC, 2> {
        SESENDDET_W::new(self)
    }
    #[doc = "Bit 8 - Session Request Success Status Change"]
    #[inline(always)]
    #[must_use]
    pub fn sesreqsucstschng(&mut self) -> SESREQSUCSTSCHNG_W<GOTGINT_SPEC, 8> {
        SESREQSUCSTSCHNG_W::new(self)
    }
    #[doc = "Bit 9 - Host Negotiation Success Status Change"]
    #[inline(always)]
    #[must_use]
    pub fn hstnegsucstschng(&mut self) -> HSTNEGSUCSTSCHNG_W<GOTGINT_SPEC, 9> {
        HSTNEGSUCSTSCHNG_W::new(self)
    }
    #[doc = "Bit 17 - Host Negotiation Detected"]
    #[inline(always)]
    #[must_use]
    pub fn hstnegdet(&mut self) -> HSTNEGDET_W<GOTGINT_SPEC, 17> {
        HSTNEGDET_W::new(self)
    }
    #[doc = "Bit 18 - A-Device Timeout Change"]
    #[inline(always)]
    #[must_use]
    pub fn adevtoutchg(&mut self) -> ADEVTOUTCHG_W<GOTGINT_SPEC, 18> {
        ADEVTOUTCHG_W::new(self)
    }
    #[doc = "Bit 19 - Debounce Done"]
    #[inline(always)]
    #[must_use]
    pub fn dbncedone(&mut self) -> DBNCEDONE_W<GOTGINT_SPEC, 19> {
        DBNCEDONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "OTG Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gotgint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gotgint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GOTGINT_SPEC;
impl crate::RegisterSpec for GOTGINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gotgint::R`](R) reader structure"]
impl crate::Readable for GOTGINT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gotgint::W`](W) writer structure"]
impl crate::Writable for GOTGINT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GOTGINT to value 0"]
impl crate::Resettable for GOTGINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
