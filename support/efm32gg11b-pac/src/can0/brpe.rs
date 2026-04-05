#[doc = "Register `BRPE` reader"]
pub type R = crate::R<BrpeSpec>;
#[doc = "Register `BRPE` writer"]
pub type W = crate::W<BrpeSpec>;
#[doc = "Field `BRPE` reader - Baud Rate Prescaler Extension"]
pub type BrpeR = crate::FieldReader;
#[doc = "Field `BRPE` writer - Baud Rate Prescaler Extension"]
pub type BrpeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Baud Rate Prescaler Extension"]
    #[inline(always)]
    pub fn brpe(&self) -> BrpeR {
        BrpeR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Baud Rate Prescaler Extension"]
    #[inline(always)]
    pub fn brpe(&mut self) -> BrpeW<'_, BrpeSpec> {
        BrpeW::new(self, 0)
    }
}
#[doc = "BRP Extension Register\n\nYou can [`read`](crate::Reg::read) this register and get [`brpe::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brpe::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrpeSpec;
impl crate::RegisterSpec for BrpeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brpe::R`](R) reader structure"]
impl crate::Readable for BrpeSpec {}
#[doc = "`write(|w| ..)` method takes [`brpe::W`](W) writer structure"]
impl crate::Writable for BrpeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BRPE to value 0"]
impl crate::Resettable for BrpeSpec {}
