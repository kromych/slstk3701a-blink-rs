#[doc = "Register `TXLPITIME` reader"]
pub type R = crate::R<TxlpitimeSpec>;
#[doc = "Register `TXLPITIME` writer"]
pub type W = crate::W<TxlpitimeSpec>;
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
    pub fn lpitime(&mut self) -> LpitimeW<'_, TxlpitimeSpec> {
        LpitimeW::new(self, 0)
    }
}
#[doc = "Transmit LPI time\n\nYou can [`read`](crate::Reg::read) this register and get [`txlpitime::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txlpitime::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxlpitimeSpec;
impl crate::RegisterSpec for TxlpitimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txlpitime::R`](R) reader structure"]
impl crate::Readable for TxlpitimeSpec {}
#[doc = "`write(|w| ..)` method takes [`txlpitime::W`](W) writer structure"]
impl crate::Writable for TxlpitimeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXLPITIME to value 0"]
impl crate::Resettable for TxlpitimeSpec {}
