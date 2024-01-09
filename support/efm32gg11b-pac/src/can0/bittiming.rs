#[doc = "Register `BITTIMING` reader"]
pub type R = crate::R<BITTIMING_SPEC>;
#[doc = "Register `BITTIMING` writer"]
pub type W = crate::W<BITTIMING_SPEC>;
#[doc = "Field `BRP` reader - Baud Rate Prescaler"]
pub type BRP_R = crate::FieldReader;
#[doc = "Field `BRP` writer - Baud Rate Prescaler"]
pub type BRP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SJW` reader - Synchronization Jump Width"]
pub type SJW_R = crate::FieldReader;
#[doc = "Field `SJW` writer - Synchronization Jump Width"]
pub type SJW_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TSEG1` reader - Time Segment Before the Sample Point"]
pub type TSEG1_R = crate::FieldReader;
#[doc = "Field `TSEG1` writer - Time Segment Before the Sample Point"]
pub type TSEG1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TSEG2` reader - Time Segment After the Sample Point"]
pub type TSEG2_R = crate::FieldReader;
#[doc = "Field `TSEG2` writer - Time Segment After the Sample Point"]
pub type TSEG2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:5 - Baud Rate Prescaler"]
    #[inline(always)]
    pub fn brp(&self) -> BRP_R {
        BRP_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Synchronization Jump Width"]
    #[inline(always)]
    pub fn sjw(&self) -> SJW_R {
        SJW_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Time Segment Before the Sample Point"]
    #[inline(always)]
    pub fn tseg1(&self) -> TSEG1_R {
        TSEG1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Time Segment After the Sample Point"]
    #[inline(always)]
    pub fn tseg2(&self) -> TSEG2_R {
        TSEG2_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Baud Rate Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn brp(&mut self) -> BRP_W<BITTIMING_SPEC> {
        BRP_W::new(self, 0)
    }
    #[doc = "Bits 6:7 - Synchronization Jump Width"]
    #[inline(always)]
    #[must_use]
    pub fn sjw(&mut self) -> SJW_W<BITTIMING_SPEC> {
        SJW_W::new(self, 6)
    }
    #[doc = "Bits 8:11 - Time Segment Before the Sample Point"]
    #[inline(always)]
    #[must_use]
    pub fn tseg1(&mut self) -> TSEG1_W<BITTIMING_SPEC> {
        TSEG1_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Time Segment After the Sample Point"]
    #[inline(always)]
    #[must_use]
    pub fn tseg2(&mut self) -> TSEG2_W<BITTIMING_SPEC> {
        TSEG2_W::new(self, 12)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Bit Timing Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bittiming::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bittiming::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BITTIMING_SPEC;
impl crate::RegisterSpec for BITTIMING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bittiming::R`](R) reader structure"]
impl crate::Readable for BITTIMING_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bittiming::W`](W) writer structure"]
impl crate::Writable for BITTIMING_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BITTIMING to value 0x2301"]
impl crate::Resettable for BITTIMING_SPEC {
    const RESET_VALUE: u32 = 0x2301;
}
