#[doc = "Register `SEGD0H` reader"]
pub type R = crate::R<Segd0hSpec>;
#[doc = "Register `SEGD0H` writer"]
pub type W = crate::W<Segd0hSpec>;
#[doc = "Field `SEGD0H` reader - COM0 Segment Data High"]
pub type Segd0hR = crate::FieldReader;
#[doc = "Field `SEGD0H` writer - COM0 Segment Data High"]
pub type Segd0hW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - COM0 Segment Data High"]
    #[inline(always)]
    pub fn segd0h(&self) -> Segd0hR {
        Segd0hR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - COM0 Segment Data High"]
    #[inline(always)]
    pub fn segd0h(&mut self) -> Segd0hW<'_, Segd0hSpec> {
        Segd0hW::new(self, 0)
    }
}
#[doc = "Segment Data High Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`segd0h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segd0h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Segd0hSpec;
impl crate::RegisterSpec for Segd0hSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`segd0h::R`](R) reader structure"]
impl crate::Readable for Segd0hSpec {}
#[doc = "`write(|w| ..)` method takes [`segd0h::W`](W) writer structure"]
impl crate::Writable for Segd0hSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEGD0H to value 0"]
impl crate::Resettable for Segd0hSpec {}
