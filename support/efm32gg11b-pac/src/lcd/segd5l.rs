#[doc = "Register `SEGD5L` reader"]
pub type R = crate::R<Segd5lSpec>;
#[doc = "Register `SEGD5L` writer"]
pub type W = crate::W<Segd5lSpec>;
#[doc = "Field `SEGD5L` reader - COM5 Segment Data"]
pub type Segd5lR = crate::FieldReader<u32>;
#[doc = "Field `SEGD5L` writer - COM5 Segment Data"]
pub type Segd5lW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - COM5 Segment Data"]
    #[inline(always)]
    pub fn segd5l(&self) -> Segd5lR {
        Segd5lR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - COM5 Segment Data"]
    #[inline(always)]
    pub fn segd5l(&mut self) -> Segd5lW<'_, Segd5lSpec> {
        Segd5lW::new(self, 0)
    }
}
#[doc = "Segment Data Low Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`segd5l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segd5l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Segd5lSpec;
impl crate::RegisterSpec for Segd5lSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`segd5l::R`](R) reader structure"]
impl crate::Readable for Segd5lSpec {}
#[doc = "`write(|w| ..)` method takes [`segd5l::W`](W) writer structure"]
impl crate::Writable for Segd5lSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEGD5L to value 0"]
impl crate::Resettable for Segd5lSpec {}
