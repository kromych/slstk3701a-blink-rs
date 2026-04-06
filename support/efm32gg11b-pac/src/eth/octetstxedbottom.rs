#[doc = "Register `OCTETSTXEDBOTTOM` reader"]
pub type R = crate::R<OctetstxedbottomSpec>;
#[doc = "Register `OCTETSTXEDBOTTOM` writer"]
pub type W = crate::W<OctetstxedbottomSpec>;
#[doc = "Field `COUNT` reader - Transmitted octets in frame without errors \\[31:0\\]"]
pub type CountR = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - Transmitted octets in frame without errors \\[31:0\\]"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmitted octets in frame without errors \\[31:0\\]"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmitted octets in frame without errors \\[31:0\\]"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, OctetstxedbottomSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Octets transmitted 31:0\n\nYou can [`read`](crate::Reg::read) this register and get [`octetstxedbottom::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`octetstxedbottom::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OctetstxedbottomSpec;
impl crate::RegisterSpec for OctetstxedbottomSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`octetstxedbottom::R`](R) reader structure"]
impl crate::Readable for OctetstxedbottomSpec {}
#[doc = "`write(|w| ..)` method takes [`octetstxedbottom::W`](W) writer structure"]
impl crate::Writable for OctetstxedbottomSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OCTETSTXEDBOTTOM to value 0"]
impl crate::Resettable for OctetstxedbottomSpec {}
