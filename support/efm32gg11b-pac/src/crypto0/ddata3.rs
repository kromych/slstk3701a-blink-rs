#[doc = "Register `DDATA3` reader"]
pub type R = crate::R<Ddata3Spec>;
#[doc = "Register `DDATA3` writer"]
pub type W = crate::W<Ddata3Spec>;
#[doc = "Field `DDATA3` reader - Double Data 0 Access"]
pub type Ddata3R = crate::FieldReader<u32>;
#[doc = "Field `DDATA3` writer - Double Data 0 Access"]
pub type Ddata3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Double Data 0 Access"]
    #[inline(always)]
    pub fn ddata3(&self) -> Ddata3R {
        Ddata3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Double Data 0 Access"]
    #[inline(always)]
    pub fn ddata3(&mut self) -> Ddata3W<'_, Ddata3Spec> {
        Ddata3W::new(self, 0)
    }
}
#[doc = "DDATA3 Register Access\n\nYou can [`read`](crate::Reg::read) this register and get [`ddata3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddata3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct Ddata3Spec;
impl crate::RegisterSpec for Ddata3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddata3::R`](R) reader structure"]
impl crate::Readable for Ddata3Spec {}
#[doc = "`write(|w| ..)` method takes [`ddata3::W`](W) writer structure"]
impl crate::Writable for Ddata3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DDATA3 to value 0"]
impl crate::Resettable for Ddata3Spec {}
