#[doc = "Register `GINTMSK` reader"]
pub type R = crate::R<GINTMSK_SPEC>;
#[doc = "Register `GINTMSK` writer"]
pub type W = crate::W<GINTMSK_SPEC>;
#[doc = "Field `MODEMISMSK` reader - Mode Mismatch Interrupt Mask (host and device)"]
pub type MODEMISMSK_R = crate::BitReader;
#[doc = "Field `MODEMISMSK` writer - Mode Mismatch Interrupt Mask (host and device)"]
pub type MODEMISMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OTGINTMSK` reader - OTG Interrupt Mask (host and device)"]
pub type OTGINTMSK_R = crate::BitReader;
#[doc = "Field `OTGINTMSK` writer - OTG Interrupt Mask (host and device)"]
pub type OTGINTMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOFMSK` reader - Start of Frame Mask (host and device)"]
pub type SOFMSK_R = crate::BitReader;
#[doc = "Field `SOFMSK` writer - Start of Frame Mask (host and device)"]
pub type SOFMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXFLVLMSK` reader - Receive FIFO Non-Empty Mask (host and device)"]
pub type RXFLVLMSK_R = crate::BitReader;
#[doc = "Field `RXFLVLMSK` writer - Receive FIFO Non-Empty Mask (host and device)"]
pub type RXFLVLMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NPTXFEMPMSK` reader - Non-Periodic TxFIFO Empty Mask (host only)"]
pub type NPTXFEMPMSK_R = crate::BitReader;
#[doc = "Field `NPTXFEMPMSK` writer - Non-Periodic TxFIFO Empty Mask (host only)"]
pub type NPTXFEMPMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GINNAKEFFMSK` reader - Global Non-periodic IN NAK Effective Mask (device only)"]
pub type GINNAKEFFMSK_R = crate::BitReader;
#[doc = "Field `GINNAKEFFMSK` writer - Global Non-periodic IN NAK Effective Mask (device only)"]
pub type GINNAKEFFMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GOUTNAKEFFMSK` reader - Global OUT NAK Effective Mask (device only)"]
pub type GOUTNAKEFFMSK_R = crate::BitReader;
#[doc = "Field `GOUTNAKEFFMSK` writer - Global OUT NAK Effective Mask (device only)"]
pub type GOUTNAKEFFMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERLYSUSPMSK` reader - Early Suspend Mask (device only)"]
pub type ERLYSUSPMSK_R = crate::BitReader;
#[doc = "Field `ERLYSUSPMSK` writer - Early Suspend Mask (device only)"]
pub type ERLYSUSPMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USBSUSPMSK` reader - USB Suspend Mask (device only)"]
pub type USBSUSPMSK_R = crate::BitReader;
#[doc = "Field `USBSUSPMSK` writer - USB Suspend Mask (device only)"]
pub type USBSUSPMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USBRSTMSK` reader - USB Reset Mask (device only)"]
pub type USBRSTMSK_R = crate::BitReader;
#[doc = "Field `USBRSTMSK` writer - USB Reset Mask (device only)"]
pub type USBRSTMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENUMDONEMSK` reader - Enumeration Done Mask (device only)"]
pub type ENUMDONEMSK_R = crate::BitReader;
#[doc = "Field `ENUMDONEMSK` writer - Enumeration Done Mask (device only)"]
pub type ENUMDONEMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ISOOUTDROPMSK` reader - Isochronous OUT Packet Dropped Interrupt Mask (device only)"]
pub type ISOOUTDROPMSK_R = crate::BitReader;
#[doc = "Field `ISOOUTDROPMSK` writer - Isochronous OUT Packet Dropped Interrupt Mask (device only)"]
pub type ISOOUTDROPMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOPFMSK` reader - End of Periodic Frame Interrupt Mask (device only)"]
pub type EOPFMSK_R = crate::BitReader;
#[doc = "Field `EOPFMSK` writer - End of Periodic Frame Interrupt Mask (device only)"]
pub type EOPFMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPMISMSK` reader - Endpoint Mismatch Interrupt Mask (device only)"]
pub type EPMISMSK_R = crate::BitReader;
#[doc = "Field `EPMISMSK` writer - Endpoint Mismatch Interrupt Mask (device only)"]
pub type EPMISMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IEPINTMSK` reader - IN Endpoints Interrupt Mask (device only)"]
pub type IEPINTMSK_R = crate::BitReader;
#[doc = "Field `IEPINTMSK` writer - IN Endpoints Interrupt Mask (device only)"]
pub type IEPINTMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OEPINTMSK` reader - OUT Endpoints Interrupt Mask (device only)"]
pub type OEPINTMSK_R = crate::BitReader;
#[doc = "Field `OEPINTMSK` writer - OUT Endpoints Interrupt Mask (device only)"]
pub type OEPINTMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INCOMPISOINMSK` reader - Incomplete Isochronous IN Transfer Mask (device only)"]
pub type INCOMPISOINMSK_R = crate::BitReader;
#[doc = "Field `INCOMPISOINMSK` writer - Incomplete Isochronous IN Transfer Mask (device only)"]
pub type INCOMPISOINMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INCOMPLPMSK` reader - Incomplete Periodic Transfer Mask (host only)"]
pub type INCOMPLPMSK_R = crate::BitReader;
#[doc = "Field `INCOMPLPMSK` writer - Incomplete Periodic Transfer Mask (host only)"]
pub type INCOMPLPMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FETSUSPMSK` reader - Data Fetch Suspended Mask (device only)"]
pub type FETSUSPMSK_R = crate::BitReader;
#[doc = "Field `FETSUSPMSK` writer - Data Fetch Suspended Mask (device only)"]
pub type FETSUSPMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESETDETMSK` reader - Reset detected Interrupt Mask (device only)"]
pub type RESETDETMSK_R = crate::BitReader;
#[doc = "Field `RESETDETMSK` writer - Reset detected Interrupt Mask (device only)"]
pub type RESETDETMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PRTINTMSK` reader - Host Port Interrupt Mask (host only)"]
pub type PRTINTMSK_R = crate::BitReader;
#[doc = "Field `PRTINTMSK` writer - Host Port Interrupt Mask (host only)"]
pub type PRTINTMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HCHINTMSK` reader - Host Channels Interrupt Mask (host only)"]
pub type HCHINTMSK_R = crate::BitReader;
#[doc = "Field `HCHINTMSK` writer - Host Channels Interrupt Mask (host only)"]
pub type HCHINTMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PTXFEMPMSK` reader - Periodic TxFIFO Empty Mask (host only)"]
pub type PTXFEMPMSK_R = crate::BitReader;
#[doc = "Field `PTXFEMPMSK` writer - Periodic TxFIFO Empty Mask (host only)"]
pub type PTXFEMPMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CONIDSTSCHNGMSK` reader - Connector ID Status Change Mask (host and device)"]
pub type CONIDSTSCHNGMSK_R = crate::BitReader;
#[doc = "Field `CONIDSTSCHNGMSK` writer - Connector ID Status Change Mask (host and device)"]
pub type CONIDSTSCHNGMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DISCONNINTMSK` reader - Disconnect Detected Interrupt Mask (host and device)"]
pub type DISCONNINTMSK_R = crate::BitReader;
#[doc = "Field `DISCONNINTMSK` writer - Disconnect Detected Interrupt Mask (host and device)"]
pub type DISCONNINTMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SESSREQINTMSK` reader - Session Request/New Session Detected Interrupt Mask (host and device)"]
pub type SESSREQINTMSK_R = crate::BitReader;
#[doc = "Field `SESSREQINTMSK` writer - Session Request/New Session Detected Interrupt Mask (host and device)"]
pub type SESSREQINTMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WKUPINTMSK` reader - Resume/Remote Wakeup Detected Interrupt Mask (host and device)"]
pub type WKUPINTMSK_R = crate::BitReader;
#[doc = "Field `WKUPINTMSK` writer - Resume/Remote Wakeup Detected Interrupt Mask (host and device)"]
pub type WKUPINTMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - Mode Mismatch Interrupt Mask (host and device)"]
    #[inline(always)]
    pub fn modemismsk(&self) -> MODEMISMSK_R {
        MODEMISMSK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OTG Interrupt Mask (host and device)"]
    #[inline(always)]
    pub fn otgintmsk(&self) -> OTGINTMSK_R {
        OTGINTMSK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start of Frame Mask (host and device)"]
    #[inline(always)]
    pub fn sofmsk(&self) -> SOFMSK_R {
        SOFMSK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO Non-Empty Mask (host and device)"]
    #[inline(always)]
    pub fn rxflvlmsk(&self) -> RXFLVLMSK_R {
        RXFLVLMSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Non-Periodic TxFIFO Empty Mask (host only)"]
    #[inline(always)]
    pub fn nptxfempmsk(&self) -> NPTXFEMPMSK_R {
        NPTXFEMPMSK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Global Non-periodic IN NAK Effective Mask (device only)"]
    #[inline(always)]
    pub fn ginnakeffmsk(&self) -> GINNAKEFFMSK_R {
        GINNAKEFFMSK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Global OUT NAK Effective Mask (device only)"]
    #[inline(always)]
    pub fn goutnakeffmsk(&self) -> GOUTNAKEFFMSK_R {
        GOUTNAKEFFMSK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Suspend Mask (device only)"]
    #[inline(always)]
    pub fn erlysuspmsk(&self) -> ERLYSUSPMSK_R {
        ERLYSUSPMSK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USB Suspend Mask (device only)"]
    #[inline(always)]
    pub fn usbsuspmsk(&self) -> USBSUSPMSK_R {
        USBSUSPMSK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - USB Reset Mask (device only)"]
    #[inline(always)]
    pub fn usbrstmsk(&self) -> USBRSTMSK_R {
        USBRSTMSK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enumeration Done Mask (device only)"]
    #[inline(always)]
    pub fn enumdonemsk(&self) -> ENUMDONEMSK_R {
        ENUMDONEMSK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Isochronous OUT Packet Dropped Interrupt Mask (device only)"]
    #[inline(always)]
    pub fn isooutdropmsk(&self) -> ISOOUTDROPMSK_R {
        ISOOUTDROPMSK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - End of Periodic Frame Interrupt Mask (device only)"]
    #[inline(always)]
    pub fn eopfmsk(&self) -> EOPFMSK_R {
        EOPFMSK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Endpoint Mismatch Interrupt Mask (device only)"]
    #[inline(always)]
    pub fn epmismsk(&self) -> EPMISMSK_R {
        EPMISMSK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - IN Endpoints Interrupt Mask (device only)"]
    #[inline(always)]
    pub fn iepintmsk(&self) -> IEPINTMSK_R {
        IEPINTMSK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OUT Endpoints Interrupt Mask (device only)"]
    #[inline(always)]
    pub fn oepintmsk(&self) -> OEPINTMSK_R {
        OEPINTMSK_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Incomplete Isochronous IN Transfer Mask (device only)"]
    #[inline(always)]
    pub fn incompisoinmsk(&self) -> INCOMPISOINMSK_R {
        INCOMPISOINMSK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Incomplete Periodic Transfer Mask (host only)"]
    #[inline(always)]
    pub fn incomplpmsk(&self) -> INCOMPLPMSK_R {
        INCOMPLPMSK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Data Fetch Suspended Mask (device only)"]
    #[inline(always)]
    pub fn fetsuspmsk(&self) -> FETSUSPMSK_R {
        FETSUSPMSK_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Reset detected Interrupt Mask (device only)"]
    #[inline(always)]
    pub fn resetdetmsk(&self) -> RESETDETMSK_R {
        RESETDETMSK_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Host Port Interrupt Mask (host only)"]
    #[inline(always)]
    pub fn prtintmsk(&self) -> PRTINTMSK_R {
        PRTINTMSK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Host Channels Interrupt Mask (host only)"]
    #[inline(always)]
    pub fn hchintmsk(&self) -> HCHINTMSK_R {
        HCHINTMSK_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Periodic TxFIFO Empty Mask (host only)"]
    #[inline(always)]
    pub fn ptxfempmsk(&self) -> PTXFEMPMSK_R {
        PTXFEMPMSK_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Connector ID Status Change Mask (host and device)"]
    #[inline(always)]
    pub fn conidstschngmsk(&self) -> CONIDSTSCHNGMSK_R {
        CONIDSTSCHNGMSK_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Disconnect Detected Interrupt Mask (host and device)"]
    #[inline(always)]
    pub fn disconnintmsk(&self) -> DISCONNINTMSK_R {
        DISCONNINTMSK_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Session Request/New Session Detected Interrupt Mask (host and device)"]
    #[inline(always)]
    pub fn sessreqintmsk(&self) -> SESSREQINTMSK_R {
        SESSREQINTMSK_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt Mask (host and device)"]
    #[inline(always)]
    pub fn wkupintmsk(&self) -> WKUPINTMSK_R {
        WKUPINTMSK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Mode Mismatch Interrupt Mask (host and device)"]
    #[inline(always)]
    #[must_use]
    pub fn modemismsk(&mut self) -> MODEMISMSK_W<GINTMSK_SPEC, 1> {
        MODEMISMSK_W::new(self)
    }
    #[doc = "Bit 2 - OTG Interrupt Mask (host and device)"]
    #[inline(always)]
    #[must_use]
    pub fn otgintmsk(&mut self) -> OTGINTMSK_W<GINTMSK_SPEC, 2> {
        OTGINTMSK_W::new(self)
    }
    #[doc = "Bit 3 - Start of Frame Mask (host and device)"]
    #[inline(always)]
    #[must_use]
    pub fn sofmsk(&mut self) -> SOFMSK_W<GINTMSK_SPEC, 3> {
        SOFMSK_W::new(self)
    }
    #[doc = "Bit 4 - Receive FIFO Non-Empty Mask (host and device)"]
    #[inline(always)]
    #[must_use]
    pub fn rxflvlmsk(&mut self) -> RXFLVLMSK_W<GINTMSK_SPEC, 4> {
        RXFLVLMSK_W::new(self)
    }
    #[doc = "Bit 5 - Non-Periodic TxFIFO Empty Mask (host only)"]
    #[inline(always)]
    #[must_use]
    pub fn nptxfempmsk(&mut self) -> NPTXFEMPMSK_W<GINTMSK_SPEC, 5> {
        NPTXFEMPMSK_W::new(self)
    }
    #[doc = "Bit 6 - Global Non-periodic IN NAK Effective Mask (device only)"]
    #[inline(always)]
    #[must_use]
    pub fn ginnakeffmsk(&mut self) -> GINNAKEFFMSK_W<GINTMSK_SPEC, 6> {
        GINNAKEFFMSK_W::new(self)
    }
    #[doc = "Bit 7 - Global OUT NAK Effective Mask (device only)"]
    #[inline(always)]
    #[must_use]
    pub fn goutnakeffmsk(&mut self) -> GOUTNAKEFFMSK_W<GINTMSK_SPEC, 7> {
        GOUTNAKEFFMSK_W::new(self)
    }
    #[doc = "Bit 10 - Early Suspend Mask (device only)"]
    #[inline(always)]
    #[must_use]
    pub fn erlysuspmsk(&mut self) -> ERLYSUSPMSK_W<GINTMSK_SPEC, 10> {
        ERLYSUSPMSK_W::new(self)
    }
    #[doc = "Bit 11 - USB Suspend Mask (device only)"]
    #[inline(always)]
    #[must_use]
    pub fn usbsuspmsk(&mut self) -> USBSUSPMSK_W<GINTMSK_SPEC, 11> {
        USBSUSPMSK_W::new(self)
    }
    #[doc = "Bit 12 - USB Reset Mask (device only)"]
    #[inline(always)]
    #[must_use]
    pub fn usbrstmsk(&mut self) -> USBRSTMSK_W<GINTMSK_SPEC, 12> {
        USBRSTMSK_W::new(self)
    }
    #[doc = "Bit 13 - Enumeration Done Mask (device only)"]
    #[inline(always)]
    #[must_use]
    pub fn enumdonemsk(&mut self) -> ENUMDONEMSK_W<GINTMSK_SPEC, 13> {
        ENUMDONEMSK_W::new(self)
    }
    #[doc = "Bit 14 - Isochronous OUT Packet Dropped Interrupt Mask (device only)"]
    #[inline(always)]
    #[must_use]
    pub fn isooutdropmsk(&mut self) -> ISOOUTDROPMSK_W<GINTMSK_SPEC, 14> {
        ISOOUTDROPMSK_W::new(self)
    }
    #[doc = "Bit 15 - End of Periodic Frame Interrupt Mask (device only)"]
    #[inline(always)]
    #[must_use]
    pub fn eopfmsk(&mut self) -> EOPFMSK_W<GINTMSK_SPEC, 15> {
        EOPFMSK_W::new(self)
    }
    #[doc = "Bit 17 - Endpoint Mismatch Interrupt Mask (device only)"]
    #[inline(always)]
    #[must_use]
    pub fn epmismsk(&mut self) -> EPMISMSK_W<GINTMSK_SPEC, 17> {
        EPMISMSK_W::new(self)
    }
    #[doc = "Bit 18 - IN Endpoints Interrupt Mask (device only)"]
    #[inline(always)]
    #[must_use]
    pub fn iepintmsk(&mut self) -> IEPINTMSK_W<GINTMSK_SPEC, 18> {
        IEPINTMSK_W::new(self)
    }
    #[doc = "Bit 19 - OUT Endpoints Interrupt Mask (device only)"]
    #[inline(always)]
    #[must_use]
    pub fn oepintmsk(&mut self) -> OEPINTMSK_W<GINTMSK_SPEC, 19> {
        OEPINTMSK_W::new(self)
    }
    #[doc = "Bit 20 - Incomplete Isochronous IN Transfer Mask (device only)"]
    #[inline(always)]
    #[must_use]
    pub fn incompisoinmsk(&mut self) -> INCOMPISOINMSK_W<GINTMSK_SPEC, 20> {
        INCOMPISOINMSK_W::new(self)
    }
    #[doc = "Bit 21 - Incomplete Periodic Transfer Mask (host only)"]
    #[inline(always)]
    #[must_use]
    pub fn incomplpmsk(&mut self) -> INCOMPLPMSK_W<GINTMSK_SPEC, 21> {
        INCOMPLPMSK_W::new(self)
    }
    #[doc = "Bit 22 - Data Fetch Suspended Mask (device only)"]
    #[inline(always)]
    #[must_use]
    pub fn fetsuspmsk(&mut self) -> FETSUSPMSK_W<GINTMSK_SPEC, 22> {
        FETSUSPMSK_W::new(self)
    }
    #[doc = "Bit 23 - Reset detected Interrupt Mask (device only)"]
    #[inline(always)]
    #[must_use]
    pub fn resetdetmsk(&mut self) -> RESETDETMSK_W<GINTMSK_SPEC, 23> {
        RESETDETMSK_W::new(self)
    }
    #[doc = "Bit 24 - Host Port Interrupt Mask (host only)"]
    #[inline(always)]
    #[must_use]
    pub fn prtintmsk(&mut self) -> PRTINTMSK_W<GINTMSK_SPEC, 24> {
        PRTINTMSK_W::new(self)
    }
    #[doc = "Bit 25 - Host Channels Interrupt Mask (host only)"]
    #[inline(always)]
    #[must_use]
    pub fn hchintmsk(&mut self) -> HCHINTMSK_W<GINTMSK_SPEC, 25> {
        HCHINTMSK_W::new(self)
    }
    #[doc = "Bit 26 - Periodic TxFIFO Empty Mask (host only)"]
    #[inline(always)]
    #[must_use]
    pub fn ptxfempmsk(&mut self) -> PTXFEMPMSK_W<GINTMSK_SPEC, 26> {
        PTXFEMPMSK_W::new(self)
    }
    #[doc = "Bit 28 - Connector ID Status Change Mask (host and device)"]
    #[inline(always)]
    #[must_use]
    pub fn conidstschngmsk(&mut self) -> CONIDSTSCHNGMSK_W<GINTMSK_SPEC, 28> {
        CONIDSTSCHNGMSK_W::new(self)
    }
    #[doc = "Bit 29 - Disconnect Detected Interrupt Mask (host and device)"]
    #[inline(always)]
    #[must_use]
    pub fn disconnintmsk(&mut self) -> DISCONNINTMSK_W<GINTMSK_SPEC, 29> {
        DISCONNINTMSK_W::new(self)
    }
    #[doc = "Bit 30 - Session Request/New Session Detected Interrupt Mask (host and device)"]
    #[inline(always)]
    #[must_use]
    pub fn sessreqintmsk(&mut self) -> SESSREQINTMSK_W<GINTMSK_SPEC, 30> {
        SESSREQINTMSK_W::new(self)
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt Mask (host and device)"]
    #[inline(always)]
    #[must_use]
    pub fn wkupintmsk(&mut self) -> WKUPINTMSK_W<GINTMSK_SPEC, 31> {
        WKUPINTMSK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GINTMSK_SPEC;
impl crate::RegisterSpec for GINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gintmsk::R`](R) reader structure"]
impl crate::Readable for GINTMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gintmsk::W`](W) writer structure"]
impl crate::Writable for GINTMSK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GINTMSK to value 0"]
impl crate::Resettable for GINTMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
