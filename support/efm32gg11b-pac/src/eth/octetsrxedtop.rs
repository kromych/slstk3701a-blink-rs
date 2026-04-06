#[doc = "Register `OCTETSRXEDTOP` reader"]
pub type R = crate::R<OctetsrxedtopSpec>;
#[doc = "Register `OCTETSRXEDTOP` writer"]
pub type W = crate::W<OctetsrxedtopSpec>;
#[doc = "Field `COUNT` reader - Received octets in frame without errors"]
pub type CountR = crate::FieldReader<u16>;
#[doc = "Field `COUNT` writer - Received octets in frame without errors"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Received octets in frame without errors"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Received octets in frame without errors"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, OctetsrxedtopSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Octets Received 47:32\n\nYou can [`read`](crate::Reg::read) this register and get [`octetsrxedtop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`octetsrxedtop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OctetsrxedtopSpec;
impl crate::RegisterSpec for OctetsrxedtopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`octetsrxedtop::R`](R) reader structure"]
impl crate::Readable for OctetsrxedtopSpec {}
#[doc = "`write(|w| ..)` method takes [`octetsrxedtop::W`](W) writer structure"]
impl crate::Writable for OctetsrxedtopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OCTETSRXEDTOP to value 0"]
impl crate::Resettable for OctetsrxedtopSpec {}
