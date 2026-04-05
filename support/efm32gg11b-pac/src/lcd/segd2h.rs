#[doc = "Register `SEGD2H` reader"]
pub type R = crate::R<Segd2hSpec>;
#[doc = "Register `SEGD2H` writer"]
pub type W = crate::W<Segd2hSpec>;
#[doc = "Field `SEGD2H` reader - COM2 Segment Data High"]
pub type Segd2hR = crate::FieldReader;
#[doc = "Field `SEGD2H` writer - COM2 Segment Data High"]
pub type Segd2hW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - COM2 Segment Data High"]
    #[inline(always)]
    pub fn segd2h(&self) -> Segd2hR {
        Segd2hR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - COM2 Segment Data High"]
    #[inline(always)]
    pub fn segd2h(&mut self) -> Segd2hW<'_, Segd2hSpec> {
        Segd2hW::new(self, 0)
    }
}
#[doc = "Segment Data High Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`segd2h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segd2h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Segd2hSpec;
impl crate::RegisterSpec for Segd2hSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`segd2h::R`](R) reader structure"]
impl crate::Readable for Segd2hSpec {}
#[doc = "`write(|w| ..)` method takes [`segd2h::W`](W) writer structure"]
impl crate::Writable for Segd2hSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEGD2H to value 0"]
impl crate::Resettable for Segd2hSpec {}
