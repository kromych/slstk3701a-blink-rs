#[doc = "Register `TXQPTR` reader"]
pub type R = crate::R<TxqptrSpec>;
#[doc = "Register `TXQPTR` writer"]
pub type W = crate::W<TxqptrSpec>;
#[doc = "Field `DMATXQPTR` reader - Transmit buffer queue base address"]
pub type DmatxqptrR = crate::FieldReader<u32>;
#[doc = "Field `DMATXQPTR` writer - Transmit buffer queue base address"]
pub type DmatxqptrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Transmit buffer queue base address"]
    #[inline(always)]
    pub fn dmatxqptr(&self) -> DmatxqptrR {
        DmatxqptrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Transmit buffer queue base address"]
    #[inline(always)]
    pub fn dmatxqptr(&mut self) -> DmatxqptrW<'_, TxqptrSpec> {
        DmatxqptrW::new(self, 2)
    }
}
#[doc = "Start address of the transmit buffer queue\n\nYou can [`read`](crate::Reg::read) this register and get [`txqptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txqptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxqptrSpec;
impl crate::RegisterSpec for TxqptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txqptr::R`](R) reader structure"]
impl crate::Readable for TxqptrSpec {}
#[doc = "`write(|w| ..)` method takes [`txqptr::W`](W) writer structure"]
impl crate::Writable for TxqptrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXQPTR to value 0"]
impl crate::Resettable for TxqptrSpec {}
