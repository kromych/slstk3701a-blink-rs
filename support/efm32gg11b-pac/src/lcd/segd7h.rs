#[doc = "Register `SEGD7H` reader"]
pub type R = crate::R<Segd7hSpec>;
#[doc = "Register `SEGD7H` writer"]
pub type W = crate::W<Segd7hSpec>;
#[doc = "Field `SEGD7H` reader - COM3 Segment Data High"]
pub type Segd7hR = crate::FieldReader;
#[doc = "Field `SEGD7H` writer - COM3 Segment Data High"]
pub type Segd7hW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - COM3 Segment Data High"]
    #[inline(always)]
    pub fn segd7h(&self) -> Segd7hR {
        Segd7hR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - COM3 Segment Data High"]
    #[inline(always)]
    pub fn segd7h(&mut self) -> Segd7hW<'_, Segd7hSpec> {
        Segd7hW::new(self, 0)
    }
}
#[doc = "Segment Data High Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`segd7h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segd7h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Segd7hSpec;
impl crate::RegisterSpec for Segd7hSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`segd7h::R`](R) reader structure"]
impl crate::Readable for Segd7hSpec {}
#[doc = "`write(|w| ..)` method takes [`segd7h::W`](W) writer structure"]
impl crate::Writable for Segd7hSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEGD7H to value 0"]
impl crate::Resettable for Segd7hSpec {}
