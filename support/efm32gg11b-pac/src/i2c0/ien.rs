#[doc = "Register `IEN` reader"]
pub type R = crate::R<IEN_SPEC>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IEN_SPEC>;
#[doc = "Field `START` reader - START Interrupt Enable"]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - START Interrupt Enable"]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTART` reader - RSTART Interrupt Enable"]
pub type RSTART_R = crate::BitReader;
#[doc = "Field `RSTART` writer - RSTART Interrupt Enable"]
pub type RSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR` reader - ADDR Interrupt Enable"]
pub type ADDR_R = crate::BitReader;
#[doc = "Field `ADDR` writer - ADDR Interrupt Enable"]
pub type ADDR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXC` reader - TXC Interrupt Enable"]
pub type TXC_R = crate::BitReader;
#[doc = "Field `TXC` writer - TXC Interrupt Enable"]
pub type TXC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBL` reader - TXBL Interrupt Enable"]
pub type TXBL_R = crate::BitReader;
#[doc = "Field `TXBL` writer - TXBL Interrupt Enable"]
pub type TXBL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDATAV` reader - RXDATAV Interrupt Enable"]
pub type RXDATAV_R = crate::BitReader;
#[doc = "Field `RXDATAV` writer - RXDATAV Interrupt Enable"]
pub type RXDATAV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` reader - ACK Interrupt Enable"]
pub type ACK_R = crate::BitReader;
#[doc = "Field `ACK` writer - ACK Interrupt Enable"]
pub type ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` reader - NACK Interrupt Enable"]
pub type NACK_R = crate::BitReader;
#[doc = "Field `NACK` writer - NACK Interrupt Enable"]
pub type NACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTOP` reader - MSTOP Interrupt Enable"]
pub type MSTOP_R = crate::BitReader;
#[doc = "Field `MSTOP` writer - MSTOP Interrupt Enable"]
pub type MSTOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBLOST` reader - ARBLOST Interrupt Enable"]
pub type ARBLOST_R = crate::BitReader;
#[doc = "Field `ARBLOST` writer - ARBLOST Interrupt Enable"]
pub type ARBLOST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSERR` reader - BUSERR Interrupt Enable"]
pub type BUSERR_R = crate::BitReader;
#[doc = "Field `BUSERR` writer - BUSERR Interrupt Enable"]
pub type BUSERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSHOLD` reader - BUSHOLD Interrupt Enable"]
pub type BUSHOLD_R = crate::BitReader;
#[doc = "Field `BUSHOLD` writer - BUSHOLD Interrupt Enable"]
pub type BUSHOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOF` reader - TXOF Interrupt Enable"]
pub type TXOF_R = crate::BitReader;
#[doc = "Field `TXOF` writer - TXOF Interrupt Enable"]
pub type TXOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUF` reader - RXUF Interrupt Enable"]
pub type RXUF_R = crate::BitReader;
#[doc = "Field `RXUF` writer - RXUF Interrupt Enable"]
pub type RXUF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BITO` reader - BITO Interrupt Enable"]
pub type BITO_R = crate::BitReader;
#[doc = "Field `BITO` writer - BITO Interrupt Enable"]
pub type BITO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLTO` reader - CLTO Interrupt Enable"]
pub type CLTO_R = crate::BitReader;
#[doc = "Field `CLTO` writer - CLTO Interrupt Enable"]
pub type CLTO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSTOP` reader - SSTOP Interrupt Enable"]
pub type SSTOP_R = crate::BitReader;
#[doc = "Field `SSTOP` writer - SSTOP Interrupt Enable"]
pub type SSTOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFULL` reader - RXFULL Interrupt Enable"]
pub type RXFULL_R = crate::BitReader;
#[doc = "Field `RXFULL` writer - RXFULL Interrupt Enable"]
pub type RXFULL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLERR` reader - CLERR Interrupt Enable"]
pub type CLERR_R = crate::BitReader;
#[doc = "Field `CLERR` writer - CLERR Interrupt Enable"]
pub type CLERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - START Interrupt Enable"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RSTART Interrupt Enable"]
    #[inline(always)]
    pub fn rstart(&self) -> RSTART_R {
        RSTART_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADDR Interrupt Enable"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TXC Interrupt Enable"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TXBL Interrupt Enable"]
    #[inline(always)]
    pub fn txbl(&self) -> TXBL_R {
        TXBL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXDATAV Interrupt Enable"]
    #[inline(always)]
    pub fn rxdatav(&self) -> RXDATAV_R {
        RXDATAV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ACK Interrupt Enable"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NACK Interrupt Enable"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MSTOP Interrupt Enable"]
    #[inline(always)]
    pub fn mstop(&self) -> MSTOP_R {
        MSTOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ARBLOST Interrupt Enable"]
    #[inline(always)]
    pub fn arblost(&self) -> ARBLOST_R {
        ARBLOST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BUSERR Interrupt Enable"]
    #[inline(always)]
    pub fn buserr(&self) -> BUSERR_R {
        BUSERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BUSHOLD Interrupt Enable"]
    #[inline(always)]
    pub fn bushold(&self) -> BUSHOLD_R {
        BUSHOLD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TXOF Interrupt Enable"]
    #[inline(always)]
    pub fn txof(&self) -> TXOF_R {
        TXOF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RXUF Interrupt Enable"]
    #[inline(always)]
    pub fn rxuf(&self) -> RXUF_R {
        RXUF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - BITO Interrupt Enable"]
    #[inline(always)]
    pub fn bito(&self) -> BITO_R {
        BITO_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CLTO Interrupt Enable"]
    #[inline(always)]
    pub fn clto(&self) -> CLTO_R {
        CLTO_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SSTOP Interrupt Enable"]
    #[inline(always)]
    pub fn sstop(&self) -> SSTOP_R {
        SSTOP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RXFULL Interrupt Enable"]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CLERR Interrupt Enable"]
    #[inline(always)]
    pub fn clerr(&self) -> CLERR_R {
        CLERR_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - START Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<IEN_SPEC> {
        START_W::new(self, 0)
    }
    #[doc = "Bit 1 - RSTART Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rstart(&mut self) -> RSTART_W<IEN_SPEC> {
        RSTART_W::new(self, 1)
    }
    #[doc = "Bit 2 - ADDR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<IEN_SPEC> {
        ADDR_W::new(self, 2)
    }
    #[doc = "Bit 3 - TXC Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txc(&mut self) -> TXC_W<IEN_SPEC> {
        TXC_W::new(self, 3)
    }
    #[doc = "Bit 4 - TXBL Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txbl(&mut self) -> TXBL_W<IEN_SPEC> {
        TXBL_W::new(self, 4)
    }
    #[doc = "Bit 5 - RXDATAV Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxdatav(&mut self) -> RXDATAV_W<IEN_SPEC> {
        RXDATAV_W::new(self, 5)
    }
    #[doc = "Bit 6 - ACK Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> ACK_W<IEN_SPEC> {
        ACK_W::new(self, 6)
    }
    #[doc = "Bit 7 - NACK Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NACK_W<IEN_SPEC> {
        NACK_W::new(self, 7)
    }
    #[doc = "Bit 8 - MSTOP Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mstop(&mut self) -> MSTOP_W<IEN_SPEC> {
        MSTOP_W::new(self, 8)
    }
    #[doc = "Bit 9 - ARBLOST Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn arblost(&mut self) -> ARBLOST_W<IEN_SPEC> {
        ARBLOST_W::new(self, 9)
    }
    #[doc = "Bit 10 - BUSERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn buserr(&mut self) -> BUSERR_W<IEN_SPEC> {
        BUSERR_W::new(self, 10)
    }
    #[doc = "Bit 11 - BUSHOLD Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bushold(&mut self) -> BUSHOLD_W<IEN_SPEC> {
        BUSHOLD_W::new(self, 11)
    }
    #[doc = "Bit 12 - TXOF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txof(&mut self) -> TXOF_W<IEN_SPEC> {
        TXOF_W::new(self, 12)
    }
    #[doc = "Bit 13 - RXUF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxuf(&mut self) -> RXUF_W<IEN_SPEC> {
        RXUF_W::new(self, 13)
    }
    #[doc = "Bit 14 - BITO Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bito(&mut self) -> BITO_W<IEN_SPEC> {
        BITO_W::new(self, 14)
    }
    #[doc = "Bit 15 - CLTO Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn clto(&mut self) -> CLTO_W<IEN_SPEC> {
        CLTO_W::new(self, 15)
    }
    #[doc = "Bit 16 - SSTOP Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sstop(&mut self) -> SSTOP_W<IEN_SPEC> {
        SSTOP_W::new(self, 16)
    }
    #[doc = "Bit 17 - RXFULL Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxfull(&mut self) -> RXFULL_W<IEN_SPEC> {
        RXFULL_W::new(self, 17)
    }
    #[doc = "Bit 18 - CLERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn clerr(&mut self) -> CLERR_W<IEN_SPEC> {
        CLERR_W::new(self, 18)
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
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ien::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
