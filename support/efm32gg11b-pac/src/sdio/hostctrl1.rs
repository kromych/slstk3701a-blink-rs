#[doc = "Register `HOSTCTRL1` reader"]
pub type R = crate::R<Hostctrl1Spec>;
#[doc = "Register `HOSTCTRL1` writer"]
pub type W = crate::W<Hostctrl1Spec>;
#[doc = "Field `LEDCTRL` reader - LED Control"]
pub type LedctrlR = crate::BitReader;
#[doc = "Field `LEDCTRL` writer - LED Control"]
pub type LedctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATTRANWD` reader - Data Transfer Width 1-bit or 4-bit Mode"]
pub type DattranwdR = crate::BitReader;
#[doc = "Field `DATTRANWD` writer - Data Transfer Width 1-bit or 4-bit Mode"]
pub type DattranwdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEN` reader - High Speed Enable"]
pub type HsenR = crate::BitReader;
#[doc = "Field `HSEN` writer - High Speed Enable"]
pub type HsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "DMA Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dmasel {
    #[doc = "0: SDMA selected"]
    Sdma = 0,
    #[doc = "1: 32-bit ADMA1 selected"]
    Adma1 = 1,
    #[doc = "2: 32-bit ADMA2 selected"]
    Adma2 = 2,
    #[doc = "3: 64-bit ADMA2 selected"]
    _64bitadma2 = 3,
}
impl From<Dmasel> for u8 {
    #[inline(always)]
    fn from(variant: Dmasel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dmasel {
    type Ux = u8;
}
impl crate::IsEnum for Dmasel {}
#[doc = "Field `DMASEL` reader - DMA Select"]
pub type DmaselR = crate::FieldReader<Dmasel>;
impl DmaselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmasel {
        match self.bits {
            0 => Dmasel::Sdma,
            1 => Dmasel::Adma1,
            2 => Dmasel::Adma2,
            3 => Dmasel::_64bitadma2,
            _ => unreachable!(),
        }
    }
    #[doc = "SDMA selected"]
    #[inline(always)]
    pub fn is_sdma(&self) -> bool {
        *self == Dmasel::Sdma
    }
    #[doc = "32-bit ADMA1 selected"]
    #[inline(always)]
    pub fn is_adma1(&self) -> bool {
        *self == Dmasel::Adma1
    }
    #[doc = "32-bit ADMA2 selected"]
    #[inline(always)]
    pub fn is_adma2(&self) -> bool {
        *self == Dmasel::Adma2
    }
    #[doc = "64-bit ADMA2 selected"]
    #[inline(always)]
    pub fn is_64bitadma2(&self) -> bool {
        *self == Dmasel::_64bitadma2
    }
}
#[doc = "Field `DMASEL` writer - DMA Select"]
pub type DmaselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dmasel, crate::Safe>;
impl<'a, REG> DmaselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SDMA selected"]
    #[inline(always)]
    pub fn sdma(self) -> &'a mut crate::W<REG> {
        self.variant(Dmasel::Sdma)
    }
    #[doc = "32-bit ADMA1 selected"]
    #[inline(always)]
    pub fn adma1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmasel::Adma1)
    }
    #[doc = "32-bit ADMA2 selected"]
    #[inline(always)]
    pub fn adma2(self) -> &'a mut crate::W<REG> {
        self.variant(Dmasel::Adma2)
    }
    #[doc = "64-bit ADMA2 selected"]
    #[inline(always)]
    pub fn _64bitadma2(self) -> &'a mut crate::W<REG> {
        self.variant(Dmasel::_64bitadma2)
    }
}
#[doc = "Field `EXTDATTRANWD` reader - Extended Data Transfer Width"]
pub type ExtdattranwdR = crate::BitReader;
#[doc = "Field `EXTDATTRANWD` writer - Extended Data Transfer Width"]
pub type ExtdattranwdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDTSTLVL` reader - Card Detect Test Level"]
pub type CdtstlvlR = crate::BitReader;
#[doc = "Field `CDTSTLVL` writer - Card Detect Test Level"]
pub type CdtstlvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDSIGDET` reader - Card Detetct Signal Detection"]
pub type CdsigdetR = crate::BitReader;
#[doc = "Field `CDSIGDET` writer - Card Detetct Signal Detection"]
pub type CdsigdetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDBUSPOWER` reader - SD Bus Power"]
pub type SdbuspowerR = crate::BitReader;
#[doc = "Field `SDBUSPOWER` writer - SD Bus Power"]
pub type SdbuspowerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "SD Bus Voltage Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sdbusvoltsel {
    #[doc = "5: Select 1.8V"]
    _1p8v = 5,
    #[doc = "6: Select 3.0V"]
    _3p0v = 6,
    #[doc = "7: Select 3.3V"]
    _3p3v = 7,
}
impl From<Sdbusvoltsel> for u8 {
    #[inline(always)]
    fn from(variant: Sdbusvoltsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sdbusvoltsel {
    type Ux = u8;
}
impl crate::IsEnum for Sdbusvoltsel {}
#[doc = "Field `SDBUSVOLTSEL` reader - SD Bus Voltage Select"]
pub type SdbusvoltselR = crate::FieldReader<Sdbusvoltsel>;
impl SdbusvoltselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sdbusvoltsel> {
        match self.bits {
            5 => Some(Sdbusvoltsel::_1p8v),
            6 => Some(Sdbusvoltsel::_3p0v),
            7 => Some(Sdbusvoltsel::_3p3v),
            _ => None,
        }
    }
    #[doc = "Select 1.8V"]
    #[inline(always)]
    pub fn is_1p8v(&self) -> bool {
        *self == Sdbusvoltsel::_1p8v
    }
    #[doc = "Select 3.0V"]
    #[inline(always)]
    pub fn is_3p0v(&self) -> bool {
        *self == Sdbusvoltsel::_3p0v
    }
    #[doc = "Select 3.3V"]
    #[inline(always)]
    pub fn is_3p3v(&self) -> bool {
        *self == Sdbusvoltsel::_3p3v
    }
}
#[doc = "Field `SDBUSVOLTSEL` writer - SD Bus Voltage Select"]
pub type SdbusvoltselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sdbusvoltsel>;
impl<'a, REG> SdbusvoltselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select 1.8V"]
    #[inline(always)]
    pub fn _1p8v(self) -> &'a mut crate::W<REG> {
        self.variant(Sdbusvoltsel::_1p8v)
    }
    #[doc = "Select 3.0V"]
    #[inline(always)]
    pub fn _3p0v(self) -> &'a mut crate::W<REG> {
        self.variant(Sdbusvoltsel::_3p0v)
    }
    #[doc = "Select 3.3V"]
    #[inline(always)]
    pub fn _3p3v(self) -> &'a mut crate::W<REG> {
        self.variant(Sdbusvoltsel::_3p3v)
    }
}
#[doc = "Field `HRDRST` reader - Hardware Reset Signal"]
pub type HrdrstR = crate::BitReader;
#[doc = "Field `HRDRST` writer - Hardware Reset Signal"]
pub type HrdrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPATBLKGAPREQ` reader - Stop at Block Gap Request"]
pub type StopatblkgapreqR = crate::BitReader;
#[doc = "Field `STOPATBLKGAPREQ` writer - Stop at Block Gap Request"]
pub type StopatblkgapreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONTINUEREQ` reader - Continue Request"]
pub type ContinuereqR = crate::BitReader;
#[doc = "Field `CONTINUEREQ` writer - Continue Request"]
pub type ContinuereqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDWAITCTRL` reader - Read Wait Control"]
pub type RdwaitctrlR = crate::BitReader;
#[doc = "Field `RDWAITCTRL` writer - Read Wait Control"]
pub type RdwaitctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTATBLKGAP` reader - Interrupt at Block Gap"]
pub type IntatblkgapR = crate::BitReader;
#[doc = "Field `INTATBLKGAP` writer - Interrupt at Block Gap"]
pub type IntatblkgapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIMODE` reader - SPI Mode Enable"]
pub type SpimodeR = crate::BitReader;
#[doc = "Field `SPIMODE` writer - SPI Mode Enable"]
pub type SpimodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOTEN` reader - Boot Enable"]
pub type BootenR = crate::BitReader;
#[doc = "Field `BOOTEN` writer - Boot Enable"]
pub type BootenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALTBOOTEN` reader - Alternate Boot Enable"]
pub type AltbootenR = crate::BitReader;
#[doc = "Field `ALTBOOTEN` writer - Alternate Boot Enable"]
pub type AltbootenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOTACKCHK` reader - Boot Ack Check"]
pub type BootackchkR = crate::BitReader;
#[doc = "Field `BOOTACKCHK` writer - Boot Ack Check"]
pub type BootackchkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPEVNTENONCARDINT` reader - Wakeup Event Enable on Card Interrupt"]
pub type WkupevntenoncardintR = crate::BitReader;
#[doc = "Field `WKUPEVNTENONCARDINT` writer - Wakeup Event Enable on Card Interrupt"]
pub type WkupevntenoncardintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPEVNTENONCINS` reader - Wakeup Event Enable on SD Card Insertion"]
pub type WkupevntenoncinsR = crate::BitReader;
#[doc = "Field `WKUPEVNTENONCINS` writer - Wakeup Event Enable on SD Card Insertion"]
pub type WkupevntenoncinsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPEVNTENONCRM` reader - Wakeup Event Enable on SD Card Removal"]
pub type WkupevntenoncrmR = crate::BitReader;
#[doc = "Field `WKUPEVNTENONCRM` writer - Wakeup Event Enable on SD Card Removal"]
pub type WkupevntenoncrmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LED Control"]
    #[inline(always)]
    pub fn ledctrl(&self) -> LedctrlR {
        LedctrlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data Transfer Width 1-bit or 4-bit Mode"]
    #[inline(always)]
    pub fn dattranwd(&self) -> DattranwdR {
        DattranwdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - High Speed Enable"]
    #[inline(always)]
    pub fn hsen(&self) -> HsenR {
        HsenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - DMA Select"]
    #[inline(always)]
    pub fn dmasel(&self) -> DmaselR {
        DmaselR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Extended Data Transfer Width"]
    #[inline(always)]
    pub fn extdattranwd(&self) -> ExtdattranwdR {
        ExtdattranwdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Card Detect Test Level"]
    #[inline(always)]
    pub fn cdtstlvl(&self) -> CdtstlvlR {
        CdtstlvlR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card Detetct Signal Detection"]
    #[inline(always)]
    pub fn cdsigdet(&self) -> CdsigdetR {
        CdsigdetR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SD Bus Power"]
    #[inline(always)]
    pub fn sdbuspower(&self) -> SdbuspowerR {
        SdbuspowerR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - SD Bus Voltage Select"]
    #[inline(always)]
    pub fn sdbusvoltsel(&self) -> SdbusvoltselR {
        SdbusvoltselR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - Hardware Reset Signal"]
    #[inline(always)]
    pub fn hrdrst(&self) -> HrdrstR {
        HrdrstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Stop at Block Gap Request"]
    #[inline(always)]
    pub fn stopatblkgapreq(&self) -> StopatblkgapreqR {
        StopatblkgapreqR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Continue Request"]
    #[inline(always)]
    pub fn continuereq(&self) -> ContinuereqR {
        ContinuereqR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Read Wait Control"]
    #[inline(always)]
    pub fn rdwaitctrl(&self) -> RdwaitctrlR {
        RdwaitctrlR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt at Block Gap"]
    #[inline(always)]
    pub fn intatblkgap(&self) -> IntatblkgapR {
        IntatblkgapR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SPI Mode Enable"]
    #[inline(always)]
    pub fn spimode(&self) -> SpimodeR {
        SpimodeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Boot Enable"]
    #[inline(always)]
    pub fn booten(&self) -> BootenR {
        BootenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Alternate Boot Enable"]
    #[inline(always)]
    pub fn altbooten(&self) -> AltbootenR {
        AltbootenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Boot Ack Check"]
    #[inline(always)]
    pub fn bootackchk(&self) -> BootackchkR {
        BootackchkR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Wakeup Event Enable on Card Interrupt"]
    #[inline(always)]
    pub fn wkupevntenoncardint(&self) -> WkupevntenoncardintR {
        WkupevntenoncardintR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Wakeup Event Enable on SD Card Insertion"]
    #[inline(always)]
    pub fn wkupevntenoncins(&self) -> WkupevntenoncinsR {
        WkupevntenoncinsR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Wakeup Event Enable on SD Card Removal"]
    #[inline(always)]
    pub fn wkupevntenoncrm(&self) -> WkupevntenoncrmR {
        WkupevntenoncrmR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LED Control"]
    #[inline(always)]
    pub fn ledctrl(&mut self) -> LedctrlW<'_, Hostctrl1Spec> {
        LedctrlW::new(self, 0)
    }
    #[doc = "Bit 1 - Data Transfer Width 1-bit or 4-bit Mode"]
    #[inline(always)]
    pub fn dattranwd(&mut self) -> DattranwdW<'_, Hostctrl1Spec> {
        DattranwdW::new(self, 1)
    }
    #[doc = "Bit 2 - High Speed Enable"]
    #[inline(always)]
    pub fn hsen(&mut self) -> HsenW<'_, Hostctrl1Spec> {
        HsenW::new(self, 2)
    }
    #[doc = "Bits 3:4 - DMA Select"]
    #[inline(always)]
    pub fn dmasel(&mut self) -> DmaselW<'_, Hostctrl1Spec> {
        DmaselW::new(self, 3)
    }
    #[doc = "Bit 5 - Extended Data Transfer Width"]
    #[inline(always)]
    pub fn extdattranwd(&mut self) -> ExtdattranwdW<'_, Hostctrl1Spec> {
        ExtdattranwdW::new(self, 5)
    }
    #[doc = "Bit 6 - Card Detect Test Level"]
    #[inline(always)]
    pub fn cdtstlvl(&mut self) -> CdtstlvlW<'_, Hostctrl1Spec> {
        CdtstlvlW::new(self, 6)
    }
    #[doc = "Bit 7 - Card Detetct Signal Detection"]
    #[inline(always)]
    pub fn cdsigdet(&mut self) -> CdsigdetW<'_, Hostctrl1Spec> {
        CdsigdetW::new(self, 7)
    }
    #[doc = "Bit 8 - SD Bus Power"]
    #[inline(always)]
    pub fn sdbuspower(&mut self) -> SdbuspowerW<'_, Hostctrl1Spec> {
        SdbuspowerW::new(self, 8)
    }
    #[doc = "Bits 9:11 - SD Bus Voltage Select"]
    #[inline(always)]
    pub fn sdbusvoltsel(&mut self) -> SdbusvoltselW<'_, Hostctrl1Spec> {
        SdbusvoltselW::new(self, 9)
    }
    #[doc = "Bit 12 - Hardware Reset Signal"]
    #[inline(always)]
    pub fn hrdrst(&mut self) -> HrdrstW<'_, Hostctrl1Spec> {
        HrdrstW::new(self, 12)
    }
    #[doc = "Bit 16 - Stop at Block Gap Request"]
    #[inline(always)]
    pub fn stopatblkgapreq(&mut self) -> StopatblkgapreqW<'_, Hostctrl1Spec> {
        StopatblkgapreqW::new(self, 16)
    }
    #[doc = "Bit 17 - Continue Request"]
    #[inline(always)]
    pub fn continuereq(&mut self) -> ContinuereqW<'_, Hostctrl1Spec> {
        ContinuereqW::new(self, 17)
    }
    #[doc = "Bit 18 - Read Wait Control"]
    #[inline(always)]
    pub fn rdwaitctrl(&mut self) -> RdwaitctrlW<'_, Hostctrl1Spec> {
        RdwaitctrlW::new(self, 18)
    }
    #[doc = "Bit 19 - Interrupt at Block Gap"]
    #[inline(always)]
    pub fn intatblkgap(&mut self) -> IntatblkgapW<'_, Hostctrl1Spec> {
        IntatblkgapW::new(self, 19)
    }
    #[doc = "Bit 20 - SPI Mode Enable"]
    #[inline(always)]
    pub fn spimode(&mut self) -> SpimodeW<'_, Hostctrl1Spec> {
        SpimodeW::new(self, 20)
    }
    #[doc = "Bit 21 - Boot Enable"]
    #[inline(always)]
    pub fn booten(&mut self) -> BootenW<'_, Hostctrl1Spec> {
        BootenW::new(self, 21)
    }
    #[doc = "Bit 22 - Alternate Boot Enable"]
    #[inline(always)]
    pub fn altbooten(&mut self) -> AltbootenW<'_, Hostctrl1Spec> {
        AltbootenW::new(self, 22)
    }
    #[doc = "Bit 23 - Boot Ack Check"]
    #[inline(always)]
    pub fn bootackchk(&mut self) -> BootackchkW<'_, Hostctrl1Spec> {
        BootackchkW::new(self, 23)
    }
    #[doc = "Bit 24 - Wakeup Event Enable on Card Interrupt"]
    #[inline(always)]
    pub fn wkupevntenoncardint(&mut self) -> WkupevntenoncardintW<'_, Hostctrl1Spec> {
        WkupevntenoncardintW::new(self, 24)
    }
    #[doc = "Bit 25 - Wakeup Event Enable on SD Card Insertion"]
    #[inline(always)]
    pub fn wkupevntenoncins(&mut self) -> WkupevntenoncinsW<'_, Hostctrl1Spec> {
        WkupevntenoncinsW::new(self, 25)
    }
    #[doc = "Bit 26 - Wakeup Event Enable on SD Card Removal"]
    #[inline(always)]
    pub fn wkupevntenoncrm(&mut self) -> WkupevntenoncrmW<'_, Hostctrl1Spec> {
        WkupevntenoncrmW::new(self, 26)
    }
}
#[doc = "Host Control1, Power, Block Gap and Wakeup-up Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hostctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hostctrl1Spec;
impl crate::RegisterSpec for Hostctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hostctrl1::R`](R) reader structure"]
impl crate::Readable for Hostctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`hostctrl1::W`](W) writer structure"]
impl crate::Writable for Hostctrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HOSTCTRL1 to value 0x0080_0000"]
impl crate::Resettable for Hostctrl1Spec {
    const RESET_VALUE: u32 = 0x0080_0000;
}
