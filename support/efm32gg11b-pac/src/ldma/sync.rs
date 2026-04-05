#[doc = "Register `SYNC` reader"]
pub type R = crate::R<SyncSpec>;
#[doc = "Register `SYNC` writer"]
pub type W = crate::W<SyncSpec>;
#[doc = "Field `SYNCTRIG` reader - Synchronization Trigger"]
pub type SynctrigR = crate::FieldReader;
#[doc = "Field `SYNCTRIG` writer - Synchronization Trigger"]
pub type SynctrigW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Synchronization Trigger"]
    #[inline(always)]
    pub fn synctrig(&self) -> SynctrigR {
        SynctrigR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Synchronization Trigger"]
    #[inline(always)]
    pub fn synctrig(&mut self) -> SynctrigW<'_, SyncSpec> {
        SynctrigW::new(self, 0)
    }
}
#[doc = "DMA Synchronization Trigger Register (Single-Cycle RMW)\n\nYou can [`read`](crate::Reg::read) this register and get [`sync::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sync::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncSpec;
impl crate::RegisterSpec for SyncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sync::R`](R) reader structure"]
impl crate::Readable for SyncSpec {}
#[doc = "`write(|w| ..)` method takes [`sync::W`](W) writer structure"]
impl crate::Writable for SyncSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYNC to value 0"]
impl crate::Resettable for SyncSpec {}
