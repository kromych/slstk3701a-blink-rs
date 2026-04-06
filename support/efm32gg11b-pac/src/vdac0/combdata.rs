#[doc = "Register `COMBDATA` reader"]
pub type R = crate::R<CombdataSpec>;
#[doc = "Register `COMBDATA` writer"]
pub type W = crate::W<CombdataSpec>;
#[doc = "Field `CH0DATA` reader - Channel 0 Data"]
pub type Ch0dataR = crate::FieldReader<u16>;
#[doc = "Field `CH0DATA` writer - Channel 0 Data"]
pub type Ch0dataW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `CH1DATA` reader - Channel 1 Data"]
pub type Ch1dataR = crate::FieldReader<u16>;
#[doc = "Field `CH1DATA` writer - Channel 1 Data"]
pub type Ch1dataW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Channel 0 Data"]
    #[inline(always)]
    pub fn ch0data(&self) -> Ch0dataR {
        Ch0dataR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Channel 1 Data"]
    #[inline(always)]
    pub fn ch1data(&self) -> Ch1dataR {
        Ch1dataR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Channel 0 Data"]
    #[inline(always)]
    pub fn ch0data(&mut self) -> Ch0dataW<'_, CombdataSpec> {
        Ch0dataW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Channel 1 Data"]
    #[inline(always)]
    pub fn ch1data(&mut self) -> Ch1dataW<'_, CombdataSpec> {
        Ch1dataW::new(self, 16)
    }
}
#[doc = "Combined Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`combdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`combdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CombdataSpec;
impl crate::RegisterSpec for CombdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`combdata::R`](R) reader structure"]
impl crate::Readable for CombdataSpec {}
#[doc = "`write(|w| ..)` method takes [`combdata::W`](W) writer structure"]
impl crate::Writable for CombdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMBDATA to value 0x0800_0800"]
impl crate::Resettable for CombdataSpec {
    const RESET_VALUE: u32 = 0x0800_0800;
}
