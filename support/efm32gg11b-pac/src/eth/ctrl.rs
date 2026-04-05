#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "TSU Clock selection value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tsuclksel {
    #[doc = "0: No TSU clock source selected"]
    Noclock = 0,
    #[doc = "1: Select system clock as TSU Clock"]
    Pll = 1,
    #[doc = "2: Select ethernet RX Clock as TSU Clock"]
    Rxclk = 2,
    #[doc = "3: Select ref clock as TSU Clock"]
    Refclk = 3,
    #[doc = "4: Select tsu external pin as TSU Clock"]
    Tsuextclk = 4,
}
impl From<Tsuclksel> for u8 {
    #[inline(always)]
    fn from(variant: Tsuclksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tsuclksel {
    type Ux = u8;
}
impl crate::IsEnum for Tsuclksel {}
#[doc = "Field `TSUCLKSEL` reader - TSU Clock selection value"]
pub type TsuclkselR = crate::FieldReader<Tsuclksel>;
impl TsuclkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tsuclksel> {
        match self.bits {
            0 => Some(Tsuclksel::Noclock),
            1 => Some(Tsuclksel::Pll),
            2 => Some(Tsuclksel::Rxclk),
            3 => Some(Tsuclksel::Refclk),
            4 => Some(Tsuclksel::Tsuextclk),
            _ => None,
        }
    }
    #[doc = "No TSU clock source selected"]
    #[inline(always)]
    pub fn is_noclock(&self) -> bool {
        *self == Tsuclksel::Noclock
    }
    #[doc = "Select system clock as TSU Clock"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == Tsuclksel::Pll
    }
    #[doc = "Select ethernet RX Clock as TSU Clock"]
    #[inline(always)]
    pub fn is_rxclk(&self) -> bool {
        *self == Tsuclksel::Rxclk
    }
    #[doc = "Select ref clock as TSU Clock"]
    #[inline(always)]
    pub fn is_refclk(&self) -> bool {
        *self == Tsuclksel::Refclk
    }
    #[doc = "Select tsu external pin as TSU Clock"]
    #[inline(always)]
    pub fn is_tsuextclk(&self) -> bool {
        *self == Tsuclksel::Tsuextclk
    }
}
#[doc = "Field `TSUCLKSEL` writer - TSU Clock selection value"]
pub type TsuclkselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Tsuclksel>;
impl<'a, REG> TsuclkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No TSU clock source selected"]
    #[inline(always)]
    pub fn noclock(self) -> &'a mut crate::W<REG> {
        self.variant(Tsuclksel::Noclock)
    }
    #[doc = "Select system clock as TSU Clock"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut crate::W<REG> {
        self.variant(Tsuclksel::Pll)
    }
    #[doc = "Select ethernet RX Clock as TSU Clock"]
    #[inline(always)]
    pub fn rxclk(self) -> &'a mut crate::W<REG> {
        self.variant(Tsuclksel::Rxclk)
    }
    #[doc = "Select ref clock as TSU Clock"]
    #[inline(always)]
    pub fn refclk(self) -> &'a mut crate::W<REG> {
        self.variant(Tsuclksel::Refclk)
    }
    #[doc = "Select tsu external pin as TSU Clock"]
    #[inline(always)]
    pub fn tsuextclk(self) -> &'a mut crate::W<REG> {
        self.variant(Tsuclksel::Tsuextclk)
    }
}
#[doc = "Field `TSUPRESC` reader - Clock division factor of TSUPRESC+1"]
pub type TsuprescR = crate::FieldReader;
#[doc = "Field `TSUPRESC` writer - Clock division factor of TSUPRESC+1"]
pub type TsuprescW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MIISEL` reader - MII select signal"]
pub type MiiselR = crate::BitReader;
#[doc = "Field `MIISEL` writer - MII select signal"]
pub type MiiselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GBLCLKEN` reader - Global Clock Enable signal for Ethernet clocks tsu_clk, tx_clk, rx_clk and ref_clk"]
pub type GblclkenR = crate::BitReader;
#[doc = "Field `GBLCLKEN` writer - Global Clock Enable signal for Ethernet clocks tsu_clk, tx_clk, rx_clk and ref_clk"]
pub type GblclkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXREFCLKSEL` reader - REFCLK source select for RMII_TXD and RMII_TX_EN"]
pub type TxrefclkselR = crate::BitReader;
#[doc = "Field `TXREFCLKSEL` writer - REFCLK source select for RMII_TXD and RMII_TX_EN"]
pub type TxrefclkselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - TSU Clock selection value"]
    #[inline(always)]
    pub fn tsuclksel(&self) -> TsuclkselR {
        TsuclkselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:7 - Clock division factor of TSUPRESC+1"]
    #[inline(always)]
    pub fn tsupresc(&self) -> TsuprescR {
        TsuprescR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - MII select signal"]
    #[inline(always)]
    pub fn miisel(&self) -> MiiselR {
        MiiselR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Global Clock Enable signal for Ethernet clocks tsu_clk, tx_clk, rx_clk and ref_clk"]
    #[inline(always)]
    pub fn gblclken(&self) -> GblclkenR {
        GblclkenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - REFCLK source select for RMII_TXD and RMII_TX_EN"]
    #[inline(always)]
    pub fn txrefclksel(&self) -> TxrefclkselR {
        TxrefclkselR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - TSU Clock selection value"]
    #[inline(always)]
    pub fn tsuclksel(&mut self) -> TsuclkselW<'_, CtrlSpec> {
        TsuclkselW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Clock division factor of TSUPRESC+1"]
    #[inline(always)]
    pub fn tsupresc(&mut self) -> TsuprescW<'_, CtrlSpec> {
        TsuprescW::new(self, 4)
    }
    #[doc = "Bit 8 - MII select signal"]
    #[inline(always)]
    pub fn miisel(&mut self) -> MiiselW<'_, CtrlSpec> {
        MiiselW::new(self, 8)
    }
    #[doc = "Bit 9 - Global Clock Enable signal for Ethernet clocks tsu_clk, tx_clk, rx_clk and ref_clk"]
    #[inline(always)]
    pub fn gblclken(&mut self) -> GblclkenW<'_, CtrlSpec> {
        GblclkenW::new(self, 9)
    }
    #[doc = "Bit 10 - REFCLK source select for RMII_TXD and RMII_TX_EN"]
    #[inline(always)]
    pub fn txrefclksel(&mut self) -> TxrefclkselW<'_, CtrlSpec> {
        TxrefclkselW::new(self, 10)
    }
}
#[doc = "Ethernet control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
