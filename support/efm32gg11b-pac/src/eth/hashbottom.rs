#[doc = "Register `HASHBOTTOM` reader"]
pub type R = crate::R<HashbottomSpec>;
#[doc = "Register `HASHBOTTOM` writer"]
pub type W = crate::W<HashbottomSpec>;
#[doc = "Field `ADDR` reader - The first 32 bits of the hash address register."]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - The first 32 bits of the hash address register."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The first 32 bits of the hash address register."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The first 32 bits of the hash address register."]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, HashbottomSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Hash Register Bottom \\[31:0\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`hashbottom::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashbottom::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashbottomSpec;
impl crate::RegisterSpec for HashbottomSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hashbottom::R`](R) reader structure"]
impl crate::Readable for HashbottomSpec {}
#[doc = "`write(|w| ..)` method takes [`hashbottom::W`](W) writer structure"]
impl crate::Writable for HashbottomSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HASHBOTTOM to value 0"]
impl crate::Resettable for HashbottomSpec {}
