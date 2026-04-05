#[doc = "Register `EM4WUEN` reader"]
pub type R = crate::R<Em4wuenSpec>;
#[doc = "Register `EM4WUEN` writer"]
pub type W = crate::W<Em4wuenSpec>;
#[doc = "Field `EM4WU` reader - EM4 Wake-up Enable"]
pub type Em4wuR = crate::BitReader;
#[doc = "Field `EM4WU` writer - EM4 Wake-up Enable"]
pub type Em4wuW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EM4 Wake-up Enable"]
    #[inline(always)]
    pub fn em4wu(&self) -> Em4wuR {
        Em4wuR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EM4 Wake-up Enable"]
    #[inline(always)]
    pub fn em4wu(&mut self) -> Em4wuW<'_, Em4wuenSpec> {
        Em4wuW::new(self, 0)
    }
}
#[doc = "Wake Up Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`em4wuen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em4wuen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Em4wuenSpec;
impl crate::RegisterSpec for Em4wuenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`em4wuen::R`](R) reader structure"]
impl crate::Readable for Em4wuenSpec {}
#[doc = "`write(|w| ..)` method takes [`em4wuen::W`](W) writer structure"]
impl crate::Writable for Em4wuenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EM4WUEN to value 0"]
impl crate::Resettable for Em4wuenSpec {}
