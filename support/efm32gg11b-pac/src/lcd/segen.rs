#[doc = "Register `SEGEN` reader"]
pub type R = crate::R<SegenSpec>;
#[doc = "Register `SEGEN` writer"]
pub type W = crate::W<SegenSpec>;
#[doc = "Field `SEGEN` reader - Segment Enable"]
pub type SegenR = crate::FieldReader<u32>;
#[doc = "Field `SEGEN` writer - Segment Enable"]
pub type SegenW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Segment Enable"]
    #[inline(always)]
    pub fn segen(&self) -> SegenR {
        SegenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Segment Enable"]
    #[inline(always)]
    pub fn segen(&mut self) -> SegenW<'_, SegenSpec> {
        SegenW::new(self, 0)
    }
}
#[doc = "Segment Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`segen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SegenSpec;
impl crate::RegisterSpec for SegenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`segen::R`](R) reader structure"]
impl crate::Readable for SegenSpec {}
#[doc = "`write(|w| ..)` method takes [`segen::W`](W) writer structure"]
impl crate::Writable for SegenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEGEN to value 0"]
impl crate::Resettable for SegenSpec {}
