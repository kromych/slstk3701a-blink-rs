#[doc = "Register `CH0DATA` reader"]
pub type R = crate::R<Ch0dataSpec>;
#[doc = "Register `CH0DATA` writer"]
pub type W = crate::W<Ch0dataSpec>;
#[doc = "Field `DATA` reader - Channel 0 Data"]
pub type DataR = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - Channel 0 Data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Channel 0 Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Channel 0 Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, Ch0dataSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Channel 0 Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch0dataSpec;
impl crate::RegisterSpec for Ch0dataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch0data::R`](R) reader structure"]
impl crate::Readable for Ch0dataSpec {}
#[doc = "`write(|w| ..)` method takes [`ch0data::W`](W) writer structure"]
impl crate::Writable for Ch0dataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH0DATA to value 0x0800"]
impl crate::Resettable for Ch0dataSpec {
    const RESET_VALUE: u32 = 0x0800;
}
