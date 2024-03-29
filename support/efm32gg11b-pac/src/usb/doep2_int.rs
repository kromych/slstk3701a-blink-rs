#[doc = "Register `DOEP2_INT` reader"]
pub type R = crate::R<DOEP2_INT_SPEC>;
#[doc = "Register `DOEP2_INT` writer"]
pub type W = crate::W<DOEP2_INT_SPEC>;
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
#[doc = "Field `SETUP` reader - Setup Phase Done"]
pub type SETUP_R = crate::BitReader;
#[doc = "Field `SETUP` writer - Setup Phase Done"]
pub type SETUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTTKNEPDIS` reader - OUT Token Received When Endpoint Disabled"]
pub type OUTTKNEPDIS_R = crate::BitReader;
#[doc = "Field `OUTTKNEPDIS` writer - OUT Token Received When Endpoint Disabled"]
pub type OUTTKNEPDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STSPHSERCVD` reader - Status Phase Received For Control Write"]
pub type STSPHSERCVD_R = crate::BitReader;
#[doc = "Field `STSPHSERCVD` writer - Status Phase Received For Control Write"]
pub type STSPHSERCVD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BACK2BACKSETUP` reader - Back-to-Back SETUP Packets Received"]
pub type BACK2BACKSETUP_R = crate::BitReader;
#[doc = "Field `BACK2BACKSETUP` writer - Back-to-Back SETUP Packets Received"]
pub type BACK2BACKSETUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTPKTERR` reader - OUT Packet Error"]
pub type OUTPKTERR_R = crate::BitReader;
#[doc = "Field `OUTPKTERR` writer - OUT Packet Error"]
pub type OUTPKTERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKTDRPSTS` reader - Packet Drop Status"]
pub type PKTDRPSTS_R = crate::BitReader;
#[doc = "Field `PKTDRPSTS` writer - Packet Drop Status"]
pub type PKTDRPSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBLEERR` reader - Babble Error"]
pub type BBLEERR_R = crate::BitReader;
#[doc = "Field `BBLEERR` writer - Babble Error"]
pub type BBLEERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKINTRPT` reader - NAK Interrupt"]
pub type NAKINTRPT_R = crate::BitReader;
#[doc = "Field `NAKINTRPT` writer - NAK Interrupt"]
pub type NAKINTRPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STUPPKTRCVD` reader - "]
pub type STUPPKTRCVD_R = crate::BitReader;
#[doc = "Field `STUPPKTRCVD` writer - "]
pub type STUPPKTRCVD_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 3 - Setup Phase Done"]
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OUT Token Received When Endpoint Disabled"]
    #[inline(always)]
    pub fn outtknepdis(&self) -> OUTTKNEPDIS_R {
        OUTTKNEPDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Status Phase Received For Control Write"]
    #[inline(always)]
    pub fn stsphsercvd(&self) -> STSPHSERCVD_R {
        STSPHSERCVD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Back-to-Back SETUP Packets Received"]
    #[inline(always)]
    pub fn back2backsetup(&self) -> BACK2BACKSETUP_R {
        BACK2BACKSETUP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - OUT Packet Error"]
    #[inline(always)]
    pub fn outpkterr(&self) -> OUTPKTERR_R {
        OUTPKTERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Packet Drop Status"]
    #[inline(always)]
    pub fn pktdrpsts(&self) -> PKTDRPSTS_R {
        PKTDRPSTS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Babble Error"]
    #[inline(always)]
    pub fn bbleerr(&self) -> BBLEERR_R {
        BBLEERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NAK Interrupt"]
    #[inline(always)]
    pub fn nakintrpt(&self) -> NAKINTRPT_R {
        NAKINTRPT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn stuppktrcvd(&self) -> STUPPKTRCVD_R {
        STUPPKTRCVD_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn xfercompl(&mut self) -> XFERCOMPL_W<DOEP2_INT_SPEC> {
        XFERCOMPL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn epdisbld(&mut self) -> EPDISBLD_W<DOEP2_INT_SPEC> {
        EPDISBLD_W::new(self, 1)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    #[must_use]
    pub fn ahberr(&mut self) -> AHBERR_W<DOEP2_INT_SPEC> {
        AHBERR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Setup Phase Done"]
    #[inline(always)]
    #[must_use]
    pub fn setup(&mut self) -> SETUP_W<DOEP2_INT_SPEC> {
        SETUP_W::new(self, 3)
    }
    #[doc = "Bit 4 - OUT Token Received When Endpoint Disabled"]
    #[inline(always)]
    #[must_use]
    pub fn outtknepdis(&mut self) -> OUTTKNEPDIS_W<DOEP2_INT_SPEC> {
        OUTTKNEPDIS_W::new(self, 4)
    }
    #[doc = "Bit 5 - Status Phase Received For Control Write"]
    #[inline(always)]
    #[must_use]
    pub fn stsphsercvd(&mut self) -> STSPHSERCVD_W<DOEP2_INT_SPEC> {
        STSPHSERCVD_W::new(self, 5)
    }
    #[doc = "Bit 6 - Back-to-Back SETUP Packets Received"]
    #[inline(always)]
    #[must_use]
    pub fn back2backsetup(&mut self) -> BACK2BACKSETUP_W<DOEP2_INT_SPEC> {
        BACK2BACKSETUP_W::new(self, 6)
    }
    #[doc = "Bit 8 - OUT Packet Error"]
    #[inline(always)]
    #[must_use]
    pub fn outpkterr(&mut self) -> OUTPKTERR_W<DOEP2_INT_SPEC> {
        OUTPKTERR_W::new(self, 8)
    }
    #[doc = "Bit 11 - Packet Drop Status"]
    #[inline(always)]
    #[must_use]
    pub fn pktdrpsts(&mut self) -> PKTDRPSTS_W<DOEP2_INT_SPEC> {
        PKTDRPSTS_W::new(self, 11)
    }
    #[doc = "Bit 12 - Babble Error"]
    #[inline(always)]
    #[must_use]
    pub fn bbleerr(&mut self) -> BBLEERR_W<DOEP2_INT_SPEC> {
        BBLEERR_W::new(self, 12)
    }
    #[doc = "Bit 13 - NAK Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nakintrpt(&mut self) -> NAKINTRPT_W<DOEP2_INT_SPEC> {
        NAKINTRPT_W::new(self, 13)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn stuppktrcvd(&mut self) -> STUPPKTRCVD_W<DOEP2_INT_SPEC> {
        STUPPKTRCVD_W::new(self, 15)
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
#[doc = "Device OUT Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep2_int::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep2_int::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEP2_INT_SPEC;
impl crate::RegisterSpec for DOEP2_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doep2_int::R`](R) reader structure"]
impl crate::Readable for DOEP2_INT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doep2_int::W`](W) writer structure"]
impl crate::Writable for DOEP2_INT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEP2_INT to value 0"]
impl crate::Resettable for DOEP2_INT_SPEC {
    const RESET_VALUE: u32 = 0;
}
