#[doc = "Register `SEGD1H` reader"]
pub type R = crate::R<Segd1hSpec>;
#[doc = "Register `SEGD1H` writer"]
pub type W = crate::W<Segd1hSpec>;
#[doc = "Field `SEGD1H` reader - COM1 Segment Data High"]
pub type Segd1hR = crate::FieldReader;
#[doc = "Field `SEGD1H` writer - COM1 Segment Data High"]
pub type Segd1hW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - COM1 Segment Data High"]
    #[inline(always)]
    pub fn segd1h(&self) -> Segd1hR {
        Segd1hR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - COM1 Segment Data High"]
    #[inline(always)]
    pub fn segd1h(&mut self) -> Segd1hW<'_, Segd1hSpec> {
        Segd1hW::new(self, 0)
    }
}
#[doc = "Segment Data High Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`segd1h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segd1h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Segd1hSpec;
impl crate::RegisterSpec for Segd1hSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`segd1h::R`](R) reader structure"]
impl crate::Readable for Segd1hSpec {}
#[doc = "`write(|w| ..)` method takes [`segd1h::W`](W) writer structure"]
impl crate::Writable for Segd1hSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEGD1H to value 0"]
impl crate::Resettable for Segd1hSpec {}
