#[doc = "Register `OCTETSRXEDBOTTOM` reader"]
pub type R = crate::R<OctetsrxedbottomSpec>;
#[doc = "Register `OCTETSRXEDBOTTOM` writer"]
pub type W = crate::W<OctetsrxedbottomSpec>;
#[doc = "Field `COUNT` reader - Received octets in frame without errors"]
pub type CountR = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - Received octets in frame without errors"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Received octets in frame without errors"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Received octets in frame without errors"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, OctetsrxedbottomSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Octets Received 31:0\n\nYou can [`read`](crate::Reg::read) this register and get [`octetsrxedbottom::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`octetsrxedbottom::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OctetsrxedbottomSpec;
impl crate::RegisterSpec for OctetsrxedbottomSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`octetsrxedbottom::R`](R) reader structure"]
impl crate::Readable for OctetsrxedbottomSpec {}
#[doc = "`write(|w| ..)` method takes [`octetsrxedbottom::W`](W) writer structure"]
impl crate::Writable for OctetsrxedbottomSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OCTETSRXEDBOTTOM to value 0"]
impl crate::Resettable for OctetsrxedbottomSpec {}
