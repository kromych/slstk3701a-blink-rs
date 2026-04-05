#[doc = "Register `BLKSIZE` reader"]
pub type R = crate::R<BlksizeSpec>;
#[doc = "Register `BLKSIZE` writer"]
pub type W = crate::W<BlksizeSpec>;
#[doc = "Transfer Block Size, Specifies the Block Size for Block Data Transfers for CMD17, CMD18, CMD24, CMD25, and CMD53\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Tfrblksize {
    #[doc = "0: `0`"]
    Noxfer = 0,
}
impl From<Tfrblksize> for u16 {
    #[inline(always)]
    fn from(variant: Tfrblksize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tfrblksize {
    type Ux = u16;
}
impl crate::IsEnum for Tfrblksize {}
#[doc = "Field `TFRBLKSIZE` reader - Transfer Block Size, Specifies the Block Size for Block Data Transfers for CMD17, CMD18, CMD24, CMD25, and CMD53"]
pub type TfrblksizeR = crate::FieldReader<Tfrblksize>;
impl TfrblksizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tfrblksize> {
        match self.bits {
            0 => Some(Tfrblksize::Noxfer),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_noxfer(&self) -> bool {
        *self == Tfrblksize::Noxfer
    }
}
#[doc = "Field `TFRBLKSIZE` writer - Transfer Block Size, Specifies the Block Size for Block Data Transfers for CMD17, CMD18, CMD24, CMD25, and CMD53"]
pub type TfrblksizeW<'a, REG> = crate::FieldWriter<'a, REG, 12, Tfrblksize>;
impl<'a, REG> TfrblksizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn noxfer(self) -> &'a mut crate::W<REG> {
        self.variant(Tfrblksize::Noxfer)
    }
}
#[doc = "Host SDMA Buffer Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hstsdmabufsize {
    #[doc = "0: 4KB(Detects A11 Carry out)"]
    Size4 = 0,
    #[doc = "1: 8KB(Detects A12 Carry out)"]
    Size8 = 1,
    #[doc = "2: 16KB(Detects A13 Carry out)"]
    Size16 = 2,
    #[doc = "3: 32KB(Detects A14 Carry out)"]
    Size32 = 3,
    #[doc = "4: 64KB(Detects A15 Carry out)"]
    Size64 = 4,
    #[doc = "5: 128KB(Detects A16 Carry out)"]
    Size128 = 5,
    #[doc = "6: 256KB(Detects A17 Carry out)"]
    Size256 = 6,
    #[doc = "7: 512KB(Detects A18 Carry out)"]
    Size512 = 7,
}
impl From<Hstsdmabufsize> for u8 {
    #[inline(always)]
    fn from(variant: Hstsdmabufsize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hstsdmabufsize {
    type Ux = u8;
}
impl crate::IsEnum for Hstsdmabufsize {}
#[doc = "Field `HSTSDMABUFSIZE` reader - Host SDMA Buffer Size"]
pub type HstsdmabufsizeR = crate::FieldReader<Hstsdmabufsize>;
impl HstsdmabufsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hstsdmabufsize {
        match self.bits {
            0 => Hstsdmabufsize::Size4,
            1 => Hstsdmabufsize::Size8,
            2 => Hstsdmabufsize::Size16,
            3 => Hstsdmabufsize::Size32,
            4 => Hstsdmabufsize::Size64,
            5 => Hstsdmabufsize::Size128,
            6 => Hstsdmabufsize::Size256,
            7 => Hstsdmabufsize::Size512,
            _ => unreachable!(),
        }
    }
    #[doc = "4KB(Detects A11 Carry out)"]
    #[inline(always)]
    pub fn is_size4(&self) -> bool {
        *self == Hstsdmabufsize::Size4
    }
    #[doc = "8KB(Detects A12 Carry out)"]
    #[inline(always)]
    pub fn is_size8(&self) -> bool {
        *self == Hstsdmabufsize::Size8
    }
    #[doc = "16KB(Detects A13 Carry out)"]
    #[inline(always)]
    pub fn is_size16(&self) -> bool {
        *self == Hstsdmabufsize::Size16
    }
    #[doc = "32KB(Detects A14 Carry out)"]
    #[inline(always)]
    pub fn is_size32(&self) -> bool {
        *self == Hstsdmabufsize::Size32
    }
    #[doc = "64KB(Detects A15 Carry out)"]
    #[inline(always)]
    pub fn is_size64(&self) -> bool {
        *self == Hstsdmabufsize::Size64
    }
    #[doc = "128KB(Detects A16 Carry out)"]
    #[inline(always)]
    pub fn is_size128(&self) -> bool {
        *self == Hstsdmabufsize::Size128
    }
    #[doc = "256KB(Detects A17 Carry out)"]
    #[inline(always)]
    pub fn is_size256(&self) -> bool {
        *self == Hstsdmabufsize::Size256
    }
    #[doc = "512KB(Detects A18 Carry out)"]
    #[inline(always)]
    pub fn is_size512(&self) -> bool {
        *self == Hstsdmabufsize::Size512
    }
}
#[doc = "Field `HSTSDMABUFSIZE` writer - Host SDMA Buffer Size"]
pub type HstsdmabufsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Hstsdmabufsize, crate::Safe>;
impl<'a, REG> HstsdmabufsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4KB(Detects A11 Carry out)"]
    #[inline(always)]
    pub fn size4(self) -> &'a mut crate::W<REG> {
        self.variant(Hstsdmabufsize::Size4)
    }
    #[doc = "8KB(Detects A12 Carry out)"]
    #[inline(always)]
    pub fn size8(self) -> &'a mut crate::W<REG> {
        self.variant(Hstsdmabufsize::Size8)
    }
    #[doc = "16KB(Detects A13 Carry out)"]
    #[inline(always)]
    pub fn size16(self) -> &'a mut crate::W<REG> {
        self.variant(Hstsdmabufsize::Size16)
    }
    #[doc = "32KB(Detects A14 Carry out)"]
    #[inline(always)]
    pub fn size32(self) -> &'a mut crate::W<REG> {
        self.variant(Hstsdmabufsize::Size32)
    }
    #[doc = "64KB(Detects A15 Carry out)"]
    #[inline(always)]
    pub fn size64(self) -> &'a mut crate::W<REG> {
        self.variant(Hstsdmabufsize::Size64)
    }
    #[doc = "128KB(Detects A16 Carry out)"]
    #[inline(always)]
    pub fn size128(self) -> &'a mut crate::W<REG> {
        self.variant(Hstsdmabufsize::Size128)
    }
    #[doc = "256KB(Detects A17 Carry out)"]
    #[inline(always)]
    pub fn size256(self) -> &'a mut crate::W<REG> {
        self.variant(Hstsdmabufsize::Size256)
    }
    #[doc = "512KB(Detects A18 Carry out)"]
    #[inline(always)]
    pub fn size512(self) -> &'a mut crate::W<REG> {
        self.variant(Hstsdmabufsize::Size512)
    }
}
#[doc = "Blocks Count for Current Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Blkscntforcurrtfr {
    #[doc = "0: `0`"]
    Stopcnt = 0,
}
impl From<Blkscntforcurrtfr> for u16 {
    #[inline(always)]
    fn from(variant: Blkscntforcurrtfr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Blkscntforcurrtfr {
    type Ux = u16;
}
impl crate::IsEnum for Blkscntforcurrtfr {}
#[doc = "Field `BLKSCNTFORCURRTFR` reader - Blocks Count for Current Transfer"]
pub type BlkscntforcurrtfrR = crate::FieldReader<Blkscntforcurrtfr>;
impl BlkscntforcurrtfrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Blkscntforcurrtfr> {
        match self.bits {
            0 => Some(Blkscntforcurrtfr::Stopcnt),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_stopcnt(&self) -> bool {
        *self == Blkscntforcurrtfr::Stopcnt
    }
}
#[doc = "Field `BLKSCNTFORCURRTFR` writer - Blocks Count for Current Transfer"]
pub type BlkscntforcurrtfrW<'a, REG> = crate::FieldWriter<'a, REG, 16, Blkscntforcurrtfr>;
impl<'a, REG> BlkscntforcurrtfrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn stopcnt(self) -> &'a mut crate::W<REG> {
        self.variant(Blkscntforcurrtfr::Stopcnt)
    }
}
impl R {
    #[doc = "Bits 0:11 - Transfer Block Size, Specifies the Block Size for Block Data Transfers for CMD17, CMD18, CMD24, CMD25, and CMD53"]
    #[inline(always)]
    pub fn tfrblksize(&self) -> TfrblksizeR {
        TfrblksizeR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:14 - Host SDMA Buffer Size"]
    #[inline(always)]
    pub fn hstsdmabufsize(&self) -> HstsdmabufsizeR {
        HstsdmabufsizeR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:31 - Blocks Count for Current Transfer"]
    #[inline(always)]
    pub fn blkscntforcurrtfr(&self) -> BlkscntforcurrtfrR {
        BlkscntforcurrtfrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Transfer Block Size, Specifies the Block Size for Block Data Transfers for CMD17, CMD18, CMD24, CMD25, and CMD53"]
    #[inline(always)]
    pub fn tfrblksize(&mut self) -> TfrblksizeW<'_, BlksizeSpec> {
        TfrblksizeW::new(self, 0)
    }
    #[doc = "Bits 12:14 - Host SDMA Buffer Size"]
    #[inline(always)]
    pub fn hstsdmabufsize(&mut self) -> HstsdmabufsizeW<'_, BlksizeSpec> {
        HstsdmabufsizeW::new(self, 12)
    }
    #[doc = "Bits 16:31 - Blocks Count for Current Transfer"]
    #[inline(always)]
    pub fn blkscntforcurrtfr(&mut self) -> BlkscntforcurrtfrW<'_, BlksizeSpec> {
        BlkscntforcurrtfrW::new(self, 16)
    }
}
#[doc = "Block Size and Block Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`blksize::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blksize::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlksizeSpec;
impl crate::RegisterSpec for BlksizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blksize::R`](R) reader structure"]
impl crate::Readable for BlksizeSpec {}
#[doc = "`write(|w| ..)` method takes [`blksize::W`](W) writer structure"]
impl crate::Writable for BlksizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLKSIZE to value 0"]
impl crate::Resettable for BlksizeSpec {}
