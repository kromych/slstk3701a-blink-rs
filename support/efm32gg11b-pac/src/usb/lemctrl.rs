#[doc = "Register `LEMCTRL` reader"]
pub type R = crate::R<LemctrlSpec>;
#[doc = "Register `LEMCTRL` writer"]
pub type W = crate::W<LemctrlSpec>;
#[doc = "Field `TIMEBASE` reader - Set the Number of LFC Clk Counts to Form 3ms"]
pub type TimebaseR = crate::FieldReader<u16>;
#[doc = "Field `TIMEBASE` writer - Set the Number of LFC Clk Counts to Form 3ms"]
pub type TimebaseW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Set the Number of LFC Clk Counts to Form 3ms"]
    #[inline(always)]
    pub fn timebase(&self) -> TimebaseR {
        TimebaseR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Set the Number of LFC Clk Counts to Form 3ms"]
    #[inline(always)]
    pub fn timebase(&mut self) -> TimebaseW<'_, LemctrlSpec> {
        TimebaseW::new(self, 0)
    }
}
#[doc = "USB LEM Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lemctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lemctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LemctrlSpec;
impl crate::RegisterSpec for LemctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lemctrl::R`](R) reader structure"]
impl crate::Readable for LemctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`lemctrl::W`](W) writer structure"]
impl crate::Writable for LemctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LEMCTRL to value 0x67"]
impl crate::Resettable for LemctrlSpec {
    const RESET_VALUE: u32 = 0x67;
}
