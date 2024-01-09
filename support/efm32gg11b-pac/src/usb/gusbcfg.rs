#[doc = "Register `GUSBCFG` reader"]
pub type R = crate::R<GUSBCFG_SPEC>;
#[doc = "Register `GUSBCFG` writer"]
pub type W = crate::W<GUSBCFG_SPEC>;
#[doc = "Field `TOUTCAL` reader - Timeout Calibration (host and device)"]
pub type TOUTCAL_R = crate::FieldReader;
#[doc = "Field `TOUTCAL` writer - Timeout Calibration (host and device)"]
pub type TOUTCAL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FSINTF` reader - Full-Speed Serial Interface Select"]
pub type FSINTF_R = crate::BitReader;
#[doc = "Field `FSINTF` writer - Full-Speed Serial Interface Select"]
pub type FSINTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRPCAP` reader - SRP-Capable"]
pub type SRPCAP_R = crate::BitReader;
#[doc = "Field `SRPCAP` writer - SRP-Capable"]
pub type SRPCAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HNPCAP` reader - HNP-Capable"]
pub type HNPCAP_R = crate::BitReader;
#[doc = "Field `HNPCAP` writer - HNP-Capable"]
pub type HNPCAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBTRDTIM` reader - USB Turnaround Time"]
pub type USBTRDTIM_R = crate::FieldReader;
#[doc = "Field `USBTRDTIM` writer - USB Turnaround Time"]
pub type USBTRDTIM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TERMSELDLPULSE` reader - TermSel DLine Pulsing Selection"]
pub type TERMSELDLPULSE_R = crate::BitReader;
#[doc = "Field `TERMSELDLPULSE` writer - TermSel DLine Pulsing Selection"]
pub type TERMSELDLPULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXENDDELAY` reader - Tx End Delay"]
pub type TXENDDELAY_R = crate::BitReader;
#[doc = "Field `TXENDDELAY` writer - Tx End Delay"]
pub type TXENDDELAY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCEHSTMODE` reader - Force Host Mode"]
pub type FORCEHSTMODE_R = crate::BitReader;
#[doc = "Field `FORCEHSTMODE` writer - Force Host Mode"]
pub type FORCEHSTMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCEDEVMODE` reader - Force Device Mode"]
pub type FORCEDEVMODE_R = crate::BitReader;
#[doc = "Field `FORCEDEVMODE` writer - Force Device Mode"]
pub type FORCEDEVMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORRUPTTXPKT` writer - Corrupt Tx packet (host and device)"]
pub type CORRUPTTXPKT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Timeout Calibration (host and device)"]
    #[inline(always)]
    pub fn toutcal(&self) -> TOUTCAL_R {
        TOUTCAL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 5 - Full-Speed Serial Interface Select"]
    #[inline(always)]
    pub fn fsintf(&self) -> FSINTF_R {
        FSINTF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - SRP-Capable"]
    #[inline(always)]
    pub fn srpcap(&self) -> SRPCAP_R {
        SRPCAP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HNP-Capable"]
    #[inline(always)]
    pub fn hnpcap(&self) -> HNPCAP_R {
        HNPCAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - USB Turnaround Time"]
    #[inline(always)]
    pub fn usbtrdtim(&self) -> USBTRDTIM_R {
        USBTRDTIM_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - TermSel DLine Pulsing Selection"]
    #[inline(always)]
    pub fn termseldlpulse(&self) -> TERMSELDLPULSE_R {
        TERMSELDLPULSE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 28 - Tx End Delay"]
    #[inline(always)]
    pub fn txenddelay(&self) -> TXENDDELAY_R {
        TXENDDELAY_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Force Host Mode"]
    #[inline(always)]
    pub fn forcehstmode(&self) -> FORCEHSTMODE_R {
        FORCEHSTMODE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Force Device Mode"]
    #[inline(always)]
    pub fn forcedevmode(&self) -> FORCEDEVMODE_R {
        FORCEDEVMODE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Timeout Calibration (host and device)"]
    #[inline(always)]
    #[must_use]
    pub fn toutcal(&mut self) -> TOUTCAL_W<GUSBCFG_SPEC> {
        TOUTCAL_W::new(self, 0)
    }
    #[doc = "Bit 5 - Full-Speed Serial Interface Select"]
    #[inline(always)]
    #[must_use]
    pub fn fsintf(&mut self) -> FSINTF_W<GUSBCFG_SPEC> {
        FSINTF_W::new(self, 5)
    }
    #[doc = "Bit 8 - SRP-Capable"]
    #[inline(always)]
    #[must_use]
    pub fn srpcap(&mut self) -> SRPCAP_W<GUSBCFG_SPEC> {
        SRPCAP_W::new(self, 8)
    }
    #[doc = "Bit 9 - HNP-Capable"]
    #[inline(always)]
    #[must_use]
    pub fn hnpcap(&mut self) -> HNPCAP_W<GUSBCFG_SPEC> {
        HNPCAP_W::new(self, 9)
    }
    #[doc = "Bits 10:13 - USB Turnaround Time"]
    #[inline(always)]
    #[must_use]
    pub fn usbtrdtim(&mut self) -> USBTRDTIM_W<GUSBCFG_SPEC> {
        USBTRDTIM_W::new(self, 10)
    }
    #[doc = "Bit 22 - TermSel DLine Pulsing Selection"]
    #[inline(always)]
    #[must_use]
    pub fn termseldlpulse(&mut self) -> TERMSELDLPULSE_W<GUSBCFG_SPEC> {
        TERMSELDLPULSE_W::new(self, 22)
    }
    #[doc = "Bit 28 - Tx End Delay"]
    #[inline(always)]
    #[must_use]
    pub fn txenddelay(&mut self) -> TXENDDELAY_W<GUSBCFG_SPEC> {
        TXENDDELAY_W::new(self, 28)
    }
    #[doc = "Bit 29 - Force Host Mode"]
    #[inline(always)]
    #[must_use]
    pub fn forcehstmode(&mut self) -> FORCEHSTMODE_W<GUSBCFG_SPEC> {
        FORCEHSTMODE_W::new(self, 29)
    }
    #[doc = "Bit 30 - Force Device Mode"]
    #[inline(always)]
    #[must_use]
    pub fn forcedevmode(&mut self) -> FORCEDEVMODE_W<GUSBCFG_SPEC> {
        FORCEDEVMODE_W::new(self, 30)
    }
    #[doc = "Bit 31 - Corrupt Tx packet (host and device)"]
    #[inline(always)]
    #[must_use]
    pub fn corrupttxpkt(&mut self) -> CORRUPTTXPKT_W<GUSBCFG_SPEC> {
        CORRUPTTXPKT_W::new(self, 31)
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
#[doc = "USB Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gusbcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gusbcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GUSBCFG_SPEC;
impl crate::RegisterSpec for GUSBCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gusbcfg::R`](R) reader structure"]
impl crate::Readable for GUSBCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gusbcfg::W`](W) writer structure"]
impl crate::Writable for GUSBCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GUSBCFG to value 0x1400"]
impl crate::Resettable for GUSBCFG_SPEC {
    const RESET_VALUE: u32 = 0x1400;
}
