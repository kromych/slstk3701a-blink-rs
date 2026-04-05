#[doc = "Register `DDATA0BIG` reader"]
pub type R = crate::R<Ddata0bigSpec>;
#[doc = "Register `DDATA0BIG` writer"]
pub type W = crate::W<Ddata0bigSpec>;
#[doc = "Field `DDATA0BIG` reader - Double Data 0 Big Endian Access"]
pub type Ddata0bigR = crate::FieldReader<u32>;
#[doc = "Field `DDATA0BIG` writer - Double Data 0 Big Endian Access"]
pub type Ddata0bigW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Double Data 0 Big Endian Access"]
    #[inline(always)]
    pub fn ddata0big(&self) -> Ddata0bigR {
        Ddata0bigR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Double Data 0 Big Endian Access"]
    #[inline(always)]
    pub fn ddata0big(&mut self) -> Ddata0bigW<'_, Ddata0bigSpec> {
        Ddata0bigW::new(self, 0)
    }
}
#[doc = "DDATA0 Register Big Endian Access\n\nYou can [`read`](crate::Reg::read) this register and get [`ddata0big::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddata0big::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct Ddata0bigSpec;
impl crate::RegisterSpec for Ddata0bigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddata0big::R`](R) reader structure"]
impl crate::Readable for Ddata0bigSpec {}
#[doc = "`write(|w| ..)` method takes [`ddata0big::W`](W) writer structure"]
impl crate::Writable for Ddata0bigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DDATA0BIG to value 0"]
impl crate::Resettable for Ddata0bigSpec {}
