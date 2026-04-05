#[doc = "Register `TXLPI` reader"]
pub type R = crate::R<TxlpiSpec>;
#[doc = "Register `TXLPI` writer"]
pub type W = crate::W<TxlpiSpec>;
#[doc = "Field `COUNT` reader - Count of LPI transmitions"]
pub type CountR = crate::FieldReader<u16>;
#[doc = "Field `COUNT` writer - Count of LPI transmitions"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Count of LPI transmitions"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Count of LPI transmitions"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, TxlpiSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Transmit LPI transitions\n\nYou can [`read`](crate::Reg::read) this register and get [`txlpi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txlpi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxlpiSpec;
impl crate::RegisterSpec for TxlpiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txlpi::R`](R) reader structure"]
impl crate::Readable for TxlpiSpec {}
#[doc = "`write(|w| ..)` method takes [`txlpi::W`](W) writer structure"]
impl crate::Writable for TxlpiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXLPI to value 0"]
impl crate::Resettable for TxlpiSpec {}
