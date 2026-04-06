#[doc = "Register `TEMPLIMITS` reader"]
pub type R = crate::R<TemplimitsSpec>;
#[doc = "Register `TEMPLIMITS` writer"]
pub type W = crate::W<TemplimitsSpec>;
#[doc = "Field `TEMPLOW` reader - Temperature Low Limit"]
pub type TemplowR = crate::FieldReader;
#[doc = "Field `TEMPLOW` writer - Temperature Low Limit"]
pub type TemplowW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TEMPHIGH` reader - Temperature High Limit"]
pub type TemphighR = crate::FieldReader;
#[doc = "Field `TEMPHIGH` writer - Temperature High Limit"]
pub type TemphighW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EM4WUEN` reader - Enable EM4 Wakeup Due to Low/high Temperature"]
pub type Em4wuenR = crate::BitReader;
#[doc = "Field `EM4WUEN` writer - Enable EM4 Wakeup Due to Low/high Temperature"]
pub type Em4wuenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Temperature Low Limit"]
    #[inline(always)]
    pub fn templow(&self) -> TemplowR {
        TemplowR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Temperature High Limit"]
    #[inline(always)]
    pub fn temphigh(&self) -> TemphighR {
        TemphighR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Enable EM4 Wakeup Due to Low/high Temperature"]
    #[inline(always)]
    pub fn em4wuen(&self) -> Em4wuenR {
        Em4wuenR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Temperature Low Limit"]
    #[inline(always)]
    pub fn templow(&mut self) -> TemplowW<'_, TemplimitsSpec> {
        TemplowW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Temperature High Limit"]
    #[inline(always)]
    pub fn temphigh(&mut self) -> TemphighW<'_, TemplimitsSpec> {
        TemphighW::new(self, 8)
    }
    #[doc = "Bit 16 - Enable EM4 Wakeup Due to Low/high Temperature"]
    #[inline(always)]
    pub fn em4wuen(&mut self) -> Em4wuenW<'_, TemplimitsSpec> {
        Em4wuenW::new(self, 16)
    }
}
#[doc = "Temperature Limits for Interrupt Generation\n\nYou can [`read`](crate::Reg::read) this register and get [`templimits::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`templimits::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TemplimitsSpec;
impl crate::RegisterSpec for TemplimitsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`templimits::R`](R) reader structure"]
impl crate::Readable for TemplimitsSpec {}
#[doc = "`write(|w| ..)` method takes [`templimits::W`](W) writer structure"]
impl crate::Writable for TemplimitsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TEMPLIMITS to value 0xff00"]
impl crate::Resettable for TemplimitsSpec {
    const RESET_VALUE: u32 = 0xff00;
}
