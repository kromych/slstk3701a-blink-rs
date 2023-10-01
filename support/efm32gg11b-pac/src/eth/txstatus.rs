#[doc = "Register `TXSTATUS` reader"]
pub type R = crate::R<TXSTATUS_SPEC>;
#[doc = "Register `TXSTATUS` writer"]
pub type W = crate::W<TXSTATUS_SPEC>;
#[doc = "Field `USEDBITREAD` reader - Used bit read"]
pub type USEDBITREAD_R = crate::BitReader;
#[doc = "Field `USEDBITREAD` writer - Used bit read"]
pub type USEDBITREAD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COLOCCRD` reader - Collision occurred"]
pub type COLOCCRD_R = crate::BitReader;
#[doc = "Field `COLOCCRD` writer - Collision occurred"]
pub type COLOCCRD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RETRYLMTEXCD` reader - Retry limit exceeded"]
pub type RETRYLMTEXCD_R = crate::BitReader;
#[doc = "Field `RETRYLMTEXCD` writer - Retry limit exceeded"]
pub type RETRYLMTEXCD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXGO` reader - Transmit go"]
pub type TXGO_R = crate::BitReader;
#[doc = "Field `AMBAERR` reader - Transmit frame corruption due to AMBA (AHB) errors."]
pub type AMBAERR_R = crate::BitReader;
#[doc = "Field `AMBAERR` writer - Transmit frame corruption due to AMBA (AHB) errors."]
pub type AMBAERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXCMPLT` reader - Transmit complete"]
pub type TXCMPLT_R = crate::BitReader;
#[doc = "Field `TXCMPLT` writer - Transmit complete"]
pub type TXCMPLT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXUNDERRUN` reader - Transmit under run"]
pub type TXUNDERRUN_R = crate::BitReader;
#[doc = "Field `TXUNDERRUN` writer - Transmit under run"]
pub type TXUNDERRUN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LATECOLOCCRD` reader - Late collision occurred"]
pub type LATECOLOCCRD_R = crate::BitReader;
#[doc = "Field `LATECOLOCCRD` writer - Late collision occurred"]
pub type LATECOLOCCRD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESPNOTOK` reader - bresp/hresp not OK"]
pub type RESPNOTOK_R = crate::BitReader;
#[doc = "Field `RESPNOTOK` writer - bresp/hresp not OK"]
pub type RESPNOTOK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Used bit read"]
    #[inline(always)]
    pub fn usedbitread(&self) -> USEDBITREAD_R {
        USEDBITREAD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Collision occurred"]
    #[inline(always)]
    pub fn coloccrd(&self) -> COLOCCRD_R {
        COLOCCRD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Retry limit exceeded"]
    #[inline(always)]
    pub fn retrylmtexcd(&self) -> RETRYLMTEXCD_R {
        RETRYLMTEXCD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit go"]
    #[inline(always)]
    pub fn txgo(&self) -> TXGO_R {
        TXGO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit frame corruption due to AMBA (AHB) errors."]
    #[inline(always)]
    pub fn ambaerr(&self) -> AMBAERR_R {
        AMBAERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit complete"]
    #[inline(always)]
    pub fn txcmplt(&self) -> TXCMPLT_R {
        TXCMPLT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit under run"]
    #[inline(always)]
    pub fn txunderrun(&self) -> TXUNDERRUN_R {
        TXUNDERRUN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Late collision occurred"]
    #[inline(always)]
    pub fn latecoloccrd(&self) -> LATECOLOCCRD_R {
        LATECOLOCCRD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - bresp/hresp not OK"]
    #[inline(always)]
    pub fn respnotok(&self) -> RESPNOTOK_R {
        RESPNOTOK_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Used bit read"]
    #[inline(always)]
    #[must_use]
    pub fn usedbitread(&mut self) -> USEDBITREAD_W<TXSTATUS_SPEC, 0> {
        USEDBITREAD_W::new(self)
    }
    #[doc = "Bit 1 - Collision occurred"]
    #[inline(always)]
    #[must_use]
    pub fn coloccrd(&mut self) -> COLOCCRD_W<TXSTATUS_SPEC, 1> {
        COLOCCRD_W::new(self)
    }
    #[doc = "Bit 2 - Retry limit exceeded"]
    #[inline(always)]
    #[must_use]
    pub fn retrylmtexcd(&mut self) -> RETRYLMTEXCD_W<TXSTATUS_SPEC, 2> {
        RETRYLMTEXCD_W::new(self)
    }
    #[doc = "Bit 4 - Transmit frame corruption due to AMBA (AHB) errors."]
    #[inline(always)]
    #[must_use]
    pub fn ambaerr(&mut self) -> AMBAERR_W<TXSTATUS_SPEC, 4> {
        AMBAERR_W::new(self)
    }
    #[doc = "Bit 5 - Transmit complete"]
    #[inline(always)]
    #[must_use]
    pub fn txcmplt(&mut self) -> TXCMPLT_W<TXSTATUS_SPEC, 5> {
        TXCMPLT_W::new(self)
    }
    #[doc = "Bit 6 - Transmit under run"]
    #[inline(always)]
    #[must_use]
    pub fn txunderrun(&mut self) -> TXUNDERRUN_W<TXSTATUS_SPEC, 6> {
        TXUNDERRUN_W::new(self)
    }
    #[doc = "Bit 7 - Late collision occurred"]
    #[inline(always)]
    #[must_use]
    pub fn latecoloccrd(&mut self) -> LATECOLOCCRD_W<TXSTATUS_SPEC, 7> {
        LATECOLOCCRD_W::new(self)
    }
    #[doc = "Bit 8 - bresp/hresp not OK"]
    #[inline(always)]
    #[must_use]
    pub fn respnotok(&mut self) -> RESPNOTOK_W<TXSTATUS_SPEC, 8> {
        RESPNOTOK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transmit status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txstatus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txstatus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXSTATUS_SPEC;
impl crate::RegisterSpec for TXSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txstatus::R`](R) reader structure"]
impl crate::Readable for TXSTATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txstatus::W`](W) writer structure"]
impl crate::Writable for TXSTATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXSTATUS to value 0"]
impl crate::Resettable for TXSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
