#[doc = "Register `TXUNDERRUNS` reader"]
pub type R = crate::R<TxunderrunsSpec>;
#[doc = "Register `TXUNDERRUNS` writer"]
pub type W = crate::W<TxunderrunsSpec>;
#[doc = "Field `COUNT` reader - Transmit under runs"]
pub type CountR = crate::FieldReader<u16>;
#[doc = "Field `COUNT` writer - Transmit under runs"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Transmit under runs"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Transmit under runs"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, TxunderrunsSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Transmit Under Runs\n\nYou can [`read`](crate::Reg::read) this register and get [`txunderruns::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txunderruns::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxunderrunsSpec;
impl crate::RegisterSpec for TxunderrunsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txunderruns::R`](R) reader structure"]
impl crate::Readable for TxunderrunsSpec {}
#[doc = "`write(|w| ..)` method takes [`txunderruns::W`](W) writer structure"]
impl crate::Writable for TxunderrunsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXUNDERRUNS to value 0"]
impl crate::Resettable for TxunderrunsSpec {}
