#[doc = "Register `TSUMSBSECCMP` reader"]
pub type R = crate::R<TsumsbseccmpSpec>;
#[doc = "Register `TSUMSBSECCMP` writer"]
pub type W = crate::W<TsumsbseccmpSpec>;
#[doc = "Field `COMPVAL` reader - TSU timer comparison value (s)"]
pub type CompvalR = crate::FieldReader<u16>;
#[doc = "Field `COMPVAL` writer - TSU timer comparison value (s)"]
pub type CompvalW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - TSU timer comparison value (s)"]
    #[inline(always)]
    pub fn compval(&self) -> CompvalR {
        CompvalR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TSU timer comparison value (s)"]
    #[inline(always)]
    pub fn compval(&mut self) -> CompvalW<'_, TsumsbseccmpSpec> {
        CompvalW::new(self, 0)
    }
}
#[doc = "TSU timer comparison value seconds \\[47:32\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`tsumsbseccmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsumsbseccmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsumsbseccmpSpec;
impl crate::RegisterSpec for TsumsbseccmpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsumsbseccmp::R`](R) reader structure"]
impl crate::Readable for TsumsbseccmpSpec {}
#[doc = "`write(|w| ..)` method takes [`tsumsbseccmp::W`](W) writer structure"]
impl crate::Writable for TsumsbseccmpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TSUMSBSECCMP to value 0"]
impl crate::Resettable for TsumsbseccmpSpec {}
