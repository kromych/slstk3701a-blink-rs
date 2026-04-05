#[doc = "Register `ETMTSEVR` reader"]
pub type R = crate::R<EtmtsevrSpec>;
#[doc = "Register `ETMTSEVR` writer"]
pub type W = crate::W<EtmtsevrSpec>;
#[doc = "Field `RESAEVT` reader - ETM Resource A Event"]
pub type ResaevtR = crate::FieldReader;
#[doc = "Field `RESAEVT` writer - ETM Resource A Event"]
pub type ResaevtW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RESBEVT` reader - ETM Resource B Event"]
pub type ResbevtR = crate::FieldReader;
#[doc = "Field `RESBEVT` writer - ETM Resource B Event"]
pub type ResbevtW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ETMFCNEVT` reader - ETM Function Event"]
pub type EtmfcnevtR = crate::FieldReader;
#[doc = "Field `ETMFCNEVT` writer - ETM Function Event"]
pub type EtmfcnevtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:6 - ETM Resource A Event"]
    #[inline(always)]
    pub fn resaevt(&self) -> ResaevtR {
        ResaevtR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - ETM Resource B Event"]
    #[inline(always)]
    pub fn resbevt(&self) -> ResbevtR {
        ResbevtR::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:16 - ETM Function Event"]
    #[inline(always)]
    pub fn etmfcnevt(&self) -> EtmfcnevtR {
        EtmfcnevtR::new(((self.bits >> 14) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - ETM Resource A Event"]
    #[inline(always)]
    pub fn resaevt(&mut self) -> ResaevtW<'_, EtmtsevrSpec> {
        ResaevtW::new(self, 0)
    }
    #[doc = "Bits 7:13 - ETM Resource B Event"]
    #[inline(always)]
    pub fn resbevt(&mut self) -> ResbevtW<'_, EtmtsevrSpec> {
        ResbevtW::new(self, 7)
    }
    #[doc = "Bits 14:16 - ETM Function Event"]
    #[inline(always)]
    pub fn etmfcnevt(&mut self) -> EtmfcnevtW<'_, EtmtsevrSpec> {
        EtmfcnevtW::new(self, 14)
    }
}
#[doc = "Timestamp Event Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmtsevr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etmtsevr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtmtsevrSpec;
impl crate::RegisterSpec for EtmtsevrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmtsevr::R`](R) reader structure"]
impl crate::Readable for EtmtsevrSpec {}
#[doc = "`write(|w| ..)` method takes [`etmtsevr::W`](W) writer structure"]
impl crate::Writable for EtmtsevrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETMTSEVR to value 0"]
impl crate::Resettable for EtmtsevrSpec {}
