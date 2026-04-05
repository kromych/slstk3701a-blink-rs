#[doc = "Register `SPECADDR4BOTTOM` reader"]
pub type R = crate::R<Specaddr4bottomSpec>;
#[doc = "Register `SPECADDR4BOTTOM` writer"]
pub type W = crate::W<Specaddr4bottomSpec>;
#[doc = "Field `ADDR` reader - Least significant 32 bits of the destination address"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Least significant 32 bits of the destination address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Least significant 32 bits of the destination address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Least significant 32 bits of the destination address"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, Specaddr4bottomSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Specific Address 4 Bottom\n\nYou can [`read`](crate::Reg::read) this register and get [`specaddr4bottom::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`specaddr4bottom::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Specaddr4bottomSpec;
impl crate::RegisterSpec for Specaddr4bottomSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`specaddr4bottom::R`](R) reader structure"]
impl crate::Readable for Specaddr4bottomSpec {}
#[doc = "`write(|w| ..)` method takes [`specaddr4bottom::W`](W) writer structure"]
impl crate::Writable for Specaddr4bottomSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPECADDR4BOTTOM to value 0"]
impl crate::Resettable for Specaddr4bottomSpec {}
