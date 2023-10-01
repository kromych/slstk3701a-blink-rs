#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `TSUCLKSEL` reader - TSU Clock selection value"]
pub type TSUCLKSEL_R = crate::FieldReader<TSUCLKSEL_A>;
#[doc = "TSU Clock selection value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSUCLKSEL_A {
    #[doc = "0: No TSU clock source selected"]
    NOCLOCK = 0,
    #[doc = "1: Select system clock as TSU Clock"]
    PLL = 1,
    #[doc = "2: Select ethernet RX Clock as TSU Clock"]
    RXCLK = 2,
    #[doc = "3: Select ref clock as TSU Clock"]
    REFCLK = 3,
    #[doc = "4: Select tsu external pin as TSU Clock"]
    TSUEXTCLK = 4,
}
impl From<TSUCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TSUCLKSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TSUCLKSEL_A {
    type Ux = u8;
}
impl TSUCLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSUCLKSEL_A> {
        match self.bits {
            0 => Some(TSUCLKSEL_A::NOCLOCK),
            1 => Some(TSUCLKSEL_A::PLL),
            2 => Some(TSUCLKSEL_A::RXCLK),
            3 => Some(TSUCLKSEL_A::REFCLK),
            4 => Some(TSUCLKSEL_A::TSUEXTCLK),
            _ => None,
        }
    }
    #[doc = "No TSU clock source selected"]
    #[inline(always)]
    pub fn is_noclock(&self) -> bool {
        *self == TSUCLKSEL_A::NOCLOCK
    }
    #[doc = "Select system clock as TSU Clock"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == TSUCLKSEL_A::PLL
    }
    #[doc = "Select ethernet RX Clock as TSU Clock"]
    #[inline(always)]
    pub fn is_rxclk(&self) -> bool {
        *self == TSUCLKSEL_A::RXCLK
    }
    #[doc = "Select ref clock as TSU Clock"]
    #[inline(always)]
    pub fn is_refclk(&self) -> bool {
        *self == TSUCLKSEL_A::REFCLK
    }
    #[doc = "Select tsu external pin as TSU Clock"]
    #[inline(always)]
    pub fn is_tsuextclk(&self) -> bool {
        *self == TSUCLKSEL_A::TSUEXTCLK
    }
}
#[doc = "Field `TSUCLKSEL` writer - TSU Clock selection value"]
pub type TSUCLKSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, TSUCLKSEL_A>;
impl<'a, REG, const O: u8> TSUCLKSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No TSU clock source selected"]
    #[inline(always)]
    pub fn noclock(self) -> &'a mut crate::W<REG> {
        self.variant(TSUCLKSEL_A::NOCLOCK)
    }
    #[doc = "Select system clock as TSU Clock"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut crate::W<REG> {
        self.variant(TSUCLKSEL_A::PLL)
    }
    #[doc = "Select ethernet RX Clock as TSU Clock"]
    #[inline(always)]
    pub fn rxclk(self) -> &'a mut crate::W<REG> {
        self.variant(TSUCLKSEL_A::RXCLK)
    }
    #[doc = "Select ref clock as TSU Clock"]
    #[inline(always)]
    pub fn refclk(self) -> &'a mut crate::W<REG> {
        self.variant(TSUCLKSEL_A::REFCLK)
    }
    #[doc = "Select tsu external pin as TSU Clock"]
    #[inline(always)]
    pub fn tsuextclk(self) -> &'a mut crate::W<REG> {
        self.variant(TSUCLKSEL_A::TSUEXTCLK)
    }
}
#[doc = "Field `TSUPRESC` reader - Clock division factor of TSUPRESC+1"]
pub type TSUPRESC_R = crate::FieldReader;
#[doc = "Field `TSUPRESC` writer - Clock division factor of TSUPRESC+1"]
pub type TSUPRESC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `MIISEL` reader - MII select signal"]
pub type MIISEL_R = crate::BitReader;
#[doc = "Field `MIISEL` writer - MII select signal"]
pub type MIISEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GBLCLKEN` reader - Global Clock Enable signal for Ethernet clocks tsu_clk, tx_clk, rx_clk and ref_clk"]
pub type GBLCLKEN_R = crate::BitReader;
#[doc = "Field `GBLCLKEN` writer - Global Clock Enable signal for Ethernet clocks tsu_clk, tx_clk, rx_clk and ref_clk"]
pub type GBLCLKEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXREFCLKSEL` reader - REFCLK source select for RMII_TXD and RMII_TX_EN"]
pub type TXREFCLKSEL_R = crate::BitReader;
#[doc = "Field `TXREFCLKSEL` writer - REFCLK source select for RMII_TXD and RMII_TX_EN"]
pub type TXREFCLKSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:2 - TSU Clock selection value"]
    #[inline(always)]
    pub fn tsuclksel(&self) -> TSUCLKSEL_R {
        TSUCLKSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:7 - Clock division factor of TSUPRESC+1"]
    #[inline(always)]
    pub fn tsupresc(&self) -> TSUPRESC_R {
        TSUPRESC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - MII select signal"]
    #[inline(always)]
    pub fn miisel(&self) -> MIISEL_R {
        MIISEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Global Clock Enable signal for Ethernet clocks tsu_clk, tx_clk, rx_clk and ref_clk"]
    #[inline(always)]
    pub fn gblclken(&self) -> GBLCLKEN_R {
        GBLCLKEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - REFCLK source select for RMII_TXD and RMII_TX_EN"]
    #[inline(always)]
    pub fn txrefclksel(&self) -> TXREFCLKSEL_R {
        TXREFCLKSEL_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - TSU Clock selection value"]
    #[inline(always)]
    #[must_use]
    pub fn tsuclksel(&mut self) -> TSUCLKSEL_W<CTRL_SPEC, 0> {
        TSUCLKSEL_W::new(self)
    }
    #[doc = "Bits 4:7 - Clock division factor of TSUPRESC+1"]
    #[inline(always)]
    #[must_use]
    pub fn tsupresc(&mut self) -> TSUPRESC_W<CTRL_SPEC, 4> {
        TSUPRESC_W::new(self)
    }
    #[doc = "Bit 8 - MII select signal"]
    #[inline(always)]
    #[must_use]
    pub fn miisel(&mut self) -> MIISEL_W<CTRL_SPEC, 8> {
        MIISEL_W::new(self)
    }
    #[doc = "Bit 9 - Global Clock Enable signal for Ethernet clocks tsu_clk, tx_clk, rx_clk and ref_clk"]
    #[inline(always)]
    #[must_use]
    pub fn gblclken(&mut self) -> GBLCLKEN_W<CTRL_SPEC, 9> {
        GBLCLKEN_W::new(self)
    }
    #[doc = "Bit 10 - REFCLK source select for RMII_TXD and RMII_TX_EN"]
    #[inline(always)]
    #[must_use]
    pub fn txrefclksel(&mut self) -> TXREFCLKSEL_W<CTRL_SPEC, 10> {
        TXREFCLKSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Ethernet control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
