#[doc = "Register `MIR0_CMDMASK` reader"]
pub type R = crate::R<Mir0CmdmaskSpec>;
#[doc = "Register `MIR0_CMDMASK` writer"]
pub type W = crate::W<Mir0CmdmaskSpec>;
#[doc = "Field `DATAB` reader - CC Channel Mode"]
pub type DatabR = crate::BitReader;
#[doc = "Field `DATAB` writer - CC Channel Mode"]
pub type DatabW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAA` reader - Access Data Bytes 0-3"]
pub type DataaR = crate::BitReader;
#[doc = "Field `DATAA` writer - Access Data Bytes 0-3"]
pub type DataaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRQSTNEWDAT` reader - Transmission Request Bit/ New Data Bit"]
pub type TxrqstnewdatR = crate::BitReader;
#[doc = "Field `TXRQSTNEWDAT` writer - Transmission Request Bit/ New Data Bit"]
pub type TxrqstnewdatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRINTPND` reader - Clear Interrupt Pending Bit"]
pub type ClrintpndR = crate::BitReader;
#[doc = "Field `CLRINTPND` writer - Clear Interrupt Pending Bit"]
pub type ClrintpndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONTROL` reader - Access Control Bits"]
pub type ControlR = crate::BitReader;
#[doc = "Field `CONTROL` writer - Access Control Bits"]
pub type ControlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBACC` reader - Access Arbitration Bits"]
pub type ArbaccR = crate::BitReader;
#[doc = "Field `ARBACC` writer - Access Arbitration Bits"]
pub type ArbaccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASKACC` reader - Access Mask Bits"]
pub type MaskaccR = crate::BitReader;
#[doc = "Field `MASKACC` writer - Access Mask Bits"]
pub type MaskaccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRRD` reader - Write/Read RAM"]
pub type WrrdR = crate::BitReader;
#[doc = "Field `WRRD` writer - Write/Read RAM"]
pub type WrrdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CC Channel Mode"]
    #[inline(always)]
    pub fn datab(&self) -> DatabR {
        DatabR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Access Data Bytes 0-3"]
    #[inline(always)]
    pub fn dataa(&self) -> DataaR {
        DataaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmission Request Bit/ New Data Bit"]
    #[inline(always)]
    pub fn txrqstnewdat(&self) -> TxrqstnewdatR {
        TxrqstnewdatR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear Interrupt Pending Bit"]
    #[inline(always)]
    pub fn clrintpnd(&self) -> ClrintpndR {
        ClrintpndR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Access Control Bits"]
    #[inline(always)]
    pub fn control(&self) -> ControlR {
        ControlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Access Arbitration Bits"]
    #[inline(always)]
    pub fn arbacc(&self) -> ArbaccR {
        ArbaccR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Access Mask Bits"]
    #[inline(always)]
    pub fn maskacc(&self) -> MaskaccR {
        MaskaccR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write/Read RAM"]
    #[inline(always)]
    pub fn wrrd(&self) -> WrrdR {
        WrrdR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CC Channel Mode"]
    #[inline(always)]
    pub fn datab(&mut self) -> DatabW<'_, Mir0CmdmaskSpec> {
        DatabW::new(self, 0)
    }
    #[doc = "Bit 1 - Access Data Bytes 0-3"]
    #[inline(always)]
    pub fn dataa(&mut self) -> DataaW<'_, Mir0CmdmaskSpec> {
        DataaW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmission Request Bit/ New Data Bit"]
    #[inline(always)]
    pub fn txrqstnewdat(&mut self) -> TxrqstnewdatW<'_, Mir0CmdmaskSpec> {
        TxrqstnewdatW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear Interrupt Pending Bit"]
    #[inline(always)]
    pub fn clrintpnd(&mut self) -> ClrintpndW<'_, Mir0CmdmaskSpec> {
        ClrintpndW::new(self, 3)
    }
    #[doc = "Bit 4 - Access Control Bits"]
    #[inline(always)]
    pub fn control(&mut self) -> ControlW<'_, Mir0CmdmaskSpec> {
        ControlW::new(self, 4)
    }
    #[doc = "Bit 5 - Access Arbitration Bits"]
    #[inline(always)]
    pub fn arbacc(&mut self) -> ArbaccW<'_, Mir0CmdmaskSpec> {
        ArbaccW::new(self, 5)
    }
    #[doc = "Bit 6 - Access Mask Bits"]
    #[inline(always)]
    pub fn maskacc(&mut self) -> MaskaccW<'_, Mir0CmdmaskSpec> {
        MaskaccW::new(self, 6)
    }
    #[doc = "Bit 7 - Write/Read RAM"]
    #[inline(always)]
    pub fn wrrd(&mut self) -> WrrdW<'_, Mir0CmdmaskSpec> {
        WrrdW::new(self, 7)
    }
}
#[doc = "Interface Command Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mir0_cmdmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mir0_cmdmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mir0CmdmaskSpec;
impl crate::RegisterSpec for Mir0CmdmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mir0_cmdmask::R`](R) reader structure"]
impl crate::Readable for Mir0CmdmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`mir0_cmdmask::W`](W) writer structure"]
impl crate::Writable for Mir0CmdmaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MIR0_CMDMASK to value 0"]
impl crate::Resettable for Mir0CmdmaskSpec {}
