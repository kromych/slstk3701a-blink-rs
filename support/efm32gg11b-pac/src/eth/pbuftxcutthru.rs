#[doc = "Register `PBUFTXCUTTHRU` reader"]
pub type R = crate::R<PbuftxcutthruSpec>;
#[doc = "Register `PBUFTXCUTTHRU` writer"]
pub type W = crate::W<PbuftxcutthruSpec>;
#[doc = "Field `DMATXCUTTHRUTHR` reader - Watermark value"]
pub type DmatxcutthruthrR = crate::FieldReader<u16>;
#[doc = "Field `DMATXCUTTHRUTHR` writer - Watermark value"]
pub type DmatxcutthruthrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DMATXCUTTHRU` reader - Enable TX partial store and forward operation"]
pub type DmatxcutthruR = crate::BitReader;
#[doc = "Field `DMATXCUTTHRU` writer - Enable TX partial store and forward operation"]
pub type DmatxcutthruW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Watermark value"]
    #[inline(always)]
    pub fn dmatxcutthruthr(&self) -> DmatxcutthruthrR {
        DmatxcutthruthrR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - Enable TX partial store and forward operation"]
    #[inline(always)]
    pub fn dmatxcutthru(&self) -> DmatxcutthruR {
        DmatxcutthruR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Watermark value"]
    #[inline(always)]
    pub fn dmatxcutthruthr(&mut self) -> DmatxcutthruthrW<'_, PbuftxcutthruSpec> {
        DmatxcutthruthrW::new(self, 0)
    }
    #[doc = "Bit 31 - Enable TX partial store and forward operation"]
    #[inline(always)]
    pub fn dmatxcutthru(&mut self) -> DmatxcutthruW<'_, PbuftxcutthruSpec> {
        DmatxcutthruW::new(self, 31)
    }
}
#[doc = "TX Partial Store and Forward\n\nYou can [`read`](crate::Reg::read) this register and get [`pbuftxcutthru::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbuftxcutthru::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbuftxcutthruSpec;
impl crate::RegisterSpec for PbuftxcutthruSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbuftxcutthru::R`](R) reader structure"]
impl crate::Readable for PbuftxcutthruSpec {}
#[doc = "`write(|w| ..)` method takes [`pbuftxcutthru::W`](W) writer structure"]
impl crate::Writable for PbuftxcutthruSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PBUFTXCUTTHRU to value 0x03ff"]
impl crate::Resettable for PbuftxcutthruSpec {
    const RESET_VALUE: u32 = 0x03ff;
}
