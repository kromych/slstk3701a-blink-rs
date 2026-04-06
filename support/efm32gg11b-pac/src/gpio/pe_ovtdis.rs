#[doc = "Register `PE_OVTDIS` reader"]
pub type R = crate::R<PeOvtdisSpec>;
#[doc = "Register `PE_OVTDIS` writer"]
pub type W = crate::W<PeOvtdisSpec>;
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
    pub fn ovtdis(&mut self) -> OvtdisW<'_, PeOvtdisSpec> {
        OvtdisW::new(self, 0)
    }
}
#[doc = "Over Voltage Disable for All Modes\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_ovtdis::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_ovtdis::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeOvtdisSpec;
impl crate::RegisterSpec for PeOvtdisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pe_ovtdis::R`](R) reader structure"]
impl crate::Readable for PeOvtdisSpec {}
#[doc = "`write(|w| ..)` method takes [`pe_ovtdis::W`](W) writer structure"]
impl crate::Writable for PeOvtdisSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PE_OVTDIS to value 0"]
impl crate::Resettable for PeOvtdisSpec {}
