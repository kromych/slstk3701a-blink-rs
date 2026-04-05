#[doc = "Register `RXLPITIME` reader"]
pub type R = crate::R<RxlpitimeSpec>;
#[doc = "Register `RXLPITIME` writer"]
pub type W = crate::W<RxlpitimeSpec>;
#[doc = "Field `LPITIME` reader - Time in LPI"]
pub type LpitimeR = crate::FieldReader<u32>;
#[doc = "Field `LPITIME` writer - Time in LPI"]
pub type LpitimeW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Time in LPI"]
    #[inline(always)]
    pub fn lpitime(&self) -> LpitimeR {
        LpitimeR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Time in LPI"]
    #[inline(always)]
    pub fn lpitime(&mut self) -> LpitimeW<'_, RxlpitimeSpec> {
        LpitimeW::new(self, 0)
    }
}
#[doc = "Received LPI time\n\nYou can [`read`](crate::Reg::read) this register and get [`rxlpitime::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxlpitime::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxlpitimeSpec;
impl crate::RegisterSpec for RxlpitimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxlpitime::R`](R) reader structure"]
impl crate::Readable for RxlpitimeSpec {}
#[doc = "`write(|w| ..)` method takes [`rxlpitime::W`](W) writer structure"]
impl crate::Writable for RxlpitimeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXLPITIME to value 0"]
impl crate::Resettable for RxlpitimeSpec {}
