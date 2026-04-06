#[doc = "Register `TXSTATUS` reader"]
pub type R = crate::R<TxstatusSpec>;
#[doc = "Register `TXSTATUS` writer"]
pub type W = crate::W<TxstatusSpec>;
#[doc = "Field `USEDBITREAD` reader - Used bit read"]
pub type UsedbitreadR = crate::BitReader;
#[doc = "Field `USEDBITREAD` writer - Used bit read"]
pub type UsedbitreadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COLOCCRD` reader - Collision occurred"]
pub type ColoccrdR = crate::BitReader;
#[doc = "Field `COLOCCRD` writer - Collision occurred"]
pub type ColoccrdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETRYLMTEXCD` reader - Retry limit exceeded"]
pub type RetrylmtexcdR = crate::BitReader;
#[doc = "Field `RETRYLMTEXCD` writer - Retry limit exceeded"]
pub type RetrylmtexcdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXGO` reader - Transmit go"]
pub type TxgoR = crate::BitReader;
#[doc = "Field `AMBAERR` reader - Transmit frame corruption due to AMBA (AHB) errors."]
pub type AmbaerrR = crate::BitReader;
#[doc = "Field `AMBAERR` writer - Transmit frame corruption due to AMBA (AHB) errors."]
pub type AmbaerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXCMPLT` reader - Transmit complete"]
pub type TxcmpltR = crate::BitReader;
#[doc = "Field `TXCMPLT` writer - Transmit complete"]
pub type TxcmpltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUNDERRUN` reader - Transmit under run"]
pub type TxunderrunR = crate::BitReader;
#[doc = "Field `TXUNDERRUN` writer - Transmit under run"]
pub type TxunderrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LATECOLOCCRD` reader - Late collision occurred"]
pub type LatecoloccrdR = crate::BitReader;
#[doc = "Field `LATECOLOCCRD` writer - Late collision occurred"]
pub type LatecoloccrdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESPNOTOK` reader - bresp/hresp not OK"]
pub type RespnotokR = crate::BitReader;
#[doc = "Field `RESPNOTOK` writer - bresp/hresp not OK"]
pub type RespnotokW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Used bit read"]
    #[inline(always)]
    pub fn usedbitread(&self) -> UsedbitreadR {
        UsedbitreadR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Collision occurred"]
    #[inline(always)]
    pub fn coloccrd(&self) -> ColoccrdR {
        ColoccrdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Retry limit exceeded"]
    #[inline(always)]
    pub fn retrylmtexcd(&self) -> RetrylmtexcdR {
        RetrylmtexcdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit go"]
    #[inline(always)]
    pub fn txgo(&self) -> TxgoR {
        TxgoR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit frame corruption due to AMBA (AHB) errors."]
    #[inline(always)]
    pub fn ambaerr(&self) -> AmbaerrR {
        AmbaerrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit complete"]
    #[inline(always)]
    pub fn txcmplt(&self) -> TxcmpltR {
        TxcmpltR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit under run"]
    #[inline(always)]
    pub fn txunderrun(&self) -> TxunderrunR {
        TxunderrunR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Late collision occurred"]
    #[inline(always)]
    pub fn latecoloccrd(&self) -> LatecoloccrdR {
        LatecoloccrdR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - bresp/hresp not OK"]
    #[inline(always)]
    pub fn respnotok(&self) -> RespnotokR {
        RespnotokR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Used bit read"]
    #[inline(always)]
    pub fn usedbitread(&mut self) -> UsedbitreadW<'_, TxstatusSpec> {
        UsedbitreadW::new(self, 0)
    }
    #[doc = "Bit 1 - Collision occurred"]
    #[inline(always)]
    pub fn coloccrd(&mut self) -> ColoccrdW<'_, TxstatusSpec> {
        ColoccrdW::new(self, 1)
    }
    #[doc = "Bit 2 - Retry limit exceeded"]
    #[inline(always)]
    pub fn retrylmtexcd(&mut self) -> RetrylmtexcdW<'_, TxstatusSpec> {
        RetrylmtexcdW::new(self, 2)
    }
    #[doc = "Bit 4 - Transmit frame corruption due to AMBA (AHB) errors."]
    #[inline(always)]
    pub fn ambaerr(&mut self) -> AmbaerrW<'_, TxstatusSpec> {
        AmbaerrW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit complete"]
    #[inline(always)]
    pub fn txcmplt(&mut self) -> TxcmpltW<'_, TxstatusSpec> {
        TxcmpltW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmit under run"]
    #[inline(always)]
    pub fn txunderrun(&mut self) -> TxunderrunW<'_, TxstatusSpec> {
        TxunderrunW::new(self, 6)
    }
    #[doc = "Bit 7 - Late collision occurred"]
    #[inline(always)]
    pub fn latecoloccrd(&mut self) -> LatecoloccrdW<'_, TxstatusSpec> {
        LatecoloccrdW::new(self, 7)
    }
    #[doc = "Bit 8 - bresp/hresp not OK"]
    #[inline(always)]
    pub fn respnotok(&mut self) -> RespnotokW<'_, TxstatusSpec> {
        RespnotokW::new(self, 8)
    }
}
#[doc = "Transmit status register\n\nYou can [`read`](crate::Reg::read) this register and get [`txstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxstatusSpec;
impl crate::RegisterSpec for TxstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txstatus::R`](R) reader structure"]
impl crate::Readable for TxstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`txstatus::W`](W) writer structure"]
impl crate::Writable for TxstatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXSTATUS to value 0"]
impl crate::Resettable for TxstatusSpec {}
