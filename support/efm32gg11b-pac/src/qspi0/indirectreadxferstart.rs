#[doc = "Register `INDIRECTREADXFERSTART` reader"]
pub type R = crate::R<IndirectreadxferstartSpec>;
#[doc = "Register `INDIRECTREADXFERSTART` writer"]
pub type W = crate::W<IndirectreadxferstartSpec>;
#[doc = "Field `ADDR` reader - Indirect Read Transfer Start Address"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Indirect Read Transfer Start Address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Indirect Read Transfer Start Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Indirect Read Transfer Start Address"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, IndirectreadxferstartSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Indirect Read Transfer Start Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`indirectreadxferstart::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`indirectreadxferstart::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IndirectreadxferstartSpec;
impl crate::RegisterSpec for IndirectreadxferstartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`indirectreadxferstart::R`](R) reader structure"]
impl crate::Readable for IndirectreadxferstartSpec {}
#[doc = "`write(|w| ..)` method takes [`indirectreadxferstart::W`](W) writer structure"]
impl crate::Writable for IndirectreadxferstartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INDIRECTREADXFERSTART to value 0"]
impl crate::Resettable for IndirectreadxferstartSpec {}
