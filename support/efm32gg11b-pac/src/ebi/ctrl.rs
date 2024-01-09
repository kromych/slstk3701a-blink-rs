#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `MODE` reader - Mode"]
pub type MODE_R = crate::FieldReader<MODE_A>;
#[doc = "Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled."]
    D8A8 = 0,
    #[doc = "1: EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    D16A16ALE = 1,
    #[doc = "2: EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    D8A24ALE = 2,
    #[doc = "3: EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled."]
    D16 = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE_A {
    type Ux = u8;
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::D8A8,
            1 => MODE_A::D16A16ALE,
            2 => MODE_A::D8A24ALE,
            3 => MODE_A::D16,
            _ => unreachable!(),
        }
    }
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn is_d8a8(&self) -> bool {
        *self == MODE_A::D8A8
    }
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn is_d16a16ale(&self) -> bool {
        *self == MODE_A::D16A16ALE
    }
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn is_d8a24ale(&self) -> bool {
        *self == MODE_A::D8A24ALE
    }
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn is_d16(&self) -> bool {
        *self == MODE_A::D16
    }
}
#[doc = "Field `MODE` writer - Mode"]
pub type MODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, MODE_A>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d8a8(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::D8A8)
    }
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d16a16ale(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::D16A16ALE)
    }
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d8a24ale(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::D8A24ALE)
    }
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d16(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::D16)
    }
}
#[doc = "Field `MODE1` reader - Mode 1"]
pub type MODE1_R = crate::FieldReader<MODE1_A>;
#[doc = "Mode 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE1_A {
    #[doc = "0: EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled."]
    D8A8 = 0,
    #[doc = "1: EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    D16A16ALE = 1,
    #[doc = "2: EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    D8A24ALE = 2,
    #[doc = "3: EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled."]
    D16 = 3,
}
impl From<MODE1_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE1_A {
    type Ux = u8;
}
impl MODE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE1_A {
        match self.bits {
            0 => MODE1_A::D8A8,
            1 => MODE1_A::D16A16ALE,
            2 => MODE1_A::D8A24ALE,
            3 => MODE1_A::D16,
            _ => unreachable!(),
        }
    }
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn is_d8a8(&self) -> bool {
        *self == MODE1_A::D8A8
    }
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn is_d16a16ale(&self) -> bool {
        *self == MODE1_A::D16A16ALE
    }
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn is_d8a24ale(&self) -> bool {
        *self == MODE1_A::D8A24ALE
    }
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn is_d16(&self) -> bool {
        *self == MODE1_A::D16
    }
}
#[doc = "Field `MODE1` writer - Mode 1"]
pub type MODE1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, MODE1_A>;
impl<'a, REG> MODE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d8a8(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1_A::D8A8)
    }
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d16a16ale(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1_A::D16A16ALE)
    }
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d8a24ale(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1_A::D8A24ALE)
    }
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d16(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1_A::D16)
    }
}
#[doc = "Field `MODE2` reader - Mode 2"]
pub type MODE2_R = crate::FieldReader<MODE2_A>;
#[doc = "Mode 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE2_A {
    #[doc = "0: EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled."]
    D8A8 = 0,
    #[doc = "1: EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    D16A16ALE = 1,
    #[doc = "2: EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    D8A24ALE = 2,
    #[doc = "3: EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled."]
    D16 = 3,
}
impl From<MODE2_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE2_A {
    type Ux = u8;
}
impl MODE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE2_A {
        match self.bits {
            0 => MODE2_A::D8A8,
            1 => MODE2_A::D16A16ALE,
            2 => MODE2_A::D8A24ALE,
            3 => MODE2_A::D16,
            _ => unreachable!(),
        }
    }
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn is_d8a8(&self) -> bool {
        *self == MODE2_A::D8A8
    }
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn is_d16a16ale(&self) -> bool {
        *self == MODE2_A::D16A16ALE
    }
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn is_d8a24ale(&self) -> bool {
        *self == MODE2_A::D8A24ALE
    }
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn is_d16(&self) -> bool {
        *self == MODE2_A::D16
    }
}
#[doc = "Field `MODE2` writer - Mode 2"]
pub type MODE2_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, MODE2_A>;
impl<'a, REG> MODE2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d8a8(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2_A::D8A8)
    }
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d16a16ale(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2_A::D16A16ALE)
    }
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d8a24ale(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2_A::D8A24ALE)
    }
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d16(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2_A::D16)
    }
}
#[doc = "Field `MODE3` reader - Mode 3"]
pub type MODE3_R = crate::FieldReader<MODE3_A>;
#[doc = "Mode 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE3_A {
    #[doc = "0: EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled."]
    D8A8 = 0,
    #[doc = "1: EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    D16A16ALE = 1,
    #[doc = "2: EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    D8A24ALE = 2,
    #[doc = "3: EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled."]
    D16 = 3,
}
impl From<MODE3_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE3_A {
    type Ux = u8;
}
impl MODE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE3_A {
        match self.bits {
            0 => MODE3_A::D8A8,
            1 => MODE3_A::D16A16ALE,
            2 => MODE3_A::D8A24ALE,
            3 => MODE3_A::D16,
            _ => unreachable!(),
        }
    }
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn is_d8a8(&self) -> bool {
        *self == MODE3_A::D8A8
    }
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn is_d16a16ale(&self) -> bool {
        *self == MODE3_A::D16A16ALE
    }
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn is_d8a24ale(&self) -> bool {
        *self == MODE3_A::D8A24ALE
    }
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn is_d16(&self) -> bool {
        *self == MODE3_A::D16
    }
}
#[doc = "Field `MODE3` writer - Mode 3"]
pub type MODE3_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, MODE3_A>;
impl<'a, REG> MODE3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d8a8(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3_A::D8A8)
    }
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d16a16ale(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3_A::D16A16ALE)
    }
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d8a24ale(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3_A::D8A24ALE)
    }
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d16(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3_A::D16)
    }
}
#[doc = "Field `BANK0EN` reader - Bank 0 Enable"]
pub type BANK0EN_R = crate::BitReader;
#[doc = "Field `BANK0EN` writer - Bank 0 Enable"]
pub type BANK0EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BANK1EN` reader - Bank 1 Enable"]
pub type BANK1EN_R = crate::BitReader;
#[doc = "Field `BANK1EN` writer - Bank 1 Enable"]
pub type BANK1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BANK2EN` reader - Bank 2 Enable"]
pub type BANK2EN_R = crate::BitReader;
#[doc = "Field `BANK2EN` writer - Bank 2 Enable"]
pub type BANK2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BANK3EN` reader - Bank 3 Enable"]
pub type BANK3EN_R = crate::BitReader;
#[doc = "Field `BANK3EN` writer - Bank 3 Enable"]
pub type BANK3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOIDLE` reader - No Idle Cycle Insertion on Bank 0"]
pub type NOIDLE_R = crate::BitReader;
#[doc = "Field `NOIDLE` writer - No Idle Cycle Insertion on Bank 0"]
pub type NOIDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOIDLE1` reader - No Idle Cycle Insertion on Bank 1"]
pub type NOIDLE1_R = crate::BitReader;
#[doc = "Field `NOIDLE1` writer - No Idle Cycle Insertion on Bank 1"]
pub type NOIDLE1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOIDLE2` reader - No Idle Cycle Insertion on Bank 2"]
pub type NOIDLE2_R = crate::BitReader;
#[doc = "Field `NOIDLE2` writer - No Idle Cycle Insertion on Bank 2"]
pub type NOIDLE2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOIDLE3` reader - No Idle Cycle Insertion on Bank 3"]
pub type NOIDLE3_R = crate::BitReader;
#[doc = "Field `NOIDLE3` writer - No Idle Cycle Insertion on Bank 3"]
pub type NOIDLE3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARDYEN` reader - ARDY Enable"]
pub type ARDYEN_R = crate::BitReader;
#[doc = "Field `ARDYEN` writer - ARDY Enable"]
pub type ARDYEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARDYTODIS` reader - ARDY Timeout Disable"]
pub type ARDYTODIS_R = crate::BitReader;
#[doc = "Field `ARDYTODIS` writer - ARDY Timeout Disable"]
pub type ARDYTODIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARDY1EN` reader - ARDY Enable for Bank 1"]
pub type ARDY1EN_R = crate::BitReader;
#[doc = "Field `ARDY1EN` writer - ARDY Enable for Bank 1"]
pub type ARDY1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARDYTO1DIS` reader - ARDY Timeout Disable for Bank 1"]
pub type ARDYTO1DIS_R = crate::BitReader;
#[doc = "Field `ARDYTO1DIS` writer - ARDY Timeout Disable for Bank 1"]
pub type ARDYTO1DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARDY2EN` reader - ARDY Enable for Bank 2"]
pub type ARDY2EN_R = crate::BitReader;
#[doc = "Field `ARDY2EN` writer - ARDY Enable for Bank 2"]
pub type ARDY2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARDYTO2DIS` reader - ARDY Timeout Disable for Bank 2"]
pub type ARDYTO2DIS_R = crate::BitReader;
#[doc = "Field `ARDYTO2DIS` writer - ARDY Timeout Disable for Bank 2"]
pub type ARDYTO2DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARDY3EN` reader - ARDY Enable for Bank 3"]
pub type ARDY3EN_R = crate::BitReader;
#[doc = "Field `ARDY3EN` writer - ARDY Enable for Bank 3"]
pub type ARDY3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARDYTO3DIS` reader - ARDY Timeout Disable for Bank 3"]
pub type ARDYTO3DIS_R = crate::BitReader;
#[doc = "Field `ARDYTO3DIS` writer - ARDY Timeout Disable for Bank 3"]
pub type ARDYTO3DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BL` reader - Byte Lane Enable for Bank 0"]
pub type BL_R = crate::BitReader;
#[doc = "Field `BL` writer - Byte Lane Enable for Bank 0"]
pub type BL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BL1` reader - Byte Lane Enable for Bank 1"]
pub type BL1_R = crate::BitReader;
#[doc = "Field `BL1` writer - Byte Lane Enable for Bank 1"]
pub type BL1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BL2` reader - Byte Lane Enable for Bank 2"]
pub type BL2_R = crate::BitReader;
#[doc = "Field `BL2` writer - Byte Lane Enable for Bank 2"]
pub type BL2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BL3` reader - Byte Lane Enable for Bank 3"]
pub type BL3_R = crate::BitReader;
#[doc = "Field `BL3` writer - Byte Lane Enable for Bank 3"]
pub type BL3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITS` reader - Individual Timing Set, Line Polarity and Mode Definition Enable"]
pub type ITS_R = crate::BitReader;
#[doc = "Field `ITS` writer - Individual Timing Set, Line Polarity and Mode Definition Enable"]
pub type ITS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALTMAP` reader - Alternative Address Map Enable"]
pub type ALTMAP_R = crate::BitReader;
#[doc = "Field `ALTMAP` writer - Alternative Address Map Enable"]
pub type ALTMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Mode 1"]
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Mode 2"]
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Mode 3"]
    #[inline(always)]
    pub fn mode3(&self) -> MODE3_R {
        MODE3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Bank 0 Enable"]
    #[inline(always)]
    pub fn bank0en(&self) -> BANK0EN_R {
        BANK0EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Bank 1 Enable"]
    #[inline(always)]
    pub fn bank1en(&self) -> BANK1EN_R {
        BANK1EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Bank 2 Enable"]
    #[inline(always)]
    pub fn bank2en(&self) -> BANK2EN_R {
        BANK2EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bank 3 Enable"]
    #[inline(always)]
    pub fn bank3en(&self) -> BANK3EN_R {
        BANK3EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - No Idle Cycle Insertion on Bank 0"]
    #[inline(always)]
    pub fn noidle(&self) -> NOIDLE_R {
        NOIDLE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - No Idle Cycle Insertion on Bank 1"]
    #[inline(always)]
    pub fn noidle1(&self) -> NOIDLE1_R {
        NOIDLE1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - No Idle Cycle Insertion on Bank 2"]
    #[inline(always)]
    pub fn noidle2(&self) -> NOIDLE2_R {
        NOIDLE2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - No Idle Cycle Insertion on Bank 3"]
    #[inline(always)]
    pub fn noidle3(&self) -> NOIDLE3_R {
        NOIDLE3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ARDY Enable"]
    #[inline(always)]
    pub fn ardyen(&self) -> ARDYEN_R {
        ARDYEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ARDY Timeout Disable"]
    #[inline(always)]
    pub fn ardytodis(&self) -> ARDYTODIS_R {
        ARDYTODIS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ARDY Enable for Bank 1"]
    #[inline(always)]
    pub fn ardy1en(&self) -> ARDY1EN_R {
        ARDY1EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ARDY Timeout Disable for Bank 1"]
    #[inline(always)]
    pub fn ardyto1dis(&self) -> ARDYTO1DIS_R {
        ARDYTO1DIS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ARDY Enable for Bank 2"]
    #[inline(always)]
    pub fn ardy2en(&self) -> ARDY2EN_R {
        ARDY2EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ARDY Timeout Disable for Bank 2"]
    #[inline(always)]
    pub fn ardyto2dis(&self) -> ARDYTO2DIS_R {
        ARDYTO2DIS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ARDY Enable for Bank 3"]
    #[inline(always)]
    pub fn ardy3en(&self) -> ARDY3EN_R {
        ARDY3EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ARDY Timeout Disable for Bank 3"]
    #[inline(always)]
    pub fn ardyto3dis(&self) -> ARDYTO3DIS_R {
        ARDYTO3DIS_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Byte Lane Enable for Bank 0"]
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Byte Lane Enable for Bank 1"]
    #[inline(always)]
    pub fn bl1(&self) -> BL1_R {
        BL1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Byte Lane Enable for Bank 2"]
    #[inline(always)]
    pub fn bl2(&self) -> BL2_R {
        BL2_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Byte Lane Enable for Bank 3"]
    #[inline(always)]
    pub fn bl3(&self) -> BL3_R {
        BL3_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - Individual Timing Set, Line Polarity and Mode Definition Enable"]
    #[inline(always)]
    pub fn its(&self) -> ITS_R {
        ITS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Alternative Address Map Enable"]
    #[inline(always)]
    pub fn altmap(&self) -> ALTMAP_R {
        ALTMAP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CTRL_SPEC> {
        MODE_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Mode 1"]
    #[inline(always)]
    #[must_use]
    pub fn mode1(&mut self) -> MODE1_W<CTRL_SPEC> {
        MODE1_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Mode 2"]
    #[inline(always)]
    #[must_use]
    pub fn mode2(&mut self) -> MODE2_W<CTRL_SPEC> {
        MODE2_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Mode 3"]
    #[inline(always)]
    #[must_use]
    pub fn mode3(&mut self) -> MODE3_W<CTRL_SPEC> {
        MODE3_W::new(self, 6)
    }
    #[doc = "Bit 8 - Bank 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bank0en(&mut self) -> BANK0EN_W<CTRL_SPEC> {
        BANK0EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Bank 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bank1en(&mut self) -> BANK1EN_W<CTRL_SPEC> {
        BANK1EN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Bank 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bank2en(&mut self) -> BANK2EN_W<CTRL_SPEC> {
        BANK2EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Bank 3 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bank3en(&mut self) -> BANK3EN_W<CTRL_SPEC> {
        BANK3EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - No Idle Cycle Insertion on Bank 0"]
    #[inline(always)]
    #[must_use]
    pub fn noidle(&mut self) -> NOIDLE_W<CTRL_SPEC> {
        NOIDLE_W::new(self, 12)
    }
    #[doc = "Bit 13 - No Idle Cycle Insertion on Bank 1"]
    #[inline(always)]
    #[must_use]
    pub fn noidle1(&mut self) -> NOIDLE1_W<CTRL_SPEC> {
        NOIDLE1_W::new(self, 13)
    }
    #[doc = "Bit 14 - No Idle Cycle Insertion on Bank 2"]
    #[inline(always)]
    #[must_use]
    pub fn noidle2(&mut self) -> NOIDLE2_W<CTRL_SPEC> {
        NOIDLE2_W::new(self, 14)
    }
    #[doc = "Bit 15 - No Idle Cycle Insertion on Bank 3"]
    #[inline(always)]
    #[must_use]
    pub fn noidle3(&mut self) -> NOIDLE3_W<CTRL_SPEC> {
        NOIDLE3_W::new(self, 15)
    }
    #[doc = "Bit 16 - ARDY Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ardyen(&mut self) -> ARDYEN_W<CTRL_SPEC> {
        ARDYEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - ARDY Timeout Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ardytodis(&mut self) -> ARDYTODIS_W<CTRL_SPEC> {
        ARDYTODIS_W::new(self, 17)
    }
    #[doc = "Bit 18 - ARDY Enable for Bank 1"]
    #[inline(always)]
    #[must_use]
    pub fn ardy1en(&mut self) -> ARDY1EN_W<CTRL_SPEC> {
        ARDY1EN_W::new(self, 18)
    }
    #[doc = "Bit 19 - ARDY Timeout Disable for Bank 1"]
    #[inline(always)]
    #[must_use]
    pub fn ardyto1dis(&mut self) -> ARDYTO1DIS_W<CTRL_SPEC> {
        ARDYTO1DIS_W::new(self, 19)
    }
    #[doc = "Bit 20 - ARDY Enable for Bank 2"]
    #[inline(always)]
    #[must_use]
    pub fn ardy2en(&mut self) -> ARDY2EN_W<CTRL_SPEC> {
        ARDY2EN_W::new(self, 20)
    }
    #[doc = "Bit 21 - ARDY Timeout Disable for Bank 2"]
    #[inline(always)]
    #[must_use]
    pub fn ardyto2dis(&mut self) -> ARDYTO2DIS_W<CTRL_SPEC> {
        ARDYTO2DIS_W::new(self, 21)
    }
    #[doc = "Bit 22 - ARDY Enable for Bank 3"]
    #[inline(always)]
    #[must_use]
    pub fn ardy3en(&mut self) -> ARDY3EN_W<CTRL_SPEC> {
        ARDY3EN_W::new(self, 22)
    }
    #[doc = "Bit 23 - ARDY Timeout Disable for Bank 3"]
    #[inline(always)]
    #[must_use]
    pub fn ardyto3dis(&mut self) -> ARDYTO3DIS_W<CTRL_SPEC> {
        ARDYTO3DIS_W::new(self, 23)
    }
    #[doc = "Bit 24 - Byte Lane Enable for Bank 0"]
    #[inline(always)]
    #[must_use]
    pub fn bl(&mut self) -> BL_W<CTRL_SPEC> {
        BL_W::new(self, 24)
    }
    #[doc = "Bit 25 - Byte Lane Enable for Bank 1"]
    #[inline(always)]
    #[must_use]
    pub fn bl1(&mut self) -> BL1_W<CTRL_SPEC> {
        BL1_W::new(self, 25)
    }
    #[doc = "Bit 26 - Byte Lane Enable for Bank 2"]
    #[inline(always)]
    #[must_use]
    pub fn bl2(&mut self) -> BL2_W<CTRL_SPEC> {
        BL2_W::new(self, 26)
    }
    #[doc = "Bit 27 - Byte Lane Enable for Bank 3"]
    #[inline(always)]
    #[must_use]
    pub fn bl3(&mut self) -> BL3_W<CTRL_SPEC> {
        BL3_W::new(self, 27)
    }
    #[doc = "Bit 30 - Individual Timing Set, Line Polarity and Mode Definition Enable"]
    #[inline(always)]
    #[must_use]
    pub fn its(&mut self) -> ITS_W<CTRL_SPEC> {
        ITS_W::new(self, 30)
    }
    #[doc = "Bit 31 - Alternative Address Map Enable"]
    #[inline(always)]
    #[must_use]
    pub fn altmap(&mut self) -> ALTMAP_W<CTRL_SPEC> {
        ALTMAP_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
