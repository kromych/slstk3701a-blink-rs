#[doc = "Register `SCANMASK1` reader"]
pub type R = crate::R<Scanmask1Spec>;
#[doc = "Register `SCANMASK1` writer"]
pub type W = crate::W<Scanmask1Spec>;
#[doc = "Field `SCANINPUTEN` reader - Scan Channel Mask."]
pub type ScaninputenR = crate::FieldReader<u32>;
#[doc = "Field `SCANINPUTEN` writer - Scan Channel Mask."]
pub type ScaninputenW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Scan Channel Mask."]
    #[inline(always)]
    pub fn scaninputen(&self) -> ScaninputenR {
        ScaninputenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Scan Channel Mask."]
    #[inline(always)]
    pub fn scaninputen(&mut self) -> ScaninputenW<'_, Scanmask1Spec> {
        ScaninputenW::new(self, 0)
    }
}
#[doc = "Scan Channel Mask 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scanmask1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scanmask1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scanmask1Spec;
impl crate::RegisterSpec for Scanmask1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scanmask1::R`](R) reader structure"]
impl crate::Readable for Scanmask1Spec {}
#[doc = "`write(|w| ..)` method takes [`scanmask1::W`](W) writer structure"]
impl crate::Writable for Scanmask1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCANMASK1 to value 0"]
impl crate::Resettable for Scanmask1Spec {}
