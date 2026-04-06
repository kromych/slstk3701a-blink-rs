#[doc = "Register `TSUTIMERADJUST` reader"]
pub type R = crate::R<TsutimeradjustSpec>;
#[doc = "Register `TSUTIMERADJUST` writer"]
pub type W = crate::W<TsutimeradjustSpec>;
#[doc = "Field `INCREMENTVAL` reader - Timer increment value"]
pub type IncrementvalR = crate::FieldReader<u32>;
#[doc = "Field `INCREMENTVAL` writer - Timer increment value"]
pub type IncrementvalW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
#[doc = "Field `ADDSUBTRACT` reader - Write as one to subtract from the 1588 timer"]
pub type AddsubtractR = crate::BitReader;
#[doc = "Field `ADDSUBTRACT` writer - Write as one to subtract from the 1588 timer"]
pub type AddsubtractW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:29 - Timer increment value"]
    #[inline(always)]
    pub fn incrementval(&self) -> IncrementvalR {
        IncrementvalR::new(self.bits & 0x3fff_ffff)
    }
    #[doc = "Bit 31 - Write as one to subtract from the 1588 timer"]
    #[inline(always)]
    pub fn addsubtract(&self) -> AddsubtractR {
        AddsubtractR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:29 - Timer increment value"]
    #[inline(always)]
    pub fn incrementval(&mut self) -> IncrementvalW<'_, TsutimeradjustSpec> {
        IncrementvalW::new(self, 0)
    }
    #[doc = "Bit 31 - Write as one to subtract from the 1588 timer"]
    #[inline(always)]
    pub fn addsubtract(&mut self) -> AddsubtractW<'_, TsutimeradjustSpec> {
        AddsubtractW::new(self, 31)
    }
}
#[doc = "This register returns all zeroes when read.\n\nYou can [`read`](crate::Reg::read) this register and get [`tsutimeradjust::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsutimeradjust::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsutimeradjustSpec;
impl crate::RegisterSpec for TsutimeradjustSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsutimeradjust::R`](R) reader structure"]
impl crate::Readable for TsutimeradjustSpec {}
#[doc = "`write(|w| ..)` method takes [`tsutimeradjust::W`](W) writer structure"]
impl crate::Writable for TsutimeradjustSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TSUTIMERADJUST to value 0"]
impl crate::Resettable for TsutimeradjustSpec {}
