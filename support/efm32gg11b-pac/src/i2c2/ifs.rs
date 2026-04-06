#[doc = "Register `IFS` writer"]
pub type W = crate::W<IfsSpec>;
#[doc = "Field `START` writer - Set START Interrupt Flag"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTART` writer - Set RSTART Interrupt Flag"]
pub type RstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR` writer - Set ADDR Interrupt Flag"]
pub type AddrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXC` writer - Set TXC Interrupt Flag"]
pub type TxcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` writer - Set ACK Interrupt Flag"]
pub type AckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` writer - Set NACK Interrupt Flag"]
pub type NackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTOP` writer - Set MSTOP Interrupt Flag"]
pub type MstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBLOST` writer - Set ARBLOST Interrupt Flag"]
pub type ArblostW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSERR` writer - Set BUSERR Interrupt Flag"]
pub type BuserrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSHOLD` writer - Set BUSHOLD Interrupt Flag"]
pub type BusholdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOF` writer - Set TXOF Interrupt Flag"]
pub type TxofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUF` writer - Set RXUF Interrupt Flag"]
pub type RxufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BITO` writer - Set BITO Interrupt Flag"]
pub type BitoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLTO` writer - Set CLTO Interrupt Flag"]
pub type CltoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSTOP` writer - Set SSTOP Interrupt Flag"]
pub type SstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFULL` writer - Set RXFULL Interrupt Flag"]
pub type RxfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLERR` writer - Set CLERR Interrupt Flag"]
pub type ClerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set START Interrupt Flag"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, IfsSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - Set RSTART Interrupt Flag"]
    #[inline(always)]
    pub fn rstart(&mut self) -> RstartW<'_, IfsSpec> {
        RstartW::new(self, 1)
    }
    #[doc = "Bit 2 - Set ADDR Interrupt Flag"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, IfsSpec> {
        AddrW::new(self, 2)
    }
    #[doc = "Bit 3 - Set TXC Interrupt Flag"]
    #[inline(always)]
    pub fn txc(&mut self) -> TxcW<'_, IfsSpec> {
        TxcW::new(self, 3)
    }
    #[doc = "Bit 6 - Set ACK Interrupt Flag"]
    #[inline(always)]
    pub fn ack(&mut self) -> AckW<'_, IfsSpec> {
        AckW::new(self, 6)
    }
    #[doc = "Bit 7 - Set NACK Interrupt Flag"]
    #[inline(always)]
    pub fn nack(&mut self) -> NackW<'_, IfsSpec> {
        NackW::new(self, 7)
    }
    #[doc = "Bit 8 - Set MSTOP Interrupt Flag"]
    #[inline(always)]
    pub fn mstop(&mut self) -> MstopW<'_, IfsSpec> {
        MstopW::new(self, 8)
    }
    #[doc = "Bit 9 - Set ARBLOST Interrupt Flag"]
    #[inline(always)]
    pub fn arblost(&mut self) -> ArblostW<'_, IfsSpec> {
        ArblostW::new(self, 9)
    }
    #[doc = "Bit 10 - Set BUSERR Interrupt Flag"]
    #[inline(always)]
    pub fn buserr(&mut self) -> BuserrW<'_, IfsSpec> {
        BuserrW::new(self, 10)
    }
    #[doc = "Bit 11 - Set BUSHOLD Interrupt Flag"]
    #[inline(always)]
    pub fn bushold(&mut self) -> BusholdW<'_, IfsSpec> {
        BusholdW::new(self, 11)
    }
    #[doc = "Bit 12 - Set TXOF Interrupt Flag"]
    #[inline(always)]
    pub fn txof(&mut self) -> TxofW<'_, IfsSpec> {
        TxofW::new(self, 12)
    }
    #[doc = "Bit 13 - Set RXUF Interrupt Flag"]
    #[inline(always)]
    pub fn rxuf(&mut self) -> RxufW<'_, IfsSpec> {
        RxufW::new(self, 13)
    }
    #[doc = "Bit 14 - Set BITO Interrupt Flag"]
    #[inline(always)]
    pub fn bito(&mut self) -> BitoW<'_, IfsSpec> {
        BitoW::new(self, 14)
    }
    #[doc = "Bit 15 - Set CLTO Interrupt Flag"]
    #[inline(always)]
    pub fn clto(&mut self) -> CltoW<'_, IfsSpec> {
        CltoW::new(self, 15)
    }
    #[doc = "Bit 16 - Set SSTOP Interrupt Flag"]
    #[inline(always)]
    pub fn sstop(&mut self) -> SstopW<'_, IfsSpec> {
        SstopW::new(self, 16)
    }
    #[doc = "Bit 17 - Set RXFULL Interrupt Flag"]
    #[inline(always)]
    pub fn rxfull(&mut self) -> RxfullW<'_, IfsSpec> {
        RxfullW::new(self, 17)
    }
    #[doc = "Bit 18 - Set CLERR Interrupt Flag"]
    #[inline(always)]
    pub fn clerr(&mut self) -> ClerrW<'_, IfsSpec> {
        ClerrW::new(self, 18)
    }
}
#[doc = "Interrupt Flag Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfsSpec;
impl crate::RegisterSpec for IfsSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifs::W`](W) writer structure"]
impl crate::Writable for IfsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IfsSpec {}
