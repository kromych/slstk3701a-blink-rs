#[doc = "Register `PHYCONFIGURATION` reader"]
pub type R = crate::R<PHYCONFIGURATION_SPEC>;
#[doc = "Register `PHYCONFIGURATION` writer"]
pub type W = crate::W<PHYCONFIGURATION_SPEC>;
#[doc = "Field `PHYCONFIGRXDLLDELAY` reader - RX DLL Delay"]
pub type PHYCONFIGRXDLLDELAY_R = crate::FieldReader;
#[doc = "Field `PHYCONFIGRXDLLDELAY` writer - RX DLL Delay"]
pub type PHYCONFIGRXDLLDELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PHYCONFIGTXDLLDELAY` reader - TX DLL Delay"]
pub type PHYCONFIGTXDLLDELAY_R = crate::FieldReader;
#[doc = "Field `PHYCONFIGTXDLLDELAY` writer - TX DLL Delay"]
pub type PHYCONFIGTXDLLDELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PHYCONFIGRESYNC` writer - PHY Config Resync"]
pub type PHYCONFIGRESYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - RX DLL Delay"]
    #[inline(always)]
    pub fn phyconfigrxdlldelay(&self) -> PHYCONFIGRXDLLDELAY_R {
        PHYCONFIGRXDLLDELAY_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - TX DLL Delay"]
    #[inline(always)]
    pub fn phyconfigtxdlldelay(&self) -> PHYCONFIGTXDLLDELAY_R {
        PHYCONFIGTXDLLDELAY_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - RX DLL Delay"]
    #[inline(always)]
    #[must_use]
    pub fn phyconfigrxdlldelay(&mut self) -> PHYCONFIGRXDLLDELAY_W<PHYCONFIGURATION_SPEC> {
        PHYCONFIGRXDLLDELAY_W::new(self, 0)
    }
    #[doc = "Bits 16:22 - TX DLL Delay"]
    #[inline(always)]
    #[must_use]
    pub fn phyconfigtxdlldelay(&mut self) -> PHYCONFIGTXDLLDELAY_W<PHYCONFIGURATION_SPEC> {
        PHYCONFIGTXDLLDELAY_W::new(self, 16)
    }
    #[doc = "Bit 31 - PHY Config Resync"]
    #[inline(always)]
    #[must_use]
    pub fn phyconfigresync(&mut self) -> PHYCONFIGRESYNC_W<PHYCONFIGURATION_SPEC> {
        PHYCONFIGRESYNC_W::new(self, 31)
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
#[doc = "PHY Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phyconfiguration::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phyconfiguration::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PHYCONFIGURATION_SPEC;
impl crate::RegisterSpec for PHYCONFIGURATION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phyconfiguration::R`](R) reader structure"]
impl crate::Readable for PHYCONFIGURATION_SPEC {}
#[doc = "`write(|w| ..)` method takes [`phyconfiguration::W`](W) writer structure"]
impl crate::Writable for PHYCONFIGURATION_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PHYCONFIGURATION to value 0"]
impl crate::Resettable for PHYCONFIGURATION_SPEC {
    const RESET_VALUE: u32 = 0;
}
