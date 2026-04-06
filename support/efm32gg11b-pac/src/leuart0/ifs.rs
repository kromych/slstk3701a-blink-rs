#[doc = "Register `IFS` writer"]
pub type W = crate::W<IfsSpec>;
#[doc = "Field `TXC` writer - Set TXC Interrupt Flag"]
pub type TxcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOF` writer - Set RXOF Interrupt Flag"]
pub type RxofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUF` writer - Set RXUF Interrupt Flag"]
pub type RxufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOF` writer - Set TXOF Interrupt Flag"]
pub type TxofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERR` writer - Set PERR Interrupt Flag"]
pub type PerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FERR` writer - Set FERR Interrupt Flag"]
pub type FerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPAF` writer - Set MPAF Interrupt Flag"]
pub type MpafW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTF` writer - Set STARTF Interrupt Flag"]
pub type StartfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIGF` writer - Set SIGF Interrupt Flag"]
pub type SigfW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set TXC Interrupt Flag"]
    #[inline(always)]
    pub fn txc(&mut self) -> TxcW<'_, IfsSpec> {
        TxcW::new(self, 0)
    }
    #[doc = "Bit 3 - Set RXOF Interrupt Flag"]
    #[inline(always)]
    pub fn rxof(&mut self) -> RxofW<'_, IfsSpec> {
        RxofW::new(self, 3)
    }
    #[doc = "Bit 4 - Set RXUF Interrupt Flag"]
    #[inline(always)]
    pub fn rxuf(&mut self) -> RxufW<'_, IfsSpec> {
        RxufW::new(self, 4)
    }
    #[doc = "Bit 5 - Set TXOF Interrupt Flag"]
    #[inline(always)]
    pub fn txof(&mut self) -> TxofW<'_, IfsSpec> {
        TxofW::new(self, 5)
    }
    #[doc = "Bit 6 - Set PERR Interrupt Flag"]
    #[inline(always)]
    pub fn perr(&mut self) -> PerrW<'_, IfsSpec> {
        PerrW::new(self, 6)
    }
    #[doc = "Bit 7 - Set FERR Interrupt Flag"]
    #[inline(always)]
    pub fn ferr(&mut self) -> FerrW<'_, IfsSpec> {
        FerrW::new(self, 7)
    }
    #[doc = "Bit 8 - Set MPAF Interrupt Flag"]
    #[inline(always)]
    pub fn mpaf(&mut self) -> MpafW<'_, IfsSpec> {
        MpafW::new(self, 8)
    }
    #[doc = "Bit 9 - Set STARTF Interrupt Flag"]
    #[inline(always)]
    pub fn startf(&mut self) -> StartfW<'_, IfsSpec> {
        StartfW::new(self, 9)
    }
    #[doc = "Bit 10 - Set SIGF Interrupt Flag"]
    #[inline(always)]
    pub fn sigf(&mut self) -> SigfW<'_, IfsSpec> {
        SigfW::new(self, 10)
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
