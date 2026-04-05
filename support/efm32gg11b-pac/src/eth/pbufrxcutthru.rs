#[doc = "Register `PBUFRXCUTTHRU` reader"]
pub type R = crate::R<PbufrxcutthruSpec>;
#[doc = "Register `PBUFRXCUTTHRU` writer"]
pub type W = crate::W<PbufrxcutthruSpec>;
#[doc = "Field `DMARXCUTTHRUTHR` reader - Watermark value"]
pub type DmarxcutthruthrR = crate::FieldReader<u16>;
#[doc = "Field `DMARXCUTTHRUTHR` writer - Watermark value"]
pub type DmarxcutthruthrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DMARXCUTTHRU` reader - Enable RX partial store and forward operation"]
pub type DmarxcutthruR = crate::BitReader;
#[doc = "Field `DMARXCUTTHRU` writer - Enable RX partial store and forward operation"]
pub type DmarxcutthruW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Watermark value"]
    #[inline(always)]
    pub fn dmarxcutthruthr(&self) -> DmarxcutthruthrR {
        DmarxcutthruthrR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - Enable RX partial store and forward operation"]
    #[inline(always)]
    pub fn dmarxcutthru(&self) -> DmarxcutthruR {
        DmarxcutthruR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Watermark value"]
    #[inline(always)]
    pub fn dmarxcutthruthr(&mut self) -> DmarxcutthruthrW<'_, PbufrxcutthruSpec> {
        DmarxcutthruthrW::new(self, 0)
    }
    #[doc = "Bit 31 - Enable RX partial store and forward operation"]
    #[inline(always)]
    pub fn dmarxcutthru(&mut self) -> DmarxcutthruW<'_, PbufrxcutthruSpec> {
        DmarxcutthruW::new(self, 31)
    }
}
#[doc = "RX Partial Store and Forward\n\nYou can [`read`](crate::Reg::read) this register and get [`pbufrxcutthru::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbufrxcutthru::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbufrxcutthruSpec;
impl crate::RegisterSpec for PbufrxcutthruSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbufrxcutthru::R`](R) reader structure"]
impl crate::Readable for PbufrxcutthruSpec {}
#[doc = "`write(|w| ..)` method takes [`pbufrxcutthru::W`](W) writer structure"]
impl crate::Writable for PbufrxcutthruSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PBUFRXCUTTHRU to value 0x03ff"]
impl crate::Resettable for PbufrxcutthruSpec {
    const RESET_VALUE: u32 = 0x03ff;
}
