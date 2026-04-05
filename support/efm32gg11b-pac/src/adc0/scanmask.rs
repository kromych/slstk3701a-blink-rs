#[doc = "Register `SCANMASK` reader"]
pub type R = crate::R<ScanmaskSpec>;
#[doc = "Register `SCANMASK` writer"]
pub type W = crate::W<ScanmaskSpec>;
#[doc = "Field `SCANINPUTEN` reader - Scan Sequence Input Mask"]
pub type ScaninputenR = crate::FieldReader<u32>;
#[doc = "Field `SCANINPUTEN` writer - Scan Sequence Input Mask"]
pub type ScaninputenW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Scan Sequence Input Mask"]
    #[inline(always)]
    pub fn scaninputen(&self) -> ScaninputenR {
        ScaninputenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Scan Sequence Input Mask"]
    #[inline(always)]
    pub fn scaninputen(&mut self) -> ScaninputenW<'_, ScanmaskSpec> {
        ScaninputenW::new(self, 0)
    }
}
#[doc = "Scan Sequence Input Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scanmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scanmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScanmaskSpec;
impl crate::RegisterSpec for ScanmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scanmask::R`](R) reader structure"]
impl crate::Readable for ScanmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`scanmask::W`](W) writer structure"]
impl crate::Writable for ScanmaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCANMASK to value 0"]
impl crate::Resettable for ScanmaskSpec {}
