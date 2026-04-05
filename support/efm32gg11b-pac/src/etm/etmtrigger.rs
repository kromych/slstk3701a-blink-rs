#[doc = "Register `ETMTRIGGER` reader"]
pub type R = crate::R<EtmtriggerSpec>;
#[doc = "Register `ETMTRIGGER` writer"]
pub type W = crate::W<EtmtriggerSpec>;
#[doc = "Field `RESA` reader - ETM Resource A"]
pub type ResaR = crate::FieldReader;
#[doc = "Field `RESA` writer - ETM Resource A"]
pub type ResaW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RESB` reader - ETM Resource B"]
pub type ResbR = crate::FieldReader;
#[doc = "Field `RESB` writer - ETM Resource B"]
pub type ResbW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ETMFCN` reader - ETM Function"]
pub type EtmfcnR = crate::FieldReader;
#[doc = "Field `ETMFCN` writer - ETM Function"]
pub type EtmfcnW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:6 - ETM Resource A"]
    #[inline(always)]
    pub fn resa(&self) -> ResaR {
        ResaR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - ETM Resource B"]
    #[inline(always)]
    pub fn resb(&self) -> ResbR {
        ResbR::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:16 - ETM Function"]
    #[inline(always)]
    pub fn etmfcn(&self) -> EtmfcnR {
        EtmfcnR::new(((self.bits >> 14) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - ETM Resource A"]
    #[inline(always)]
    pub fn resa(&mut self) -> ResaW<'_, EtmtriggerSpec> {
        ResaW::new(self, 0)
    }
    #[doc = "Bits 7:13 - ETM Resource B"]
    #[inline(always)]
    pub fn resb(&mut self) -> ResbW<'_, EtmtriggerSpec> {
        ResbW::new(self, 7)
    }
    #[doc = "Bits 14:16 - ETM Function"]
    #[inline(always)]
    pub fn etmfcn(&mut self) -> EtmfcnW<'_, EtmtriggerSpec> {
        EtmfcnW::new(self, 14)
    }
}
#[doc = "ETM Trigger Event Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmtrigger::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etmtrigger::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtmtriggerSpec;
impl crate::RegisterSpec for EtmtriggerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmtrigger::R`](R) reader structure"]
impl crate::Readable for EtmtriggerSpec {}
#[doc = "`write(|w| ..)` method takes [`etmtrigger::W`](W) writer structure"]
impl crate::Writable for EtmtriggerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETMTRIGGER to value 0"]
impl crate::Resettable for EtmtriggerSpec {}
