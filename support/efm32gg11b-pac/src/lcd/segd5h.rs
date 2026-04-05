#[doc = "Register `SEGD5H` reader"]
pub type R = crate::R<Segd5hSpec>;
#[doc = "Register `SEGD5H` writer"]
pub type W = crate::W<Segd5hSpec>;
#[doc = "Field `SEGD5H` reader - COM1 Segment Data High"]
pub type Segd5hR = crate::FieldReader;
#[doc = "Field `SEGD5H` writer - COM1 Segment Data High"]
pub type Segd5hW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - COM1 Segment Data High"]
    #[inline(always)]
    pub fn segd5h(&self) -> Segd5hR {
        Segd5hR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - COM1 Segment Data High"]
    #[inline(always)]
    pub fn segd5h(&mut self) -> Segd5hW<'_, Segd5hSpec> {
        Segd5hW::new(self, 0)
    }
}
#[doc = "Segment Data High Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`segd5h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segd5h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Segd5hSpec;
impl crate::RegisterSpec for Segd5hSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`segd5h::R`](R) reader structure"]
impl crate::Readable for Segd5hSpec {}
#[doc = "`write(|w| ..)` method takes [`segd5h::W`](W) writer structure"]
impl crate::Writable for Segd5hSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEGD5H to value 0"]
impl crate::Resettable for Segd5hSpec {}
