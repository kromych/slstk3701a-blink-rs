#[doc = "Register `BLKSIZE` reader"]
pub type R = crate::R<BLKSIZE_SPEC>;
#[doc = "Register `BLKSIZE` writer"]
pub type W = crate::W<BLKSIZE_SPEC>;
#[doc = "Field `TFRBLKSIZE` reader - Transfer Block Size, Specifies the Block Size for Block Data Transfers for CMD17, CMD18, CMD24, CMD25, and CMD53"]
pub type TFRBLKSIZE_R = crate::FieldReader<TFRBLKSIZE_A>;
#[doc = "Transfer Block Size, Specifies the Block Size for Block Data Transfers for CMD17, CMD18, CMD24, CMD25, and CMD53\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum TFRBLKSIZE_A {
    #[doc = "0: `0`"]
    NOXFER = 0,
}
impl From<TFRBLKSIZE_A> for u16 {
    #[inline(always)]
    fn from(variant: TFRBLKSIZE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TFRBLKSIZE_A {
    type Ux = u16;
}
impl TFRBLKSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TFRBLKSIZE_A> {
        match self.bits {
            0 => Some(TFRBLKSIZE_A::NOXFER),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_noxfer(&self) -> bool {
        *self == TFRBLKSIZE_A::NOXFER
    }
}
#[doc = "Field `TFRBLKSIZE` writer - Transfer Block Size, Specifies the Block Size for Block Data Transfers for CMD17, CMD18, CMD24, CMD25, and CMD53"]
pub type TFRBLKSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 12, TFRBLKSIZE_A>;
impl<'a, REG> TFRBLKSIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn noxfer(self) -> &'a mut crate::W<REG> {
        self.variant(TFRBLKSIZE_A::NOXFER)
    }
}
#[doc = "Field `HSTSDMABUFSIZE` reader - Host SDMA Buffer Size"]
pub type HSTSDMABUFSIZE_R = crate::FieldReader<HSTSDMABUFSIZE_A>;
#[doc = "Host SDMA Buffer Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HSTSDMABUFSIZE_A {
    #[doc = "0: 4KB(Detects A11 Carry out)"]
    SIZE4 = 0,
    #[doc = "1: 8KB(Detects A12 Carry out)"]
    SIZE8 = 1,
    #[doc = "2: 16KB(Detects A13 Carry out)"]
    SIZE16 = 2,
    #[doc = "3: 32KB(Detects A14 Carry out)"]
    SIZE32 = 3,
    #[doc = "4: 64KB(Detects A15 Carry out)"]
    SIZE64 = 4,
    #[doc = "5: 128KB(Detects A16 Carry out)"]
    SIZE128 = 5,
    #[doc = "6: 256KB(Detects A17 Carry out)"]
    SIZE256 = 6,
    #[doc = "7: 512KB(Detects A18 Carry out)"]
    SIZE512 = 7,
}
impl From<HSTSDMABUFSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: HSTSDMABUFSIZE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HSTSDMABUFSIZE_A {
    type Ux = u8;
}
impl HSTSDMABUFSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSTSDMABUFSIZE_A {
        match self.bits {
            0 => HSTSDMABUFSIZE_A::SIZE4,
            1 => HSTSDMABUFSIZE_A::SIZE8,
            2 => HSTSDMABUFSIZE_A::SIZE16,
            3 => HSTSDMABUFSIZE_A::SIZE32,
            4 => HSTSDMABUFSIZE_A::SIZE64,
            5 => HSTSDMABUFSIZE_A::SIZE128,
            6 => HSTSDMABUFSIZE_A::SIZE256,
            7 => HSTSDMABUFSIZE_A::SIZE512,
            _ => unreachable!(),
        }
    }
    #[doc = "4KB(Detects A11 Carry out)"]
    #[inline(always)]
    pub fn is_size4(&self) -> bool {
        *self == HSTSDMABUFSIZE_A::SIZE4
    }
    #[doc = "8KB(Detects A12 Carry out)"]
    #[inline(always)]
    pub fn is_size8(&self) -> bool {
        *self == HSTSDMABUFSIZE_A::SIZE8
    }
    #[doc = "16KB(Detects A13 Carry out)"]
    #[inline(always)]
    pub fn is_size16(&self) -> bool {
        *self == HSTSDMABUFSIZE_A::SIZE16
    }
    #[doc = "32KB(Detects A14 Carry out)"]
    #[inline(always)]
    pub fn is_size32(&self) -> bool {
        *self == HSTSDMABUFSIZE_A::SIZE32
    }
    #[doc = "64KB(Detects A15 Carry out)"]
    #[inline(always)]
    pub fn is_size64(&self) -> bool {
        *self == HSTSDMABUFSIZE_A::SIZE64
    }
    #[doc = "128KB(Detects A16 Carry out)"]
    #[inline(always)]
    pub fn is_size128(&self) -> bool {
        *self == HSTSDMABUFSIZE_A::SIZE128
    }
    #[doc = "256KB(Detects A17 Carry out)"]
    #[inline(always)]
    pub fn is_size256(&self) -> bool {
        *self == HSTSDMABUFSIZE_A::SIZE256
    }
    #[doc = "512KB(Detects A18 Carry out)"]
    #[inline(always)]
    pub fn is_size512(&self) -> bool {
        *self == HSTSDMABUFSIZE_A::SIZE512
    }
}
#[doc = "Field `HSTSDMABUFSIZE` writer - Host SDMA Buffer Size"]
pub type HSTSDMABUFSIZE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, HSTSDMABUFSIZE_A>;
impl<'a, REG> HSTSDMABUFSIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4KB(Detects A11 Carry out)"]
    #[inline(always)]
    pub fn size4(self) -> &'a mut crate::W<REG> {
        self.variant(HSTSDMABUFSIZE_A::SIZE4)
    }
    #[doc = "8KB(Detects A12 Carry out)"]
    #[inline(always)]
    pub fn size8(self) -> &'a mut crate::W<REG> {
        self.variant(HSTSDMABUFSIZE_A::SIZE8)
    }
    #[doc = "16KB(Detects A13 Carry out)"]
    #[inline(always)]
    pub fn size16(self) -> &'a mut crate::W<REG> {
        self.variant(HSTSDMABUFSIZE_A::SIZE16)
    }
    #[doc = "32KB(Detects A14 Carry out)"]
    #[inline(always)]
    pub fn size32(self) -> &'a mut crate::W<REG> {
        self.variant(HSTSDMABUFSIZE_A::SIZE32)
    }
    #[doc = "64KB(Detects A15 Carry out)"]
    #[inline(always)]
    pub fn size64(self) -> &'a mut crate::W<REG> {
        self.variant(HSTSDMABUFSIZE_A::SIZE64)
    }
    #[doc = "128KB(Detects A16 Carry out)"]
    #[inline(always)]
    pub fn size128(self) -> &'a mut crate::W<REG> {
        self.variant(HSTSDMABUFSIZE_A::SIZE128)
    }
    #[doc = "256KB(Detects A17 Carry out)"]
    #[inline(always)]
    pub fn size256(self) -> &'a mut crate::W<REG> {
        self.variant(HSTSDMABUFSIZE_A::SIZE256)
    }
    #[doc = "512KB(Detects A18 Carry out)"]
    #[inline(always)]
    pub fn size512(self) -> &'a mut crate::W<REG> {
        self.variant(HSTSDMABUFSIZE_A::SIZE512)
    }
}
#[doc = "Field `BLKSCNTFORCURRTFR` reader - Blocks Count for Current Transfer"]
pub type BLKSCNTFORCURRTFR_R = crate::FieldReader<BLKSCNTFORCURRTFR_A>;
#[doc = "Blocks Count for Current Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum BLKSCNTFORCURRTFR_A {
    #[doc = "0: `0`"]
    STOPCNT = 0,
}
impl From<BLKSCNTFORCURRTFR_A> for u16 {
    #[inline(always)]
    fn from(variant: BLKSCNTFORCURRTFR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BLKSCNTFORCURRTFR_A {
    type Ux = u16;
}
impl BLKSCNTFORCURRTFR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BLKSCNTFORCURRTFR_A> {
        match self.bits {
            0 => Some(BLKSCNTFORCURRTFR_A::STOPCNT),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_stopcnt(&self) -> bool {
        *self == BLKSCNTFORCURRTFR_A::STOPCNT
    }
}
#[doc = "Field `BLKSCNTFORCURRTFR` writer - Blocks Count for Current Transfer"]
pub type BLKSCNTFORCURRTFR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, BLKSCNTFORCURRTFR_A>;
impl<'a, REG> BLKSCNTFORCURRTFR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn stopcnt(self) -> &'a mut crate::W<REG> {
        self.variant(BLKSCNTFORCURRTFR_A::STOPCNT)
    }
}
impl R {
    #[doc = "Bits 0:11 - Transfer Block Size, Specifies the Block Size for Block Data Transfers for CMD17, CMD18, CMD24, CMD25, and CMD53"]
    #[inline(always)]
    pub fn tfrblksize(&self) -> TFRBLKSIZE_R {
        TFRBLKSIZE_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:14 - Host SDMA Buffer Size"]
    #[inline(always)]
    pub fn hstsdmabufsize(&self) -> HSTSDMABUFSIZE_R {
        HSTSDMABUFSIZE_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:31 - Blocks Count for Current Transfer"]
    #[inline(always)]
    pub fn blkscntforcurrtfr(&self) -> BLKSCNTFORCURRTFR_R {
        BLKSCNTFORCURRTFR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Transfer Block Size, Specifies the Block Size for Block Data Transfers for CMD17, CMD18, CMD24, CMD25, and CMD53"]
    #[inline(always)]
    #[must_use]
    pub fn tfrblksize(&mut self) -> TFRBLKSIZE_W<BLKSIZE_SPEC> {
        TFRBLKSIZE_W::new(self, 0)
    }
    #[doc = "Bits 12:14 - Host SDMA Buffer Size"]
    #[inline(always)]
    #[must_use]
    pub fn hstsdmabufsize(&mut self) -> HSTSDMABUFSIZE_W<BLKSIZE_SPEC> {
        HSTSDMABUFSIZE_W::new(self, 12)
    }
    #[doc = "Bits 16:31 - Blocks Count for Current Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn blkscntforcurrtfr(&mut self) -> BLKSCNTFORCURRTFR_W<BLKSIZE_SPEC> {
        BLKSCNTFORCURRTFR_W::new(self, 16)
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
#[doc = "Block Size and Block Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blksize::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blksize::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLKSIZE_SPEC;
impl crate::RegisterSpec for BLKSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blksize::R`](R) reader structure"]
impl crate::Readable for BLKSIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blksize::W`](W) writer structure"]
impl crate::Writable for BLKSIZE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLKSIZE to value 0"]
impl crate::Resettable for BLKSIZE_SPEC {
    const RESET_VALUE: u32 = 0;
}
