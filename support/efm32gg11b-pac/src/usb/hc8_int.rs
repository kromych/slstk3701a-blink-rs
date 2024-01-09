#[doc = "Register `HC8_INT` reader"]
pub type R = crate::R<HC8_INT_SPEC>;
#[doc = "Register `HC8_INT` writer"]
pub type W = crate::W<HC8_INT_SPEC>;
#[doc = "Field `XFERCOMPL` reader - Transfer Completed"]
pub type XFERCOMPL_R = crate::BitReader;
#[doc = "Field `XFERCOMPL` writer - Transfer Completed"]
pub type XFERCOMPL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHHLTD` reader - Channel Halted"]
pub type CHHLTD_R = crate::BitReader;
#[doc = "Field `CHHLTD` writer - Channel Halted"]
pub type CHHLTD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBERR` reader - AHB Error"]
pub type AHBERR_R = crate::BitReader;
#[doc = "Field `AHBERR` writer - AHB Error"]
pub type AHBERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL` reader - STALL Response Received Interrupt"]
pub type STALL_R = crate::BitReader;
#[doc = "Field `STALL` writer - STALL Response Received Interrupt"]
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAK` reader - NAK Response Received Interrupt"]
pub type NAK_R = crate::BitReader;
#[doc = "Field `NAK` writer - NAK Response Received Interrupt"]
pub type NAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` reader - ACK Response Received/Transmitted Interrupt"]
pub type ACK_R = crate::BitReader;
#[doc = "Field `ACK` writer - ACK Response Received/Transmitted Interrupt"]
pub type ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XACTERR` reader - Transaction Error"]
pub type XACTERR_R = crate::BitReader;
#[doc = "Field `XACTERR` writer - Transaction Error"]
pub type XACTERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBLERR` reader - Babble Error"]
pub type BBLERR_R = crate::BitReader;
#[doc = "Field `BBLERR` writer - Babble Error"]
pub type BBLERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRMOVRUN` reader - Frame Overrun"]
pub type FRMOVRUN_R = crate::BitReader;
#[doc = "Field `FRMOVRUN` writer - Frame Overrun"]
pub type FRMOVRUN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATATGLERR` reader - Data Toggle Error"]
pub type DATATGLERR_R = crate::BitReader;
#[doc = "Field `DATATGLERR` writer - Data Toggle Error"]
pub type DATATGLERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer Completed"]
    #[inline(always)]
    pub fn xfercompl(&self) -> XFERCOMPL_R {
        XFERCOMPL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Halted"]
    #[inline(always)]
    pub fn chhltd(&self) -> CHHLTD_R {
        CHHLTD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    pub fn ahberr(&self) -> AHBERR_R {
        AHBERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STALL Response Received Interrupt"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAK Response Received Interrupt"]
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ACK Response Received/Transmitted Interrupt"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Transaction Error"]
    #[inline(always)]
    pub fn xacterr(&self) -> XACTERR_R {
        XACTERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Babble Error"]
    #[inline(always)]
    pub fn bblerr(&self) -> BBLERR_R {
        BBLERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Frame Overrun"]
    #[inline(always)]
    pub fn frmovrun(&self) -> FRMOVRUN_R {
        FRMOVRUN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data Toggle Error"]
    #[inline(always)]
    pub fn datatglerr(&self) -> DATATGLERR_R {
        DATATGLERR_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed"]
    #[inline(always)]
    #[must_use]
    pub fn xfercompl(&mut self) -> XFERCOMPL_W<HC8_INT_SPEC> {
        XFERCOMPL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel Halted"]
    #[inline(always)]
    #[must_use]
    pub fn chhltd(&mut self) -> CHHLTD_W<HC8_INT_SPEC> {
        CHHLTD_W::new(self, 1)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    #[must_use]
    pub fn ahberr(&mut self) -> AHBERR_W<HC8_INT_SPEC> {
        AHBERR_W::new(self, 2)
    }
    #[doc = "Bit 3 - STALL Response Received Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<HC8_INT_SPEC> {
        STALL_W::new(self, 3)
    }
    #[doc = "Bit 4 - NAK Response Received Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nak(&mut self) -> NAK_W<HC8_INT_SPEC> {
        NAK_W::new(self, 4)
    }
    #[doc = "Bit 5 - ACK Response Received/Transmitted Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> ACK_W<HC8_INT_SPEC> {
        ACK_W::new(self, 5)
    }
    #[doc = "Bit 7 - Transaction Error"]
    #[inline(always)]
    #[must_use]
    pub fn xacterr(&mut self) -> XACTERR_W<HC8_INT_SPEC> {
        XACTERR_W::new(self, 7)
    }
    #[doc = "Bit 8 - Babble Error"]
    #[inline(always)]
    #[must_use]
    pub fn bblerr(&mut self) -> BBLERR_W<HC8_INT_SPEC> {
        BBLERR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Frame Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn frmovrun(&mut self) -> FRMOVRUN_W<HC8_INT_SPEC> {
        FRMOVRUN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Data Toggle Error"]
    #[inline(always)]
    #[must_use]
    pub fn datatglerr(&mut self) -> DATATGLERR_W<HC8_INT_SPEC> {
        DATATGLERR_W::new(self, 10)
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
#[doc = "Host Channel x Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc8_int::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc8_int::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HC8_INT_SPEC;
impl crate::RegisterSpec for HC8_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hc8_int::R`](R) reader structure"]
impl crate::Readable for HC8_INT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hc8_int::W`](W) writer structure"]
impl crate::Writable for HC8_INT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HC8_INT to value 0"]
impl crate::Resettable for HC8_INT_SPEC {
    const RESET_VALUE: u32 = 0;
}
