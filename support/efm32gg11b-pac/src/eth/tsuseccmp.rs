#[doc = "Register `TSUSECCMP` reader"]
pub type R = crate::R<TsuseccmpSpec>;
#[doc = "Register `TSUSECCMP` writer"]
pub type W = crate::W<TsuseccmpSpec>;
#[doc = "Field `COMPVAL` reader - TSU timer comparison value (s)"]
pub type CompvalR = crate::FieldReader<u32>;
#[doc = "Field `COMPVAL` writer - TSU timer comparison value (s)"]
pub type CompvalW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TSU timer comparison value (s)"]
    #[inline(always)]
    pub fn compval(&self) -> CompvalR {
        CompvalR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TSU timer comparison value (s)"]
    #[inline(always)]
    pub fn compval(&mut self) -> CompvalW<'_, TsuseccmpSpec> {
        CompvalW::new(self, 0)
    }
}
#[doc = "TSU timer comparison value seconds \\[31:0\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`tsuseccmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsuseccmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsuseccmpSpec;
impl crate::RegisterSpec for TsuseccmpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsuseccmp::R`](R) reader structure"]
impl crate::Readable for TsuseccmpSpec {}
#[doc = "`write(|w| ..)` method takes [`tsuseccmp::W`](W) writer structure"]
impl crate::Writable for TsuseccmpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TSUSECCMP to value 0"]
impl crate::Resettable for TsuseccmpSpec {}
