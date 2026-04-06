#[doc = "Register `IFC` writer"]
pub type W = crate::W<IfcSpec>;
#[doc = "Field `START` writer - Clear START Interrupt Flag"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTART` writer - Clear RSTART Interrupt Flag"]
pub type RstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR` writer - Clear ADDR Interrupt Flag"]
pub type AddrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXC` writer - Clear TXC Interrupt Flag"]
pub type TxcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` writer - Clear ACK Interrupt Flag"]
pub type AckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` writer - Clear NACK Interrupt Flag"]
pub type NackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTOP` writer - Clear MSTOP Interrupt Flag"]
pub type MstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBLOST` writer - Clear ARBLOST Interrupt Flag"]
pub type ArblostW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSERR` writer - Clear BUSERR Interrupt Flag"]
pub type BuserrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSHOLD` writer - Clear BUSHOLD Interrupt Flag"]
pub type BusholdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOF` writer - Clear TXOF Interrupt Flag"]
pub type TxofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUF` writer - Clear RXUF Interrupt Flag"]
pub type RxufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BITO` writer - Clear BITO Interrupt Flag"]
pub type BitoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLTO` writer - Clear CLTO Interrupt Flag"]
pub type CltoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSTOP` writer - Clear SSTOP Interrupt Flag"]
pub type SstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFULL` writer - Clear RXFULL Interrupt Flag"]
pub type RxfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLERR` writer - Clear CLERR Interrupt Flag"]
pub type ClerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear START Interrupt Flag"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, IfcSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear RSTART Interrupt Flag"]
    #[inline(always)]
    pub fn rstart(&mut self) -> RstartW<'_, IfcSpec> {
        RstartW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear ADDR Interrupt Flag"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, IfcSpec> {
        AddrW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear TXC Interrupt Flag"]
    #[inline(always)]
    pub fn txc(&mut self) -> TxcW<'_, IfcSpec> {
        TxcW::new(self, 3)
    }
    #[doc = "Bit 6 - Clear ACK Interrupt Flag"]
    #[inline(always)]
    pub fn ack(&mut self) -> AckW<'_, IfcSpec> {
        AckW::new(self, 6)
    }
    #[doc = "Bit 7 - Clear NACK Interrupt Flag"]
    #[inline(always)]
    pub fn nack(&mut self) -> NackW<'_, IfcSpec> {
        NackW::new(self, 7)
    }
    #[doc = "Bit 8 - Clear MSTOP Interrupt Flag"]
    #[inline(always)]
    pub fn mstop(&mut self) -> MstopW<'_, IfcSpec> {
        MstopW::new(self, 8)
    }
    #[doc = "Bit 9 - Clear ARBLOST Interrupt Flag"]
    #[inline(always)]
    pub fn arblost(&mut self) -> ArblostW<'_, IfcSpec> {
        ArblostW::new(self, 9)
    }
    #[doc = "Bit 10 - Clear BUSERR Interrupt Flag"]
    #[inline(always)]
    pub fn buserr(&mut self) -> BuserrW<'_, IfcSpec> {
        BuserrW::new(self, 10)
    }
    #[doc = "Bit 11 - Clear BUSHOLD Interrupt Flag"]
    #[inline(always)]
    pub fn bushold(&mut self) -> BusholdW<'_, IfcSpec> {
        BusholdW::new(self, 11)
    }
    #[doc = "Bit 12 - Clear TXOF Interrupt Flag"]
    #[inline(always)]
    pub fn txof(&mut self) -> TxofW<'_, IfcSpec> {
        TxofW::new(self, 12)
    }
    #[doc = "Bit 13 - Clear RXUF Interrupt Flag"]
    #[inline(always)]
    pub fn rxuf(&mut self) -> RxufW<'_, IfcSpec> {
        RxufW::new(self, 13)
    }
    #[doc = "Bit 14 - Clear BITO Interrupt Flag"]
    #[inline(always)]
    pub fn bito(&mut self) -> BitoW<'_, IfcSpec> {
        BitoW::new(self, 14)
    }
    #[doc = "Bit 15 - Clear CLTO Interrupt Flag"]
    #[inline(always)]
    pub fn clto(&mut self) -> CltoW<'_, IfcSpec> {
        CltoW::new(self, 15)
    }
    #[doc = "Bit 16 - Clear SSTOP Interrupt Flag"]
    #[inline(always)]
    pub fn sstop(&mut self) -> SstopW<'_, IfcSpec> {
        SstopW::new(self, 16)
    }
    #[doc = "Bit 17 - Clear RXFULL Interrupt Flag"]
    #[inline(always)]
    pub fn rxfull(&mut self) -> RxfullW<'_, IfcSpec> {
        RxfullW::new(self, 17)
    }
    #[doc = "Bit 18 - Clear CLERR Interrupt Flag"]
    #[inline(always)]
    pub fn clerr(&mut self) -> ClerrW<'_, IfcSpec> {
        ClerrW::new(self, 18)
    }
}
#[doc = "Interrupt Flag Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfcSpec;
impl crate::RegisterSpec for IfcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifc::W`](W) writer structure"]
impl crate::Writable for IfcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IfcSpec {}
