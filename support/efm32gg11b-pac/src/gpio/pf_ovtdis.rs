#[doc = "Register `PF_OVTDIS` reader"]
pub type R = crate::R<PfOvtdisSpec>;
#[doc = "Register `PF_OVTDIS` writer"]
pub type W = crate::W<PfOvtdisSpec>;
#[doc = "Field `OVTDIS` reader - Disable Over Voltage Capability"]
pub type OvtdisR = crate::FieldReader<u16>;
#[doc = "Field `OVTDIS` writer - Disable Over Voltage Capability"]
pub type OvtdisW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovtdis(&self) -> OvtdisR {
        OvtdisR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovtdis(&mut self) -> OvtdisW<'_, PfOvtdisSpec> {
        OvtdisW::new(self, 0)
    }
}
#[doc = "Over Voltage Disable for All Modes\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_ovtdis::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_ovtdis::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PfOvtdisSpec;
impl crate::RegisterSpec for PfOvtdisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pf_ovtdis::R`](R) reader structure"]
impl crate::Readable for PfOvtdisSpec {}
#[doc = "`write(|w| ..)` method takes [`pf_ovtdis::W`](W) writer structure"]
impl crate::Writable for PfOvtdisSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PF_OVTDIS to value 0"]
impl crate::Resettable for PfOvtdisSpec {}
