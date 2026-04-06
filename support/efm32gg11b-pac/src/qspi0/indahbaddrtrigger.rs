#[doc = "Register `INDAHBADDRTRIGGER` reader"]
pub type R = crate::R<IndahbaddrtriggerSpec>;
#[doc = "Register `INDAHBADDRTRIGGER` writer"]
pub type W = crate::W<IndahbaddrtriggerSpec>;
#[doc = "Field `ADDR` reader - Indirect Address Trigger Register"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Indirect Address Trigger Register"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Indirect Address Trigger Register"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Indirect Address Trigger Register"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, IndahbaddrtriggerSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Indirect Address Trigger Register\n\nYou can [`read`](crate::Reg::read) this register and get [`indahbaddrtrigger::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`indahbaddrtrigger::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IndahbaddrtriggerSpec;
impl crate::RegisterSpec for IndahbaddrtriggerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`indahbaddrtrigger::R`](R) reader structure"]
impl crate::Readable for IndahbaddrtriggerSpec {}
#[doc = "`write(|w| ..)` method takes [`indahbaddrtrigger::W`](W) writer structure"]
impl crate::Writable for IndahbaddrtriggerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INDAHBADDRTRIGGER to value 0"]
impl crate::Resettable for IndahbaddrtriggerSpec {}
