#[doc = "Register `QDATA1` reader"]
pub type R = crate::R<Qdata1Spec>;
#[doc = "Register `QDATA1` writer"]
pub type W = crate::W<Qdata1Spec>;
#[doc = "Field `QDATA1` reader - Quad Data 1 Access"]
pub type Qdata1R = crate::FieldReader<u32>;
#[doc = "Field `QDATA1` writer - Quad Data 1 Access"]
pub type Qdata1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Quad Data 1 Access"]
    #[inline(always)]
    pub fn qdata1(&self) -> Qdata1R {
        Qdata1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Quad Data 1 Access"]
    #[inline(always)]
    pub fn qdata1(&mut self) -> Qdata1W<'_, Qdata1Spec> {
        Qdata1W::new(self, 0)
    }
}
#[doc = "QDATA1 Register Access\n\nYou can [`read`](crate::Reg::read) this register and get [`qdata1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qdata1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct Qdata1Spec;
impl crate::RegisterSpec for Qdata1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qdata1::R`](R) reader structure"]
impl crate::Readable for Qdata1Spec {}
#[doc = "`write(|w| ..)` method takes [`qdata1::W`](W) writer structure"]
impl crate::Writable for Qdata1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets QDATA1 to value 0"]
impl crate::Resettable for Qdata1Spec {}
