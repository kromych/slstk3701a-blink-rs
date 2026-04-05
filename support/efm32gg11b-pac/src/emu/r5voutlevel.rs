#[doc = "Register `R5VOUTLEVEL` reader"]
pub type R = crate::R<R5voutlevelSpec>;
#[doc = "Register `R5VOUTLEVEL` writer"]
pub type W = crate::W<R5voutlevelSpec>;
#[doc = "Field `OUTLEVEL` reader - 5V Regulator Voltage"]
pub type OutlevelR = crate::FieldReader;
#[doc = "Field `OUTLEVEL` writer - 5V Regulator Voltage"]
pub type OutlevelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 5V Regulator Voltage"]
    #[inline(always)]
    pub fn outlevel(&self) -> OutlevelR {
        OutlevelR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 5V Regulator Voltage"]
    #[inline(always)]
    pub fn outlevel(&mut self) -> OutlevelW<'_, R5voutlevelSpec> {
        OutlevelW::new(self, 0)
    }
}
#[doc = "5V Regulator Voltage Select\n\nYou can [`read`](crate::Reg::read) this register and get [`r5voutlevel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5voutlevel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R5voutlevelSpec;
impl crate::RegisterSpec for R5voutlevelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r5voutlevel::R`](R) reader structure"]
impl crate::Readable for R5voutlevelSpec {}
#[doc = "`write(|w| ..)` method takes [`r5voutlevel::W`](W) writer structure"]
impl crate::Writable for R5voutlevelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R5VOUTLEVEL to value 0x01"]
impl crate::Resettable for R5voutlevelSpec {
    const RESET_VALUE: u32 = 0x01;
}
