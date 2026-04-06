#[doc = "Register `QDATA0` reader"]
pub type R = crate::R<Qdata0Spec>;
#[doc = "Register `QDATA0` writer"]
pub type W = crate::W<Qdata0Spec>;
#[doc = "Field `QDATA0` reader - Quad Data 0 Access"]
pub type Qdata0R = crate::FieldReader<u32>;
#[doc = "Field `QDATA0` writer - Quad Data 0 Access"]
pub type Qdata0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Quad Data 0 Access"]
    #[inline(always)]
    pub fn qdata0(&self) -> Qdata0R {
        Qdata0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Quad Data 0 Access"]
    #[inline(always)]
    pub fn qdata0(&mut self) -> Qdata0W<'_, Qdata0Spec> {
        Qdata0W::new(self, 0)
    }
}
#[doc = "QDATA0 Register Access\n\nYou can [`read`](crate::Reg::read) this register and get [`qdata0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qdata0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct Qdata0Spec;
impl crate::RegisterSpec for Qdata0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qdata0::R`](R) reader structure"]
impl crate::Readable for Qdata0Spec {}
#[doc = "`write(|w| ..)` method takes [`qdata0::W`](W) writer structure"]
impl crate::Writable for Qdata0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets QDATA0 to value 0"]
impl crate::Resettable for Qdata0Spec {}
