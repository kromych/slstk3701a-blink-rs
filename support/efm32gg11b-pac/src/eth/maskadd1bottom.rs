#[doc = "Register `MASKADD1BOTTOM` reader"]
pub type R = crate::R<Maskadd1bottomSpec>;
#[doc = "Register `MASKADD1BOTTOM` writer"]
pub type W = crate::W<Maskadd1bottomSpec>;
#[doc = "Field `ADDRMASK` reader - Specific Address Mask"]
pub type AddrmaskR = crate::FieldReader<u32>;
#[doc = "Field `ADDRMASK` writer - Specific Address Mask"]
pub type AddrmaskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specific Address Mask"]
    #[inline(always)]
    pub fn addrmask(&self) -> AddrmaskR {
        AddrmaskR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specific Address Mask"]
    #[inline(always)]
    pub fn addrmask(&mut self) -> AddrmaskW<'_, Maskadd1bottomSpec> {
        AddrmaskW::new(self, 0)
    }
}
#[doc = "Specific Address Mask 1 Bottom 31:0\n\nYou can [`read`](crate::Reg::read) this register and get [`maskadd1bottom::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maskadd1bottom::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Maskadd1bottomSpec;
impl crate::RegisterSpec for Maskadd1bottomSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maskadd1bottom::R`](R) reader structure"]
impl crate::Readable for Maskadd1bottomSpec {}
#[doc = "`write(|w| ..)` method takes [`maskadd1bottom::W`](W) writer structure"]
impl crate::Writable for Maskadd1bottomSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MASKADD1BOTTOM to value 0"]
impl crate::Resettable for Maskadd1bottomSpec {}
