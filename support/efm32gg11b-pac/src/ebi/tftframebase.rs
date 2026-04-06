#[doc = "Register `TFTFRAMEBASE` reader"]
pub type R = crate::R<TftframebaseSpec>;
#[doc = "Register `TFTFRAMEBASE` writer"]
pub type W = crate::W<TftframebaseSpec>;
#[doc = "Field `FRAMEBASE` reader - Frame Base Address"]
pub type FramebaseR = crate::FieldReader<u32>;
#[doc = "Field `FRAMEBASE` writer - Frame Base Address"]
pub type FramebaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - Frame Base Address"]
    #[inline(always)]
    pub fn framebase(&self) -> FramebaseR {
        FramebaseR::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - Frame Base Address"]
    #[inline(always)]
    pub fn framebase(&mut self) -> FramebaseW<'_, TftframebaseSpec> {
        FramebaseW::new(self, 0)
    }
}
#[doc = "TFT Frame Base Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tftframebase::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tftframebase::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TftframebaseSpec;
impl crate::RegisterSpec for TftframebaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tftframebase::R`](R) reader structure"]
impl crate::Readable for TftframebaseSpec {}
#[doc = "`write(|w| ..)` method takes [`tftframebase::W`](W) writer structure"]
impl crate::Writable for TftframebaseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TFTFRAMEBASE to value 0"]
impl crate::Resettable for TftframebaseSpec {}
