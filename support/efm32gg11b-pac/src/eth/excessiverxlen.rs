#[doc = "Register `EXCESSIVERXLEN` reader"]
pub type R = crate::R<ExcessiverxlenSpec>;
#[doc = "Register `EXCESSIVERXLEN` writer"]
pub type W = crate::W<ExcessiverxlenSpec>;
#[doc = "Field `COUNT` reader - Oversize frames received"]
pub type CountR = crate::FieldReader<u16>;
#[doc = "Field `COUNT` writer - Oversize frames received"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Oversize frames received"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Oversize frames received"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, ExcessiverxlenSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Oversize Frames Received\n\nYou can [`read`](crate::Reg::read) this register and get [`excessiverxlen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`excessiverxlen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExcessiverxlenSpec;
impl crate::RegisterSpec for ExcessiverxlenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`excessiverxlen::R`](R) reader structure"]
impl crate::Readable for ExcessiverxlenSpec {}
#[doc = "`write(|w| ..)` method takes [`excessiverxlen::W`](W) writer structure"]
impl crate::Writable for ExcessiverxlenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXCESSIVERXLEN to value 0"]
impl crate::Resettable for ExcessiverxlenSpec {}
