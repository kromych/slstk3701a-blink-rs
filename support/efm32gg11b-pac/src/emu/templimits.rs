#[doc = "Register `TEMPLIMITS` reader"]
pub type R = crate::R<TEMPLIMITS_SPEC>;
#[doc = "Register `TEMPLIMITS` writer"]
pub type W = crate::W<TEMPLIMITS_SPEC>;
#[doc = "Field `TEMPLOW` reader - Temperature Low Limit"]
pub type TEMPLOW_R = crate::FieldReader;
#[doc = "Field `TEMPLOW` writer - Temperature Low Limit"]
pub type TEMPLOW_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TEMPHIGH` reader - Temperature High Limit"]
pub type TEMPHIGH_R = crate::FieldReader;
#[doc = "Field `TEMPHIGH` writer - Temperature High Limit"]
pub type TEMPHIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EM4WUEN` reader - Enable EM4 Wakeup Due to Low/high Temperature"]
pub type EM4WUEN_R = crate::BitReader;
#[doc = "Field `EM4WUEN` writer - Enable EM4 Wakeup Due to Low/high Temperature"]
pub type EM4WUEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Temperature Low Limit"]
    #[inline(always)]
    pub fn templow(&self) -> TEMPLOW_R {
        TEMPLOW_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Temperature High Limit"]
    #[inline(always)]
    pub fn temphigh(&self) -> TEMPHIGH_R {
        TEMPHIGH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Enable EM4 Wakeup Due to Low/high Temperature"]
    #[inline(always)]
    pub fn em4wuen(&self) -> EM4WUEN_R {
        EM4WUEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Temperature Low Limit"]
    #[inline(always)]
    #[must_use]
    pub fn templow(&mut self) -> TEMPLOW_W<TEMPLIMITS_SPEC> {
        TEMPLOW_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Temperature High Limit"]
    #[inline(always)]
    #[must_use]
    pub fn temphigh(&mut self) -> TEMPHIGH_W<TEMPLIMITS_SPEC> {
        TEMPHIGH_W::new(self, 8)
    }
    #[doc = "Bit 16 - Enable EM4 Wakeup Due to Low/high Temperature"]
    #[inline(always)]
    #[must_use]
    pub fn em4wuen(&mut self) -> EM4WUEN_W<TEMPLIMITS_SPEC> {
        EM4WUEN_W::new(self, 16)
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
#[doc = "Temperature Limits for Interrupt Generation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`templimits::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`templimits::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TEMPLIMITS_SPEC;
impl crate::RegisterSpec for TEMPLIMITS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`templimits::R`](R) reader structure"]
impl crate::Readable for TEMPLIMITS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`templimits::W`](W) writer structure"]
impl crate::Writable for TEMPLIMITS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TEMPLIMITS to value 0xff00"]
impl crate::Resettable for TEMPLIMITS_SPEC {
    const RESET_VALUE: u32 = 0xff00;
}
