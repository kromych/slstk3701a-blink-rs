#[doc = "Register `DIEP0INT` reader"]
pub type R = crate::R<DIEP0INT_SPEC>;
#[doc = "Register `DIEP0INT` writer"]
pub type W = crate::W<DIEP0INT_SPEC>;
#[doc = "Field `XFERCOMPL` reader - Transfer Completed Interrupt"]
pub type XFERCOMPL_R = crate::BitReader;
#[doc = "Field `XFERCOMPL` writer - Transfer Completed Interrupt"]
pub type XFERCOMPL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISBLD` reader - Endpoint Disabled Interrupt"]
pub type EPDISBLD_R = crate::BitReader;
#[doc = "Field `EPDISBLD` writer - Endpoint Disabled Interrupt"]
pub type EPDISBLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBERR` reader - AHB Error"]
pub type AHBERR_R = crate::BitReader;
#[doc = "Field `AHBERR` writer - AHB Error"]
pub type AHBERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT` reader - Timeout Condition"]
pub type TIMEOUT_R = crate::BitReader;
#[doc = "Field `TIMEOUT` writer - Timeout Condition"]
pub type TIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTKNTXFEMP` reader - IN Token Received When TxFIFO is Empty"]
pub type INTKNTXFEMP_R = crate::BitReader;
#[doc = "Field `INTKNTXFEMP` writer - IN Token Received When TxFIFO is Empty"]
pub type INTKNTXFEMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTKNEPMIS` reader - IN Token Received with EP Mismatch"]
pub type INTKNEPMIS_R = crate::BitReader;
#[doc = "Field `INTKNEPMIS` writer - IN Token Received with EP Mismatch"]
pub type INTKNEPMIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPNAKEFF` reader - IN Endpoint NAK Effective"]
pub type INEPNAKEFF_R = crate::BitReader;
#[doc = "Field `INEPNAKEFF` writer - IN Endpoint NAK Effective"]
pub type INEPNAKEFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFEMP` reader - Transmit FIFO Empty"]
pub type TXFEMP_R = crate::BitReader;
#[doc = "Field `TXFIFOUNDRN` reader - Fifo Underrun"]
pub type TXFIFOUNDRN_R = crate::BitReader;
#[doc = "Field `TXFIFOUNDRN` writer - Fifo Underrun"]
pub type TXFIFOUNDRN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKTDRPSTS` reader - Packet Drop Status"]
pub type PKTDRPSTS_R = crate::BitReader;
#[doc = "Field `PKTDRPSTS` writer - Packet Drop Status"]
pub type PKTDRPSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBLEERR` reader - Babble Interrupt"]
pub type BBLEERR_R = crate::BitReader;
#[doc = "Field `BBLEERR` writer - Babble Interrupt"]
pub type BBLEERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKINTRPT` reader - NAK Interrupt"]
pub type NAKINTRPT_R = crate::BitReader;
#[doc = "Field `NAKINTRPT` writer - NAK Interrupt"]
pub type NAKINTRPT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer Completed Interrupt"]
    #[inline(always)]
    pub fn xfercompl(&self) -> XFERCOMPL_R {
        XFERCOMPL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt"]
    #[inline(always)]
    pub fn epdisbld(&self) -> EPDISBLD_R {
        EPDISBLD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    pub fn ahberr(&self) -> AHBERR_R {
        AHBERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timeout Condition"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IN Token Received When TxFIFO is Empty"]
    #[inline(always)]
    pub fn intkntxfemp(&self) -> INTKNTXFEMP_R {
        INTKNTXFEMP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IN Token Received with EP Mismatch"]
    #[inline(always)]
    pub fn intknepmis(&self) -> INTKNEPMIS_R {
        INTKNEPMIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IN Endpoint NAK Effective"]
    #[inline(always)]
    pub fn inepnakeff(&self) -> INEPNAKEFF_R {
        INEPNAKEFF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO Empty"]
    #[inline(always)]
    pub fn txfemp(&self) -> TXFEMP_R {
        TXFEMP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Fifo Underrun"]
    #[inline(always)]
    pub fn txfifoundrn(&self) -> TXFIFOUNDRN_R {
        TXFIFOUNDRN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Packet Drop Status"]
    #[inline(always)]
    pub fn pktdrpsts(&self) -> PKTDRPSTS_R {
        PKTDRPSTS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Babble Interrupt"]
    #[inline(always)]
    pub fn bbleerr(&self) -> BBLEERR_R {
        BBLEERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NAK Interrupt"]
    #[inline(always)]
    pub fn nakintrpt(&self) -> NAKINTRPT_R {
        NAKINTRPT_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn xfercompl(&mut self) -> XFERCOMPL_W<DIEP0INT_SPEC> {
        XFERCOMPL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn epdisbld(&mut self) -> EPDISBLD_W<DIEP0INT_SPEC> {
        EPDISBLD_W::new(self, 1)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    #[must_use]
    pub fn ahberr(&mut self) -> AHBERR_W<DIEP0INT_SPEC> {
        AHBERR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Timeout Condition"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TIMEOUT_W<DIEP0INT_SPEC> {
        TIMEOUT_W::new(self, 3)
    }
    #[doc = "Bit 4 - IN Token Received When TxFIFO is Empty"]
    #[inline(always)]
    #[must_use]
    pub fn intkntxfemp(&mut self) -> INTKNTXFEMP_W<DIEP0INT_SPEC> {
        INTKNTXFEMP_W::new(self, 4)
    }
    #[doc = "Bit 5 - IN Token Received with EP Mismatch"]
    #[inline(always)]
    #[must_use]
    pub fn intknepmis(&mut self) -> INTKNEPMIS_W<DIEP0INT_SPEC> {
        INTKNEPMIS_W::new(self, 5)
    }
    #[doc = "Bit 6 - IN Endpoint NAK Effective"]
    #[inline(always)]
    #[must_use]
    pub fn inepnakeff(&mut self) -> INEPNAKEFF_W<DIEP0INT_SPEC> {
        INEPNAKEFF_W::new(self, 6)
    }
    #[doc = "Bit 8 - Fifo Underrun"]
    #[inline(always)]
    #[must_use]
    pub fn txfifoundrn(&mut self) -> TXFIFOUNDRN_W<DIEP0INT_SPEC> {
        TXFIFOUNDRN_W::new(self, 8)
    }
    #[doc = "Bit 11 - Packet Drop Status"]
    #[inline(always)]
    #[must_use]
    pub fn pktdrpsts(&mut self) -> PKTDRPSTS_W<DIEP0INT_SPEC> {
        PKTDRPSTS_W::new(self, 11)
    }
    #[doc = "Bit 12 - Babble Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn bbleerr(&mut self) -> BBLEERR_W<DIEP0INT_SPEC> {
        BBLEERR_W::new(self, 12)
    }
    #[doc = "Bit 13 - NAK Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nakintrpt(&mut self) -> NAKINTRPT_W<DIEP0INT_SPEC> {
        NAKINTRPT_W::new(self, 13)
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
#[doc = "Device IN Endpoint 0 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0int::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep0int::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEP0INT_SPEC;
impl crate::RegisterSpec for DIEP0INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diep0int::R`](R) reader structure"]
impl crate::Readable for DIEP0INT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diep0int::W`](W) writer structure"]
impl crate::Writable for DIEP0INT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEP0INT to value 0x80"]
impl crate::Resettable for DIEP0INT_SPEC {
    const RESET_VALUE: u32 = 0x80;
}
