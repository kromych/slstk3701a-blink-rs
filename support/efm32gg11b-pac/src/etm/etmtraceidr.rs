#[doc = "Register `ETMTRACEIDR` reader"]
pub type R = crate::R<EtmtraceidrSpec>;
#[doc = "Register `ETMTRACEIDR` writer"]
pub type W = crate::W<EtmtraceidrSpec>;
#[doc = "Field `TRACEID` reader - Trace ID"]
pub type TraceidR = crate::FieldReader;
#[doc = "Field `TRACEID` writer - Trace ID"]
pub type TraceidW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Trace ID"]
    #[inline(always)]
    pub fn traceid(&self) -> TraceidR {
        TraceidR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Trace ID"]
    #[inline(always)]
    pub fn traceid(&mut self) -> TraceidW<'_, EtmtraceidrSpec> {
        TraceidW::new(self, 0)
    }
}
#[doc = "CoreSight Trace ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmtraceidr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etmtraceidr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtmtraceidrSpec;
impl crate::RegisterSpec for EtmtraceidrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmtraceidr::R`](R) reader structure"]
impl crate::Readable for EtmtraceidrSpec {}
#[doc = "`write(|w| ..)` method takes [`etmtraceidr::W`](W) writer structure"]
impl crate::Writable for EtmtraceidrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETMTRACEIDR to value 0"]
impl crate::Resettable for EtmtraceidrSpec {}
