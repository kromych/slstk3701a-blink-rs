#[doc = "Register `TSUNSECCMP` reader"]
pub type R = crate::R<TsunseccmpSpec>;
#[doc = "Register `TSUNSECCMP` writer"]
pub type W = crate::W<TsunseccmpSpec>;
#[doc = "Field `COMPVAL` reader - TSU timer comparison value (ns)"]
pub type CompvalR = crate::FieldReader<u32>;
#[doc = "Field `COMPVAL` writer - TSU timer comparison value (ns)"]
pub type CompvalW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:21 - TSU timer comparison value (ns)"]
    #[inline(always)]
    pub fn compval(&self) -> CompvalR {
        CompvalR::new(self.bits & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:21 - TSU timer comparison value (ns)"]
    #[inline(always)]
    pub fn compval(&mut self) -> CompvalW<'_, TsunseccmpSpec> {
        CompvalW::new(self, 0)
    }
}
#[doc = "TSU timer comparison value nanoseconds\n\nYou can [`read`](crate::Reg::read) this register and get [`tsunseccmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsunseccmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsunseccmpSpec;
impl crate::RegisterSpec for TsunseccmpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsunseccmp::R`](R) reader structure"]
impl crate::Readable for TsunseccmpSpec {}
#[doc = "`write(|w| ..)` method takes [`tsunseccmp::W`](W) writer structure"]
impl crate::Writable for TsunseccmpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TSUNSECCMP to value 0"]
impl crate::Resettable for TsunseccmpSpec {}
