#[doc = "Register `CC0_TIME` reader"]
pub type R = crate::R<Cc0TimeSpec>;
#[doc = "Register `CC0_TIME` writer"]
pub type W = crate::W<Cc0TimeSpec>;
#[doc = "Field `SECU` reader - Seconds, Units"]
pub type SecuR = crate::FieldReader;
#[doc = "Field `SECU` writer - Seconds, Units"]
pub type SecuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SECT` reader - Seconds, Tens"]
pub type SectR = crate::FieldReader;
#[doc = "Field `SECT` writer - Seconds, Tens"]
pub type SectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MINU` reader - Minutes, Units"]
pub type MinuR = crate::FieldReader;
#[doc = "Field `MINU` writer - Minutes, Units"]
pub type MinuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MINT` reader - Minutes, Tens"]
pub type MintR = crate::FieldReader;
#[doc = "Field `MINT` writer - Minutes, Tens"]
pub type MintW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HOURU` reader - Hours, Units"]
pub type HouruR = crate::FieldReader;
#[doc = "Field `HOURU` writer - Hours, Units"]
pub type HouruW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HOURT` reader - Hours, Tens"]
pub type HourtR = crate::FieldReader;
#[doc = "Field `HOURT` writer - Hours, Tens"]
pub type HourtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - Seconds, Units"]
    #[inline(always)]
    pub fn secu(&self) -> SecuR {
        SecuR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Seconds, Tens"]
    #[inline(always)]
    pub fn sect(&self) -> SectR {
        SectR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Minutes, Units"]
    #[inline(always)]
    pub fn minu(&self) -> MinuR {
        MinuR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Minutes, Tens"]
    #[inline(always)]
    pub fn mint(&self) -> MintR {
        MintR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Hours, Units"]
    #[inline(always)]
    pub fn houru(&self) -> HouruR {
        HouruR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Hours, Tens"]
    #[inline(always)]
    pub fn hourt(&self) -> HourtR {
        HourtR::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Seconds, Units"]
    #[inline(always)]
    pub fn secu(&mut self) -> SecuW<'_, Cc0TimeSpec> {
        SecuW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Seconds, Tens"]
    #[inline(always)]
    pub fn sect(&mut self) -> SectW<'_, Cc0TimeSpec> {
        SectW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Minutes, Units"]
    #[inline(always)]
    pub fn minu(&mut self) -> MinuW<'_, Cc0TimeSpec> {
        MinuW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Minutes, Tens"]
    #[inline(always)]
    pub fn mint(&mut self) -> MintW<'_, Cc0TimeSpec> {
        MintW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Hours, Units"]
    #[inline(always)]
    pub fn houru(&mut self) -> HouruW<'_, Cc0TimeSpec> {
        HouruW::new(self, 16)
    }
    #[doc = "Bits 20:21 - Hours, Tens"]
    #[inline(always)]
    pub fn hourt(&mut self) -> HourtW<'_, Cc0TimeSpec> {
        HourtW::new(self, 20)
    }
}
#[doc = "Capture/Compare Time Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc0_time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc0_time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cc0TimeSpec;
impl crate::RegisterSpec for Cc0TimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc0_time::R`](R) reader structure"]
impl crate::Readable for Cc0TimeSpec {}
#[doc = "`write(|w| ..)` method takes [`cc0_time::W`](W) writer structure"]
impl crate::Writable for Cc0TimeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CC0_TIME to value 0"]
impl crate::Resettable for Cc0TimeSpec {}
