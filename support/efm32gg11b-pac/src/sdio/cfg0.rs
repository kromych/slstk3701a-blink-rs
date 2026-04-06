#[doc = "Register `CFG0` reader"]
pub type R = crate::R<Cfg0Spec>;
#[doc = "Register `CFG0` writer"]
pub type W = crate::W<Cfg0Spec>;
#[doc = "Field `TUNINGCNT` reader - Tuning Counter Value"]
pub type TuningcntR = crate::FieldReader;
#[doc = "Field `TUNINGCNT` writer - Tuning Counter Value"]
pub type TuningcntW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TOUTCLKFREQ` reader - Timeout Clock Frequency"]
pub type ToutclkfreqR = crate::FieldReader;
#[doc = "Field `TOUTCLKFREQ` writer - Timeout Clock Frequency"]
pub type ToutclkfreqW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TOUTCLKUNIT` reader - Timeout Clock Unit in kHz or MHz"]
pub type ToutclkunitR = crate::BitReader;
#[doc = "Field `TOUTCLKUNIT` writer - Timeout Clock Unit in kHz or MHz"]
pub type ToutclkunitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BASECLKFREQ` reader - Base Clock Frequency for SD_CLK"]
pub type BaseclkfreqR = crate::FieldReader;
#[doc = "Field `BASECLKFREQ` writer - Base Clock Frequency for SD_CLK"]
pub type BaseclkfreqW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "MAX Block Length of Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Maxblklen {
    #[doc = "0: 512 Bytes are Selected"]
    _512b = 0,
    #[doc = "1: 1024 Bytes are Selected"]
    _1024b = 1,
    #[doc = "2: 2048 Bytes are Selected"]
    _2048b = 2,
}
impl From<Maxblklen> for u8 {
    #[inline(always)]
    fn from(variant: Maxblklen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Maxblklen {
    type Ux = u8;
}
impl crate::IsEnum for Maxblklen {}
#[doc = "Field `MAXBLKLEN` reader - MAX Block Length of Transfer"]
pub type MaxblklenR = crate::FieldReader<Maxblklen>;
impl MaxblklenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Maxblklen> {
        match self.bits {
            0 => Some(Maxblklen::_512b),
            1 => Some(Maxblklen::_1024b),
            2 => Some(Maxblklen::_2048b),
            _ => None,
        }
    }
    #[doc = "512 Bytes are Selected"]
    #[inline(always)]
    pub fn is_512b(&self) -> bool {
        *self == Maxblklen::_512b
    }
    #[doc = "1024 Bytes are Selected"]
    #[inline(always)]
    pub fn is_1024b(&self) -> bool {
        *self == Maxblklen::_1024b
    }
    #[doc = "2048 Bytes are Selected"]
    #[inline(always)]
    pub fn is_2048b(&self) -> bool {
        *self == Maxblklen::_2048b
    }
}
#[doc = "Field `MAXBLKLEN` writer - MAX Block Length of Transfer"]
pub type MaxblklenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Maxblklen>;
impl<'a, REG> MaxblklenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "512 Bytes are Selected"]
    #[inline(always)]
    pub fn _512b(self) -> &'a mut crate::W<REG> {
        self.variant(Maxblklen::_512b)
    }
    #[doc = "1024 Bytes are Selected"]
    #[inline(always)]
    pub fn _1024b(self) -> &'a mut crate::W<REG> {
        self.variant(Maxblklen::_1024b)
    }
    #[doc = "2048 Bytes are Selected"]
    #[inline(always)]
    pub fn _2048b(self) -> &'a mut crate::W<REG> {
        self.variant(Maxblklen::_2048b)
    }
}
#[doc = "Field `C8BITSUP` reader - 8-bit Interface Support"]
pub type C8bitsupR = crate::BitReader;
#[doc = "Field `C8BITSUP` writer - 8-bit Interface Support"]
pub type C8bitsupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CADMA2SUP` reader - ADMA2 Mode Support"]
pub type Cadma2supR = crate::BitReader;
#[doc = "Field `CADMA2SUP` writer - ADMA2 Mode Support"]
pub type Cadma2supW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSSUP` reader - High Speed Mode Support"]
pub type ChssupR = crate::BitReader;
#[doc = "Field `CHSSUP` writer - High Speed Mode Support"]
pub type ChssupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSDMASUP` reader - SDMA Mode Support"]
pub type CsdmasupR = crate::BitReader;
#[doc = "Field `CSDMASUP` writer - SDMA Mode Support"]
pub type CsdmasupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSUSPRESSUP` reader - Suspend/Resume Support"]
pub type CsuspressupR = crate::BitReader;
#[doc = "Field `CSUSPRESSUP` writer - Suspend/Resume Support"]
pub type CsuspressupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C3P3VSUP` reader - Core 3P3V Support"]
pub type C3p3vsupR = crate::BitReader;
#[doc = "Field `C3P3VSUP` writer - Core 3P3V Support"]
pub type C3p3vsupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C3P0VSUP` reader - 3P0V Support"]
pub type C3p0vsupR = crate::BitReader;
#[doc = "Field `C3P0VSUP` writer - 3P0V Support"]
pub type C3p0vsupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C1P8VSUP` reader - 1P8V Support"]
pub type C1p8vsupR = crate::BitReader;
#[doc = "Field `C1P8VSUP` writer - 1P8V Support"]
pub type C1p8vsupW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Tuning Counter Value"]
    #[inline(always)]
    pub fn tuningcnt(&self) -> TuningcntR {
        TuningcntR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Timeout Clock Frequency"]
    #[inline(always)]
    pub fn toutclkfreq(&self) -> ToutclkfreqR {
        ToutclkfreqR::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bit 12 - Timeout Clock Unit in kHz or MHz"]
    #[inline(always)]
    pub fn toutclkunit(&self) -> ToutclkunitR {
        ToutclkunitR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:20 - Base Clock Frequency for SD_CLK"]
    #[inline(always)]
    pub fn baseclkfreq(&self) -> BaseclkfreqR {
        BaseclkfreqR::new(((self.bits >> 13) & 0xff) as u8)
    }
    #[doc = "Bits 21:22 - MAX Block Length of Transfer"]
    #[inline(always)]
    pub fn maxblklen(&self) -> MaxblklenR {
        MaxblklenR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - 8-bit Interface Support"]
    #[inline(always)]
    pub fn c8bitsup(&self) -> C8bitsupR {
        C8bitsupR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - ADMA2 Mode Support"]
    #[inline(always)]
    pub fn cadma2sup(&self) -> Cadma2supR {
        Cadma2supR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - High Speed Mode Support"]
    #[inline(always)]
    pub fn chssup(&self) -> ChssupR {
        ChssupR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - SDMA Mode Support"]
    #[inline(always)]
    pub fn csdmasup(&self) -> CsdmasupR {
        CsdmasupR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Suspend/Resume Support"]
    #[inline(always)]
    pub fn csuspressup(&self) -> CsuspressupR {
        CsuspressupR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Core 3P3V Support"]
    #[inline(always)]
    pub fn c3p3vsup(&self) -> C3p3vsupR {
        C3p3vsupR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 3P0V Support"]
    #[inline(always)]
    pub fn c3p0vsup(&self) -> C3p0vsupR {
        C3p0vsupR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1P8V Support"]
    #[inline(always)]
    pub fn c1p8vsup(&self) -> C1p8vsupR {
        C1p8vsupR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Tuning Counter Value"]
    #[inline(always)]
    pub fn tuningcnt(&mut self) -> TuningcntW<'_, Cfg0Spec> {
        TuningcntW::new(self, 0)
    }
    #[doc = "Bits 6:11 - Timeout Clock Frequency"]
    #[inline(always)]
    pub fn toutclkfreq(&mut self) -> ToutclkfreqW<'_, Cfg0Spec> {
        ToutclkfreqW::new(self, 6)
    }
    #[doc = "Bit 12 - Timeout Clock Unit in kHz or MHz"]
    #[inline(always)]
    pub fn toutclkunit(&mut self) -> ToutclkunitW<'_, Cfg0Spec> {
        ToutclkunitW::new(self, 12)
    }
    #[doc = "Bits 13:20 - Base Clock Frequency for SD_CLK"]
    #[inline(always)]
    pub fn baseclkfreq(&mut self) -> BaseclkfreqW<'_, Cfg0Spec> {
        BaseclkfreqW::new(self, 13)
    }
    #[doc = "Bits 21:22 - MAX Block Length of Transfer"]
    #[inline(always)]
    pub fn maxblklen(&mut self) -> MaxblklenW<'_, Cfg0Spec> {
        MaxblklenW::new(self, 21)
    }
    #[doc = "Bit 23 - 8-bit Interface Support"]
    #[inline(always)]
    pub fn c8bitsup(&mut self) -> C8bitsupW<'_, Cfg0Spec> {
        C8bitsupW::new(self, 23)
    }
    #[doc = "Bit 24 - ADMA2 Mode Support"]
    #[inline(always)]
    pub fn cadma2sup(&mut self) -> Cadma2supW<'_, Cfg0Spec> {
        Cadma2supW::new(self, 24)
    }
    #[doc = "Bit 25 - High Speed Mode Support"]
    #[inline(always)]
    pub fn chssup(&mut self) -> ChssupW<'_, Cfg0Spec> {
        ChssupW::new(self, 25)
    }
    #[doc = "Bit 26 - SDMA Mode Support"]
    #[inline(always)]
    pub fn csdmasup(&mut self) -> CsdmasupW<'_, Cfg0Spec> {
        CsdmasupW::new(self, 26)
    }
    #[doc = "Bit 27 - Suspend/Resume Support"]
    #[inline(always)]
    pub fn csuspressup(&mut self) -> CsuspressupW<'_, Cfg0Spec> {
        CsuspressupW::new(self, 27)
    }
    #[doc = "Bit 28 - Core 3P3V Support"]
    #[inline(always)]
    pub fn c3p3vsup(&mut self) -> C3p3vsupW<'_, Cfg0Spec> {
        C3p3vsupW::new(self, 28)
    }
    #[doc = "Bit 29 - 3P0V Support"]
    #[inline(always)]
    pub fn c3p0vsup(&mut self) -> C3p0vsupW<'_, Cfg0Spec> {
        C3p0vsupW::new(self, 29)
    }
    #[doc = "Bit 30 - 1P8V Support"]
    #[inline(always)]
    pub fn c1p8vsup(&mut self) -> C1p8vsupW<'_, Cfg0Spec> {
        C1p8vsupW::new(self, 30)
    }
}
#[doc = "Core Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Spec;
impl crate::RegisterSpec for Cfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0::R`](R) reader structure"]
impl crate::Readable for Cfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0::W`](W) writer structure"]
impl crate::Writable for Cfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG0 to value 0"]
impl crate::Resettable for Cfg0Spec {}
