#[doc = "Register `CH2_TIMING` reader"]
pub type R = crate::R<Ch2TimingSpec>;
#[doc = "Register `CH2_TIMING` writer"]
pub type W = crate::W<Ch2TimingSpec>;
#[doc = "Field `EXTIME` reader - Set Excitation Time"]
pub type ExtimeR = crate::FieldReader;
#[doc = "Field `EXTIME` writer - Set Excitation Time"]
pub type ExtimeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SAMPLEDLY` reader - Set Sample Delay"]
pub type SampledlyR = crate::FieldReader;
#[doc = "Field `SAMPLEDLY` writer - Set Sample Delay"]
pub type SampledlyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MEASUREDLY` reader - Set Measure Delay"]
pub type MeasuredlyR = crate::FieldReader<u16>;
#[doc = "Field `MEASUREDLY` writer - Set Measure Delay"]
pub type MeasuredlyW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:5 - Set Excitation Time"]
    #[inline(always)]
    pub fn extime(&self) -> ExtimeR {
        ExtimeR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:13 - Set Sample Delay"]
    #[inline(always)]
    pub fn sampledly(&self) -> SampledlyR {
        SampledlyR::new(((self.bits >> 6) & 0xff) as u8)
    }
    #[doc = "Bits 14:23 - Set Measure Delay"]
    #[inline(always)]
    pub fn measuredly(&self) -> MeasuredlyR {
        MeasuredlyR::new(((self.bits >> 14) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - Set Excitation Time"]
    #[inline(always)]
    pub fn extime(&mut self) -> ExtimeW<'_, Ch2TimingSpec> {
        ExtimeW::new(self, 0)
    }
    #[doc = "Bits 6:13 - Set Sample Delay"]
    #[inline(always)]
    pub fn sampledly(&mut self) -> SampledlyW<'_, Ch2TimingSpec> {
        SampledlyW::new(self, 6)
    }
    #[doc = "Bits 14:23 - Set Measure Delay"]
    #[inline(always)]
    pub fn measuredly(&mut self) -> MeasuredlyW<'_, Ch2TimingSpec> {
        MeasuredlyW::new(self, 14)
    }
}
#[doc = "Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_timing::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_timing::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch2TimingSpec;
impl crate::RegisterSpec for Ch2TimingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2_timing::R`](R) reader structure"]
impl crate::Readable for Ch2TimingSpec {}
#[doc = "`write(|w| ..)` method takes [`ch2_timing::W`](W) writer structure"]
impl crate::Writable for Ch2TimingSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH2_TIMING to value 0"]
impl crate::Resettable for Ch2TimingSpec {}
