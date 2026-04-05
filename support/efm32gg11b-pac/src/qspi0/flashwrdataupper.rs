#[doc = "Register `FLASHWRDATAUPPER` reader"]
pub type R = crate::R<FlashwrdataupperSpec>;
#[doc = "Register `FLASHWRDATAUPPER` writer"]
pub type W = crate::W<FlashwrdataupperSpec>;
#[doc = "Field `DATA` reader - Command Write Data Upper Byte"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Command Write Data Upper Byte"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command Write Data Upper Byte"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command Write Data Upper Byte"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, FlashwrdataupperSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Flash Command Write Data Register (Upper) (STIG)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashwrdataupper::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashwrdataupper::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashwrdataupperSpec;
impl crate::RegisterSpec for FlashwrdataupperSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashwrdataupper::R`](R) reader structure"]
impl crate::Readable for FlashwrdataupperSpec {}
#[doc = "`write(|w| ..)` method takes [`flashwrdataupper::W`](W) writer structure"]
impl crate::Writable for FlashwrdataupperSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASHWRDATAUPPER to value 0"]
impl crate::Resettable for FlashwrdataupperSpec {}
