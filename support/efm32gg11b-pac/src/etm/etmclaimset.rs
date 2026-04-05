#[doc = "Register `ETMCLAIMSET` reader"]
pub type R = crate::R<EtmclaimsetSpec>;
#[doc = "Register `ETMCLAIMSET` writer"]
pub type W = crate::W<EtmclaimsetSpec>;
#[doc = "Field `SETTAG` reader - Tag Bits"]
pub type SettagR = crate::FieldReader;
#[doc = "Field `SETTAG` writer - Tag Bits"]
pub type SettagW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Tag Bits"]
    #[inline(always)]
    pub fn settag(&self) -> SettagR {
        SettagR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Tag Bits"]
    #[inline(always)]
    pub fn settag(&mut self) -> SettagW<'_, EtmclaimsetSpec> {
        SettagW::new(self, 0)
    }
}
#[doc = "ETM Claim Tag Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmclaimset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etmclaimset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtmclaimsetSpec;
impl crate::RegisterSpec for EtmclaimsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmclaimset::R`](R) reader structure"]
impl crate::Readable for EtmclaimsetSpec {}
#[doc = "`write(|w| ..)` method takes [`etmclaimset::W`](W) writer structure"]
impl crate::Writable for EtmclaimsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETMCLAIMSET to value 0x0f"]
impl crate::Resettable for EtmclaimsetSpec {
    const RESET_VALUE: u32 = 0x0f;
}
