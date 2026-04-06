#[doc = "Register `SEGD3H` reader"]
pub type R = crate::R<Segd3hSpec>;
#[doc = "Register `SEGD3H` writer"]
pub type W = crate::W<Segd3hSpec>;
#[doc = "Field `SEGD3H` reader - COM3 Segment Data High"]
pub type Segd3hR = crate::FieldReader;
#[doc = "Field `SEGD3H` writer - COM3 Segment Data High"]
pub type Segd3hW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - COM3 Segment Data High"]
    #[inline(always)]
    pub fn segd3h(&self) -> Segd3hR {
        Segd3hR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - COM3 Segment Data High"]
    #[inline(always)]
    pub fn segd3h(&mut self) -> Segd3hW<'_, Segd3hSpec> {
        Segd3hW::new(self, 0)
    }
}
#[doc = "Segment Data High Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`segd3h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segd3h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Segd3hSpec;
impl crate::RegisterSpec for Segd3hSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`segd3h::R`](R) reader structure"]
impl crate::Readable for Segd3hSpec {}
#[doc = "`write(|w| ..)` method takes [`segd3h::W`](W) writer structure"]
impl crate::Writable for Segd3hSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEGD3H to value 0"]
impl crate::Resettable for Segd3hSpec {}
