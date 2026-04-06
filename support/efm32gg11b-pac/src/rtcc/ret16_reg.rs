#[doc = "Register `RET16_REG` reader"]
pub type R = crate::R<Ret16RegSpec>;
#[doc = "Register `RET16_REG` writer"]
pub type W = crate::W<Ret16RegSpec>;
#[doc = "Field `REG` reader - General Purpose Retention Register"]
pub type RegR = crate::FieldReader<u32>;
#[doc = "Field `REG` writer - General Purpose Retention Register"]
pub type RegW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - General Purpose Retention Register"]
    #[inline(always)]
    pub fn reg(&self) -> RegR {
        RegR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - General Purpose Retention Register"]
    #[inline(always)]
    pub fn reg(&mut self) -> RegW<'_, Ret16RegSpec> {
        RegW::new(self, 0)
    }
}
#[doc = "Retention Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ret16_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret16_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ret16RegSpec;
impl crate::RegisterSpec for Ret16RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ret16_reg::R`](R) reader structure"]
impl crate::Readable for Ret16RegSpec {}
#[doc = "`write(|w| ..)` method takes [`ret16_reg::W`](W) writer structure"]
impl crate::Writable for Ret16RegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RET16_REG to value 0"]
impl crate::Resettable for Ret16RegSpec {}
