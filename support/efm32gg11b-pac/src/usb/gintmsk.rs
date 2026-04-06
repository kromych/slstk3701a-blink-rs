#[doc = "Register `GINTMSK` reader"]
pub type R = crate::R<GintmskSpec>;
#[doc = "Register `GINTMSK` writer"]
pub type W = crate::W<GintmskSpec>;
#[doc = "Field `MODEMISMSK` reader - Mode Mismatch Interrupt Mask (host and device)"]
pub type ModemismskR = crate::BitReader;
#[doc = "Field `MODEMISMSK` writer - Mode Mismatch Interrupt Mask (host and device)"]
pub type ModemismskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGINTMSK` reader - OTG Interrupt Mask (host and device)"]
pub type OtgintmskR = crate::BitReader;
#[doc = "Field `OTGINTMSK` writer - OTG Interrupt Mask (host and device)"]
pub type OtgintmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFMSK` reader - Start of Frame Mask (host and device)"]
pub type SofmskR = crate::BitReader;
#[doc = "Field `SOFMSK` writer - Start of Frame Mask (host and device)"]
pub type SofmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFLVLMSK` reader - Receive FIFO Non-Empty Mask (host and device)"]
pub type RxflvlmskR = crate::BitReader;
#[doc = "Field `RXFLVLMSK` writer - Receive FIFO Non-Empty Mask (host and device)"]
pub type RxflvlmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NPTXFEMPMSK` reader - Non-Periodic TxFIFO Empty Mask (host only)"]
pub type NptxfempmskR = crate::BitReader;
#[doc = "Field `NPTXFEMPMSK` writer - Non-Periodic TxFIFO Empty Mask (host only)"]
pub type NptxfempmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GINNAKEFFMSK` reader - Global Non-periodic IN NAK Effective Mask (device only)"]
pub type GinnakeffmskR = crate::BitReader;
#[doc = "Field `GINNAKEFFMSK` writer - Global Non-periodic IN NAK Effective Mask (device only)"]
pub type GinnakeffmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GOUTNAKEFFMSK` reader - Global OUT NAK Effective Mask (device only)"]
pub type GoutnakeffmskR = crate::BitReader;
#[doc = "Field `GOUTNAKEFFMSK` writer - Global OUT NAK Effective Mask (device only)"]
pub type GoutnakeffmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERLYSUSPMSK` reader - Early Suspend Mask (device only)"]
pub type ErlysuspmskR = crate::BitReader;
#[doc = "Field `ERLYSUSPMSK` writer - Early Suspend Mask (device only)"]
pub type ErlysuspmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBSUSPMSK` reader - USB Suspend Mask (device only)"]
pub type UsbsuspmskR = crate::BitReader;
#[doc = "Field `USBSUSPMSK` writer - USB Suspend Mask (device only)"]
pub type UsbsuspmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBRSTMSK` reader - USB Reset Mask (device only)"]
pub type UsbrstmskR = crate::BitReader;
#[doc = "Field `USBRSTMSK` writer - USB Reset Mask (device only)"]
pub type UsbrstmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENUMDONEMSK` reader - Enumeration Done Mask (device only)"]
pub type EnumdonemskR = crate::BitReader;
#[doc = "Field `ENUMDONEMSK` writer - Enumeration Done Mask (device only)"]
pub type EnumdonemskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISOOUTDROPMSK` reader - Isochronous OUT Packet Dropped Interrupt Mask (device only)"]
pub type IsooutdropmskR = crate::BitReader;
#[doc = "Field `ISOOUTDROPMSK` writer - Isochronous OUT Packet Dropped Interrupt Mask (device only)"]
pub type IsooutdropmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOPFMSK` reader - End of Periodic Frame Interrupt Mask (device only)"]
pub type EopfmskR = crate::BitReader;
#[doc = "Field `EOPFMSK` writer - End of Periodic Frame Interrupt Mask (device only)"]
pub type EopfmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPMISMSK` reader - Endpoint Mismatch Interrupt Mask (device only)"]
pub type EpmismskR = crate::BitReader;
#[doc = "Field `EPMISMSK` writer - Endpoint Mismatch Interrupt Mask (device only)"]
pub type EpmismskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEPINTMSK` reader - IN Endpoints Interrupt Mask (device only)"]
pub type IepintmskR = crate::BitReader;
#[doc = "Field `IEPINTMSK` writer - IN Endpoints Interrupt Mask (device only)"]
pub type IepintmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEPINTMSK` reader - OUT Endpoints Interrupt Mask (device only)"]
pub type OepintmskR = crate::BitReader;
#[doc = "Field `OEPINTMSK` writer - OUT Endpoints Interrupt Mask (device only)"]
pub type OepintmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INCOMPISOINMSK` reader - Incomplete Isochronous IN Transfer Mask (device only)"]
pub type IncompisoinmskR = crate::BitReader;
#[doc = "Field `INCOMPISOINMSK` writer - Incomplete Isochronous IN Transfer Mask (device only)"]
pub type IncompisoinmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INCOMPLPMSK` reader - Incomplete Periodic Transfer Mask (host only)"]
pub type IncomplpmskR = crate::BitReader;
#[doc = "Field `INCOMPLPMSK` writer - Incomplete Periodic Transfer Mask (host only)"]
pub type IncomplpmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FETSUSPMSK` reader - Data Fetch Suspended Mask (device only)"]
pub type FetsuspmskR = crate::BitReader;
#[doc = "Field `FETSUSPMSK` writer - Data Fetch Suspended Mask (device only)"]
pub type FetsuspmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETDETMSK` reader - Reset detected Interrupt Mask (device only)"]
pub type ResetdetmskR = crate::BitReader;
#[doc = "Field `RESETDETMSK` writer - Reset detected Interrupt Mask (device only)"]
pub type ResetdetmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTINTMSK` reader - Host Port Interrupt Mask (host only)"]
pub type PrtintmskR = crate::BitReader;
#[doc = "Field `PRTINTMSK` writer - Host Port Interrupt Mask (host only)"]
pub type PrtintmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCHINTMSK` reader - Host Channels Interrupt Mask (host only)"]
pub type HchintmskR = crate::BitReader;
#[doc = "Field `HCHINTMSK` writer - Host Channels Interrupt Mask (host only)"]
pub type HchintmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTXFEMPMSK` reader - Periodic TxFIFO Empty Mask (host only)"]
pub type PtxfempmskR = crate::BitReader;
#[doc = "Field `PTXFEMPMSK` writer - Periodic TxFIFO Empty Mask (host only)"]
pub type PtxfempmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONIDSTSCHNGMSK` reader - Connector ID Status Change Mask (host and device)"]
pub type ConidstschngmskR = crate::BitReader;
#[doc = "Field `CONIDSTSCHNGMSK` writer - Connector ID Status Change Mask (host and device)"]
pub type ConidstschngmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCONNINTMSK` reader - Disconnect Detected Interrupt Mask (host and device)"]
pub type DisconnintmskR = crate::BitReader;
#[doc = "Field `DISCONNINTMSK` writer - Disconnect Detected Interrupt Mask (host and device)"]
pub type DisconnintmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SESSREQINTMSK` reader - Session Request/New Session Detected Interrupt Mask (host and device)"]
pub type SessreqintmskR = crate::BitReader;
#[doc = "Field `SESSREQINTMSK` writer - Session Request/New Session Detected Interrupt Mask (host and device)"]
pub type SessreqintmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPINTMSK` reader - Resume/Remote Wakeup Detected Interrupt Mask (host and device)"]
pub type WkupintmskR = crate::BitReader;
#[doc = "Field `WKUPINTMSK` writer - Resume/Remote Wakeup Detected Interrupt Mask (host and device)"]
pub type WkupintmskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Mode Mismatch Interrupt Mask (host and device)"]
    #[inline(always)]
    pub fn modemismsk(&self) -> ModemismskR {
        ModemismskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OTG Interrupt Mask (host and device)"]
    #[inline(always)]
    pub fn otgintmsk(&self) -> OtgintmskR {
        OtgintmskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start of Frame Mask (host and device)"]
    #[inline(always)]
    pub fn sofmsk(&self) -> SofmskR {
        SofmskR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO Non-Empty Mask (host and device)"]
    #[inline(always)]
    pub fn rxflvlmsk(&self) -> RxflvlmskR {
        RxflvlmskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Non-Periodic TxFIFO Empty Mask (host only)"]
    #[inline(always)]
    pub fn nptxfempmsk(&self) -> NptxfempmskR {
        NptxfempmskR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Global Non-periodic IN NAK Effective Mask (device only)"]
    #[inline(always)]
    pub fn ginnakeffmsk(&self) -> GinnakeffmskR {
        GinnakeffmskR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Global OUT NAK Effective Mask (device only)"]
    #[inline(always)]
    pub fn goutnakeffmsk(&self) -> GoutnakeffmskR {
        GoutnakeffmskR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Suspend Mask (device only)"]
    #[inline(always)]
    pub fn erlysuspmsk(&self) -> ErlysuspmskR {
        ErlysuspmskR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USB Suspend Mask (device only)"]
    #[inline(always)]
    pub fn usbsuspmsk(&self) -> UsbsuspmskR {
        UsbsuspmskR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - USB Reset Mask (device only)"]
    #[inline(always)]
    pub fn usbrstmsk(&self) -> UsbrstmskR {
        UsbrstmskR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enumeration Done Mask (device only)"]
    #[inline(always)]
    pub fn enumdonemsk(&self) -> EnumdonemskR {
        EnumdonemskR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Isochronous OUT Packet Dropped Interrupt Mask (device only)"]
    #[inline(always)]
    pub fn isooutdropmsk(&self) -> IsooutdropmskR {
        IsooutdropmskR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - End of Periodic Frame Interrupt Mask (device only)"]
    #[inline(always)]
    pub fn eopfmsk(&self) -> EopfmskR {
        EopfmskR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Endpoint Mismatch Interrupt Mask (device only)"]
    #[inline(always)]
    pub fn epmismsk(&self) -> EpmismskR {
        EpmismskR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - IN Endpoints Interrupt Mask (device only)"]
    #[inline(always)]
    pub fn iepintmsk(&self) -> IepintmskR {
        IepintmskR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OUT Endpoints Interrupt Mask (device only)"]
    #[inline(always)]
    pub fn oepintmsk(&self) -> OepintmskR {
        OepintmskR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Incomplete Isochronous IN Transfer Mask (device only)"]
    #[inline(always)]
    pub fn incompisoinmsk(&self) -> IncompisoinmskR {
        IncompisoinmskR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Incomplete Periodic Transfer Mask (host only)"]
    #[inline(always)]
    pub fn incomplpmsk(&self) -> IncomplpmskR {
        IncomplpmskR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Data Fetch Suspended Mask (device only)"]
    #[inline(always)]
    pub fn fetsuspmsk(&self) -> FetsuspmskR {
        FetsuspmskR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Reset detected Interrupt Mask (device only)"]
    #[inline(always)]
    pub fn resetdetmsk(&self) -> ResetdetmskR {
        ResetdetmskR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Host Port Interrupt Mask (host only)"]
    #[inline(always)]
    pub fn prtintmsk(&self) -> PrtintmskR {
        PrtintmskR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Host Channels Interrupt Mask (host only)"]
    #[inline(always)]
    pub fn hchintmsk(&self) -> HchintmskR {
        HchintmskR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Periodic TxFIFO Empty Mask (host only)"]
    #[inline(always)]
    pub fn ptxfempmsk(&self) -> PtxfempmskR {
        PtxfempmskR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Connector ID Status Change Mask (host and device)"]
    #[inline(always)]
    pub fn conidstschngmsk(&self) -> ConidstschngmskR {
        ConidstschngmskR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Disconnect Detected Interrupt Mask (host and device)"]
    #[inline(always)]
    pub fn disconnintmsk(&self) -> DisconnintmskR {
        DisconnintmskR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Session Request/New Session Detected Interrupt Mask (host and device)"]
    #[inline(always)]
    pub fn sessreqintmsk(&self) -> SessreqintmskR {
        SessreqintmskR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt Mask (host and device)"]
    #[inline(always)]
    pub fn wkupintmsk(&self) -> WkupintmskR {
        WkupintmskR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Mode Mismatch Interrupt Mask (host and device)"]
    #[inline(always)]
    pub fn modemismsk(&mut self) -> ModemismskW<'_, GintmskSpec> {
        ModemismskW::new(self, 1)
    }
    #[doc = "Bit 2 - OTG Interrupt Mask (host and device)"]
    #[inline(always)]
    pub fn otgintmsk(&mut self) -> OtgintmskW<'_, GintmskSpec> {
        OtgintmskW::new(self, 2)
    }
    #[doc = "Bit 3 - Start of Frame Mask (host and device)"]
    #[inline(always)]
    pub fn sofmsk(&mut self) -> SofmskW<'_, GintmskSpec> {
        SofmskW::new(self, 3)
    }
    #[doc = "Bit 4 - Receive FIFO Non-Empty Mask (host and device)"]
    #[inline(always)]
    pub fn rxflvlmsk(&mut self) -> RxflvlmskW<'_, GintmskSpec> {
        RxflvlmskW::new(self, 4)
    }
    #[doc = "Bit 5 - Non-Periodic TxFIFO Empty Mask (host only)"]
    #[inline(always)]
    pub fn nptxfempmsk(&mut self) -> NptxfempmskW<'_, GintmskSpec> {
        NptxfempmskW::new(self, 5)
    }
    #[doc = "Bit 6 - Global Non-periodic IN NAK Effective Mask (device only)"]
    #[inline(always)]
    pub fn ginnakeffmsk(&mut self) -> GinnakeffmskW<'_, GintmskSpec> {
        GinnakeffmskW::new(self, 6)
    }
    #[doc = "Bit 7 - Global OUT NAK Effective Mask (device only)"]
    #[inline(always)]
    pub fn goutnakeffmsk(&mut self) -> GoutnakeffmskW<'_, GintmskSpec> {
        GoutnakeffmskW::new(self, 7)
    }
    #[doc = "Bit 10 - Early Suspend Mask (device only)"]
    #[inline(always)]
    pub fn erlysuspmsk(&mut self) -> ErlysuspmskW<'_, GintmskSpec> {
        ErlysuspmskW::new(self, 10)
    }
    #[doc = "Bit 11 - USB Suspend Mask (device only)"]
    #[inline(always)]
    pub fn usbsuspmsk(&mut self) -> UsbsuspmskW<'_, GintmskSpec> {
        UsbsuspmskW::new(self, 11)
    }
    #[doc = "Bit 12 - USB Reset Mask (device only)"]
    #[inline(always)]
    pub fn usbrstmsk(&mut self) -> UsbrstmskW<'_, GintmskSpec> {
        UsbrstmskW::new(self, 12)
    }
    #[doc = "Bit 13 - Enumeration Done Mask (device only)"]
    #[inline(always)]
    pub fn enumdonemsk(&mut self) -> EnumdonemskW<'_, GintmskSpec> {
        EnumdonemskW::new(self, 13)
    }
    #[doc = "Bit 14 - Isochronous OUT Packet Dropped Interrupt Mask (device only)"]
    #[inline(always)]
    pub fn isooutdropmsk(&mut self) -> IsooutdropmskW<'_, GintmskSpec> {
        IsooutdropmskW::new(self, 14)
    }
    #[doc = "Bit 15 - End of Periodic Frame Interrupt Mask (device only)"]
    #[inline(always)]
    pub fn eopfmsk(&mut self) -> EopfmskW<'_, GintmskSpec> {
        EopfmskW::new(self, 15)
    }
    #[doc = "Bit 17 - Endpoint Mismatch Interrupt Mask (device only)"]
    #[inline(always)]
    pub fn epmismsk(&mut self) -> EpmismskW<'_, GintmskSpec> {
        EpmismskW::new(self, 17)
    }
    #[doc = "Bit 18 - IN Endpoints Interrupt Mask (device only)"]
    #[inline(always)]
    pub fn iepintmsk(&mut self) -> IepintmskW<'_, GintmskSpec> {
        IepintmskW::new(self, 18)
    }
    #[doc = "Bit 19 - OUT Endpoints Interrupt Mask (device only)"]
    #[inline(always)]
    pub fn oepintmsk(&mut self) -> OepintmskW<'_, GintmskSpec> {
        OepintmskW::new(self, 19)
    }
    #[doc = "Bit 20 - Incomplete Isochronous IN Transfer Mask (device only)"]
    #[inline(always)]
    pub fn incompisoinmsk(&mut self) -> IncompisoinmskW<'_, GintmskSpec> {
        IncompisoinmskW::new(self, 20)
    }
    #[doc = "Bit 21 - Incomplete Periodic Transfer Mask (host only)"]
    #[inline(always)]
    pub fn incomplpmsk(&mut self) -> IncomplpmskW<'_, GintmskSpec> {
        IncomplpmskW::new(self, 21)
    }
    #[doc = "Bit 22 - Data Fetch Suspended Mask (device only)"]
    #[inline(always)]
    pub fn fetsuspmsk(&mut self) -> FetsuspmskW<'_, GintmskSpec> {
        FetsuspmskW::new(self, 22)
    }
    #[doc = "Bit 23 - Reset detected Interrupt Mask (device only)"]
    #[inline(always)]
    pub fn resetdetmsk(&mut self) -> ResetdetmskW<'_, GintmskSpec> {
        ResetdetmskW::new(self, 23)
    }
    #[doc = "Bit 24 - Host Port Interrupt Mask (host only)"]
    #[inline(always)]
    pub fn prtintmsk(&mut self) -> PrtintmskW<'_, GintmskSpec> {
        PrtintmskW::new(self, 24)
    }
    #[doc = "Bit 25 - Host Channels Interrupt Mask (host only)"]
    #[inline(always)]
    pub fn hchintmsk(&mut self) -> HchintmskW<'_, GintmskSpec> {
        HchintmskW::new(self, 25)
    }
    #[doc = "Bit 26 - Periodic TxFIFO Empty Mask (host only)"]
    #[inline(always)]
    pub fn ptxfempmsk(&mut self) -> PtxfempmskW<'_, GintmskSpec> {
        PtxfempmskW::new(self, 26)
    }
    #[doc = "Bit 28 - Connector ID Status Change Mask (host and device)"]
    #[inline(always)]
    pub fn conidstschngmsk(&mut self) -> ConidstschngmskW<'_, GintmskSpec> {
        ConidstschngmskW::new(self, 28)
    }
    #[doc = "Bit 29 - Disconnect Detected Interrupt Mask (host and device)"]
    #[inline(always)]
    pub fn disconnintmsk(&mut self) -> DisconnintmskW<'_, GintmskSpec> {
        DisconnintmskW::new(self, 29)
    }
    #[doc = "Bit 30 - Session Request/New Session Detected Interrupt Mask (host and device)"]
    #[inline(always)]
    pub fn sessreqintmsk(&mut self) -> SessreqintmskW<'_, GintmskSpec> {
        SessreqintmskW::new(self, 30)
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt Mask (host and device)"]
    #[inline(always)]
    pub fn wkupintmsk(&mut self) -> WkupintmskW<'_, GintmskSpec> {
        WkupintmskW::new(self, 31)
    }
}
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gintmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gintmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GintmskSpec;
impl crate::RegisterSpec for GintmskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gintmsk::R`](R) reader structure"]
impl crate::Readable for GintmskSpec {}
#[doc = "`write(|w| ..)` method takes [`gintmsk::W`](W) writer structure"]
impl crate::Writable for GintmskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GINTMSK to value 0"]
impl crate::Resettable for GintmskSpec {}
