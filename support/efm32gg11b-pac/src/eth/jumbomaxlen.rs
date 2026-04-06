#[doc = "Register `JUMBOMAXLEN` reader"]
pub type R = crate::R<JumbomaxlenSpec>;
#[doc = "Register `JUMBOMAXLEN` writer"]
pub type W = crate::W<JumbomaxlenSpec>;
#[doc = "Field `JUMBOMAXLEN` reader - Maximum Jumbo Frame Size - resets to the gem_jumbo_max_length define value."]
pub type JumbomaxlenR = crate::FieldReader<u16>;
#[doc = "Field `JUMBOMAXLEN` writer - Maximum Jumbo Frame Size - resets to the gem_jumbo_max_length define value."]
pub type JumbomaxlenW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Maximum Jumbo Frame Size - resets to the gem_jumbo_max_length define value."]
    #[inline(always)]
    pub fn jumbomaxlen(&self) -> JumbomaxlenR {
        JumbomaxlenR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Maximum Jumbo Frame Size - resets to the gem_jumbo_max_length define value."]
    #[inline(always)]
    pub fn jumbomaxlen(&mut self) -> JumbomaxlenW<'_, JumbomaxlenSpec> {
        JumbomaxlenW::new(self, 0)
    }
}
#[doc = "Maximum Jumbo Frame Size.\n\nYou can [`read`](crate::Reg::read) this register and get [`jumbomaxlen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jumbomaxlen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JumbomaxlenSpec;
impl crate::RegisterSpec for JumbomaxlenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jumbomaxlen::R`](R) reader structure"]
impl crate::Readable for JumbomaxlenSpec {}
#[doc = "`write(|w| ..)` method takes [`jumbomaxlen::W`](W) writer structure"]
impl crate::Writable for JumbomaxlenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets JUMBOMAXLEN to value 0x2800"]
impl crate::Resettable for JumbomaxlenSpec {
    const RESET_VALUE: u32 = 0x2800;
}
