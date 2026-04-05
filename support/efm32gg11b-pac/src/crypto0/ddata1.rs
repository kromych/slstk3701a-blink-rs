#[doc = "Register `DDATA1` reader"]
pub type R = crate::R<Ddata1Spec>;
#[doc = "Register `DDATA1` writer"]
pub type W = crate::W<Ddata1Spec>;
#[doc = "Field `DDATA1` reader - Double Data 0 Access"]
pub type Ddata1R = crate::FieldReader<u32>;
#[doc = "Field `DDATA1` writer - Double Data 0 Access"]
pub type Ddata1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Double Data 0 Access"]
    #[inline(always)]
    pub fn ddata1(&self) -> Ddata1R {
        Ddata1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Double Data 0 Access"]
    #[inline(always)]
    pub fn ddata1(&mut self) -> Ddata1W<'_, Ddata1Spec> {
        Ddata1W::new(self, 0)
    }
}
#[doc = "DDATA1 Register Access\n\nYou can [`read`](crate::Reg::read) this register and get [`ddata1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddata1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct Ddata1Spec;
impl crate::RegisterSpec for Ddata1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddata1::R`](R) reader structure"]
impl crate::Readable for Ddata1Spec {}
#[doc = "`write(|w| ..)` method takes [`ddata1::W`](W) writer structure"]
impl crate::Writable for Ddata1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DDATA1 to value 0"]
impl crate::Resettable for Ddata1Spec {}
