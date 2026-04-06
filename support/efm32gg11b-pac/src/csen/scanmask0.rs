#[doc = "Register `SCANMASK0` reader"]
pub type R = crate::R<Scanmask0Spec>;
#[doc = "Register `SCANMASK0` writer"]
pub type W = crate::W<Scanmask0Spec>;
#[doc = "Field `SCANINPUTEN` reader - Scan Channel Mask"]
pub type ScaninputenR = crate::FieldReader<u32>;
#[doc = "Field `SCANINPUTEN` writer - Scan Channel Mask"]
pub type ScaninputenW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Scan Channel Mask"]
    #[inline(always)]
    pub fn scaninputen(&self) -> ScaninputenR {
        ScaninputenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Scan Channel Mask"]
    #[inline(always)]
    pub fn scaninputen(&mut self) -> ScaninputenW<'_, Scanmask0Spec> {
        ScaninputenW::new(self, 0)
    }
}
#[doc = "Scan Channel Mask 0\n\nYou can [`read`](crate::Reg::read) this register and get [`scanmask0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scanmask0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scanmask0Spec;
impl crate::RegisterSpec for Scanmask0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scanmask0::R`](R) reader structure"]
impl crate::Readable for Scanmask0Spec {}
#[doc = "`write(|w| ..)` method takes [`scanmask0::W`](W) writer structure"]
impl crate::Writable for Scanmask0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCANMASK0 to value 0"]
impl crate::Resettable for Scanmask0Spec {}
