#[doc = "Register `FRAMESRXEDOK` reader"]
pub type R = crate::R<FramesrxedokSpec>;
#[doc = "Register `FRAMESRXEDOK` writer"]
pub type W = crate::W<FramesrxedokSpec>;
#[doc = "Field `COUNT` reader - Frames received without error"]
pub type CountR = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - Frames received without error"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Frames received without error"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Frames received without error"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, FramesrxedokSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Frames Received\n\nYou can [`read`](crate::Reg::read) this register and get [`framesrxedok::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`framesrxedok::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FramesrxedokSpec;
impl crate::RegisterSpec for FramesrxedokSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`framesrxedok::R`](R) reader structure"]
impl crate::Readable for FramesrxedokSpec {}
#[doc = "`write(|w| ..)` method takes [`framesrxedok::W`](W) writer structure"]
impl crate::Writable for FramesrxedokSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRAMESRXEDOK to value 0"]
impl crate::Resettable for FramesrxedokSpec {}
