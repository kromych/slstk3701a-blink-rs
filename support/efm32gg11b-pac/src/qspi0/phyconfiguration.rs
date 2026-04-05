#[doc = "Register `PHYCONFIGURATION` reader"]
pub type R = crate::R<PhyconfigurationSpec>;
#[doc = "Register `PHYCONFIGURATION` writer"]
pub type W = crate::W<PhyconfigurationSpec>;
#[doc = "Field `PHYCONFIGRXDLLDELAY` reader - RX DLL Delay"]
pub type PhyconfigrxdlldelayR = crate::FieldReader;
#[doc = "Field `PHYCONFIGRXDLLDELAY` writer - RX DLL Delay"]
pub type PhyconfigrxdlldelayW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PHYCONFIGTXDLLDELAY` reader - TX DLL Delay"]
pub type PhyconfigtxdlldelayR = crate::FieldReader;
#[doc = "Field `PHYCONFIGTXDLLDELAY` writer - TX DLL Delay"]
pub type PhyconfigtxdlldelayW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PHYCONFIGRESYNC` writer - PHY Config Resync"]
pub type PhyconfigresyncW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - RX DLL Delay"]
    #[inline(always)]
    pub fn phyconfigrxdlldelay(&self) -> PhyconfigrxdlldelayR {
        PhyconfigrxdlldelayR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - TX DLL Delay"]
    #[inline(always)]
    pub fn phyconfigtxdlldelay(&self) -> PhyconfigtxdlldelayR {
        PhyconfigtxdlldelayR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - RX DLL Delay"]
    #[inline(always)]
    pub fn phyconfigrxdlldelay(&mut self) -> PhyconfigrxdlldelayW<'_, PhyconfigurationSpec> {
        PhyconfigrxdlldelayW::new(self, 0)
    }
    #[doc = "Bits 16:22 - TX DLL Delay"]
    #[inline(always)]
    pub fn phyconfigtxdlldelay(&mut self) -> PhyconfigtxdlldelayW<'_, PhyconfigurationSpec> {
        PhyconfigtxdlldelayW::new(self, 16)
    }
    #[doc = "Bit 31 - PHY Config Resync"]
    #[inline(always)]
    pub fn phyconfigresync(&mut self) -> PhyconfigresyncW<'_, PhyconfigurationSpec> {
        PhyconfigresyncW::new(self, 31)
    }
}
#[doc = "PHY Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`phyconfiguration::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phyconfiguration::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyconfigurationSpec;
impl crate::RegisterSpec for PhyconfigurationSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phyconfiguration::R`](R) reader structure"]
impl crate::Readable for PhyconfigurationSpec {}
#[doc = "`write(|w| ..)` method takes [`phyconfiguration::W`](W) writer structure"]
impl crate::Writable for PhyconfigurationSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHYCONFIGURATION to value 0"]
impl crate::Resettable for PhyconfigurationSpec {}
