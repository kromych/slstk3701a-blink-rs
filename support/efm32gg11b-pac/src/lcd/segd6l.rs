#[doc = "Register `SEGD6L` reader"]
pub type R = crate::R<Segd6lSpec>;
#[doc = "Register `SEGD6L` writer"]
pub type W = crate::W<Segd6lSpec>;
#[doc = "Field `SEGD6L` reader - COM6 Segment Data"]
pub type Segd6lR = crate::FieldReader<u32>;
#[doc = "Field `SEGD6L` writer - COM6 Segment Data"]
pub type Segd6lW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - COM6 Segment Data"]
    #[inline(always)]
    pub fn segd6l(&self) -> Segd6lR {
        Segd6lR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - COM6 Segment Data"]
    #[inline(always)]
    pub fn segd6l(&mut self) -> Segd6lW<'_, Segd6lSpec> {
        Segd6lW::new(self, 0)
    }
}
#[doc = "Segment Data Low Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`segd6l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segd6l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Segd6lSpec;
impl crate::RegisterSpec for Segd6lSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`segd6l::R`](R) reader structure"]
impl crate::Readable for Segd6lSpec {}
#[doc = "`write(|w| ..)` method takes [`segd6l::W`](W) writer structure"]
impl crate::Writable for Segd6lSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEGD6L to value 0"]
impl crate::Resettable for Segd6lSpec {}
