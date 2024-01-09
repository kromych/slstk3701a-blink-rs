#[doc = "Register `PBUFTXCUTTHRU` reader"]
pub type R = crate::R<PBUFTXCUTTHRU_SPEC>;
#[doc = "Register `PBUFTXCUTTHRU` writer"]
pub type W = crate::W<PBUFTXCUTTHRU_SPEC>;
#[doc = "Field `DMATXCUTTHRUTHR` reader - Watermark value"]
pub type DMATXCUTTHRUTHR_R = crate::FieldReader<u16>;
#[doc = "Field `DMATXCUTTHRUTHR` writer - Watermark value"]
pub type DMATXCUTTHRUTHR_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DMATXCUTTHRU` reader - Enable TX partial store and forward operation"]
pub type DMATXCUTTHRU_R = crate::BitReader;
#[doc = "Field `DMATXCUTTHRU` writer - Enable TX partial store and forward operation"]
pub type DMATXCUTTHRU_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Watermark value"]
    #[inline(always)]
    pub fn dmatxcutthruthr(&self) -> DMATXCUTTHRUTHR_R {
        DMATXCUTTHRUTHR_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - Enable TX partial store and forward operation"]
    #[inline(always)]
    pub fn dmatxcutthru(&self) -> DMATXCUTTHRU_R {
        DMATXCUTTHRU_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Watermark value"]
    #[inline(always)]
    #[must_use]
    pub fn dmatxcutthruthr(&mut self) -> DMATXCUTTHRUTHR_W<PBUFTXCUTTHRU_SPEC> {
        DMATXCUTTHRUTHR_W::new(self, 0)
    }
    #[doc = "Bit 31 - Enable TX partial store and forward operation"]
    #[inline(always)]
    #[must_use]
    pub fn dmatxcutthru(&mut self) -> DMATXCUTTHRU_W<PBUFTXCUTTHRU_SPEC> {
        DMATXCUTTHRU_W::new(self, 31)
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
#[doc = "TX Partial Store and Forward\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pbuftxcutthru::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pbuftxcutthru::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PBUFTXCUTTHRU_SPEC;
impl crate::RegisterSpec for PBUFTXCUTTHRU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbuftxcutthru::R`](R) reader structure"]
impl crate::Readable for PBUFTXCUTTHRU_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pbuftxcutthru::W`](W) writer structure"]
impl crate::Writable for PBUFTXCUTTHRU_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PBUFTXCUTTHRU to value 0x03ff"]
impl crate::Resettable for PBUFTXCUTTHRU_SPEC {
    const RESET_VALUE: u32 = 0x03ff;
}
