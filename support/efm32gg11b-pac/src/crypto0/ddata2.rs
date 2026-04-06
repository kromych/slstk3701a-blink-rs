#[doc = "Register `DDATA2` reader"]
pub type R = crate::R<Ddata2Spec>;
#[doc = "Register `DDATA2` writer"]
pub type W = crate::W<Ddata2Spec>;
#[doc = "Field `DDATA2` reader - Double Data 0 Access"]
pub type Ddata2R = crate::FieldReader<u32>;
#[doc = "Field `DDATA2` writer - Double Data 0 Access"]
pub type Ddata2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Double Data 0 Access"]
    #[inline(always)]
    pub fn ddata2(&self) -> Ddata2R {
        Ddata2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Double Data 0 Access"]
    #[inline(always)]
    pub fn ddata2(&mut self) -> Ddata2W<'_, Ddata2Spec> {
        Ddata2W::new(self, 0)
    }
}
#[doc = "DDATA2 Register Access\n\nYou can [`read`](crate::Reg::read) this register and get [`ddata2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddata2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct Ddata2Spec;
impl crate::RegisterSpec for Ddata2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddata2::R`](R) reader structure"]
impl crate::Readable for Ddata2Spec {}
#[doc = "`write(|w| ..)` method takes [`ddata2::W`](W) writer structure"]
impl crate::Writable for Ddata2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DDATA2 to value 0"]
impl crate::Resettable for Ddata2Spec {}
