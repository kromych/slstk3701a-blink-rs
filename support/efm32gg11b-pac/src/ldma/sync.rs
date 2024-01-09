#[doc = "Register `SYNC` reader"]
pub type R = crate::R<SYNC_SPEC>;
#[doc = "Register `SYNC` writer"]
pub type W = crate::W<SYNC_SPEC>;
#[doc = "Field `SYNCTRIG` reader - Synchronization Trigger"]
pub type SYNCTRIG_R = crate::FieldReader;
#[doc = "Field `SYNCTRIG` writer - Synchronization Trigger"]
pub type SYNCTRIG_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Synchronization Trigger"]
    #[inline(always)]
    pub fn synctrig(&self) -> SYNCTRIG_R {
        SYNCTRIG_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Synchronization Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn synctrig(&mut self) -> SYNCTRIG_W<SYNC_SPEC> {
        SYNCTRIG_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA Synchronization Trigger Register (Single-Cycle RMW)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sync::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sync::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYNC_SPEC;
impl crate::RegisterSpec for SYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sync::R`](R) reader structure"]
impl crate::Readable for SYNC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sync::W`](W) writer structure"]
impl crate::Writable for SYNC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYNC to value 0"]
impl crate::Resettable for SYNC_SPEC {
    const RESET_VALUE: u32 = 0;
}
