#[doc = "Register `HOSTCTRL1` reader"]
pub type R = crate::R<HOSTCTRL1_SPEC>;
#[doc = "Register `HOSTCTRL1` writer"]
pub type W = crate::W<HOSTCTRL1_SPEC>;
#[doc = "Field `LEDCTRL` reader - LED Control"]
pub type LEDCTRL_R = crate::BitReader;
#[doc = "Field `LEDCTRL` writer - LED Control"]
pub type LEDCTRL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATTRANWD` reader - Data Transfer Width 1-bit or 4-bit Mode"]
pub type DATTRANWD_R = crate::BitReader;
#[doc = "Field `DATTRANWD` writer - Data Transfer Width 1-bit or 4-bit Mode"]
pub type DATTRANWD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSEN` reader - High Speed Enable"]
pub type HSEN_R = crate::BitReader;
#[doc = "Field `HSEN` writer - High Speed Enable"]
pub type HSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMASEL` reader - DMA Select"]
pub type DMASEL_R = crate::FieldReader<DMASEL_A>;
#[doc = "DMA Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMASEL_A {
    #[doc = "0: SDMA selected"]
    SDMA = 0,
    #[doc = "1: 32-bit ADMA1 selected"]
    ADMA1 = 1,
    #[doc = "2: 32-bit ADMA2 selected"]
    ADMA2 = 2,
    #[doc = "3: 64-bit ADMA2 selected"]
    _64BITADMA2 = 3,
}
impl From<DMASEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DMASEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DMASEL_A {
    type Ux = u8;
}
impl DMASEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMASEL_A {
        match self.bits {
            0 => DMASEL_A::SDMA,
            1 => DMASEL_A::ADMA1,
            2 => DMASEL_A::ADMA2,
            3 => DMASEL_A::_64BITADMA2,
            _ => unreachable!(),
        }
    }
    #[doc = "SDMA selected"]
    #[inline(always)]
    pub fn is_sdma(&self) -> bool {
        *self == DMASEL_A::SDMA
    }
    #[doc = "32-bit ADMA1 selected"]
    #[inline(always)]
    pub fn is_adma1(&self) -> bool {
        *self == DMASEL_A::ADMA1
    }
    #[doc = "32-bit ADMA2 selected"]
    #[inline(always)]
    pub fn is_adma2(&self) -> bool {
        *self == DMASEL_A::ADMA2
    }
    #[doc = "64-bit ADMA2 selected"]
    #[inline(always)]
    pub fn is_64bitadma2(&self) -> bool {
        *self == DMASEL_A::_64BITADMA2
    }
}
#[doc = "Field `DMASEL` writer - DMA Select"]
pub type DMASEL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, DMASEL_A>;
impl<'a, REG, const O: u8> DMASEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SDMA selected"]
    #[inline(always)]
    pub fn sdma(self) -> &'a mut crate::W<REG> {
        self.variant(DMASEL_A::SDMA)
    }
    #[doc = "32-bit ADMA1 selected"]
    #[inline(always)]
    pub fn adma1(self) -> &'a mut crate::W<REG> {
        self.variant(DMASEL_A::ADMA1)
    }
    #[doc = "32-bit ADMA2 selected"]
    #[inline(always)]
    pub fn adma2(self) -> &'a mut crate::W<REG> {
        self.variant(DMASEL_A::ADMA2)
    }
    #[doc = "64-bit ADMA2 selected"]
    #[inline(always)]
    pub fn _64bitadma2(self) -> &'a mut crate::W<REG> {
        self.variant(DMASEL_A::_64BITADMA2)
    }
}
#[doc = "Field `EXTDATTRANWD` reader - Extended Data Transfer Width"]
pub type EXTDATTRANWD_R = crate::BitReader;
#[doc = "Field `EXTDATTRANWD` writer - Extended Data Transfer Width"]
pub type EXTDATTRANWD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CDTSTLVL` reader - Card Detect Test Level"]
pub type CDTSTLVL_R = crate::BitReader;
#[doc = "Field `CDTSTLVL` writer - Card Detect Test Level"]
pub type CDTSTLVL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CDSIGDET` reader - Card Detetct Signal Detection"]
pub type CDSIGDET_R = crate::BitReader;
#[doc = "Field `CDSIGDET` writer - Card Detetct Signal Detection"]
pub type CDSIGDET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SDBUSPOWER` reader - SD Bus Power"]
pub type SDBUSPOWER_R = crate::BitReader;
#[doc = "Field `SDBUSPOWER` writer - SD Bus Power"]
pub type SDBUSPOWER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SDBUSVOLTSEL` reader - SD Bus Voltage Select"]
pub type SDBUSVOLTSEL_R = crate::FieldReader<SDBUSVOLTSEL_A>;
#[doc = "SD Bus Voltage Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SDBUSVOLTSEL_A {
    #[doc = "5: Select 1.8V"]
    _1P8V = 5,
    #[doc = "6: Select 3.0V"]
    _3P0V = 6,
    #[doc = "7: Select 3.3V"]
    _3P3V = 7,
}
impl From<SDBUSVOLTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SDBUSVOLTSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SDBUSVOLTSEL_A {
    type Ux = u8;
}
impl SDBUSVOLTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SDBUSVOLTSEL_A> {
        match self.bits {
            5 => Some(SDBUSVOLTSEL_A::_1P8V),
            6 => Some(SDBUSVOLTSEL_A::_3P0V),
            7 => Some(SDBUSVOLTSEL_A::_3P3V),
            _ => None,
        }
    }
    #[doc = "Select 1.8V"]
    #[inline(always)]
    pub fn is_1p8v(&self) -> bool {
        *self == SDBUSVOLTSEL_A::_1P8V
    }
    #[doc = "Select 3.0V"]
    #[inline(always)]
    pub fn is_3p0v(&self) -> bool {
        *self == SDBUSVOLTSEL_A::_3P0V
    }
    #[doc = "Select 3.3V"]
    #[inline(always)]
    pub fn is_3p3v(&self) -> bool {
        *self == SDBUSVOLTSEL_A::_3P3V
    }
}
#[doc = "Field `SDBUSVOLTSEL` writer - SD Bus Voltage Select"]
pub type SDBUSVOLTSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, SDBUSVOLTSEL_A>;
impl<'a, REG, const O: u8> SDBUSVOLTSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select 1.8V"]
    #[inline(always)]
    pub fn _1p8v(self) -> &'a mut crate::W<REG> {
        self.variant(SDBUSVOLTSEL_A::_1P8V)
    }
    #[doc = "Select 3.0V"]
    #[inline(always)]
    pub fn _3p0v(self) -> &'a mut crate::W<REG> {
        self.variant(SDBUSVOLTSEL_A::_3P0V)
    }
    #[doc = "Select 3.3V"]
    #[inline(always)]
    pub fn _3p3v(self) -> &'a mut crate::W<REG> {
        self.variant(SDBUSVOLTSEL_A::_3P3V)
    }
}
#[doc = "Field `HRDRST` reader - Hardware Reset Signal"]
pub type HRDRST_R = crate::BitReader;
#[doc = "Field `HRDRST` writer - Hardware Reset Signal"]
pub type HRDRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STOPATBLKGAPREQ` reader - Stop at Block Gap Request"]
pub type STOPATBLKGAPREQ_R = crate::BitReader;
#[doc = "Field `STOPATBLKGAPREQ` writer - Stop at Block Gap Request"]
pub type STOPATBLKGAPREQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CONTINUEREQ` reader - Continue Request"]
pub type CONTINUEREQ_R = crate::BitReader;
#[doc = "Field `CONTINUEREQ` writer - Continue Request"]
pub type CONTINUEREQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RDWAITCTRL` reader - Read Wait Control"]
pub type RDWAITCTRL_R = crate::BitReader;
#[doc = "Field `RDWAITCTRL` writer - Read Wait Control"]
pub type RDWAITCTRL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTATBLKGAP` reader - Interrupt at Block Gap"]
pub type INTATBLKGAP_R = crate::BitReader;
#[doc = "Field `INTATBLKGAP` writer - Interrupt at Block Gap"]
pub type INTATBLKGAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPIMODE` reader - SPI Mode Enable"]
pub type SPIMODE_R = crate::BitReader;
#[doc = "Field `SPIMODE` writer - SPI Mode Enable"]
pub type SPIMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BOOTEN` reader - Boot Enable"]
pub type BOOTEN_R = crate::BitReader;
#[doc = "Field `BOOTEN` writer - Boot Enable"]
pub type BOOTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ALTBOOTEN` reader - Alternate Boot Enable"]
pub type ALTBOOTEN_R = crate::BitReader;
#[doc = "Field `ALTBOOTEN` writer - Alternate Boot Enable"]
pub type ALTBOOTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BOOTACKCHK` reader - Boot Ack Check"]
pub type BOOTACKCHK_R = crate::BitReader;
#[doc = "Field `BOOTACKCHK` writer - Boot Ack Check"]
pub type BOOTACKCHK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WKUPEVNTENONCARDINT` reader - Wakeup Event Enable on Card Interrupt"]
pub type WKUPEVNTENONCARDINT_R = crate::BitReader;
#[doc = "Field `WKUPEVNTENONCARDINT` writer - Wakeup Event Enable on Card Interrupt"]
pub type WKUPEVNTENONCARDINT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WKUPEVNTENONCINS` reader - Wakeup Event Enable on SD Card Insertion"]
pub type WKUPEVNTENONCINS_R = crate::BitReader;
#[doc = "Field `WKUPEVNTENONCINS` writer - Wakeup Event Enable on SD Card Insertion"]
pub type WKUPEVNTENONCINS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WKUPEVNTENONCRM` reader - Wakeup Event Enable on SD Card Removal"]
pub type WKUPEVNTENONCRM_R = crate::BitReader;
#[doc = "Field `WKUPEVNTENONCRM` writer - Wakeup Event Enable on SD Card Removal"]
pub type WKUPEVNTENONCRM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - LED Control"]
    #[inline(always)]
    pub fn ledctrl(&self) -> LEDCTRL_R {
        LEDCTRL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data Transfer Width 1-bit or 4-bit Mode"]
    #[inline(always)]
    pub fn dattranwd(&self) -> DATTRANWD_R {
        DATTRANWD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - High Speed Enable"]
    #[inline(always)]
    pub fn hsen(&self) -> HSEN_R {
        HSEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - DMA Select"]
    #[inline(always)]
    pub fn dmasel(&self) -> DMASEL_R {
        DMASEL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Extended Data Transfer Width"]
    #[inline(always)]
    pub fn extdattranwd(&self) -> EXTDATTRANWD_R {
        EXTDATTRANWD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Card Detect Test Level"]
    #[inline(always)]
    pub fn cdtstlvl(&self) -> CDTSTLVL_R {
        CDTSTLVL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card Detetct Signal Detection"]
    #[inline(always)]
    pub fn cdsigdet(&self) -> CDSIGDET_R {
        CDSIGDET_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SD Bus Power"]
    #[inline(always)]
    pub fn sdbuspower(&self) -> SDBUSPOWER_R {
        SDBUSPOWER_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - SD Bus Voltage Select"]
    #[inline(always)]
    pub fn sdbusvoltsel(&self) -> SDBUSVOLTSEL_R {
        SDBUSVOLTSEL_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - Hardware Reset Signal"]
    #[inline(always)]
    pub fn hrdrst(&self) -> HRDRST_R {
        HRDRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Stop at Block Gap Request"]
    #[inline(always)]
    pub fn stopatblkgapreq(&self) -> STOPATBLKGAPREQ_R {
        STOPATBLKGAPREQ_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Continue Request"]
    #[inline(always)]
    pub fn continuereq(&self) -> CONTINUEREQ_R {
        CONTINUEREQ_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Read Wait Control"]
    #[inline(always)]
    pub fn rdwaitctrl(&self) -> RDWAITCTRL_R {
        RDWAITCTRL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt at Block Gap"]
    #[inline(always)]
    pub fn intatblkgap(&self) -> INTATBLKGAP_R {
        INTATBLKGAP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SPI Mode Enable"]
    #[inline(always)]
    pub fn spimode(&self) -> SPIMODE_R {
        SPIMODE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Boot Enable"]
    #[inline(always)]
    pub fn booten(&self) -> BOOTEN_R {
        BOOTEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Alternate Boot Enable"]
    #[inline(always)]
    pub fn altbooten(&self) -> ALTBOOTEN_R {
        ALTBOOTEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Boot Ack Check"]
    #[inline(always)]
    pub fn bootackchk(&self) -> BOOTACKCHK_R {
        BOOTACKCHK_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Wakeup Event Enable on Card Interrupt"]
    #[inline(always)]
    pub fn wkupevntenoncardint(&self) -> WKUPEVNTENONCARDINT_R {
        WKUPEVNTENONCARDINT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Wakeup Event Enable on SD Card Insertion"]
    #[inline(always)]
    pub fn wkupevntenoncins(&self) -> WKUPEVNTENONCINS_R {
        WKUPEVNTENONCINS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Wakeup Event Enable on SD Card Removal"]
    #[inline(always)]
    pub fn wkupevntenoncrm(&self) -> WKUPEVNTENONCRM_R {
        WKUPEVNTENONCRM_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LED Control"]
    #[inline(always)]
    #[must_use]
    pub fn ledctrl(&mut self) -> LEDCTRL_W<HOSTCTRL1_SPEC, 0> {
        LEDCTRL_W::new(self)
    }
    #[doc = "Bit 1 - Data Transfer Width 1-bit or 4-bit Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dattranwd(&mut self) -> DATTRANWD_W<HOSTCTRL1_SPEC, 1> {
        DATTRANWD_W::new(self)
    }
    #[doc = "Bit 2 - High Speed Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsen(&mut self) -> HSEN_W<HOSTCTRL1_SPEC, 2> {
        HSEN_W::new(self)
    }
    #[doc = "Bits 3:4 - DMA Select"]
    #[inline(always)]
    #[must_use]
    pub fn dmasel(&mut self) -> DMASEL_W<HOSTCTRL1_SPEC, 3> {
        DMASEL_W::new(self)
    }
    #[doc = "Bit 5 - Extended Data Transfer Width"]
    #[inline(always)]
    #[must_use]
    pub fn extdattranwd(&mut self) -> EXTDATTRANWD_W<HOSTCTRL1_SPEC, 5> {
        EXTDATTRANWD_W::new(self)
    }
    #[doc = "Bit 6 - Card Detect Test Level"]
    #[inline(always)]
    #[must_use]
    pub fn cdtstlvl(&mut self) -> CDTSTLVL_W<HOSTCTRL1_SPEC, 6> {
        CDTSTLVL_W::new(self)
    }
    #[doc = "Bit 7 - Card Detetct Signal Detection"]
    #[inline(always)]
    #[must_use]
    pub fn cdsigdet(&mut self) -> CDSIGDET_W<HOSTCTRL1_SPEC, 7> {
        CDSIGDET_W::new(self)
    }
    #[doc = "Bit 8 - SD Bus Power"]
    #[inline(always)]
    #[must_use]
    pub fn sdbuspower(&mut self) -> SDBUSPOWER_W<HOSTCTRL1_SPEC, 8> {
        SDBUSPOWER_W::new(self)
    }
    #[doc = "Bits 9:11 - SD Bus Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn sdbusvoltsel(&mut self) -> SDBUSVOLTSEL_W<HOSTCTRL1_SPEC, 9> {
        SDBUSVOLTSEL_W::new(self)
    }
    #[doc = "Bit 12 - Hardware Reset Signal"]
    #[inline(always)]
    #[must_use]
    pub fn hrdrst(&mut self) -> HRDRST_W<HOSTCTRL1_SPEC, 12> {
        HRDRST_W::new(self)
    }
    #[doc = "Bit 16 - Stop at Block Gap Request"]
    #[inline(always)]
    #[must_use]
    pub fn stopatblkgapreq(&mut self) -> STOPATBLKGAPREQ_W<HOSTCTRL1_SPEC, 16> {
        STOPATBLKGAPREQ_W::new(self)
    }
    #[doc = "Bit 17 - Continue Request"]
    #[inline(always)]
    #[must_use]
    pub fn continuereq(&mut self) -> CONTINUEREQ_W<HOSTCTRL1_SPEC, 17> {
        CONTINUEREQ_W::new(self)
    }
    #[doc = "Bit 18 - Read Wait Control"]
    #[inline(always)]
    #[must_use]
    pub fn rdwaitctrl(&mut self) -> RDWAITCTRL_W<HOSTCTRL1_SPEC, 18> {
        RDWAITCTRL_W::new(self)
    }
    #[doc = "Bit 19 - Interrupt at Block Gap"]
    #[inline(always)]
    #[must_use]
    pub fn intatblkgap(&mut self) -> INTATBLKGAP_W<HOSTCTRL1_SPEC, 19> {
        INTATBLKGAP_W::new(self)
    }
    #[doc = "Bit 20 - SPI Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spimode(&mut self) -> SPIMODE_W<HOSTCTRL1_SPEC, 20> {
        SPIMODE_W::new(self)
    }
    #[doc = "Bit 21 - Boot Enable"]
    #[inline(always)]
    #[must_use]
    pub fn booten(&mut self) -> BOOTEN_W<HOSTCTRL1_SPEC, 21> {
        BOOTEN_W::new(self)
    }
    #[doc = "Bit 22 - Alternate Boot Enable"]
    #[inline(always)]
    #[must_use]
    pub fn altbooten(&mut self) -> ALTBOOTEN_W<HOSTCTRL1_SPEC, 22> {
        ALTBOOTEN_W::new(self)
    }
    #[doc = "Bit 23 - Boot Ack Check"]
    #[inline(always)]
    #[must_use]
    pub fn bootackchk(&mut self) -> BOOTACKCHK_W<HOSTCTRL1_SPEC, 23> {
        BOOTACKCHK_W::new(self)
    }
    #[doc = "Bit 24 - Wakeup Event Enable on Card Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wkupevntenoncardint(&mut self) -> WKUPEVNTENONCARDINT_W<HOSTCTRL1_SPEC, 24> {
        WKUPEVNTENONCARDINT_W::new(self)
    }
    #[doc = "Bit 25 - Wakeup Event Enable on SD Card Insertion"]
    #[inline(always)]
    #[must_use]
    pub fn wkupevntenoncins(&mut self) -> WKUPEVNTENONCINS_W<HOSTCTRL1_SPEC, 25> {
        WKUPEVNTENONCINS_W::new(self)
    }
    #[doc = "Bit 26 - Wakeup Event Enable on SD Card Removal"]
    #[inline(always)]
    #[must_use]
    pub fn wkupevntenoncrm(&mut self) -> WKUPEVNTENONCRM_W<HOSTCTRL1_SPEC, 26> {
        WKUPEVNTENONCRM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Host Control1, Power, Block Gap and Wakeup-up Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hostctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hostctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOSTCTRL1_SPEC;
impl crate::RegisterSpec for HOSTCTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hostctrl1::R`](R) reader structure"]
impl crate::Readable for HOSTCTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hostctrl1::W`](W) writer structure"]
impl crate::Writable for HOSTCTRL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOSTCTRL1 to value 0x0080_0000"]
impl crate::Resettable for HOSTCTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0080_0000;
}
