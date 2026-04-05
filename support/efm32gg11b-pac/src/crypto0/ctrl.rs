#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `AES` reader - AES Mode"]
pub type AesR = crate::BitReader;
#[doc = "Field `AES` writer - AES Mode"]
pub type AesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYBUFDIS` reader - Key Buffer Disable"]
pub type KeybufdisR = crate::BitReader;
#[doc = "Field `KEYBUFDIS` writer - Key Buffer Disable"]
pub type KeybufdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHA` reader - SHA Mode"]
pub type ShaR = crate::BitReader;
#[doc = "Field `SHA` writer - SHA Mode"]
pub type ShaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOBUSYSTALL` reader - No Stalling of Bus When Busy"]
pub type NobusystallR = crate::BitReader;
#[doc = "Field `NOBUSYSTALL` writer - No Stalling of Bus When Busy"]
pub type NobusystallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Increment Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Incwidth {
    #[doc = "0: Byte 15 in DATA1 is used for the increment function."]
    Incwidth1 = 0,
    #[doc = "1: Bytes 14 and 15 in DATA1 are used for the increment function."]
    Incwidth2 = 1,
    #[doc = "2: Bytes 13 to 15 in DATA1 are used for the increment function."]
    Incwidth3 = 2,
    #[doc = "3: Bytes 12 to 15 in DATA1 are used for the increment function."]
    Incwidth4 = 3,
}
impl From<Incwidth> for u8 {
    #[inline(always)]
    fn from(variant: Incwidth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Incwidth {
    type Ux = u8;
}
impl crate::IsEnum for Incwidth {}
#[doc = "Field `INCWIDTH` reader - Increment Width"]
pub type IncwidthR = crate::FieldReader<Incwidth>;
impl IncwidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Incwidth {
        match self.bits {
            0 => Incwidth::Incwidth1,
            1 => Incwidth::Incwidth2,
            2 => Incwidth::Incwidth3,
            3 => Incwidth::Incwidth4,
            _ => unreachable!(),
        }
    }
    #[doc = "Byte 15 in DATA1 is used for the increment function."]
    #[inline(always)]
    pub fn is_incwidth1(&self) -> bool {
        *self == Incwidth::Incwidth1
    }
    #[doc = "Bytes 14 and 15 in DATA1 are used for the increment function."]
    #[inline(always)]
    pub fn is_incwidth2(&self) -> bool {
        *self == Incwidth::Incwidth2
    }
    #[doc = "Bytes 13 to 15 in DATA1 are used for the increment function."]
    #[inline(always)]
    pub fn is_incwidth3(&self) -> bool {
        *self == Incwidth::Incwidth3
    }
    #[doc = "Bytes 12 to 15 in DATA1 are used for the increment function."]
    #[inline(always)]
    pub fn is_incwidth4(&self) -> bool {
        *self == Incwidth::Incwidth4
    }
}
#[doc = "Field `INCWIDTH` writer - Increment Width"]
pub type IncwidthW<'a, REG> = crate::FieldWriter<'a, REG, 2, Incwidth, crate::Safe>;
impl<'a, REG> IncwidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Byte 15 in DATA1 is used for the increment function."]
    #[inline(always)]
    pub fn incwidth1(self) -> &'a mut crate::W<REG> {
        self.variant(Incwidth::Incwidth1)
    }
    #[doc = "Bytes 14 and 15 in DATA1 are used for the increment function."]
    #[inline(always)]
    pub fn incwidth2(self) -> &'a mut crate::W<REG> {
        self.variant(Incwidth::Incwidth2)
    }
    #[doc = "Bytes 13 to 15 in DATA1 are used for the increment function."]
    #[inline(always)]
    pub fn incwidth3(self) -> &'a mut crate::W<REG> {
        self.variant(Incwidth::Incwidth3)
    }
    #[doc = "Bytes 12 to 15 in DATA1 are used for the increment function."]
    #[inline(always)]
    pub fn incwidth4(self) -> &'a mut crate::W<REG> {
        self.variant(Incwidth::Incwidth4)
    }
}
#[doc = "DMA0 Read Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dma0mode {
    #[doc = "0: Target register is fully read/written during every DMA transaction"]
    Full = 0,
    #[doc = "1: Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + necessary zero padding is read. Zero padding is automatically added when writing."]
    Lenlimit = 1,
    #[doc = "2: Target register is fully read/written during every DMA transaction. Bytewise DMA."]
    Fullbyte = 2,
    #[doc = "3: Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + necessary zero padding is read. Bytewise DMA. Zero padding is automatically added when writing."]
    Lenlimitbyte = 3,
}
impl From<Dma0mode> for u8 {
    #[inline(always)]
    fn from(variant: Dma0mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dma0mode {
    type Ux = u8;
}
impl crate::IsEnum for Dma0mode {}
#[doc = "Field `DMA0MODE` reader - DMA0 Read Mode"]
pub type Dma0modeR = crate::FieldReader<Dma0mode>;
impl Dma0modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma0mode {
        match self.bits {
            0 => Dma0mode::Full,
            1 => Dma0mode::Lenlimit,
            2 => Dma0mode::Fullbyte,
            3 => Dma0mode::Lenlimitbyte,
            _ => unreachable!(),
        }
    }
    #[doc = "Target register is fully read/written during every DMA transaction"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Dma0mode::Full
    }
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + necessary zero padding is read. Zero padding is automatically added when writing."]
    #[inline(always)]
    pub fn is_lenlimit(&self) -> bool {
        *self == Dma0mode::Lenlimit
    }
    #[doc = "Target register is fully read/written during every DMA transaction. Bytewise DMA."]
    #[inline(always)]
    pub fn is_fullbyte(&self) -> bool {
        *self == Dma0mode::Fullbyte
    }
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + necessary zero padding is read. Bytewise DMA. Zero padding is automatically added when writing."]
    #[inline(always)]
    pub fn is_lenlimitbyte(&self) -> bool {
        *self == Dma0mode::Lenlimitbyte
    }
}
#[doc = "Field `DMA0MODE` writer - DMA0 Read Mode"]
pub type Dma0modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dma0mode, crate::Safe>;
impl<'a, REG> Dma0modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Target register is fully read/written during every DMA transaction"]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0mode::Full)
    }
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + necessary zero padding is read. Zero padding is automatically added when writing."]
    #[inline(always)]
    pub fn lenlimit(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0mode::Lenlimit)
    }
    #[doc = "Target register is fully read/written during every DMA transaction. Bytewise DMA."]
    #[inline(always)]
    pub fn fullbyte(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0mode::Fullbyte)
    }
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + necessary zero padding is read. Bytewise DMA. Zero padding is automatically added when writing."]
    #[inline(always)]
    pub fn lenlimitbyte(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0mode::Lenlimitbyte)
    }
}
#[doc = "DMA0 Read Register Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dma0rsel {
    #[doc = "0: `0`"]
    Data0 = 0,
    #[doc = "1: `1`"]
    Ddata0 = 1,
    #[doc = "2: `10`"]
    Ddata0big = 2,
    #[doc = "3: `11`"]
    Qdata0 = 3,
}
impl From<Dma0rsel> for u8 {
    #[inline(always)]
    fn from(variant: Dma0rsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dma0rsel {
    type Ux = u8;
}
impl crate::IsEnum for Dma0rsel {}
#[doc = "Field `DMA0RSEL` reader - DMA0 Read Register Select"]
pub type Dma0rselR = crate::FieldReader<Dma0rsel>;
impl Dma0rselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma0rsel {
        match self.bits {
            0 => Dma0rsel::Data0,
            1 => Dma0rsel::Ddata0,
            2 => Dma0rsel::Ddata0big,
            3 => Dma0rsel::Qdata0,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_data0(&self) -> bool {
        *self == Dma0rsel::Data0
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_ddata0(&self) -> bool {
        *self == Dma0rsel::Ddata0
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_ddata0big(&self) -> bool {
        *self == Dma0rsel::Ddata0big
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_qdata0(&self) -> bool {
        *self == Dma0rsel::Qdata0
    }
}
#[doc = "Field `DMA0RSEL` writer - DMA0 Read Register Select"]
pub type Dma0rselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dma0rsel, crate::Safe>;
impl<'a, REG> Dma0rselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn data0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0rsel::Data0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn ddata0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0rsel::Ddata0)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn ddata0big(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0rsel::Ddata0big)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn qdata0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0rsel::Qdata0)
    }
}
#[doc = "DMA1 Read Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dma1mode {
    #[doc = "0: Target register is fully read/written during every DMA transaction"]
    Full = 0,
    #[doc = "1: Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + 1 bytes + necessary zero padding is read. Zero padding is automatically added when writing."]
    Lenlimit = 1,
    #[doc = "2: Target register is fully read/written during every DMA transaction. Bytewise DMA."]
    Fullbyte = 2,
    #[doc = "3: Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + 1 bytes + necessary zero padding is read. Bytewise DMA. Zero padding is automatically added when writing."]
    Lenlimitbyte = 3,
}
impl From<Dma1mode> for u8 {
    #[inline(always)]
    fn from(variant: Dma1mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dma1mode {
    type Ux = u8;
}
impl crate::IsEnum for Dma1mode {}
#[doc = "Field `DMA1MODE` reader - DMA1 Read Mode"]
pub type Dma1modeR = crate::FieldReader<Dma1mode>;
impl Dma1modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma1mode {
        match self.bits {
            0 => Dma1mode::Full,
            1 => Dma1mode::Lenlimit,
            2 => Dma1mode::Fullbyte,
            3 => Dma1mode::Lenlimitbyte,
            _ => unreachable!(),
        }
    }
    #[doc = "Target register is fully read/written during every DMA transaction"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Dma1mode::Full
    }
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + 1 bytes + necessary zero padding is read. Zero padding is automatically added when writing."]
    #[inline(always)]
    pub fn is_lenlimit(&self) -> bool {
        *self == Dma1mode::Lenlimit
    }
    #[doc = "Target register is fully read/written during every DMA transaction. Bytewise DMA."]
    #[inline(always)]
    pub fn is_fullbyte(&self) -> bool {
        *self == Dma1mode::Fullbyte
    }
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + 1 bytes + necessary zero padding is read. Bytewise DMA. Zero padding is automatically added when writing."]
    #[inline(always)]
    pub fn is_lenlimitbyte(&self) -> bool {
        *self == Dma1mode::Lenlimitbyte
    }
}
#[doc = "Field `DMA1MODE` writer - DMA1 Read Mode"]
pub type Dma1modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dma1mode, crate::Safe>;
impl<'a, REG> Dma1modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Target register is fully read/written during every DMA transaction"]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1mode::Full)
    }
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + 1 bytes + necessary zero padding is read. Zero padding is automatically added when writing."]
    #[inline(always)]
    pub fn lenlimit(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1mode::Lenlimit)
    }
    #[doc = "Target register is fully read/written during every DMA transaction. Bytewise DMA."]
    #[inline(always)]
    pub fn fullbyte(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1mode::Fullbyte)
    }
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + 1 bytes + necessary zero padding is read. Bytewise DMA. Zero padding is automatically added when writing."]
    #[inline(always)]
    pub fn lenlimitbyte(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1mode::Lenlimitbyte)
    }
}
#[doc = "DATA0 DMA Unaligned Read Register Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dma1rsel {
    #[doc = "0: `0`"]
    Data1 = 0,
    #[doc = "1: `1`"]
    Ddata1 = 1,
    #[doc = "2: `10`"]
    Qdata1 = 2,
    #[doc = "3: `11`"]
    Qdata1big = 3,
}
impl From<Dma1rsel> for u8 {
    #[inline(always)]
    fn from(variant: Dma1rsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dma1rsel {
    type Ux = u8;
}
impl crate::IsEnum for Dma1rsel {}
#[doc = "Field `DMA1RSEL` reader - DATA0 DMA Unaligned Read Register Select"]
pub type Dma1rselR = crate::FieldReader<Dma1rsel>;
impl Dma1rselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma1rsel {
        match self.bits {
            0 => Dma1rsel::Data1,
            1 => Dma1rsel::Ddata1,
            2 => Dma1rsel::Qdata1,
            3 => Dma1rsel::Qdata1big,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        *self == Dma1rsel::Data1
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_ddata1(&self) -> bool {
        *self == Dma1rsel::Ddata1
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_qdata1(&self) -> bool {
        *self == Dma1rsel::Qdata1
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_qdata1big(&self) -> bool {
        *self == Dma1rsel::Qdata1big
    }
}
#[doc = "Field `DMA1RSEL` writer - DATA0 DMA Unaligned Read Register Select"]
pub type Dma1rselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dma1rsel, crate::Safe>;
impl<'a, REG> Dma1rselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn data1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1rsel::Data1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn ddata1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1rsel::Ddata1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn qdata1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1rsel::Qdata1)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn qdata1big(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1rsel::Qdata1big)
    }
}
#[doc = "Field `COMBDMA0WEREQ` reader - Combined Data0 Write DMA Request"]
pub type Combdma0wereqR = crate::BitReader;
#[doc = "Field `COMBDMA0WEREQ` writer - Combined Data0 Write DMA Request"]
pub type Combdma0wereqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AES Mode"]
    #[inline(always)]
    pub fn aes(&self) -> AesR {
        AesR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Key Buffer Disable"]
    #[inline(always)]
    pub fn keybufdis(&self) -> KeybufdisR {
        KeybufdisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SHA Mode"]
    #[inline(always)]
    pub fn sha(&self) -> ShaR {
        ShaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 10 - No Stalling of Bus When Busy"]
    #[inline(always)]
    pub fn nobusystall(&self) -> NobusystallR {
        NobusystallR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Increment Width"]
    #[inline(always)]
    pub fn incwidth(&self) -> IncwidthR {
        IncwidthR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - DMA0 Read Mode"]
    #[inline(always)]
    pub fn dma0mode(&self) -> Dma0modeR {
        Dma0modeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - DMA0 Read Register Select"]
    #[inline(always)]
    pub fn dma0rsel(&self) -> Dma0rselR {
        Dma0rselR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - DMA1 Read Mode"]
    #[inline(always)]
    pub fn dma1mode(&self) -> Dma1modeR {
        Dma1modeR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - DATA0 DMA Unaligned Read Register Select"]
    #[inline(always)]
    pub fn dma1rsel(&self) -> Dma1rselR {
        Dma1rselR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - Combined Data0 Write DMA Request"]
    #[inline(always)]
    pub fn combdma0wereq(&self) -> Combdma0wereqR {
        Combdma0wereqR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AES Mode"]
    #[inline(always)]
    pub fn aes(&mut self) -> AesW<'_, CtrlSpec> {
        AesW::new(self, 0)
    }
    #[doc = "Bit 1 - Key Buffer Disable"]
    #[inline(always)]
    pub fn keybufdis(&mut self) -> KeybufdisW<'_, CtrlSpec> {
        KeybufdisW::new(self, 1)
    }
    #[doc = "Bit 2 - SHA Mode"]
    #[inline(always)]
    pub fn sha(&mut self) -> ShaW<'_, CtrlSpec> {
        ShaW::new(self, 2)
    }
    #[doc = "Bit 10 - No Stalling of Bus When Busy"]
    #[inline(always)]
    pub fn nobusystall(&mut self) -> NobusystallW<'_, CtrlSpec> {
        NobusystallW::new(self, 10)
    }
    #[doc = "Bits 14:15 - Increment Width"]
    #[inline(always)]
    pub fn incwidth(&mut self) -> IncwidthW<'_, CtrlSpec> {
        IncwidthW::new(self, 14)
    }
    #[doc = "Bits 16:17 - DMA0 Read Mode"]
    #[inline(always)]
    pub fn dma0mode(&mut self) -> Dma0modeW<'_, CtrlSpec> {
        Dma0modeW::new(self, 16)
    }
    #[doc = "Bits 20:21 - DMA0 Read Register Select"]
    #[inline(always)]
    pub fn dma0rsel(&mut self) -> Dma0rselW<'_, CtrlSpec> {
        Dma0rselW::new(self, 20)
    }
    #[doc = "Bits 24:25 - DMA1 Read Mode"]
    #[inline(always)]
    pub fn dma1mode(&mut self) -> Dma1modeW<'_, CtrlSpec> {
        Dma1modeW::new(self, 24)
    }
    #[doc = "Bits 28:29 - DATA0 DMA Unaligned Read Register Select"]
    #[inline(always)]
    pub fn dma1rsel(&mut self) -> Dma1rselW<'_, CtrlSpec> {
        Dma1rselW::new(self, 28)
    }
    #[doc = "Bit 31 - Combined Data0 Write DMA Request"]
    #[inline(always)]
    pub fn combdma0wereq(&mut self) -> Combdma0wereqW<'_, CtrlSpec> {
        Combdma0wereqW::new(self, 31)
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
