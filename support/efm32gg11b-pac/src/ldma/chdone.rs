#[doc = "Register `CHDONE` reader"]
pub type R = crate::R<ChdoneSpec>;
#[doc = "Register `CHDONE` writer"]
pub type W = crate::W<ChdoneSpec>;
#[doc = "Field `CHDONE` reader - DMA Channel Linking or Done"]
pub type ChdoneR = crate::FieldReader<u32>;
#[doc = "Field `CHDONE` writer - DMA Channel Linking or Done"]
pub type ChdoneW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - DMA Channel Linking or Done"]
    #[inline(always)]
    pub fn chdone(&self) -> ChdoneR {
        ChdoneR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - DMA Channel Linking or Done"]
    #[inline(always)]
    pub fn chdone(&mut self) -> ChdoneW<'_, ChdoneSpec> {
        ChdoneW::new(self, 0)
    }
}
#[doc = "DMA Channel Linking Done Register (Single-Cycle RMW)\n\nYou can [`read`](crate::Reg::read) this register and get [`chdone::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chdone::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChdoneSpec;
impl crate::RegisterSpec for ChdoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chdone::R`](R) reader structure"]
impl crate::Readable for ChdoneSpec {}
#[doc = "`write(|w| ..)` method takes [`chdone::W`](W) writer structure"]
impl crate::Writable for ChdoneSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHDONE to value 0"]
impl crate::Resettable for ChdoneSpec {}
