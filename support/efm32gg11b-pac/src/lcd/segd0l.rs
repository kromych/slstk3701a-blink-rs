#[doc = "Register `SEGD0L` reader"]
pub type R = crate::R<Segd0lSpec>;
#[doc = "Register `SEGD0L` writer"]
pub type W = crate::W<Segd0lSpec>;
#[doc = "Field `SEGD0L` reader - COM0 Segment Data Low"]
pub type Segd0lR = crate::FieldReader<u32>;
#[doc = "Field `SEGD0L` writer - COM0 Segment Data Low"]
pub type Segd0lW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - COM0 Segment Data Low"]
    #[inline(always)]
    pub fn segd0l(&self) -> Segd0lR {
        Segd0lR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - COM0 Segment Data Low"]
    #[inline(always)]
    pub fn segd0l(&mut self) -> Segd0lW<'_, Segd0lSpec> {
        Segd0lW::new(self, 0)
    }
}
#[doc = "Segment Data Low Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`segd0l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segd0l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Segd0lSpec;
impl crate::RegisterSpec for Segd0lSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`segd0l::R`](R) reader structure"]
impl crate::Readable for Segd0lSpec {}
#[doc = "`write(|w| ..)` method takes [`segd0l::W`](W) writer structure"]
impl crate::Writable for Segd0lSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEGD0L to value 0"]
impl crate::Resettable for Segd0lSpec {}
