#[doc = "Register `HASHTOP` reader"]
pub type R = crate::R<HashtopSpec>;
#[doc = "Register `HASHTOP` writer"]
pub type W = crate::W<HashtopSpec>;
#[doc = "Field `ADDR` reader - The remaining 32 bits of the hash address register."]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - The remaining 32 bits of the hash address register."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The remaining 32 bits of the hash address register."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The remaining 32 bits of the hash address register."]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, HashtopSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Hash Register Top \\[63:32\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`hashtop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashtop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashtopSpec;
impl crate::RegisterSpec for HashtopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hashtop::R`](R) reader structure"]
impl crate::Readable for HashtopSpec {}
#[doc = "`write(|w| ..)` method takes [`hashtop::W`](W) writer structure"]
impl crate::Writable for HashtopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HASHTOP to value 0"]
impl crate::Resettable for HashtopSpec {}
