#[doc = "Register `SEGD2L` reader"]
pub type R = crate::R<Segd2lSpec>;
#[doc = "Register `SEGD2L` writer"]
pub type W = crate::W<Segd2lSpec>;
#[doc = "Field `SEGD2L` reader - COM2 Segment Data Low"]
pub type Segd2lR = crate::FieldReader<u32>;
#[doc = "Field `SEGD2L` writer - COM2 Segment Data Low"]
pub type Segd2lW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - COM2 Segment Data Low"]
    #[inline(always)]
    pub fn segd2l(&self) -> Segd2lR {
        Segd2lR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - COM2 Segment Data Low"]
    #[inline(always)]
    pub fn segd2l(&mut self) -> Segd2lW<'_, Segd2lSpec> {
        Segd2lW::new(self, 0)
    }
}
#[doc = "Segment Data Low Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`segd2l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segd2l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Segd2lSpec;
impl crate::RegisterSpec for Segd2lSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`segd2l::R`](R) reader structure"]
impl crate::Readable for Segd2lSpec {}
#[doc = "`write(|w| ..)` method takes [`segd2l::W`](W) writer structure"]
impl crate::Writable for Segd2lSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEGD2L to value 0"]
impl crate::Resettable for Segd2lSpec {}
