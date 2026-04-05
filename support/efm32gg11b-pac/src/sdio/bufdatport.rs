#[doc = "Register `BUFDATPORT` reader"]
pub type R = crate::R<BufdatportSpec>;
#[doc = "Register `BUFDATPORT` writer"]
pub type W = crate::W<BufdatportSpec>;
#[doc = "Field `BUFDAT` reader - Buffer Data"]
pub type BufdatR = crate::FieldReader<u32>;
#[doc = "Field `BUFDAT` writer - Buffer Data"]
pub type BufdatW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer Data"]
    #[inline(always)]
    pub fn bufdat(&self) -> BufdatR {
        BufdatR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer Data"]
    #[inline(always)]
    pub fn bufdat(&mut self) -> BufdatW<'_, BufdatportSpec> {
        BufdatW::new(self, 0)
    }
}
#[doc = "Buffer Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bufdatport::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bufdatport::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BufdatportSpec;
impl crate::RegisterSpec for BufdatportSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bufdatport::R`](R) reader structure"]
impl crate::Readable for BufdatportSpec {}
#[doc = "`write(|w| ..)` method takes [`bufdatport::W`](W) writer structure"]
impl crate::Writable for BufdatportSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUFDATPORT to value 0"]
impl crate::Resettable for BufdatportSpec {}
