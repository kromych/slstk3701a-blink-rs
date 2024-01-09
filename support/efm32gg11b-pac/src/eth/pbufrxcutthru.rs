#[doc = "Register `PBUFRXCUTTHRU` reader"]
pub type R = crate::R<PBUFRXCUTTHRU_SPEC>;
#[doc = "Register `PBUFRXCUTTHRU` writer"]
pub type W = crate::W<PBUFRXCUTTHRU_SPEC>;
#[doc = "Field `DMARXCUTTHRUTHR` reader - Watermark value"]
pub type DMARXCUTTHRUTHR_R = crate::FieldReader<u16>;
#[doc = "Field `DMARXCUTTHRUTHR` writer - Watermark value"]
pub type DMARXCUTTHRUTHR_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DMARXCUTTHRU` reader - Enable RX partial store and forward operation"]
pub type DMARXCUTTHRU_R = crate::BitReader;
#[doc = "Field `DMARXCUTTHRU` writer - Enable RX partial store and forward operation"]
pub type DMARXCUTTHRU_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Watermark value"]
    #[inline(always)]
    pub fn dmarxcutthruthr(&self) -> DMARXCUTTHRUTHR_R {
        DMARXCUTTHRUTHR_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - Enable RX partial store and forward operation"]
    #[inline(always)]
    pub fn dmarxcutthru(&self) -> DMARXCUTTHRU_R {
        DMARXCUTTHRU_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Watermark value"]
    #[inline(always)]
    #[must_use]
    pub fn dmarxcutthruthr(&mut self) -> DMARXCUTTHRUTHR_W<PBUFRXCUTTHRU_SPEC> {
        DMARXCUTTHRUTHR_W::new(self, 0)
    }
    #[doc = "Bit 31 - Enable RX partial store and forward operation"]
    #[inline(always)]
    #[must_use]
    pub fn dmarxcutthru(&mut self) -> DMARXCUTTHRU_W<PBUFRXCUTTHRU_SPEC> {
        DMARXCUTTHRU_W::new(self, 31)
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
#[doc = "RX Partial Store and Forward\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pbufrxcutthru::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pbufrxcutthru::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PBUFRXCUTTHRU_SPEC;
impl crate::RegisterSpec for PBUFRXCUTTHRU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbufrxcutthru::R`](R) reader structure"]
impl crate::Readable for PBUFRXCUTTHRU_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pbufrxcutthru::W`](W) writer structure"]
impl crate::Writable for PBUFRXCUTTHRU_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PBUFRXCUTTHRU to value 0x03ff"]
impl crate::Resettable for PBUFRXCUTTHRU_SPEC {
    const RESET_VALUE: u32 = 0x03ff;
}
