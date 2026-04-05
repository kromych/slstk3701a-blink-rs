#[doc = "Register `CAL` reader"]
pub type R = crate::R<CalSpec>;
#[doc = "Register `CAL` writer"]
pub type W = crate::W<CalSpec>;
#[doc = "Field `OFFSETTRIM` reader - Input Buffer Offset Calibration Value"]
pub type OffsettrimR = crate::FieldReader;
#[doc = "Field `OFFSETTRIM` writer - Input Buffer Offset Calibration Value"]
pub type OffsettrimW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAINERRTRIM` reader - Gain Error Trim Value"]
pub type GainerrtrimR = crate::FieldReader;
#[doc = "Field `GAINERRTRIM` writer - Gain Error Trim Value"]
pub type GainerrtrimW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `GAINERRTRIMCH1` reader - Gain Error Trim Value for CH1"]
pub type Gainerrtrimch1R = crate::FieldReader;
#[doc = "Field `GAINERRTRIMCH1` writer - Gain Error Trim Value for CH1"]
pub type Gainerrtrimch1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - Input Buffer Offset Calibration Value"]
    #[inline(always)]
    pub fn offsettrim(&self) -> OffsettrimR {
        OffsettrimR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:13 - Gain Error Trim Value"]
    #[inline(always)]
    pub fn gainerrtrim(&self) -> GainerrtrimR {
        GainerrtrimR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:19 - Gain Error Trim Value for CH1"]
    #[inline(always)]
    pub fn gainerrtrimch1(&self) -> Gainerrtrimch1R {
        Gainerrtrimch1R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Input Buffer Offset Calibration Value"]
    #[inline(always)]
    pub fn offsettrim(&mut self) -> OffsettrimW<'_, CalSpec> {
        OffsettrimW::new(self, 0)
    }
    #[doc = "Bits 8:13 - Gain Error Trim Value"]
    #[inline(always)]
    pub fn gainerrtrim(&mut self) -> GainerrtrimW<'_, CalSpec> {
        GainerrtrimW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Gain Error Trim Value for CH1"]
    #[inline(always)]
    pub fn gainerrtrimch1(&mut self) -> Gainerrtrimch1W<'_, CalSpec> {
        Gainerrtrimch1W::new(self, 16)
    }
}
#[doc = "Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalSpec;
impl crate::RegisterSpec for CalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cal::R`](R) reader structure"]
impl crate::Readable for CalSpec {}
#[doc = "`write(|w| ..)` method takes [`cal::W`](W) writer structure"]
impl crate::Writable for CalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAL to value 0x0008_2004"]
impl crate::Resettable for CalSpec {
    const RESET_VALUE: u32 = 0x0008_2004;
}
