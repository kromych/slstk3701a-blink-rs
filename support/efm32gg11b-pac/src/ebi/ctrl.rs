#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled."]
    D8a8 = 0,
    #[doc = "1: EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    D16a16ale = 1,
    #[doc = "2: EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    D8a24ale = 2,
    #[doc = "3: EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled."]
    D16 = 3,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - Mode"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            0 => Mode::D8a8,
            1 => Mode::D16a16ale,
            2 => Mode::D8a24ale,
            3 => Mode::D16,
            _ => unreachable!(),
        }
    }
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn is_d8a8(&self) -> bool {
        *self == Mode::D8a8
    }
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn is_d16a16ale(&self) -> bool {
        *self == Mode::D16a16ale
    }
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn is_d8a24ale(&self) -> bool {
        *self == Mode::D8a24ale
    }
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn is_d16(&self) -> bool {
        *self == Mode::D16
    }
}
#[doc = "Field `MODE` writer - Mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode, crate::Safe>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d8a8(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::D8a8)
    }
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d16a16ale(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::D16a16ale)
    }
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d8a24ale(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::D8a24ale)
    }
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d16(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::D16)
    }
}
#[doc = "Mode 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode1 {
    #[doc = "0: EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled."]
    D8a8 = 0,
    #[doc = "1: EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    D16a16ale = 1,
    #[doc = "2: EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    D8a24ale = 2,
    #[doc = "3: EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled."]
    D16 = 3,
}
impl From<Mode1> for u8 {
    #[inline(always)]
    fn from(variant: Mode1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode1 {
    type Ux = u8;
}
impl crate::IsEnum for Mode1 {}
#[doc = "Field `MODE1` reader - Mode 1"]
pub type Mode1R = crate::FieldReader<Mode1>;
impl Mode1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode1 {
        match self.bits {
            0 => Mode1::D8a8,
            1 => Mode1::D16a16ale,
            2 => Mode1::D8a24ale,
            3 => Mode1::D16,
            _ => unreachable!(),
        }
    }
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn is_d8a8(&self) -> bool {
        *self == Mode1::D8a8
    }
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn is_d16a16ale(&self) -> bool {
        *self == Mode1::D16a16ale
    }
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn is_d8a24ale(&self) -> bool {
        *self == Mode1::D8a24ale
    }
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn is_d16(&self) -> bool {
        *self == Mode1::D16
    }
}
#[doc = "Field `MODE1` writer - Mode 1"]
pub type Mode1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode1, crate::Safe>;
impl<'a, REG> Mode1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d8a8(self) -> &'a mut crate::W<REG> {
        self.variant(Mode1::D8a8)
    }
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d16a16ale(self) -> &'a mut crate::W<REG> {
        self.variant(Mode1::D16a16ale)
    }
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d8a24ale(self) -> &'a mut crate::W<REG> {
        self.variant(Mode1::D8a24ale)
    }
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d16(self) -> &'a mut crate::W<REG> {
        self.variant(Mode1::D16)
    }
}
#[doc = "Mode 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode2 {
    #[doc = "0: EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled."]
    D8a8 = 0,
    #[doc = "1: EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    D16a16ale = 1,
    #[doc = "2: EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    D8a24ale = 2,
    #[doc = "3: EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled."]
    D16 = 3,
}
impl From<Mode2> for u8 {
    #[inline(always)]
    fn from(variant: Mode2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode2 {
    type Ux = u8;
}
impl crate::IsEnum for Mode2 {}
#[doc = "Field `MODE2` reader - Mode 2"]
pub type Mode2R = crate::FieldReader<Mode2>;
impl Mode2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode2 {
        match self.bits {
            0 => Mode2::D8a8,
            1 => Mode2::D16a16ale,
            2 => Mode2::D8a24ale,
            3 => Mode2::D16,
            _ => unreachable!(),
        }
    }
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn is_d8a8(&self) -> bool {
        *self == Mode2::D8a8
    }
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn is_d16a16ale(&self) -> bool {
        *self == Mode2::D16a16ale
    }
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn is_d8a24ale(&self) -> bool {
        *self == Mode2::D8a24ale
    }
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn is_d16(&self) -> bool {
        *self == Mode2::D16
    }
}
#[doc = "Field `MODE2` writer - Mode 2"]
pub type Mode2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode2, crate::Safe>;
impl<'a, REG> Mode2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d8a8(self) -> &'a mut crate::W<REG> {
        self.variant(Mode2::D8a8)
    }
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d16a16ale(self) -> &'a mut crate::W<REG> {
        self.variant(Mode2::D16a16ale)
    }
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d8a24ale(self) -> &'a mut crate::W<REG> {
        self.variant(Mode2::D8a24ale)
    }
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d16(self) -> &'a mut crate::W<REG> {
        self.variant(Mode2::D16)
    }
}
#[doc = "Mode 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode3 {
    #[doc = "0: EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled."]
    D8a8 = 0,
    #[doc = "1: EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    D16a16ale = 1,
    #[doc = "2: EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    D8a24ale = 2,
    #[doc = "3: EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled."]
    D16 = 3,
}
impl From<Mode3> for u8 {
    #[inline(always)]
    fn from(variant: Mode3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode3 {
    type Ux = u8;
}
impl crate::IsEnum for Mode3 {}
#[doc = "Field `MODE3` reader - Mode 3"]
pub type Mode3R = crate::FieldReader<Mode3>;
impl Mode3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode3 {
        match self.bits {
            0 => Mode3::D8a8,
            1 => Mode3::D16a16ale,
            2 => Mode3::D8a24ale,
            3 => Mode3::D16,
            _ => unreachable!(),
        }
    }
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn is_d8a8(&self) -> bool {
        *self == Mode3::D8a8
    }
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn is_d16a16ale(&self) -> bool {
        *self == Mode3::D16a16ale
    }
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn is_d8a24ale(&self) -> bool {
        *self == Mode3::D8a24ale
    }
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn is_d16(&self) -> bool {
        *self == Mode3::D16
    }
}
#[doc = "Field `MODE3` writer - Mode 3"]
pub type Mode3W<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode3, crate::Safe>;
impl<'a, REG> Mode3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d8a8(self) -> &'a mut crate::W<REG> {
        self.variant(Mode3::D8a8)
    }
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d16a16ale(self) -> &'a mut crate::W<REG> {
        self.variant(Mode3::D16a16ale)
    }
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d8a24ale(self) -> &'a mut crate::W<REG> {
        self.variant(Mode3::D8a24ale)
    }
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d16(self) -> &'a mut crate::W<REG> {
        self.variant(Mode3::D16)
    }
}
#[doc = "Field `BANK0EN` reader - Bank 0 Enable"]
pub type Bank0enR = crate::BitReader;
#[doc = "Field `BANK0EN` writer - Bank 0 Enable"]
pub type Bank0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BANK1EN` reader - Bank 1 Enable"]
pub type Bank1enR = crate::BitReader;
#[doc = "Field `BANK1EN` writer - Bank 1 Enable"]
pub type Bank1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BANK2EN` reader - Bank 2 Enable"]
pub type Bank2enR = crate::BitReader;
#[doc = "Field `BANK2EN` writer - Bank 2 Enable"]
pub type Bank2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BANK3EN` reader - Bank 3 Enable"]
pub type Bank3enR = crate::BitReader;
#[doc = "Field `BANK3EN` writer - Bank 3 Enable"]
pub type Bank3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOIDLE` reader - No Idle Cycle Insertion on Bank 0"]
pub type NoidleR = crate::BitReader;
#[doc = "Field `NOIDLE` writer - No Idle Cycle Insertion on Bank 0"]
pub type NoidleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOIDLE1` reader - No Idle Cycle Insertion on Bank 1"]
pub type Noidle1R = crate::BitReader;
#[doc = "Field `NOIDLE1` writer - No Idle Cycle Insertion on Bank 1"]
pub type Noidle1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOIDLE2` reader - No Idle Cycle Insertion on Bank 2"]
pub type Noidle2R = crate::BitReader;
#[doc = "Field `NOIDLE2` writer - No Idle Cycle Insertion on Bank 2"]
pub type Noidle2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOIDLE3` reader - No Idle Cycle Insertion on Bank 3"]
pub type Noidle3R = crate::BitReader;
#[doc = "Field `NOIDLE3` writer - No Idle Cycle Insertion on Bank 3"]
pub type Noidle3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARDYEN` reader - ARDY Enable"]
pub type ArdyenR = crate::BitReader;
#[doc = "Field `ARDYEN` writer - ARDY Enable"]
pub type ArdyenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARDYTODIS` reader - ARDY Timeout Disable"]
pub type ArdytodisR = crate::BitReader;
#[doc = "Field `ARDYTODIS` writer - ARDY Timeout Disable"]
pub type ArdytodisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARDY1EN` reader - ARDY Enable for Bank 1"]
pub type Ardy1enR = crate::BitReader;
#[doc = "Field `ARDY1EN` writer - ARDY Enable for Bank 1"]
pub type Ardy1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARDYTO1DIS` reader - ARDY Timeout Disable for Bank 1"]
pub type Ardyto1disR = crate::BitReader;
#[doc = "Field `ARDYTO1DIS` writer - ARDY Timeout Disable for Bank 1"]
pub type Ardyto1disW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARDY2EN` reader - ARDY Enable for Bank 2"]
pub type Ardy2enR = crate::BitReader;
#[doc = "Field `ARDY2EN` writer - ARDY Enable for Bank 2"]
pub type Ardy2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARDYTO2DIS` reader - ARDY Timeout Disable for Bank 2"]
pub type Ardyto2disR = crate::BitReader;
#[doc = "Field `ARDYTO2DIS` writer - ARDY Timeout Disable for Bank 2"]
pub type Ardyto2disW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARDY3EN` reader - ARDY Enable for Bank 3"]
pub type Ardy3enR = crate::BitReader;
#[doc = "Field `ARDY3EN` writer - ARDY Enable for Bank 3"]
pub type Ardy3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARDYTO3DIS` reader - ARDY Timeout Disable for Bank 3"]
pub type Ardyto3disR = crate::BitReader;
#[doc = "Field `ARDYTO3DIS` writer - ARDY Timeout Disable for Bank 3"]
pub type Ardyto3disW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BL` reader - Byte Lane Enable for Bank 0"]
pub type BlR = crate::BitReader;
#[doc = "Field `BL` writer - Byte Lane Enable for Bank 0"]
pub type BlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BL1` reader - Byte Lane Enable for Bank 1"]
pub type Bl1R = crate::BitReader;
#[doc = "Field `BL1` writer - Byte Lane Enable for Bank 1"]
pub type Bl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BL2` reader - Byte Lane Enable for Bank 2"]
pub type Bl2R = crate::BitReader;
#[doc = "Field `BL2` writer - Byte Lane Enable for Bank 2"]
pub type Bl2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BL3` reader - Byte Lane Enable for Bank 3"]
pub type Bl3R = crate::BitReader;
#[doc = "Field `BL3` writer - Byte Lane Enable for Bank 3"]
pub type Bl3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITS` reader - Individual Timing Set, Line Polarity and Mode Definition Enable"]
pub type ItsR = crate::BitReader;
#[doc = "Field `ITS` writer - Individual Timing Set, Line Polarity and Mode Definition Enable"]
pub type ItsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALTMAP` reader - Alternative Address Map Enable"]
pub type AltmapR = crate::BitReader;
#[doc = "Field `ALTMAP` writer - Alternative Address Map Enable"]
pub type AltmapW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Mode 1"]
    #[inline(always)]
    pub fn mode1(&self) -> Mode1R {
        Mode1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Mode 2"]
    #[inline(always)]
    pub fn mode2(&self) -> Mode2R {
        Mode2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Mode 3"]
    #[inline(always)]
    pub fn mode3(&self) -> Mode3R {
        Mode3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Bank 0 Enable"]
    #[inline(always)]
    pub fn bank0en(&self) -> Bank0enR {
        Bank0enR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Bank 1 Enable"]
    #[inline(always)]
    pub fn bank1en(&self) -> Bank1enR {
        Bank1enR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Bank 2 Enable"]
    #[inline(always)]
    pub fn bank2en(&self) -> Bank2enR {
        Bank2enR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bank 3 Enable"]
    #[inline(always)]
    pub fn bank3en(&self) -> Bank3enR {
        Bank3enR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - No Idle Cycle Insertion on Bank 0"]
    #[inline(always)]
    pub fn noidle(&self) -> NoidleR {
        NoidleR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - No Idle Cycle Insertion on Bank 1"]
    #[inline(always)]
    pub fn noidle1(&self) -> Noidle1R {
        Noidle1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - No Idle Cycle Insertion on Bank 2"]
    #[inline(always)]
    pub fn noidle2(&self) -> Noidle2R {
        Noidle2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - No Idle Cycle Insertion on Bank 3"]
    #[inline(always)]
    pub fn noidle3(&self) -> Noidle3R {
        Noidle3R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ARDY Enable"]
    #[inline(always)]
    pub fn ardyen(&self) -> ArdyenR {
        ArdyenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ARDY Timeout Disable"]
    #[inline(always)]
    pub fn ardytodis(&self) -> ArdytodisR {
        ArdytodisR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ARDY Enable for Bank 1"]
    #[inline(always)]
    pub fn ardy1en(&self) -> Ardy1enR {
        Ardy1enR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ARDY Timeout Disable for Bank 1"]
    #[inline(always)]
    pub fn ardyto1dis(&self) -> Ardyto1disR {
        Ardyto1disR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ARDY Enable for Bank 2"]
    #[inline(always)]
    pub fn ardy2en(&self) -> Ardy2enR {
        Ardy2enR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ARDY Timeout Disable for Bank 2"]
    #[inline(always)]
    pub fn ardyto2dis(&self) -> Ardyto2disR {
        Ardyto2disR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ARDY Enable for Bank 3"]
    #[inline(always)]
    pub fn ardy3en(&self) -> Ardy3enR {
        Ardy3enR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ARDY Timeout Disable for Bank 3"]
    #[inline(always)]
    pub fn ardyto3dis(&self) -> Ardyto3disR {
        Ardyto3disR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Byte Lane Enable for Bank 0"]
    #[inline(always)]
    pub fn bl(&self) -> BlR {
        BlR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Byte Lane Enable for Bank 1"]
    #[inline(always)]
    pub fn bl1(&self) -> Bl1R {
        Bl1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Byte Lane Enable for Bank 2"]
    #[inline(always)]
    pub fn bl2(&self) -> Bl2R {
        Bl2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Byte Lane Enable for Bank 3"]
    #[inline(always)]
    pub fn bl3(&self) -> Bl3R {
        Bl3R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - Individual Timing Set, Line Polarity and Mode Definition Enable"]
    #[inline(always)]
    pub fn its(&self) -> ItsR {
        ItsR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Alternative Address Map Enable"]
    #[inline(always)]
    pub fn altmap(&self) -> AltmapR {
        AltmapR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, CtrlSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Mode 1"]
    #[inline(always)]
    pub fn mode1(&mut self) -> Mode1W<'_, CtrlSpec> {
        Mode1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Mode 2"]
    #[inline(always)]
    pub fn mode2(&mut self) -> Mode2W<'_, CtrlSpec> {
        Mode2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Mode 3"]
    #[inline(always)]
    pub fn mode3(&mut self) -> Mode3W<'_, CtrlSpec> {
        Mode3W::new(self, 6)
    }
    #[doc = "Bit 8 - Bank 0 Enable"]
    #[inline(always)]
    pub fn bank0en(&mut self) -> Bank0enW<'_, CtrlSpec> {
        Bank0enW::new(self, 8)
    }
    #[doc = "Bit 9 - Bank 1 Enable"]
    #[inline(always)]
    pub fn bank1en(&mut self) -> Bank1enW<'_, CtrlSpec> {
        Bank1enW::new(self, 9)
    }
    #[doc = "Bit 10 - Bank 2 Enable"]
    #[inline(always)]
    pub fn bank2en(&mut self) -> Bank2enW<'_, CtrlSpec> {
        Bank2enW::new(self, 10)
    }
    #[doc = "Bit 11 - Bank 3 Enable"]
    #[inline(always)]
    pub fn bank3en(&mut self) -> Bank3enW<'_, CtrlSpec> {
        Bank3enW::new(self, 11)
    }
    #[doc = "Bit 12 - No Idle Cycle Insertion on Bank 0"]
    #[inline(always)]
    pub fn noidle(&mut self) -> NoidleW<'_, CtrlSpec> {
        NoidleW::new(self, 12)
    }
    #[doc = "Bit 13 - No Idle Cycle Insertion on Bank 1"]
    #[inline(always)]
    pub fn noidle1(&mut self) -> Noidle1W<'_, CtrlSpec> {
        Noidle1W::new(self, 13)
    }
    #[doc = "Bit 14 - No Idle Cycle Insertion on Bank 2"]
    #[inline(always)]
    pub fn noidle2(&mut self) -> Noidle2W<'_, CtrlSpec> {
        Noidle2W::new(self, 14)
    }
    #[doc = "Bit 15 - No Idle Cycle Insertion on Bank 3"]
    #[inline(always)]
    pub fn noidle3(&mut self) -> Noidle3W<'_, CtrlSpec> {
        Noidle3W::new(self, 15)
    }
    #[doc = "Bit 16 - ARDY Enable"]
    #[inline(always)]
    pub fn ardyen(&mut self) -> ArdyenW<'_, CtrlSpec> {
        ArdyenW::new(self, 16)
    }
    #[doc = "Bit 17 - ARDY Timeout Disable"]
    #[inline(always)]
    pub fn ardytodis(&mut self) -> ArdytodisW<'_, CtrlSpec> {
        ArdytodisW::new(self, 17)
    }
    #[doc = "Bit 18 - ARDY Enable for Bank 1"]
    #[inline(always)]
    pub fn ardy1en(&mut self) -> Ardy1enW<'_, CtrlSpec> {
        Ardy1enW::new(self, 18)
    }
    #[doc = "Bit 19 - ARDY Timeout Disable for Bank 1"]
    #[inline(always)]
    pub fn ardyto1dis(&mut self) -> Ardyto1disW<'_, CtrlSpec> {
        Ardyto1disW::new(self, 19)
    }
    #[doc = "Bit 20 - ARDY Enable for Bank 2"]
    #[inline(always)]
    pub fn ardy2en(&mut self) -> Ardy2enW<'_, CtrlSpec> {
        Ardy2enW::new(self, 20)
    }
    #[doc = "Bit 21 - ARDY Timeout Disable for Bank 2"]
    #[inline(always)]
    pub fn ardyto2dis(&mut self) -> Ardyto2disW<'_, CtrlSpec> {
        Ardyto2disW::new(self, 21)
    }
    #[doc = "Bit 22 - ARDY Enable for Bank 3"]
    #[inline(always)]
    pub fn ardy3en(&mut self) -> Ardy3enW<'_, CtrlSpec> {
        Ardy3enW::new(self, 22)
    }
    #[doc = "Bit 23 - ARDY Timeout Disable for Bank 3"]
    #[inline(always)]
    pub fn ardyto3dis(&mut self) -> Ardyto3disW<'_, CtrlSpec> {
        Ardyto3disW::new(self, 23)
    }
    #[doc = "Bit 24 - Byte Lane Enable for Bank 0"]
    #[inline(always)]
    pub fn bl(&mut self) -> BlW<'_, CtrlSpec> {
        BlW::new(self, 24)
    }
    #[doc = "Bit 25 - Byte Lane Enable for Bank 1"]
    #[inline(always)]
    pub fn bl1(&mut self) -> Bl1W<'_, CtrlSpec> {
        Bl1W::new(self, 25)
    }
    #[doc = "Bit 26 - Byte Lane Enable for Bank 2"]
    #[inline(always)]
    pub fn bl2(&mut self) -> Bl2W<'_, CtrlSpec> {
        Bl2W::new(self, 26)
    }
    #[doc = "Bit 27 - Byte Lane Enable for Bank 3"]
    #[inline(always)]
    pub fn bl3(&mut self) -> Bl3W<'_, CtrlSpec> {
        Bl3W::new(self, 27)
    }
    #[doc = "Bit 30 - Individual Timing Set, Line Polarity and Mode Definition Enable"]
    #[inline(always)]
    pub fn its(&mut self) -> ItsW<'_, CtrlSpec> {
        ItsW::new(self, 30)
    }
    #[doc = "Bit 31 - Alternative Address Map Enable"]
    #[inline(always)]
    pub fn altmap(&mut self) -> AltmapW<'_, CtrlSpec> {
        AltmapW::new(self, 31)
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
