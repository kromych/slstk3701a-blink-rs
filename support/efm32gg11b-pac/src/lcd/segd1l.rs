#[doc = "Register `SEGD1L` reader"]
pub type R = crate::R<Segd1lSpec>;
#[doc = "Register `SEGD1L` writer"]
pub type W = crate::W<Segd1lSpec>;
#[doc = "Field `SEGD1L` reader - COM1 Segment Data Low"]
pub type Segd1lR = crate::FieldReader<u32>;
#[doc = "Field `SEGD1L` writer - COM1 Segment Data Low"]
pub type Segd1lW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - COM1 Segment Data Low"]
    #[inline(always)]
    pub fn segd1l(&self) -> Segd1lR {
        Segd1lR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - COM1 Segment Data Low"]
    #[inline(always)]
    pub fn segd1l(&mut self) -> Segd1lW<'_, Segd1lSpec> {
        Segd1lW::new(self, 0)
    }
}
#[doc = "Segment Data Low Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`segd1l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segd1l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Segd1lSpec;
impl crate::RegisterSpec for Segd1lSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`segd1l::R`](R) reader structure"]
impl crate::Readable for Segd1lSpec {}
#[doc = "`write(|w| ..)` method takes [`segd1l::W`](W) writer structure"]
impl crate::Writable for Segd1lSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEGD1L to value 0"]
impl crate::Resettable for Segd1lSpec {}
