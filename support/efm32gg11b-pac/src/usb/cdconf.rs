#[doc = "Register `CDCONF` reader"]
pub type R = crate::R<CdconfSpec>;
#[doc = "Register `CDCONF` writer"]
pub type W = crate::W<CdconfSpec>;
#[doc = "Field `DCDTOCONF` reader - DCD Timeout (TDCD_TIMEOUT) Configuration"]
pub type DcdtoconfR = crate::FieldReader<u16>;
#[doc = "Field `DCDTOCONF` writer - DCD Timeout (TDCD_TIMEOUT) Configuration"]
pub type DcdtoconfW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - DCD Timeout (TDCD_TIMEOUT) Configuration"]
    #[inline(always)]
    pub fn dcdtoconf(&self) -> DcdtoconfR {
        DcdtoconfR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - DCD Timeout (TDCD_TIMEOUT) Configuration"]
    #[inline(always)]
    pub fn dcdtoconf(&mut self) -> DcdtoconfW<'_, CdconfSpec> {
        DcdtoconfW::new(self, 0)
    }
}
#[doc = "Charger Detect Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cdconf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdconf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CdconfSpec;
impl crate::RegisterSpec for CdconfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdconf::R`](R) reader structure"]
impl crate::Readable for CdconfSpec {}
#[doc = "`write(|w| ..)` method takes [`cdconf::W`](W) writer structure"]
impl crate::Writable for CdconfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CDCONF to value 0"]
impl crate::Resettable for CdconfSpec {}
