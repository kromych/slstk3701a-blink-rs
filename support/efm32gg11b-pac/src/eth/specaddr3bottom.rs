#[doc = "Register `SPECADDR3BOTTOM` reader"]
pub type R = crate::R<Specaddr3bottomSpec>;
#[doc = "Register `SPECADDR3BOTTOM` writer"]
pub type W = crate::W<Specaddr3bottomSpec>;
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
    pub fn addr(&mut self) -> AddrW<'_, Specaddr3bottomSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Specific Address 3 Bottom\n\nYou can [`read`](crate::Reg::read) this register and get [`specaddr3bottom::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`specaddr3bottom::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Specaddr3bottomSpec;
impl crate::RegisterSpec for Specaddr3bottomSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`specaddr3bottom::R`](R) reader structure"]
impl crate::Readable for Specaddr3bottomSpec {}
#[doc = "`write(|w| ..)` method takes [`specaddr3bottom::W`](W) writer structure"]
impl crate::Writable for Specaddr3bottomSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPECADDR3BOTTOM to value 0"]
impl crate::Resettable for Specaddr3bottomSpec {}
