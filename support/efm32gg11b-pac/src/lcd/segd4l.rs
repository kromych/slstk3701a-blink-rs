#[doc = "Register `SEGD4L` reader"]
pub type R = crate::R<Segd4lSpec>;
#[doc = "Register `SEGD4L` writer"]
pub type W = crate::W<Segd4lSpec>;
#[doc = "Field `SEGD4L` reader - COM4 Segment Data"]
pub type Segd4lR = crate::FieldReader<u32>;
#[doc = "Field `SEGD4L` writer - COM4 Segment Data"]
pub type Segd4lW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - COM4 Segment Data"]
    #[inline(always)]
    pub fn segd4l(&self) -> Segd4lR {
        Segd4lR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - COM4 Segment Data"]
    #[inline(always)]
    pub fn segd4l(&mut self) -> Segd4lW<'_, Segd4lSpec> {
        Segd4lW::new(self, 0)
    }
}
#[doc = "Segment Data Low Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`segd4l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segd4l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Segd4lSpec;
impl crate::RegisterSpec for Segd4lSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`segd4l::R`](R) reader structure"]
impl crate::Readable for Segd4lSpec {}
#[doc = "`write(|w| ..)` method takes [`segd4l::W`](W) writer structure"]
impl crate::Writable for Segd4lSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEGD4L to value 0"]
impl crate::Resettable for Segd4lSpec {}
