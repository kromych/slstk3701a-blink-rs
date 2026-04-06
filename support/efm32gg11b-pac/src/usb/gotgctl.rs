#[doc = "Register `GOTGCTL` reader"]
pub type R = crate::R<GotgctlSpec>;
#[doc = "Register `GOTGCTL` writer"]
pub type W = crate::W<GotgctlSpec>;
#[doc = "Field `SESREQSCS` reader - Session Request Success"]
pub type SesreqscsR = crate::BitReader;
#[doc = "Field `SESREQ` reader - Session Request"]
pub type SesreqR = crate::BitReader;
#[doc = "Field `SESREQ` writer - Session Request"]
pub type SesreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBVALIDOVEN` reader - VBUS Valid Override Enable"]
pub type VbvalidovenR = crate::BitReader;
#[doc = "Field `VBVALIDOVEN` writer - VBUS Valid Override Enable"]
pub type VbvalidovenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBVALIDOVVAL` reader - VBUS Valid OverrideValue"]
pub type VbvalidovvalR = crate::BitReader;
#[doc = "Field `VBVALIDOVVAL` writer - VBUS Valid OverrideValue"]
pub type VbvalidovvalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AVALIDOVEN` reader - A-Peripheral Session Valid Override Enable"]
pub type AvalidovenR = crate::BitReader;
#[doc = "Field `AVALIDOVEN` writer - A-Peripheral Session Valid Override Enable"]
pub type AvalidovenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AVALIDOVVAL` reader - A-Peripheral Session Valid OverrideValue"]
pub type AvalidovvalR = crate::BitReader;
#[doc = "Field `AVALIDOVVAL` writer - A-Peripheral Session Valid OverrideValue"]
pub type AvalidovvalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BVALIDOVEN` reader - B-Peripheral Session Valid Override Enable"]
pub type BvalidovenR = crate::BitReader;
#[doc = "Field `BVALIDOVEN` writer - B-Peripheral Session Valid Override Enable"]
pub type BvalidovenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BVALIDOVVAL` reader - B-Peripheral Session Valid OverrideValue"]
pub type BvalidovvalR = crate::BitReader;
#[doc = "Field `BVALIDOVVAL` writer - B-Peripheral Session Valid OverrideValue"]
pub type BvalidovvalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTNEGSCS` reader - Host Negotiation Success"]
pub type HstnegscsR = crate::BitReader;
#[doc = "Field `HNPREQ` reader - HNP Request"]
pub type HnpreqR = crate::BitReader;
#[doc = "Field `HNPREQ` writer - HNP Request"]
pub type HnpreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTSETHNPEN` reader - Host Set HNP Enable"]
pub type HstsethnpenR = crate::BitReader;
#[doc = "Field `HSTSETHNPEN` writer - Host Set HNP Enable"]
pub type HstsethnpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEVHNPEN` reader - Device HNP Enabled"]
pub type DevhnpenR = crate::BitReader;
#[doc = "Field `DEVHNPEN` writer - Device HNP Enabled"]
pub type DevhnpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EHEN` reader - Embedded Host Enable"]
pub type EhenR = crate::BitReader;
#[doc = "Field `EHEN` writer - Embedded Host Enable"]
pub type EhenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBNCEFLTRBYPASS` reader - Debounce Filter Bypass"]
pub type DbncefltrbypassR = crate::BitReader;
#[doc = "Field `DBNCEFLTRBYPASS` writer - Debounce Filter Bypass"]
pub type DbncefltrbypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONIDSTS` reader - Connector ID Status"]
pub type ConidstsR = crate::BitReader;
#[doc = "Field `DBNCTIME` reader - Long/Short Debounce Time"]
pub type DbnctimeR = crate::BitReader;
#[doc = "Field `ASESVLD` reader - A-Session Valid"]
pub type AsesvldR = crate::BitReader;
#[doc = "Field `BSESVLD` reader - B-Session Valid"]
pub type BsesvldR = crate::BitReader;
#[doc = "Field `OTGVER` reader - OTG Version"]
pub type OtgverR = crate::BitReader;
#[doc = "Field `OTGVER` writer - OTG Version"]
pub type OtgverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CURMOD` reader - Current Mode of Operation"]
pub type CurmodR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Session Request Success"]
    #[inline(always)]
    pub fn sesreqscs(&self) -> SesreqscsR {
        SesreqscsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Session Request"]
    #[inline(always)]
    pub fn sesreq(&self) -> SesreqR {
        SesreqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VBUS Valid Override Enable"]
    #[inline(always)]
    pub fn vbvalidoven(&self) -> VbvalidovenR {
        VbvalidovenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VBUS Valid OverrideValue"]
    #[inline(always)]
    pub fn vbvalidovval(&self) -> VbvalidovvalR {
        VbvalidovvalR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - A-Peripheral Session Valid Override Enable"]
    #[inline(always)]
    pub fn avalidoven(&self) -> AvalidovenR {
        AvalidovenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - A-Peripheral Session Valid OverrideValue"]
    #[inline(always)]
    pub fn avalidovval(&self) -> AvalidovvalR {
        AvalidovvalR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - B-Peripheral Session Valid Override Enable"]
    #[inline(always)]
    pub fn bvalidoven(&self) -> BvalidovenR {
        BvalidovenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - B-Peripheral Session Valid OverrideValue"]
    #[inline(always)]
    pub fn bvalidovval(&self) -> BvalidovvalR {
        BvalidovvalR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Host Negotiation Success"]
    #[inline(always)]
    pub fn hstnegscs(&self) -> HstnegscsR {
        HstnegscsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HNP Request"]
    #[inline(always)]
    pub fn hnpreq(&self) -> HnpreqR {
        HnpreqR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Host Set HNP Enable"]
    #[inline(always)]
    pub fn hstsethnpen(&self) -> HstsethnpenR {
        HstsethnpenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Device HNP Enabled"]
    #[inline(always)]
    pub fn devhnpen(&self) -> DevhnpenR {
        DevhnpenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Embedded Host Enable"]
    #[inline(always)]
    pub fn ehen(&self) -> EhenR {
        EhenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Debounce Filter Bypass"]
    #[inline(always)]
    pub fn dbncefltrbypass(&self) -> DbncefltrbypassR {
        DbncefltrbypassR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Connector ID Status"]
    #[inline(always)]
    pub fn conidsts(&self) -> ConidstsR {
        ConidstsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Long/Short Debounce Time"]
    #[inline(always)]
    pub fn dbnctime(&self) -> DbnctimeR {
        DbnctimeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - A-Session Valid"]
    #[inline(always)]
    pub fn asesvld(&self) -> AsesvldR {
        AsesvldR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - B-Session Valid"]
    #[inline(always)]
    pub fn bsesvld(&self) -> BsesvldR {
        BsesvldR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - OTG Version"]
    #[inline(always)]
    pub fn otgver(&self) -> OtgverR {
        OtgverR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Current Mode of Operation"]
    #[inline(always)]
    pub fn curmod(&self) -> CurmodR {
        CurmodR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Session Request"]
    #[inline(always)]
    pub fn sesreq(&mut self) -> SesreqW<'_, GotgctlSpec> {
        SesreqW::new(self, 1)
    }
    #[doc = "Bit 2 - VBUS Valid Override Enable"]
    #[inline(always)]
    pub fn vbvalidoven(&mut self) -> VbvalidovenW<'_, GotgctlSpec> {
        VbvalidovenW::new(self, 2)
    }
    #[doc = "Bit 3 - VBUS Valid OverrideValue"]
    #[inline(always)]
    pub fn vbvalidovval(&mut self) -> VbvalidovvalW<'_, GotgctlSpec> {
        VbvalidovvalW::new(self, 3)
    }
    #[doc = "Bit 4 - A-Peripheral Session Valid Override Enable"]
    #[inline(always)]
    pub fn avalidoven(&mut self) -> AvalidovenW<'_, GotgctlSpec> {
        AvalidovenW::new(self, 4)
    }
    #[doc = "Bit 5 - A-Peripheral Session Valid OverrideValue"]
    #[inline(always)]
    pub fn avalidovval(&mut self) -> AvalidovvalW<'_, GotgctlSpec> {
        AvalidovvalW::new(self, 5)
    }
    #[doc = "Bit 6 - B-Peripheral Session Valid Override Enable"]
    #[inline(always)]
    pub fn bvalidoven(&mut self) -> BvalidovenW<'_, GotgctlSpec> {
        BvalidovenW::new(self, 6)
    }
    #[doc = "Bit 7 - B-Peripheral Session Valid OverrideValue"]
    #[inline(always)]
    pub fn bvalidovval(&mut self) -> BvalidovvalW<'_, GotgctlSpec> {
        BvalidovvalW::new(self, 7)
    }
    #[doc = "Bit 9 - HNP Request"]
    #[inline(always)]
    pub fn hnpreq(&mut self) -> HnpreqW<'_, GotgctlSpec> {
        HnpreqW::new(self, 9)
    }
    #[doc = "Bit 10 - Host Set HNP Enable"]
    #[inline(always)]
    pub fn hstsethnpen(&mut self) -> HstsethnpenW<'_, GotgctlSpec> {
        HstsethnpenW::new(self, 10)
    }
    #[doc = "Bit 11 - Device HNP Enabled"]
    #[inline(always)]
    pub fn devhnpen(&mut self) -> DevhnpenW<'_, GotgctlSpec> {
        DevhnpenW::new(self, 11)
    }
    #[doc = "Bit 12 - Embedded Host Enable"]
    #[inline(always)]
    pub fn ehen(&mut self) -> EhenW<'_, GotgctlSpec> {
        EhenW::new(self, 12)
    }
    #[doc = "Bit 15 - Debounce Filter Bypass"]
    #[inline(always)]
    pub fn dbncefltrbypass(&mut self) -> DbncefltrbypassW<'_, GotgctlSpec> {
        DbncefltrbypassW::new(self, 15)
    }
    #[doc = "Bit 20 - OTG Version"]
    #[inline(always)]
    pub fn otgver(&mut self) -> OtgverW<'_, GotgctlSpec> {
        OtgverW::new(self, 20)
    }
}
#[doc = "OTG Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gotgctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gotgctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GotgctlSpec;
impl crate::RegisterSpec for GotgctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gotgctl::R`](R) reader structure"]
impl crate::Readable for GotgctlSpec {}
#[doc = "`write(|w| ..)` method takes [`gotgctl::W`](W) writer structure"]
impl crate::Writable for GotgctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GOTGCTL to value 0x0001_0000"]
impl crate::Resettable for GotgctlSpec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
