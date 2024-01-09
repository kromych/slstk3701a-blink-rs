#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `AES` reader - AES Mode"]
pub type AES_R = crate::BitReader;
#[doc = "Field `AES` writer - AES Mode"]
pub type AES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYBUFDIS` reader - Key Buffer Disable"]
pub type KEYBUFDIS_R = crate::BitReader;
#[doc = "Field `KEYBUFDIS` writer - Key Buffer Disable"]
pub type KEYBUFDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHA` reader - SHA Mode"]
pub type SHA_R = crate::BitReader;
#[doc = "Field `SHA` writer - SHA Mode"]
pub type SHA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOBUSYSTALL` reader - No Stalling of Bus When Busy"]
pub type NOBUSYSTALL_R = crate::BitReader;
#[doc = "Field `NOBUSYSTALL` writer - No Stalling of Bus When Busy"]
pub type NOBUSYSTALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INCWIDTH` reader - Increment Width"]
pub type INCWIDTH_R = crate::FieldReader<INCWIDTH_A>;
#[doc = "Increment Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INCWIDTH_A {
    #[doc = "0: Byte 15 in DATA1 is used for the increment function."]
    INCWIDTH1 = 0,
    #[doc = "1: Bytes 14 and 15 in DATA1 are used for the increment function."]
    INCWIDTH2 = 1,
    #[doc = "2: Bytes 13 to 15 in DATA1 are used for the increment function."]
    INCWIDTH3 = 2,
    #[doc = "3: Bytes 12 to 15 in DATA1 are used for the increment function."]
    INCWIDTH4 = 3,
}
impl From<INCWIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: INCWIDTH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INCWIDTH_A {
    type Ux = u8;
}
impl INCWIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INCWIDTH_A {
        match self.bits {
            0 => INCWIDTH_A::INCWIDTH1,
            1 => INCWIDTH_A::INCWIDTH2,
            2 => INCWIDTH_A::INCWIDTH3,
            3 => INCWIDTH_A::INCWIDTH4,
            _ => unreachable!(),
        }
    }
    #[doc = "Byte 15 in DATA1 is used for the increment function."]
    #[inline(always)]
    pub fn is_incwidth1(&self) -> bool {
        *self == INCWIDTH_A::INCWIDTH1
    }
    #[doc = "Bytes 14 and 15 in DATA1 are used for the increment function."]
    #[inline(always)]
    pub fn is_incwidth2(&self) -> bool {
        *self == INCWIDTH_A::INCWIDTH2
    }
    #[doc = "Bytes 13 to 15 in DATA1 are used for the increment function."]
    #[inline(always)]
    pub fn is_incwidth3(&self) -> bool {
        *self == INCWIDTH_A::INCWIDTH3
    }
    #[doc = "Bytes 12 to 15 in DATA1 are used for the increment function."]
    #[inline(always)]
    pub fn is_incwidth4(&self) -> bool {
        *self == INCWIDTH_A::INCWIDTH4
    }
}
#[doc = "Field `INCWIDTH` writer - Increment Width"]
pub type INCWIDTH_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, INCWIDTH_A>;
impl<'a, REG> INCWIDTH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Byte 15 in DATA1 is used for the increment function."]
    #[inline(always)]
    pub fn incwidth1(self) -> &'a mut crate::W<REG> {
        self.variant(INCWIDTH_A::INCWIDTH1)
    }
    #[doc = "Bytes 14 and 15 in DATA1 are used for the increment function."]
    #[inline(always)]
    pub fn incwidth2(self) -> &'a mut crate::W<REG> {
        self.variant(INCWIDTH_A::INCWIDTH2)
    }
    #[doc = "Bytes 13 to 15 in DATA1 are used for the increment function."]
    #[inline(always)]
    pub fn incwidth3(self) -> &'a mut crate::W<REG> {
        self.variant(INCWIDTH_A::INCWIDTH3)
    }
    #[doc = "Bytes 12 to 15 in DATA1 are used for the increment function."]
    #[inline(always)]
    pub fn incwidth4(self) -> &'a mut crate::W<REG> {
        self.variant(INCWIDTH_A::INCWIDTH4)
    }
}
#[doc = "Field `DMA0MODE` reader - DMA0 Read Mode"]
pub type DMA0MODE_R = crate::FieldReader<DMA0MODE_A>;
#[doc = "DMA0 Read Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMA0MODE_A {
    #[doc = "0: Target register is fully read/written during every DMA transaction"]
    FULL = 0,
    #[doc = "1: Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + necessary zero padding is read. Zero padding is automatically added when writing."]
    LENLIMIT = 1,
    #[doc = "2: Target register is fully read/written during every DMA transaction. Bytewise DMA."]
    FULLBYTE = 2,
    #[doc = "3: Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + necessary zero padding is read. Bytewise DMA. Zero padding is automatically added when writing."]
    LENLIMITBYTE = 3,
}
impl From<DMA0MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA0MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DMA0MODE_A {
    type Ux = u8;
}
impl DMA0MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA0MODE_A {
        match self.bits {
            0 => DMA0MODE_A::FULL,
            1 => DMA0MODE_A::LENLIMIT,
            2 => DMA0MODE_A::FULLBYTE,
            3 => DMA0MODE_A::LENLIMITBYTE,
            _ => unreachable!(),
        }
    }
    #[doc = "Target register is fully read/written during every DMA transaction"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == DMA0MODE_A::FULL
    }
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + necessary zero padding is read. Zero padding is automatically added when writing."]
    #[inline(always)]
    pub fn is_lenlimit(&self) -> bool {
        *self == DMA0MODE_A::LENLIMIT
    }
    #[doc = "Target register is fully read/written during every DMA transaction. Bytewise DMA."]
    #[inline(always)]
    pub fn is_fullbyte(&self) -> bool {
        *self == DMA0MODE_A::FULLBYTE
    }
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + necessary zero padding is read. Bytewise DMA. Zero padding is automatically added when writing."]
    #[inline(always)]
    pub fn is_lenlimitbyte(&self) -> bool {
        *self == DMA0MODE_A::LENLIMITBYTE
    }
}
#[doc = "Field `DMA0MODE` writer - DMA0 Read Mode"]
pub type DMA0MODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DMA0MODE_A>;
impl<'a, REG> DMA0MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Target register is fully read/written during every DMA transaction"]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0MODE_A::FULL)
    }
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + necessary zero padding is read. Zero padding is automatically added when writing."]
    #[inline(always)]
    pub fn lenlimit(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0MODE_A::LENLIMIT)
    }
    #[doc = "Target register is fully read/written during every DMA transaction. Bytewise DMA."]
    #[inline(always)]
    pub fn fullbyte(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0MODE_A::FULLBYTE)
    }
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + necessary zero padding is read. Bytewise DMA. Zero padding is automatically added when writing."]
    #[inline(always)]
    pub fn lenlimitbyte(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0MODE_A::LENLIMITBYTE)
    }
}
#[doc = "Field `DMA0RSEL` reader - DMA0 Read Register Select"]
pub type DMA0RSEL_R = crate::FieldReader<DMA0RSEL_A>;
#[doc = "DMA0 Read Register Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMA0RSEL_A {
    #[doc = "0: `0`"]
    DATA0 = 0,
    #[doc = "1: `1`"]
    DDATA0 = 1,
    #[doc = "2: `10`"]
    DDATA0BIG = 2,
    #[doc = "3: `11`"]
    QDATA0 = 3,
}
impl From<DMA0RSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA0RSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DMA0RSEL_A {
    type Ux = u8;
}
impl DMA0RSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA0RSEL_A {
        match self.bits {
            0 => DMA0RSEL_A::DATA0,
            1 => DMA0RSEL_A::DDATA0,
            2 => DMA0RSEL_A::DDATA0BIG,
            3 => DMA0RSEL_A::QDATA0,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_data0(&self) -> bool {
        *self == DMA0RSEL_A::DATA0
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_ddata0(&self) -> bool {
        *self == DMA0RSEL_A::DDATA0
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_ddata0big(&self) -> bool {
        *self == DMA0RSEL_A::DDATA0BIG
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_qdata0(&self) -> bool {
        *self == DMA0RSEL_A::QDATA0
    }
}
#[doc = "Field `DMA0RSEL` writer - DMA0 Read Register Select"]
pub type DMA0RSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DMA0RSEL_A>;
impl<'a, REG> DMA0RSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn data0(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0RSEL_A::DATA0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn ddata0(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0RSEL_A::DDATA0)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn ddata0big(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0RSEL_A::DDATA0BIG)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn qdata0(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0RSEL_A::QDATA0)
    }
}
#[doc = "Field `DMA1MODE` reader - DMA1 Read Mode"]
pub type DMA1MODE_R = crate::FieldReader<DMA1MODE_A>;
#[doc = "DMA1 Read Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMA1MODE_A {
    #[doc = "0: Target register is fully read/written during every DMA transaction"]
    FULL = 0,
    #[doc = "1: Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + 1 bytes + necessary zero padding is read. Zero padding is automatically added when writing."]
    LENLIMIT = 1,
    #[doc = "2: Target register is fully read/written during every DMA transaction. Bytewise DMA."]
    FULLBYTE = 2,
    #[doc = "3: Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + 1 bytes + necessary zero padding is read. Bytewise DMA. Zero padding is automatically added when writing."]
    LENLIMITBYTE = 3,
}
impl From<DMA1MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA1MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DMA1MODE_A {
    type Ux = u8;
}
impl DMA1MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA1MODE_A {
        match self.bits {
            0 => DMA1MODE_A::FULL,
            1 => DMA1MODE_A::LENLIMIT,
            2 => DMA1MODE_A::FULLBYTE,
            3 => DMA1MODE_A::LENLIMITBYTE,
            _ => unreachable!(),
        }
    }
    #[doc = "Target register is fully read/written during every DMA transaction"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == DMA1MODE_A::FULL
    }
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + 1 bytes + necessary zero padding is read. Zero padding is automatically added when writing."]
    #[inline(always)]
    pub fn is_lenlimit(&self) -> bool {
        *self == DMA1MODE_A::LENLIMIT
    }
    #[doc = "Target register is fully read/written during every DMA transaction. Bytewise DMA."]
    #[inline(always)]
    pub fn is_fullbyte(&self) -> bool {
        *self == DMA1MODE_A::FULLBYTE
    }
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + 1 bytes + necessary zero padding is read. Bytewise DMA. Zero padding is automatically added when writing."]
    #[inline(always)]
    pub fn is_lenlimitbyte(&self) -> bool {
        *self == DMA1MODE_A::LENLIMITBYTE
    }
}
#[doc = "Field `DMA1MODE` writer - DMA1 Read Mode"]
pub type DMA1MODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DMA1MODE_A>;
impl<'a, REG> DMA1MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Target register is fully read/written during every DMA transaction"]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1MODE_A::FULL)
    }
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + 1 bytes + necessary zero padding is read. Zero padding is automatically added when writing."]
    #[inline(always)]
    pub fn lenlimit(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1MODE_A::LENLIMIT)
    }
    #[doc = "Target register is fully read/written during every DMA transaction. Bytewise DMA."]
    #[inline(always)]
    pub fn fullbyte(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1MODE_A::FULLBYTE)
    }
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + 1 bytes + necessary zero padding is read. Bytewise DMA. Zero padding is automatically added when writing."]
    #[inline(always)]
    pub fn lenlimitbyte(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1MODE_A::LENLIMITBYTE)
    }
}
#[doc = "Field `DMA1RSEL` reader - DATA0 DMA Unaligned Read Register Select"]
pub type DMA1RSEL_R = crate::FieldReader<DMA1RSEL_A>;
#[doc = "DATA0 DMA Unaligned Read Register Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMA1RSEL_A {
    #[doc = "0: `0`"]
    DATA1 = 0,
    #[doc = "1: `1`"]
    DDATA1 = 1,
    #[doc = "2: `10`"]
    QDATA1 = 2,
    #[doc = "3: `11`"]
    QDATA1BIG = 3,
}
impl From<DMA1RSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA1RSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DMA1RSEL_A {
    type Ux = u8;
}
impl DMA1RSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA1RSEL_A {
        match self.bits {
            0 => DMA1RSEL_A::DATA1,
            1 => DMA1RSEL_A::DDATA1,
            2 => DMA1RSEL_A::QDATA1,
            3 => DMA1RSEL_A::QDATA1BIG,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        *self == DMA1RSEL_A::DATA1
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_ddata1(&self) -> bool {
        *self == DMA1RSEL_A::DDATA1
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_qdata1(&self) -> bool {
        *self == DMA1RSEL_A::QDATA1
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_qdata1big(&self) -> bool {
        *self == DMA1RSEL_A::QDATA1BIG
    }
}
#[doc = "Field `DMA1RSEL` writer - DATA0 DMA Unaligned Read Register Select"]
pub type DMA1RSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DMA1RSEL_A>;
impl<'a, REG> DMA1RSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn data1(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1RSEL_A::DATA1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn ddata1(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1RSEL_A::DDATA1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn qdata1(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1RSEL_A::QDATA1)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn qdata1big(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1RSEL_A::QDATA1BIG)
    }
}
#[doc = "Field `COMBDMA0WEREQ` reader - Combined Data0 Write DMA Request"]
pub type COMBDMA0WEREQ_R = crate::BitReader;
#[doc = "Field `COMBDMA0WEREQ` writer - Combined Data0 Write DMA Request"]
pub type COMBDMA0WEREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AES Mode"]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Key Buffer Disable"]
    #[inline(always)]
    pub fn keybufdis(&self) -> KEYBUFDIS_R {
        KEYBUFDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SHA Mode"]
    #[inline(always)]
    pub fn sha(&self) -> SHA_R {
        SHA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 10 - No Stalling of Bus When Busy"]
    #[inline(always)]
    pub fn nobusystall(&self) -> NOBUSYSTALL_R {
        NOBUSYSTALL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Increment Width"]
    #[inline(always)]
    pub fn incwidth(&self) -> INCWIDTH_R {
        INCWIDTH_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - DMA0 Read Mode"]
    #[inline(always)]
    pub fn dma0mode(&self) -> DMA0MODE_R {
        DMA0MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - DMA0 Read Register Select"]
    #[inline(always)]
    pub fn dma0rsel(&self) -> DMA0RSEL_R {
        DMA0RSEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - DMA1 Read Mode"]
    #[inline(always)]
    pub fn dma1mode(&self) -> DMA1MODE_R {
        DMA1MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - DATA0 DMA Unaligned Read Register Select"]
    #[inline(always)]
    pub fn dma1rsel(&self) -> DMA1RSEL_R {
        DMA1RSEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - Combined Data0 Write DMA Request"]
    #[inline(always)]
    pub fn combdma0wereq(&self) -> COMBDMA0WEREQ_R {
        COMBDMA0WEREQ_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AES Mode"]
    #[inline(always)]
    #[must_use]
    pub fn aes(&mut self) -> AES_W<CTRL_SPEC> {
        AES_W::new(self, 0)
    }
    #[doc = "Bit 1 - Key Buffer Disable"]
    #[inline(always)]
    #[must_use]
    pub fn keybufdis(&mut self) -> KEYBUFDIS_W<CTRL_SPEC> {
        KEYBUFDIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - SHA Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sha(&mut self) -> SHA_W<CTRL_SPEC> {
        SHA_W::new(self, 2)
    }
    #[doc = "Bit 10 - No Stalling of Bus When Busy"]
    #[inline(always)]
    #[must_use]
    pub fn nobusystall(&mut self) -> NOBUSYSTALL_W<CTRL_SPEC> {
        NOBUSYSTALL_W::new(self, 10)
    }
    #[doc = "Bits 14:15 - Increment Width"]
    #[inline(always)]
    #[must_use]
    pub fn incwidth(&mut self) -> INCWIDTH_W<CTRL_SPEC> {
        INCWIDTH_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - DMA0 Read Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dma0mode(&mut self) -> DMA0MODE_W<CTRL_SPEC> {
        DMA0MODE_W::new(self, 16)
    }
    #[doc = "Bits 20:21 - DMA0 Read Register Select"]
    #[inline(always)]
    #[must_use]
    pub fn dma0rsel(&mut self) -> DMA0RSEL_W<CTRL_SPEC> {
        DMA0RSEL_W::new(self, 20)
    }
    #[doc = "Bits 24:25 - DMA1 Read Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dma1mode(&mut self) -> DMA1MODE_W<CTRL_SPEC> {
        DMA1MODE_W::new(self, 24)
    }
    #[doc = "Bits 28:29 - DATA0 DMA Unaligned Read Register Select"]
    #[inline(always)]
    #[must_use]
    pub fn dma1rsel(&mut self) -> DMA1RSEL_W<CTRL_SPEC> {
        DMA1RSEL_W::new(self, 28)
    }
    #[doc = "Bit 31 - Combined Data0 Write DMA Request"]
    #[inline(always)]
    #[must_use]
    pub fn combdma0wereq(&mut self) -> COMBDMA0WEREQ_W<CTRL_SPEC> {
        COMBDMA0WEREQ_W::new(self, 31)
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
