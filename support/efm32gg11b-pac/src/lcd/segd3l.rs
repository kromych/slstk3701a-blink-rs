#[doc = "Register `SEGD3L` reader"]
pub type R = crate::R<Segd3lSpec>;
#[doc = "Register `SEGD3L` writer"]
pub type W = crate::W<Segd3lSpec>;
#[doc = "Field `SEGD3L` reader - COM3 Segment Data Low"]
pub type Segd3lR = crate::FieldReader<u32>;
#[doc = "Field `SEGD3L` writer - COM3 Segment Data Low"]
pub type Segd3lW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - COM3 Segment Data Low"]
    #[inline(always)]
    pub fn segd3l(&self) -> Segd3lR {
        Segd3lR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - COM3 Segment Data Low"]
    #[inline(always)]
    pub fn segd3l(&mut self) -> Segd3lW<'_, Segd3lSpec> {
        Segd3lW::new(self, 0)
    }
}
#[doc = "Segment Data Low Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`segd3l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segd3l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Segd3lSpec;
impl crate::RegisterSpec for Segd3lSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`segd3l::R`](R) reader structure"]
impl crate::Readable for Segd3lSpec {}
#[doc = "`write(|w| ..)` method takes [`segd3l::W`](W) writer structure"]
impl crate::Writable for Segd3lSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEGD3L to value 0"]
impl crate::Resettable for Segd3lSpec {}
