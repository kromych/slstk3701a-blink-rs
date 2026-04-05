#[doc = "Register `MASKADD1TOP` reader"]
pub type R = crate::R<Maskadd1topSpec>;
#[doc = "Register `MASKADD1TOP` writer"]
pub type W = crate::W<Maskadd1topSpec>;
#[doc = "Field `ADDRMASK` reader - Specific Address Mask"]
pub type AddrmaskR = crate::FieldReader<u16>;
#[doc = "Field `ADDRMASK` writer - Specific Address Mask"]
pub type AddrmaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Specific Address Mask"]
    #[inline(always)]
    pub fn addrmask(&self) -> AddrmaskR {
        AddrmaskR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Specific Address Mask"]
    #[inline(always)]
    pub fn addrmask(&mut self) -> AddrmaskW<'_, Maskadd1topSpec> {
        AddrmaskW::new(self, 0)
    }
}
#[doc = "Specific Address Mask 1 Top 47:32\n\nYou can [`read`](crate::Reg::read) this register and get [`maskadd1top::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maskadd1top::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Maskadd1topSpec;
impl crate::RegisterSpec for Maskadd1topSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maskadd1top::R`](R) reader structure"]
impl crate::Readable for Maskadd1topSpec {}
#[doc = "`write(|w| ..)` method takes [`maskadd1top::W`](W) writer structure"]
impl crate::Writable for Maskadd1topSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MASKADD1TOP to value 0"]
impl crate::Resettable for Maskadd1topSpec {}
