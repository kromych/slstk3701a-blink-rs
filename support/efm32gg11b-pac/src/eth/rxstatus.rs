#[doc = "Register `RXSTATUS` reader"]
pub type R = crate::R<RXSTATUS_SPEC>;
#[doc = "Register `RXSTATUS` writer"]
pub type W = crate::W<RXSTATUS_SPEC>;
#[doc = "Field `BUFFNOTAVAIL` reader - Buffer not available"]
pub type BUFFNOTAVAIL_R = crate::BitReader;
#[doc = "Field `BUFFNOTAVAIL` writer - Buffer not available"]
pub type BUFFNOTAVAIL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FRMRX` reader - Frame received"]
pub type FRMRX_R = crate::BitReader;
#[doc = "Field `FRMRX` writer - Frame received"]
pub type FRMRX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXOVERRUN` reader - Receive overrun"]
pub type RXOVERRUN_R = crate::BitReader;
#[doc = "Field `RXOVERRUN` writer - Receive overrun"]
pub type RXOVERRUN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESPNOTOK` reader - bresp/hresp not OK"]
pub type RESPNOTOK_R = crate::BitReader;
#[doc = "Field `RESPNOTOK` writer - bresp/hresp not OK"]
pub type RESPNOTOK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Buffer not available"]
    #[inline(always)]
    pub fn buffnotavail(&self) -> BUFFNOTAVAIL_R {
        BUFFNOTAVAIL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frame received"]
    #[inline(always)]
    pub fn frmrx(&self) -> FRMRX_R {
        FRMRX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive overrun"]
    #[inline(always)]
    pub fn rxoverrun(&self) -> RXOVERRUN_R {
        RXOVERRUN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - bresp/hresp not OK"]
    #[inline(always)]
    pub fn respnotok(&self) -> RESPNOTOK_R {
        RESPNOTOK_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Buffer not available"]
    #[inline(always)]
    #[must_use]
    pub fn buffnotavail(&mut self) -> BUFFNOTAVAIL_W<RXSTATUS_SPEC, 0> {
        BUFFNOTAVAIL_W::new(self)
    }
    #[doc = "Bit 1 - Frame received"]
    #[inline(always)]
    #[must_use]
    pub fn frmrx(&mut self) -> FRMRX_W<RXSTATUS_SPEC, 1> {
        FRMRX_W::new(self)
    }
    #[doc = "Bit 2 - Receive overrun"]
    #[inline(always)]
    #[must_use]
    pub fn rxoverrun(&mut self) -> RXOVERRUN_W<RXSTATUS_SPEC, 2> {
        RXOVERRUN_W::new(self)
    }
    #[doc = "Bit 3 - bresp/hresp not OK"]
    #[inline(always)]
    #[must_use]
    pub fn respnotok(&mut self) -> RESPNOTOK_W<RXSTATUS_SPEC, 3> {
        RESPNOTOK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Receive status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxstatus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxstatus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXSTATUS_SPEC;
impl crate::RegisterSpec for RXSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxstatus::R`](R) reader structure"]
impl crate::Readable for RXSTATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxstatus::W`](W) writer structure"]
impl crate::Writable for RXSTATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXSTATUS to value 0"]
impl crate::Resettable for RXSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
