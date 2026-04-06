#[doc = "Register `BITTIMING` reader"]
pub type R = crate::R<BittimingSpec>;
#[doc = "Register `BITTIMING` writer"]
pub type W = crate::W<BittimingSpec>;
#[doc = "Field `BRP` reader - Baud Rate Prescaler"]
pub type BrpR = crate::FieldReader;
#[doc = "Field `BRP` writer - Baud Rate Prescaler"]
pub type BrpW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SJW` reader - Synchronization Jump Width"]
pub type SjwR = crate::FieldReader;
#[doc = "Field `SJW` writer - Synchronization Jump Width"]
pub type SjwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TSEG1` reader - Time Segment Before the Sample Point"]
pub type Tseg1R = crate::FieldReader;
#[doc = "Field `TSEG1` writer - Time Segment Before the Sample Point"]
pub type Tseg1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TSEG2` reader - Time Segment After the Sample Point"]
pub type Tseg2R = crate::FieldReader;
#[doc = "Field `TSEG2` writer - Time Segment After the Sample Point"]
pub type Tseg2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:5 - Baud Rate Prescaler"]
    #[inline(always)]
    pub fn brp(&self) -> BrpR {
        BrpR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Synchronization Jump Width"]
    #[inline(always)]
    pub fn sjw(&self) -> SjwR {
        SjwR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Time Segment Before the Sample Point"]
    #[inline(always)]
    pub fn tseg1(&self) -> Tseg1R {
        Tseg1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Time Segment After the Sample Point"]
    #[inline(always)]
    pub fn tseg2(&self) -> Tseg2R {
        Tseg2R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Baud Rate Prescaler"]
    #[inline(always)]
    pub fn brp(&mut self) -> BrpW<'_, BittimingSpec> {
        BrpW::new(self, 0)
    }
    #[doc = "Bits 6:7 - Synchronization Jump Width"]
    #[inline(always)]
    pub fn sjw(&mut self) -> SjwW<'_, BittimingSpec> {
        SjwW::new(self, 6)
    }
    #[doc = "Bits 8:11 - Time Segment Before the Sample Point"]
    #[inline(always)]
    pub fn tseg1(&mut self) -> Tseg1W<'_, BittimingSpec> {
        Tseg1W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Time Segment After the Sample Point"]
    #[inline(always)]
    pub fn tseg2(&mut self) -> Tseg2W<'_, BittimingSpec> {
        Tseg2W::new(self, 12)
    }
}
#[doc = "Bit Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bittiming::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bittiming::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BittimingSpec;
impl crate::RegisterSpec for BittimingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bittiming::R`](R) reader structure"]
impl crate::Readable for BittimingSpec {}
#[doc = "`write(|w| ..)` method takes [`bittiming::W`](W) writer structure"]
impl crate::Writable for BittimingSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BITTIMING to value 0x2301"]
impl crate::Resettable for BittimingSpec {
    const RESET_VALUE: u32 = 0x2301;
}
