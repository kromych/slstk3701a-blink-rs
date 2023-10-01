#[doc = "Register `CFG0` reader"]
pub type R = crate::R<CFG0_SPEC>;
#[doc = "Register `CFG0` writer"]
pub type W = crate::W<CFG0_SPEC>;
#[doc = "Field `TUNINGCNT` reader - Tuning Counter Value"]
pub type TUNINGCNT_R = crate::FieldReader;
#[doc = "Field `TUNINGCNT` writer - Tuning Counter Value"]
pub type TUNINGCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `TOUTCLKFREQ` reader - Timeout Clock Frequency"]
pub type TOUTCLKFREQ_R = crate::FieldReader;
#[doc = "Field `TOUTCLKFREQ` writer - Timeout Clock Frequency"]
pub type TOUTCLKFREQ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `TOUTCLKUNIT` reader - Timeout Clock Unit in kHz or MHz"]
pub type TOUTCLKUNIT_R = crate::BitReader;
#[doc = "Field `TOUTCLKUNIT` writer - Timeout Clock Unit in kHz or MHz"]
pub type TOUTCLKUNIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BASECLKFREQ` reader - Base Clock Frequency for SD_CLK"]
pub type BASECLKFREQ_R = crate::FieldReader;
#[doc = "Field `BASECLKFREQ` writer - Base Clock Frequency for SD_CLK"]
pub type BASECLKFREQ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `MAXBLKLEN` reader - MAX Block Length of Transfer"]
pub type MAXBLKLEN_R = crate::FieldReader<MAXBLKLEN_A>;
#[doc = "MAX Block Length of Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MAXBLKLEN_A {
    #[doc = "0: 512 Bytes are Selected"]
    _512B = 0,
    #[doc = "1: 1024 Bytes are Selected"]
    _1024B = 1,
    #[doc = "2: 2048 Bytes are Selected"]
    _2048B = 2,
}
impl From<MAXBLKLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: MAXBLKLEN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MAXBLKLEN_A {
    type Ux = u8;
}
impl MAXBLKLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MAXBLKLEN_A> {
        match self.bits {
            0 => Some(MAXBLKLEN_A::_512B),
            1 => Some(MAXBLKLEN_A::_1024B),
            2 => Some(MAXBLKLEN_A::_2048B),
            _ => None,
        }
    }
    #[doc = "512 Bytes are Selected"]
    #[inline(always)]
    pub fn is_512b(&self) -> bool {
        *self == MAXBLKLEN_A::_512B
    }
    #[doc = "1024 Bytes are Selected"]
    #[inline(always)]
    pub fn is_1024b(&self) -> bool {
        *self == MAXBLKLEN_A::_1024B
    }
    #[doc = "2048 Bytes are Selected"]
    #[inline(always)]
    pub fn is_2048b(&self) -> bool {
        *self == MAXBLKLEN_A::_2048B
    }
}
#[doc = "Field `MAXBLKLEN` writer - MAX Block Length of Transfer"]
pub type MAXBLKLEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, MAXBLKLEN_A>;
impl<'a, REG, const O: u8> MAXBLKLEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "512 Bytes are Selected"]
    #[inline(always)]
    pub fn _512b(self) -> &'a mut crate::W<REG> {
        self.variant(MAXBLKLEN_A::_512B)
    }
    #[doc = "1024 Bytes are Selected"]
    #[inline(always)]
    pub fn _1024b(self) -> &'a mut crate::W<REG> {
        self.variant(MAXBLKLEN_A::_1024B)
    }
    #[doc = "2048 Bytes are Selected"]
    #[inline(always)]
    pub fn _2048b(self) -> &'a mut crate::W<REG> {
        self.variant(MAXBLKLEN_A::_2048B)
    }
}
#[doc = "Field `C8BITSUP` reader - 8-bit Interface Support"]
pub type C8BITSUP_R = crate::BitReader;
#[doc = "Field `C8BITSUP` writer - 8-bit Interface Support"]
pub type C8BITSUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CADMA2SUP` reader - ADMA2 Mode Support"]
pub type CADMA2SUP_R = crate::BitReader;
#[doc = "Field `CADMA2SUP` writer - ADMA2 Mode Support"]
pub type CADMA2SUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHSSUP` reader - High Speed Mode Support"]
pub type CHSSUP_R = crate::BitReader;
#[doc = "Field `CHSSUP` writer - High Speed Mode Support"]
pub type CHSSUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CSDMASUP` reader - SDMA Mode Support"]
pub type CSDMASUP_R = crate::BitReader;
#[doc = "Field `CSDMASUP` writer - SDMA Mode Support"]
pub type CSDMASUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CSUSPRESSUP` reader - Suspend/Resume Support"]
pub type CSUSPRESSUP_R = crate::BitReader;
#[doc = "Field `CSUSPRESSUP` writer - Suspend/Resume Support"]
pub type CSUSPRESSUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C3P3VSUP` reader - Core 3P3V Support"]
pub type C3P3VSUP_R = crate::BitReader;
#[doc = "Field `C3P3VSUP` writer - Core 3P3V Support"]
pub type C3P3VSUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C3P0VSUP` reader - 3P0V Support"]
pub type C3P0VSUP_R = crate::BitReader;
#[doc = "Field `C3P0VSUP` writer - 3P0V Support"]
pub type C3P0VSUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C1P8VSUP` reader - 1P8V Support"]
pub type C1P8VSUP_R = crate::BitReader;
#[doc = "Field `C1P8VSUP` writer - 1P8V Support"]
pub type C1P8VSUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:5 - Tuning Counter Value"]
    #[inline(always)]
    pub fn tuningcnt(&self) -> TUNINGCNT_R {
        TUNINGCNT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Timeout Clock Frequency"]
    #[inline(always)]
    pub fn toutclkfreq(&self) -> TOUTCLKFREQ_R {
        TOUTCLKFREQ_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bit 12 - Timeout Clock Unit in kHz or MHz"]
    #[inline(always)]
    pub fn toutclkunit(&self) -> TOUTCLKUNIT_R {
        TOUTCLKUNIT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:20 - Base Clock Frequency for SD_CLK"]
    #[inline(always)]
    pub fn baseclkfreq(&self) -> BASECLKFREQ_R {
        BASECLKFREQ_R::new(((self.bits >> 13) & 0xff) as u8)
    }
    #[doc = "Bits 21:22 - MAX Block Length of Transfer"]
    #[inline(always)]
    pub fn maxblklen(&self) -> MAXBLKLEN_R {
        MAXBLKLEN_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - 8-bit Interface Support"]
    #[inline(always)]
    pub fn c8bitsup(&self) -> C8BITSUP_R {
        C8BITSUP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - ADMA2 Mode Support"]
    #[inline(always)]
    pub fn cadma2sup(&self) -> CADMA2SUP_R {
        CADMA2SUP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - High Speed Mode Support"]
    #[inline(always)]
    pub fn chssup(&self) -> CHSSUP_R {
        CHSSUP_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - SDMA Mode Support"]
    #[inline(always)]
    pub fn csdmasup(&self) -> CSDMASUP_R {
        CSDMASUP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Suspend/Resume Support"]
    #[inline(always)]
    pub fn csuspressup(&self) -> CSUSPRESSUP_R {
        CSUSPRESSUP_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Core 3P3V Support"]
    #[inline(always)]
    pub fn c3p3vsup(&self) -> C3P3VSUP_R {
        C3P3VSUP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 3P0V Support"]
    #[inline(always)]
    pub fn c3p0vsup(&self) -> C3P0VSUP_R {
        C3P0VSUP_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1P8V Support"]
    #[inline(always)]
    pub fn c1p8vsup(&self) -> C1P8VSUP_R {
        C1P8VSUP_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Tuning Counter Value"]
    #[inline(always)]
    #[must_use]
    pub fn tuningcnt(&mut self) -> TUNINGCNT_W<CFG0_SPEC, 0> {
        TUNINGCNT_W::new(self)
    }
    #[doc = "Bits 6:11 - Timeout Clock Frequency"]
    #[inline(always)]
    #[must_use]
    pub fn toutclkfreq(&mut self) -> TOUTCLKFREQ_W<CFG0_SPEC, 6> {
        TOUTCLKFREQ_W::new(self)
    }
    #[doc = "Bit 12 - Timeout Clock Unit in kHz or MHz"]
    #[inline(always)]
    #[must_use]
    pub fn toutclkunit(&mut self) -> TOUTCLKUNIT_W<CFG0_SPEC, 12> {
        TOUTCLKUNIT_W::new(self)
    }
    #[doc = "Bits 13:20 - Base Clock Frequency for SD_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn baseclkfreq(&mut self) -> BASECLKFREQ_W<CFG0_SPEC, 13> {
        BASECLKFREQ_W::new(self)
    }
    #[doc = "Bits 21:22 - MAX Block Length of Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn maxblklen(&mut self) -> MAXBLKLEN_W<CFG0_SPEC, 21> {
        MAXBLKLEN_W::new(self)
    }
    #[doc = "Bit 23 - 8-bit Interface Support"]
    #[inline(always)]
    #[must_use]
    pub fn c8bitsup(&mut self) -> C8BITSUP_W<CFG0_SPEC, 23> {
        C8BITSUP_W::new(self)
    }
    #[doc = "Bit 24 - ADMA2 Mode Support"]
    #[inline(always)]
    #[must_use]
    pub fn cadma2sup(&mut self) -> CADMA2SUP_W<CFG0_SPEC, 24> {
        CADMA2SUP_W::new(self)
    }
    #[doc = "Bit 25 - High Speed Mode Support"]
    #[inline(always)]
    #[must_use]
    pub fn chssup(&mut self) -> CHSSUP_W<CFG0_SPEC, 25> {
        CHSSUP_W::new(self)
    }
    #[doc = "Bit 26 - SDMA Mode Support"]
    #[inline(always)]
    #[must_use]
    pub fn csdmasup(&mut self) -> CSDMASUP_W<CFG0_SPEC, 26> {
        CSDMASUP_W::new(self)
    }
    #[doc = "Bit 27 - Suspend/Resume Support"]
    #[inline(always)]
    #[must_use]
    pub fn csuspressup(&mut self) -> CSUSPRESSUP_W<CFG0_SPEC, 27> {
        CSUSPRESSUP_W::new(self)
    }
    #[doc = "Bit 28 - Core 3P3V Support"]
    #[inline(always)]
    #[must_use]
    pub fn c3p3vsup(&mut self) -> C3P3VSUP_W<CFG0_SPEC, 28> {
        C3P3VSUP_W::new(self)
    }
    #[doc = "Bit 29 - 3P0V Support"]
    #[inline(always)]
    #[must_use]
    pub fn c3p0vsup(&mut self) -> C3P0VSUP_W<CFG0_SPEC, 29> {
        C3P0VSUP_W::new(self)
    }
    #[doc = "Bit 30 - 1P8V Support"]
    #[inline(always)]
    #[must_use]
    pub fn c1p8vsup(&mut self) -> C1P8VSUP_W<CFG0_SPEC, 30> {
        C1P8VSUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Core Configuration 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG0_SPEC;
impl crate::RegisterSpec for CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0::R`](R) reader structure"]
impl crate::Readable for CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg0::W`](W) writer structure"]
impl crate::Writable for CFG0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG0 to value 0"]
impl crate::Resettable for CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
