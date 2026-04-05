#[doc = "Register `RXSTATUS` reader"]
pub type R = crate::R<RxstatusSpec>;
#[doc = "Register `RXSTATUS` writer"]
pub type W = crate::W<RxstatusSpec>;
#[doc = "Field `BUFFNOTAVAIL` reader - Buffer not available"]
pub type BuffnotavailR = crate::BitReader;
#[doc = "Field `BUFFNOTAVAIL` writer - Buffer not available"]
pub type BuffnotavailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRMRX` reader - Frame received"]
pub type FrmrxR = crate::BitReader;
#[doc = "Field `FRMRX` writer - Frame received"]
pub type FrmrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOVERRUN` reader - Receive overrun"]
pub type RxoverrunR = crate::BitReader;
#[doc = "Field `RXOVERRUN` writer - Receive overrun"]
pub type RxoverrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESPNOTOK` reader - bresp/hresp not OK"]
pub type RespnotokR = crate::BitReader;
#[doc = "Field `RESPNOTOK` writer - bresp/hresp not OK"]
pub type RespnotokW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Buffer not available"]
    #[inline(always)]
    pub fn buffnotavail(&self) -> BuffnotavailR {
        BuffnotavailR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frame received"]
    #[inline(always)]
    pub fn frmrx(&self) -> FrmrxR {
        FrmrxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive overrun"]
    #[inline(always)]
    pub fn rxoverrun(&self) -> RxoverrunR {
        RxoverrunR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - bresp/hresp not OK"]
    #[inline(always)]
    pub fn respnotok(&self) -> RespnotokR {
        RespnotokR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Buffer not available"]
    #[inline(always)]
    pub fn buffnotavail(&mut self) -> BuffnotavailW<'_, RxstatusSpec> {
        BuffnotavailW::new(self, 0)
    }
    #[doc = "Bit 1 - Frame received"]
    #[inline(always)]
    pub fn frmrx(&mut self) -> FrmrxW<'_, RxstatusSpec> {
        FrmrxW::new(self, 1)
    }
    #[doc = "Bit 2 - Receive overrun"]
    #[inline(always)]
    pub fn rxoverrun(&mut self) -> RxoverrunW<'_, RxstatusSpec> {
        RxoverrunW::new(self, 2)
    }
    #[doc = "Bit 3 - bresp/hresp not OK"]
    #[inline(always)]
    pub fn respnotok(&mut self) -> RespnotokW<'_, RxstatusSpec> {
        RespnotokW::new(self, 3)
    }
}
#[doc = "Receive status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxstatusSpec;
impl crate::RegisterSpec for RxstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxstatus::R`](R) reader structure"]
impl crate::Readable for RxstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`rxstatus::W`](W) writer structure"]
impl crate::Writable for RxstatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXSTATUS to value 0"]
impl crate::Resettable for RxstatusSpec {}
