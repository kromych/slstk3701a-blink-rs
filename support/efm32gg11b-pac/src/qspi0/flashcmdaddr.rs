#[doc = "Register `FLASHCMDADDR` reader"]
pub type R = crate::R<FlashcmdaddrSpec>;
#[doc = "Register `FLASHCMDADDR` writer"]
pub type W = crate::W<FlashcmdaddrSpec>;
#[doc = "Field `ADDR` reader - Command Address"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Command Address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command Address"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, FlashcmdaddrSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Flash Command Address Register (STIG)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashcmdaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashcmdaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashcmdaddrSpec;
impl crate::RegisterSpec for FlashcmdaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashcmdaddr::R`](R) reader structure"]
impl crate::Readable for FlashcmdaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`flashcmdaddr::W`](W) writer structure"]
impl crate::Writable for FlashcmdaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASHCMDADDR to value 0"]
impl crate::Resettable for FlashcmdaddrSpec {}
