#[doc = "Register `SEGD7L` reader"]
pub type R = crate::R<Segd7lSpec>;
#[doc = "Register `SEGD7L` writer"]
pub type W = crate::W<Segd7lSpec>;
#[doc = "Field `SEGD7L` reader - COM7 Segment Data"]
pub type Segd7lR = crate::FieldReader<u32>;
#[doc = "Field `SEGD7L` writer - COM7 Segment Data"]
pub type Segd7lW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - COM7 Segment Data"]
    #[inline(always)]
    pub fn segd7l(&self) -> Segd7lR {
        Segd7lR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - COM7 Segment Data"]
    #[inline(always)]
    pub fn segd7l(&mut self) -> Segd7lW<'_, Segd7lSpec> {
        Segd7lW::new(self, 0)
    }
}
#[doc = "Segment Data Low Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`segd7l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segd7l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Segd7lSpec;
impl crate::RegisterSpec for Segd7lSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`segd7l::R`](R) reader structure"]
impl crate::Readable for Segd7lSpec {}
#[doc = "`write(|w| ..)` method takes [`segd7l::W`](W) writer structure"]
impl crate::Writable for Segd7lSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEGD7L to value 0"]
impl crate::Resettable for Segd7lSpec {}
