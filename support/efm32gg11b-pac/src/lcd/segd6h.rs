#[doc = "Register `SEGD6H` reader"]
pub type R = crate::R<Segd6hSpec>;
#[doc = "Register `SEGD6H` writer"]
pub type W = crate::W<Segd6hSpec>;
#[doc = "Field `SEGD6H` reader - COM2 Segment Data High"]
pub type Segd6hR = crate::FieldReader;
#[doc = "Field `SEGD6H` writer - COM2 Segment Data High"]
pub type Segd6hW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - COM2 Segment Data High"]
    #[inline(always)]
    pub fn segd6h(&self) -> Segd6hR {
        Segd6hR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - COM2 Segment Data High"]
    #[inline(always)]
    pub fn segd6h(&mut self) -> Segd6hW<'_, Segd6hSpec> {
        Segd6hW::new(self, 0)
    }
}
#[doc = "Segment Data High Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`segd6h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segd6h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Segd6hSpec;
impl crate::RegisterSpec for Segd6hSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`segd6h::R`](R) reader structure"]
impl crate::Readable for Segd6hSpec {}
#[doc = "`write(|w| ..)` method takes [`segd6h::W`](W) writer structure"]
impl crate::Writable for Segd6hSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEGD6H to value 0"]
impl crate::Resettable for Segd6hSpec {}
