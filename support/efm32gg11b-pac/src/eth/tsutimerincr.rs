#[doc = "Register `TSUTIMERINCR` reader"]
pub type R = crate::R<TsutimerincrSpec>;
#[doc = "Register `TSUTIMERINCR` writer"]
pub type W = crate::W<TsutimerincrSpec>;
#[doc = "Field `NSINCREMENT` reader - A count of nanoseconds by which the 1588 timer nanoseconds register will be incremented each clock cycle"]
pub type NsincrementR = crate::FieldReader;
#[doc = "Field `NSINCREMENT` writer - A count of nanoseconds by which the 1588 timer nanoseconds register will be incremented each clock cycle"]
pub type NsincrementW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ALTNSINCR` reader - Alternative nanoseconds count"]
pub type AltnsincrR = crate::FieldReader;
#[doc = "Field `ALTNSINCR` writer - Alternative nanoseconds count"]
pub type AltnsincrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NUMINCS` reader - Number of incs before alt inc"]
pub type NumincsR = crate::FieldReader;
#[doc = "Field `NUMINCS` writer - Number of incs before alt inc"]
pub type NumincsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - A count of nanoseconds by which the 1588 timer nanoseconds register will be incremented each clock cycle"]
    #[inline(always)]
    pub fn nsincrement(&self) -> NsincrementR {
        NsincrementR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Alternative nanoseconds count"]
    #[inline(always)]
    pub fn altnsincr(&self) -> AltnsincrR {
        AltnsincrR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Number of incs before alt inc"]
    #[inline(always)]
    pub fn numincs(&self) -> NumincsR {
        NumincsR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - A count of nanoseconds by which the 1588 timer nanoseconds register will be incremented each clock cycle"]
    #[inline(always)]
    pub fn nsincrement(&mut self) -> NsincrementW<'_, TsutimerincrSpec> {
        NsincrementW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Alternative nanoseconds count"]
    #[inline(always)]
    pub fn altnsincr(&mut self) -> AltnsincrW<'_, TsutimerincrSpec> {
        AltnsincrW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Number of incs before alt inc"]
    #[inline(always)]
    pub fn numincs(&mut self) -> NumincsW<'_, TsutimerincrSpec> {
        NumincsW::new(self, 16)
    }
}
#[doc = "1588 Timer Increment Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsutimerincr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsutimerincr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsutimerincrSpec;
impl crate::RegisterSpec for TsutimerincrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsutimerincr::R`](R) reader structure"]
impl crate::Readable for TsutimerincrSpec {}
#[doc = "`write(|w| ..)` method takes [`tsutimerincr::W`](W) writer structure"]
impl crate::Writable for TsutimerincrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TSUTIMERINCR to value 0"]
impl crate::Resettable for TsutimerincrSpec {}
