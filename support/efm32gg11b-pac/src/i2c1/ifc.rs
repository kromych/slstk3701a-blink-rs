#[doc = "Register `IFC` writer"]
pub type W = crate::W<IFC_SPEC>;
#[doc = "Field `START` writer - Clear START Interrupt Flag"]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTART` writer - Clear RSTART Interrupt Flag"]
pub type RSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR` writer - Clear ADDR Interrupt Flag"]
pub type ADDR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXC` writer - Clear TXC Interrupt Flag"]
pub type TXC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` writer - Clear ACK Interrupt Flag"]
pub type ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` writer - Clear NACK Interrupt Flag"]
pub type NACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTOP` writer - Clear MSTOP Interrupt Flag"]
pub type MSTOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBLOST` writer - Clear ARBLOST Interrupt Flag"]
pub type ARBLOST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSERR` writer - Clear BUSERR Interrupt Flag"]
pub type BUSERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSHOLD` writer - Clear BUSHOLD Interrupt Flag"]
pub type BUSHOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOF` writer - Clear TXOF Interrupt Flag"]
pub type TXOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUF` writer - Clear RXUF Interrupt Flag"]
pub type RXUF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BITO` writer - Clear BITO Interrupt Flag"]
pub type BITO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLTO` writer - Clear CLTO Interrupt Flag"]
pub type CLTO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSTOP` writer - Clear SSTOP Interrupt Flag"]
pub type SSTOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFULL` writer - Clear RXFULL Interrupt Flag"]
pub type RXFULL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLERR` writer - Clear CLERR Interrupt Flag"]
pub type CLERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear START Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<IFC_SPEC> {
        START_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear RSTART Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rstart(&mut self) -> RSTART_W<IFC_SPEC> {
        RSTART_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear ADDR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<IFC_SPEC> {
        ADDR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear TXC Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txc(&mut self) -> TXC_W<IFC_SPEC> {
        TXC_W::new(self, 3)
    }
    #[doc = "Bit 6 - Clear ACK Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> ACK_W<IFC_SPEC> {
        ACK_W::new(self, 6)
    }
    #[doc = "Bit 7 - Clear NACK Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NACK_W<IFC_SPEC> {
        NACK_W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear MSTOP Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn mstop(&mut self) -> MSTOP_W<IFC_SPEC> {
        MSTOP_W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear ARBLOST Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn arblost(&mut self) -> ARBLOST_W<IFC_SPEC> {
        ARBLOST_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear BUSERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn buserr(&mut self) -> BUSERR_W<IFC_SPEC> {
        BUSERR_W::new(self, 10)
    }
    #[doc = "Bit 11 - Clear BUSHOLD Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn bushold(&mut self) -> BUSHOLD_W<IFC_SPEC> {
        BUSHOLD_W::new(self, 11)
    }
    #[doc = "Bit 12 - Clear TXOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txof(&mut self) -> TXOF_W<IFC_SPEC> {
        TXOF_W::new(self, 12)
    }
    #[doc = "Bit 13 - Clear RXUF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxuf(&mut self) -> RXUF_W<IFC_SPEC> {
        RXUF_W::new(self, 13)
    }
    #[doc = "Bit 14 - Clear BITO Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn bito(&mut self) -> BITO_W<IFC_SPEC> {
        BITO_W::new(self, 14)
    }
    #[doc = "Bit 15 - Clear CLTO Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn clto(&mut self) -> CLTO_W<IFC_SPEC> {
        CLTO_W::new(self, 15)
    }
    #[doc = "Bit 16 - Clear SSTOP Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn sstop(&mut self) -> SSTOP_W<IFC_SPEC> {
        SSTOP_W::new(self, 16)
    }
    #[doc = "Bit 17 - Clear RXFULL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxfull(&mut self) -> RXFULL_W<IFC_SPEC> {
        RXFULL_W::new(self, 17)
    }
    #[doc = "Bit 18 - Clear CLERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn clerr(&mut self) -> CLERR_W<IFC_SPEC> {
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
#[doc = "Interrupt Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFC_SPEC;
impl crate::RegisterSpec for IFC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifc::W`](W) writer structure"]
impl crate::Writable for IFC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IFC_SPEC {
    const RESET_VALUE: u32 = 0;
}
