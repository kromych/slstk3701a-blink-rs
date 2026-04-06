#[doc = "Register `DDATA0` reader"]
pub type R = crate::R<Ddata0Spec>;
#[doc = "Register `DDATA0` writer"]
pub type W = crate::W<Ddata0Spec>;
#[doc = "Field `DDATA0` reader - Double Data 0 Access"]
pub type Ddata0R = crate::FieldReader<u32>;
#[doc = "Field `DDATA0` writer - Double Data 0 Access"]
pub type Ddata0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Double Data 0 Access"]
    #[inline(always)]
    pub fn ddata0(&self) -> Ddata0R {
        Ddata0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Double Data 0 Access"]
    #[inline(always)]
    pub fn ddata0(&mut self) -> Ddata0W<'_, Ddata0Spec> {
        Ddata0W::new(self, 0)
    }
}
#[doc = "DDATA0 Register Access\n\nYou can [`read`](crate::Reg::read) this register and get [`ddata0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddata0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct Ddata0Spec;
impl crate::RegisterSpec for Ddata0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddata0::R`](R) reader structure"]
impl crate::Readable for Ddata0Spec {}
#[doc = "`write(|w| ..)` method takes [`ddata0::W`](W) writer structure"]
impl crate::Writable for Ddata0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DDATA0 to value 0"]
impl crate::Resettable for Ddata0Spec {}
