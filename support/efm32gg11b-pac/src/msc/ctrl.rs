#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `ADDRFAULTEN` reader - Invalid Address Bus Fault Response Enable"]
pub type AddrfaultenR = crate::BitReader;
#[doc = "Field `ADDRFAULTEN` writer - Invalid Address Bus Fault Response Enable"]
pub type AddrfaultenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKDISFAULTEN` reader - Clock-disabled Bus Fault Response Enable"]
pub type ClkdisfaultenR = crate::BitReader;
#[doc = "Field `CLKDISFAULTEN` writer - Clock-disabled Bus Fault Response Enable"]
pub type ClkdisfaultenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRUPONDEMAND` reader - Power Up on Demand During Wake Up"]
pub type PwrupondemandR = crate::BitReader;
#[doc = "Field `PWRUPONDEMAND` writer - Power Up on Demand During Wake Up"]
pub type PwrupondemandW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFCREADCLEAR` reader - IFC Read Clears IF"]
pub type IfcreadclearR = crate::BitReader;
#[doc = "Field `IFCREADCLEAR` writer - IFC Read Clears IF"]
pub type IfcreadclearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUTFAULTEN` reader - Timeout Bus Fault Response Enable"]
pub type TimeoutfaultenR = crate::BitReader;
#[doc = "Field `TIMEOUTFAULTEN` writer - Timeout Bus Fault Response Enable"]
pub type TimeoutfaultenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMECCERRFAULTEN` reader - Two Bit ECC Error Bus Fault Response Enable"]
pub type RameccerrfaultenR = crate::BitReader;
#[doc = "Field `RAMECCERRFAULTEN` writer - Two Bit ECC Error Bus Fault Response Enable"]
pub type RameccerrfaultenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EBIFAULTEN` reader - EBI Bus Fault Response Enable"]
pub type EbifaultenR = crate::BitReader;
#[doc = "Field `EBIFAULTEN` writer - EBI Bus Fault Response Enable"]
pub type EbifaultenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAITMODE` reader - Peripheral Access Wait Mode"]
pub type WaitmodeR = crate::BitReader;
#[doc = "Field `WAITMODE` writer - Peripheral Access Wait Mode"]
pub type WaitmodeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Invalid Address Bus Fault Response Enable"]
    #[inline(always)]
    pub fn addrfaulten(&self) -> AddrfaultenR {
        AddrfaultenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock-disabled Bus Fault Response Enable"]
    #[inline(always)]
    pub fn clkdisfaulten(&self) -> ClkdisfaultenR {
        ClkdisfaultenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power Up on Demand During Wake Up"]
    #[inline(always)]
    pub fn pwrupondemand(&self) -> PwrupondemandR {
        PwrupondemandR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IFC Read Clears IF"]
    #[inline(always)]
    pub fn ifcreadclear(&self) -> IfcreadclearR {
        IfcreadclearR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timeout Bus Fault Response Enable"]
    #[inline(always)]
    pub fn timeoutfaulten(&self) -> TimeoutfaultenR {
        TimeoutfaultenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Two Bit ECC Error Bus Fault Response Enable"]
    #[inline(always)]
    pub fn rameccerrfaulten(&self) -> RameccerrfaultenR {
        RameccerrfaultenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EBI Bus Fault Response Enable"]
    #[inline(always)]
    pub fn ebifaulten(&self) -> EbifaultenR {
        EbifaultenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 12 - Peripheral Access Wait Mode"]
    #[inline(always)]
    pub fn waitmode(&self) -> WaitmodeR {
        WaitmodeR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Invalid Address Bus Fault Response Enable"]
    #[inline(always)]
    pub fn addrfaulten(&mut self) -> AddrfaultenW<'_, CtrlSpec> {
        AddrfaultenW::new(self, 0)
    }
    #[doc = "Bit 1 - Clock-disabled Bus Fault Response Enable"]
    #[inline(always)]
    pub fn clkdisfaulten(&mut self) -> ClkdisfaultenW<'_, CtrlSpec> {
        ClkdisfaultenW::new(self, 1)
    }
    #[doc = "Bit 2 - Power Up on Demand During Wake Up"]
    #[inline(always)]
    pub fn pwrupondemand(&mut self) -> PwrupondemandW<'_, CtrlSpec> {
        PwrupondemandW::new(self, 2)
    }
    #[doc = "Bit 3 - IFC Read Clears IF"]
    #[inline(always)]
    pub fn ifcreadclear(&mut self) -> IfcreadclearW<'_, CtrlSpec> {
        IfcreadclearW::new(self, 3)
    }
    #[doc = "Bit 4 - Timeout Bus Fault Response Enable"]
    #[inline(always)]
    pub fn timeoutfaulten(&mut self) -> TimeoutfaultenW<'_, CtrlSpec> {
        TimeoutfaultenW::new(self, 4)
    }
    #[doc = "Bit 5 - Two Bit ECC Error Bus Fault Response Enable"]
    #[inline(always)]
    pub fn rameccerrfaulten(&mut self) -> RameccerrfaultenW<'_, CtrlSpec> {
        RameccerrfaultenW::new(self, 5)
    }
    #[doc = "Bit 6 - EBI Bus Fault Response Enable"]
    #[inline(always)]
    pub fn ebifaulten(&mut self) -> EbifaultenW<'_, CtrlSpec> {
        EbifaultenW::new(self, 6)
    }
    #[doc = "Bit 12 - Peripheral Access Wait Mode"]
    #[inline(always)]
    pub fn waitmode(&mut self) -> WaitmodeW<'_, CtrlSpec> {
        WaitmodeW::new(self, 12)
    }
}
#[doc = "Memory System Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTRL to value 0x21"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x21;
}
