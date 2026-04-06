#[doc = "Register `ETMTEEVR` reader"]
pub type R = crate::R<EtmteevrSpec>;
#[doc = "Register `ETMTEEVR` writer"]
pub type W = crate::W<EtmteevrSpec>;
#[doc = "Field `RESA` reader - ETM Resource A Trace Enable"]
pub type ResaR = crate::FieldReader;
#[doc = "Field `RESA` writer - ETM Resource A Trace Enable"]
pub type ResaW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RESB` reader - ETM Resource B Trace Enable"]
pub type ResbR = crate::FieldReader;
#[doc = "Field `RESB` writer - ETM Resource B Trace Enable"]
pub type ResbW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ETMFCNEN` reader - ETM Function Trace Enable"]
pub type EtmfcnenR = crate::FieldReader;
#[doc = "Field `ETMFCNEN` writer - ETM Function Trace Enable"]
pub type EtmfcnenW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:6 - ETM Resource A Trace Enable"]
    #[inline(always)]
    pub fn resa(&self) -> ResaR {
        ResaR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - ETM Resource B Trace Enable"]
    #[inline(always)]
    pub fn resb(&self) -> ResbR {
        ResbR::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:16 - ETM Function Trace Enable"]
    #[inline(always)]
    pub fn etmfcnen(&self) -> EtmfcnenR {
        EtmfcnenR::new(((self.bits >> 14) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - ETM Resource A Trace Enable"]
    #[inline(always)]
    pub fn resa(&mut self) -> ResaW<'_, EtmteevrSpec> {
        ResaW::new(self, 0)
    }
    #[doc = "Bits 7:13 - ETM Resource B Trace Enable"]
    #[inline(always)]
    pub fn resb(&mut self) -> ResbW<'_, EtmteevrSpec> {
        ResbW::new(self, 7)
    }
    #[doc = "Bits 14:16 - ETM Function Trace Enable"]
    #[inline(always)]
    pub fn etmfcnen(&mut self) -> EtmfcnenW<'_, EtmteevrSpec> {
        EtmfcnenW::new(self, 14)
    }
}
#[doc = "ETM TraceEnable Event Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmteevr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etmteevr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtmteevrSpec;
impl crate::RegisterSpec for EtmteevrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmteevr::R`](R) reader structure"]
impl crate::Readable for EtmteevrSpec {}
#[doc = "`write(|w| ..)` method takes [`etmteevr::W`](W) writer structure"]
impl crate::Writable for EtmteevrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETMTEEVR to value 0"]
impl crate::Resettable for EtmteevrSpec {}
