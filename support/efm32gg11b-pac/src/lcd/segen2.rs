#[doc = "Register `SEGEN2` reader"]
pub type R = crate::R<Segen2Spec>;
#[doc = "Register `SEGEN2` writer"]
pub type W = crate::W<Segen2Spec>;
#[doc = "Field `SEGEN2` reader - Segment Enable (second Group)"]
pub type Segen2R = crate::FieldReader;
#[doc = "Field `SEGEN2` writer - Segment Enable (second Group)"]
pub type Segen2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Segment Enable (second Group)"]
    #[inline(always)]
    pub fn segen2(&self) -> Segen2R {
        Segen2R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Segment Enable (second Group)"]
    #[inline(always)]
    pub fn segen2(&mut self) -> Segen2W<'_, Segen2Spec> {
        Segen2W::new(self, 0)
    }
}
#[doc = "Segment Enable (32 to 39)\n\nYou can [`read`](crate::Reg::read) this register and get [`segen2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segen2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Segen2Spec;
impl crate::RegisterSpec for Segen2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`segen2::R`](R) reader structure"]
impl crate::Readable for Segen2Spec {}
#[doc = "`write(|w| ..)` method takes [`segen2::W`](W) writer structure"]
impl crate::Writable for Segen2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEGEN2 to value 0"]
impl crate::Resettable for Segen2Spec {}
