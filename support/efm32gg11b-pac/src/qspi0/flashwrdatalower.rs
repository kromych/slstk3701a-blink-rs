#[doc = "Register `FLASHWRDATALOWER` reader"]
pub type R = crate::R<FlashwrdatalowerSpec>;
#[doc = "Register `FLASHWRDATALOWER` writer"]
pub type W = crate::W<FlashwrdatalowerSpec>;
#[doc = "Field `DATA` reader - Command Write Data Lower Byte"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Command Write Data Lower Byte"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command Write Data Lower Byte"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command Write Data Lower Byte"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, FlashwrdatalowerSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Flash Command Write Data Register (Lower) (STIG)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashwrdatalower::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashwrdatalower::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashwrdatalowerSpec;
impl crate::RegisterSpec for FlashwrdatalowerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashwrdatalower::R`](R) reader structure"]
impl crate::Readable for FlashwrdatalowerSpec {}
#[doc = "`write(|w| ..)` method takes [`flashwrdatalower::W`](W) writer structure"]
impl crate::Writable for FlashwrdatalowerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASHWRDATALOWER to value 0"]
impl crate::Resettable for FlashwrdatalowerSpec {}
