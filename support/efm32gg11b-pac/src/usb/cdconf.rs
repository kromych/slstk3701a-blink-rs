#[doc = "Register `CDCONF` reader"]
pub type R = crate::R<CDCONF_SPEC>;
#[doc = "Register `CDCONF` writer"]
pub type W = crate::W<CDCONF_SPEC>;
#[doc = "Field `DCDTOCONF` reader - DCD Timeout (TDCD_TIMEOUT) Configuration"]
pub type DCDTOCONF_R = crate::FieldReader<u16>;
#[doc = "Field `DCDTOCONF` writer - DCD Timeout (TDCD_TIMEOUT) Configuration"]
pub type DCDTOCONF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
impl R {
    #[doc = "Bits 0:9 - DCD Timeout (TDCD_TIMEOUT) Configuration"]
    #[inline(always)]
    pub fn dcdtoconf(&self) -> DCDTOCONF_R {
        DCDTOCONF_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - DCD Timeout (TDCD_TIMEOUT) Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn dcdtoconf(&mut self) -> DCDTOCONF_W<CDCONF_SPEC, 0> {
        DCDTOCONF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Charger Detect Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdconf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdconf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CDCONF_SPEC;
impl crate::RegisterSpec for CDCONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdconf::R`](R) reader structure"]
impl crate::Readable for CDCONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cdconf::W`](W) writer structure"]
impl crate::Writable for CDCONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CDCONF to value 0"]
impl crate::Resettable for CDCONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
