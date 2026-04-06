#[doc = "Register `UNDERSIZEFRAMES` reader"]
pub type R = crate::R<UndersizeframesSpec>;
#[doc = "Register `UNDERSIZEFRAMES` writer"]
pub type W = crate::W<UndersizeframesSpec>;
#[doc = "Field `COUNT` reader - Undersize frames received"]
pub type CountR = crate::FieldReader<u16>;
#[doc = "Field `COUNT` writer - Undersize frames received"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Undersize frames received"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Undersize frames received"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, UndersizeframesSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Undersized Frames Received\n\nYou can [`read`](crate::Reg::read) this register and get [`undersizeframes::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`undersizeframes::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UndersizeframesSpec;
impl crate::RegisterSpec for UndersizeframesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`undersizeframes::R`](R) reader structure"]
impl crate::Readable for UndersizeframesSpec {}
#[doc = "`write(|w| ..)` method takes [`undersizeframes::W`](W) writer structure"]
impl crate::Writable for UndersizeframesSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UNDERSIZEFRAMES to value 0"]
impl crate::Resettable for UndersizeframesSpec {}
