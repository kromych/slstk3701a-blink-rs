#[doc = "Register `SEGD4H` reader"]
pub type R = crate::R<Segd4hSpec>;
#[doc = "Register `SEGD4H` writer"]
pub type W = crate::W<Segd4hSpec>;
#[doc = "Field `SEGD4H` reader - COM0 Segment Data High"]
pub type Segd4hR = crate::FieldReader;
#[doc = "Field `SEGD4H` writer - COM0 Segment Data High"]
pub type Segd4hW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - COM0 Segment Data High"]
    #[inline(always)]
    pub fn segd4h(&self) -> Segd4hR {
        Segd4hR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - COM0 Segment Data High"]
    #[inline(always)]
    pub fn segd4h(&mut self) -> Segd4hW<'_, Segd4hSpec> {
        Segd4hW::new(self, 0)
    }
}
#[doc = "Segment Data High Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`segd4h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segd4h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Segd4hSpec;
impl crate::RegisterSpec for Segd4hSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`segd4h::R`](R) reader structure"]
impl crate::Readable for Segd4hSpec {}
#[doc = "`write(|w| ..)` method takes [`segd4h::W`](W) writer structure"]
impl crate::Writable for Segd4hSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEGD4H to value 0"]
impl crate::Resettable for Segd4hSpec {}
