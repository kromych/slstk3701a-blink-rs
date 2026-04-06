#[doc = "Register `INDIRECTWRITEXFERSTART` reader"]
pub type R = crate::R<IndirectwritexferstartSpec>;
#[doc = "Register `INDIRECTWRITEXFERSTART` writer"]
pub type W = crate::W<IndirectwritexferstartSpec>;
#[doc = "Field `ADDR` reader - Start of Indirect Access"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Start of Indirect Access"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start of Indirect Access"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start of Indirect Access"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, IndirectwritexferstartSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Indirect Write Transfer Start Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`indirectwritexferstart::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`indirectwritexferstart::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IndirectwritexferstartSpec;
impl crate::RegisterSpec for IndirectwritexferstartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`indirectwritexferstart::R`](R) reader structure"]
impl crate::Readable for IndirectwritexferstartSpec {}
#[doc = "`write(|w| ..)` method takes [`indirectwritexferstart::W`](W) writer structure"]
impl crate::Writable for IndirectwritexferstartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INDIRECTWRITEXFERSTART to value 0"]
impl crate::Resettable for IndirectwritexferstartSpec {}
