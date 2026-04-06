#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Configure Scan Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Scanmode {
    #[doc = "0: A new scan is started each time the period counter overflows"]
    Periodic = 0,
    #[doc = "1: A single scan is performed when START in CMD is set"]
    Oneshot = 1,
    #[doc = "2: Pulse on PRS channel"]
    Prs = 2,
}
impl From<Scanmode> for u8 {
    #[inline(always)]
    fn from(variant: Scanmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Scanmode {
    type Ux = u8;
}
impl crate::IsEnum for Scanmode {}
#[doc = "Field `SCANMODE` reader - Configure Scan Mode"]
pub type ScanmodeR = crate::FieldReader<Scanmode>;
impl ScanmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Scanmode> {
        match self.bits {
            0 => Some(Scanmode::Periodic),
            1 => Some(Scanmode::Oneshot),
            2 => Some(Scanmode::Prs),
            _ => None,
        }
    }
    #[doc = "A new scan is started each time the period counter overflows"]
    #[inline(always)]
    pub fn is_periodic(&self) -> bool {
        *self == Scanmode::Periodic
    }
    #[doc = "A single scan is performed when START in CMD is set"]
    #[inline(always)]
    pub fn is_oneshot(&self) -> bool {
        *self == Scanmode::Oneshot
    }
    #[doc = "Pulse on PRS channel"]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == Scanmode::Prs
    }
}
#[doc = "Field `SCANMODE` writer - Configure Scan Mode"]
pub type ScanmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Scanmode>;
impl<'a, REG> ScanmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A new scan is started each time the period counter overflows"]
    #[inline(always)]
    pub fn periodic(self) -> &'a mut crate::W<REG> {
        self.variant(Scanmode::Periodic)
    }
    #[doc = "A single scan is performed when START in CMD is set"]
    #[inline(always)]
    pub fn oneshot(self) -> &'a mut crate::W<REG> {
        self.variant(Scanmode::Oneshot)
    }
    #[doc = "Pulse on PRS channel"]
    #[inline(always)]
    pub fn prs(self) -> &'a mut crate::W<REG> {
        self.variant(Scanmode::Prs)
    }
}
#[doc = "Scan Start PRS Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prssel {
    #[doc = "0: PRS Channel 0 selected as input"]
    Prsch0 = 0,
    #[doc = "1: PRS Channel 1 selected as input"]
    Prsch1 = 1,
    #[doc = "2: PRS Channel 2 selected as input"]
    Prsch2 = 2,
    #[doc = "3: PRS Channel 3 selected as input"]
    Prsch3 = 3,
    #[doc = "4: PRS Channel 4 selected as input"]
    Prsch4 = 4,
    #[doc = "5: PRS Channel 5 selected as input"]
    Prsch5 = 5,
    #[doc = "6: PRS Channel 6 selected as input"]
    Prsch6 = 6,
    #[doc = "7: PRS Channel 7 selected as input"]
    Prsch7 = 7,
    #[doc = "8: PRS Channel 8 selected as input"]
    Prsch8 = 8,
    #[doc = "9: PRS Channel 9 selected as input"]
    Prsch9 = 9,
    #[doc = "10: PRS Channel 10 selected as input"]
    Prsch10 = 10,
    #[doc = "11: PRS Channel 11 selected as input"]
    Prsch11 = 11,
    #[doc = "12: PRS Channel 12 selected as input"]
    Prsch12 = 12,
    #[doc = "13: PRS Channel 13 selected as input"]
    Prsch13 = 13,
    #[doc = "14: PRS Channel 14 selected as input"]
    Prsch14 = 14,
    #[doc = "15: PRS Channel 15 selected as input"]
    Prsch15 = 15,
    #[doc = "16: PRS Channel 16 selected as input"]
    Prsch16 = 16,
    #[doc = "17: PRS Channel 17 selected as input"]
    Prsch17 = 17,
    #[doc = "18: PRS Channel 18 selected as input"]
    Prsch18 = 18,
    #[doc = "19: PRS Channel 19 selected as input"]
    Prsch19 = 19,
    #[doc = "20: PRS Channel 20 selected as input"]
    Prsch20 = 20,
    #[doc = "21: PRS Channel 21 selected as input"]
    Prsch21 = 21,
    #[doc = "22: PRS Channel 22 selected as input"]
    Prsch22 = 22,
    #[doc = "23: PRS Channel 23 selected as input"]
    Prsch23 = 23,
}
impl From<Prssel> for u8 {
    #[inline(always)]
    fn from(variant: Prssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prssel {
    type Ux = u8;
}
impl crate::IsEnum for Prssel {}
#[doc = "Field `PRSSEL` reader - Scan Start PRS Select"]
pub type PrsselR = crate::FieldReader<Prssel>;
impl PrsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prssel> {
        match self.bits {
            0 => Some(Prssel::Prsch0),
            1 => Some(Prssel::Prsch1),
            2 => Some(Prssel::Prsch2),
            3 => Some(Prssel::Prsch3),
            4 => Some(Prssel::Prsch4),
            5 => Some(Prssel::Prsch5),
            6 => Some(Prssel::Prsch6),
            7 => Some(Prssel::Prsch7),
            8 => Some(Prssel::Prsch8),
            9 => Some(Prssel::Prsch9),
            10 => Some(Prssel::Prsch10),
            11 => Some(Prssel::Prsch11),
            12 => Some(Prssel::Prsch12),
            13 => Some(Prssel::Prsch13),
            14 => Some(Prssel::Prsch14),
            15 => Some(Prssel::Prsch15),
            16 => Some(Prssel::Prsch16),
            17 => Some(Prssel::Prsch17),
            18 => Some(Prssel::Prsch18),
            19 => Some(Prssel::Prsch19),
            20 => Some(Prssel::Prsch20),
            21 => Some(Prssel::Prsch21),
            22 => Some(Prssel::Prsch22),
            23 => Some(Prssel::Prsch23),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == Prssel::Prsch0
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == Prssel::Prsch1
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == Prssel::Prsch2
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == Prssel::Prsch3
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == Prssel::Prsch4
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == Prssel::Prsch5
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == Prssel::Prsch6
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == Prssel::Prsch7
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == Prssel::Prsch8
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == Prssel::Prsch9
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == Prssel::Prsch10
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == Prssel::Prsch11
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == Prssel::Prsch12
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == Prssel::Prsch13
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == Prssel::Prsch14
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == Prssel::Prsch15
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == Prssel::Prsch16
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == Prssel::Prsch17
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == Prssel::Prsch18
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == Prssel::Prsch19
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == Prssel::Prsch20
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == Prssel::Prsch21
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == Prssel::Prsch22
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == Prssel::Prsch23
    }
}
#[doc = "Field `PRSSEL` writer - Scan Start PRS Select"]
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 5, Prssel>;
impl<'a, REG> PrsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch11)
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch12)
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch13)
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch14)
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch15)
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch16)
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch17)
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch18)
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch19)
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch20)
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch21)
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch22)
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch23)
    }
}
#[doc = "Select Scan Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Scanconf {
    #[doc = "0: The channel configuration register registers used are directly mapped to the channel number."]
    Dirmap = 0,
    #[doc = "1: The channel configuration register registers used are CHX+8_CONF for channels 0-7 and CHX-8_CONF for channels 8-15."]
    Invmap = 1,
    #[doc = "2: The channel configuration register registers used toggles between CHX_CONF and CHX+8_CONF when channel x triggers"]
    Toggle = 2,
    #[doc = "3: The decoder state defines the CONF registers to be used."]
    Decdef = 3,
}
impl From<Scanconf> for u8 {
    #[inline(always)]
    fn from(variant: Scanconf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Scanconf {
    type Ux = u8;
}
impl crate::IsEnum for Scanconf {}
#[doc = "Field `SCANCONF` reader - Select Scan Configuration"]
pub type ScanconfR = crate::FieldReader<Scanconf>;
impl ScanconfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scanconf {
        match self.bits {
            0 => Scanconf::Dirmap,
            1 => Scanconf::Invmap,
            2 => Scanconf::Toggle,
            3 => Scanconf::Decdef,
            _ => unreachable!(),
        }
    }
    #[doc = "The channel configuration register registers used are directly mapped to the channel number."]
    #[inline(always)]
    pub fn is_dirmap(&self) -> bool {
        *self == Scanconf::Dirmap
    }
    #[doc = "The channel configuration register registers used are CHX+8_CONF for channels 0-7 and CHX-8_CONF for channels 8-15."]
    #[inline(always)]
    pub fn is_invmap(&self) -> bool {
        *self == Scanconf::Invmap
    }
    #[doc = "The channel configuration register registers used toggles between CHX_CONF and CHX+8_CONF when channel x triggers"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Scanconf::Toggle
    }
    #[doc = "The decoder state defines the CONF registers to be used."]
    #[inline(always)]
    pub fn is_decdef(&self) -> bool {
        *self == Scanconf::Decdef
    }
}
#[doc = "Field `SCANCONF` writer - Select Scan Configuration"]
pub type ScanconfW<'a, REG> = crate::FieldWriter<'a, REG, 2, Scanconf, crate::Safe>;
impl<'a, REG> ScanconfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The channel configuration register registers used are directly mapped to the channel number."]
    #[inline(always)]
    pub fn dirmap(self) -> &'a mut crate::W<REG> {
        self.variant(Scanconf::Dirmap)
    }
    #[doc = "The channel configuration register registers used are CHX+8_CONF for channels 0-7 and CHX-8_CONF for channels 8-15."]
    #[inline(always)]
    pub fn invmap(self) -> &'a mut crate::W<REG> {
        self.variant(Scanconf::Invmap)
    }
    #[doc = "The channel configuration register registers used toggles between CHX_CONF and CHX+8_CONF when channel x triggers"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Scanconf::Toggle)
    }
    #[doc = "The decoder state defines the CONF registers to be used."]
    #[inline(always)]
    pub fn decdef(self) -> &'a mut crate::W<REG> {
        self.variant(Scanconf::Decdef)
    }
}
#[doc = "Field `ALTEXMAP` reader - Alternative Excitation Map"]
pub type AltexmapR = crate::BitReader;
#[doc = "Field `ALTEXMAP` writer - Alternative Excitation Map"]
pub type AltexmapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUALSAMPLE` reader - Enable Dual Sample Mode"]
pub type DualsampleR = crate::BitReader;
#[doc = "Field `DUALSAMPLE` writer - Enable Dual Sample Mode"]
pub type DualsampleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFOW` reader - Result Buffer Overwrite"]
pub type BufowR = crate::BitReader;
#[doc = "Field `BUFOW` writer - Result Buffer Overwrite"]
pub type BufowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRSCANRES` reader - Enable Storing of SCANRES"]
pub type StrscanresR = crate::BitReader;
#[doc = "Field `STRSCANRES` writer - Enable Storing of SCANRES"]
pub type StrscanresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFIDL` reader - Result Buffer Interrupt and DMA Trigger Level"]
pub type BufidlR = crate::BitReader;
#[doc = "Field `BUFIDL` writer - Result Buffer Interrupt and DMA Trigger Level"]
pub type BufidlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "DMA Wake-up From EM2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dmawu {
    #[doc = "0: No DMA wake-up from EM2"]
    Disable = 0,
    #[doc = "1: DMA wake-up from EM2 when data is valid in the result buffer"]
    Bufdatav = 1,
    #[doc = "2: DMA wake-up from EM2 when the result buffer is full/half-full depending on BUFIDL configuration"]
    Buflevel = 2,
}
impl From<Dmawu> for u8 {
    #[inline(always)]
    fn from(variant: Dmawu) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dmawu {
    type Ux = u8;
}
impl crate::IsEnum for Dmawu {}
#[doc = "Field `DMAWU` reader - DMA Wake-up From EM2"]
pub type DmawuR = crate::FieldReader<Dmawu>;
impl DmawuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dmawu> {
        match self.bits {
            0 => Some(Dmawu::Disable),
            1 => Some(Dmawu::Bufdatav),
            2 => Some(Dmawu::Buflevel),
            _ => None,
        }
    }
    #[doc = "No DMA wake-up from EM2"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dmawu::Disable
    }
    #[doc = "DMA wake-up from EM2 when data is valid in the result buffer"]
    #[inline(always)]
    pub fn is_bufdatav(&self) -> bool {
        *self == Dmawu::Bufdatav
    }
    #[doc = "DMA wake-up from EM2 when the result buffer is full/half-full depending on BUFIDL configuration"]
    #[inline(always)]
    pub fn is_buflevel(&self) -> bool {
        *self == Dmawu::Buflevel
    }
}
#[doc = "Field `DMAWU` writer - DMA Wake-up From EM2"]
pub type DmawuW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dmawu>;
impl<'a, REG> DmawuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No DMA wake-up from EM2"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dmawu::Disable)
    }
    #[doc = "DMA wake-up from EM2 when data is valid in the result buffer"]
    #[inline(always)]
    pub fn bufdatav(self) -> &'a mut crate::W<REG> {
        self.variant(Dmawu::Bufdatav)
    }
    #[doc = "DMA wake-up from EM2 when the result buffer is full/half-full depending on BUFIDL configuration"]
    #[inline(always)]
    pub fn buflevel(self) -> &'a mut crate::W<REG> {
        self.variant(Dmawu::Buflevel)
    }
}
#[doc = "Field `DEBUGRUN` reader - Debug Mode Run Enable"]
pub type DebugrunR = crate::BitReader;
#[doc = "Field `DEBUGRUN` writer - Debug Mode Run Enable"]
pub type DebugrunW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Configure Scan Mode"]
    #[inline(always)]
    pub fn scanmode(&self) -> ScanmodeR {
        ScanmodeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:6 - Scan Start PRS Select"]
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 7:8 - Select Scan Configuration"]
    #[inline(always)]
    pub fn scanconf(&self) -> ScanconfR {
        ScanconfR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 11 - Alternative Excitation Map"]
    #[inline(always)]
    pub fn altexmap(&self) -> AltexmapR {
        AltexmapR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Dual Sample Mode"]
    #[inline(always)]
    pub fn dualsample(&self) -> DualsampleR {
        DualsampleR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Result Buffer Overwrite"]
    #[inline(always)]
    pub fn bufow(&self) -> BufowR {
        BufowR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Storing of SCANRES"]
    #[inline(always)]
    pub fn strscanres(&self) -> StrscanresR {
        StrscanresR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Result Buffer Interrupt and DMA Trigger Level"]
    #[inline(always)]
    pub fn bufidl(&self) -> BufidlR {
        BufidlR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - DMA Wake-up From EM2"]
    #[inline(always)]
    pub fn dmawu(&self) -> DmawuR {
        DmawuR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&self) -> DebugrunR {
        DebugrunR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Configure Scan Mode"]
    #[inline(always)]
    pub fn scanmode(&mut self) -> ScanmodeW<'_, CtrlSpec> {
        ScanmodeW::new(self, 0)
    }
    #[doc = "Bits 2:6 - Scan Start PRS Select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PrsselW<'_, CtrlSpec> {
        PrsselW::new(self, 2)
    }
    #[doc = "Bits 7:8 - Select Scan Configuration"]
    #[inline(always)]
    pub fn scanconf(&mut self) -> ScanconfW<'_, CtrlSpec> {
        ScanconfW::new(self, 7)
    }
    #[doc = "Bit 11 - Alternative Excitation Map"]
    #[inline(always)]
    pub fn altexmap(&mut self) -> AltexmapW<'_, CtrlSpec> {
        AltexmapW::new(self, 11)
    }
    #[doc = "Bit 13 - Enable Dual Sample Mode"]
    #[inline(always)]
    pub fn dualsample(&mut self) -> DualsampleW<'_, CtrlSpec> {
        DualsampleW::new(self, 13)
    }
    #[doc = "Bit 16 - Result Buffer Overwrite"]
    #[inline(always)]
    pub fn bufow(&mut self) -> BufowW<'_, CtrlSpec> {
        BufowW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Storing of SCANRES"]
    #[inline(always)]
    pub fn strscanres(&mut self) -> StrscanresW<'_, CtrlSpec> {
        StrscanresW::new(self, 17)
    }
    #[doc = "Bit 19 - Result Buffer Interrupt and DMA Trigger Level"]
    #[inline(always)]
    pub fn bufidl(&mut self) -> BufidlW<'_, CtrlSpec> {
        BufidlW::new(self, 19)
    }
    #[doc = "Bits 20:21 - DMA Wake-up From EM2"]
    #[inline(always)]
    pub fn dmawu(&mut self) -> DmawuW<'_, CtrlSpec> {
        DmawuW::new(self, 20)
    }
    #[doc = "Bit 22 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&mut self) -> DebugrunW<'_, CtrlSpec> {
        DebugrunW::new(self, 22)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
