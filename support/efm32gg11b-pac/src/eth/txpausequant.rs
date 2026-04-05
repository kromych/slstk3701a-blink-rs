#[doc = "Register `TXPAUSEQUANT` reader"]
pub type R = crate::R<TxpausequantSpec>;
#[doc = "Register `TXPAUSEQUANT` writer"]
pub type W = crate::W<TxpausequantSpec>;
#[doc = "Field `QUANT` reader - Transmit pause quantum"]
pub type QuantR = crate::FieldReader<u16>;
#[doc = "Field `QUANT` writer - Transmit pause quantum"]
pub type QuantW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `QUANTP1` reader - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 1."]
pub type Quantp1R = crate::FieldReader<u16>;
#[doc = "Field `QUANTP1` writer - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 1."]
pub type Quantp1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Transmit pause quantum"]
    #[inline(always)]
    pub fn quant(&self) -> QuantR {
        QuantR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 1."]
    #[inline(always)]
    pub fn quantp1(&self) -> Quantp1R {
        Quantp1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit pause quantum"]
    #[inline(always)]
    pub fn quant(&mut self) -> QuantW<'_, TxpausequantSpec> {
        QuantW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 1."]
    #[inline(always)]
    pub fn quantp1(&mut self) -> Quantp1W<'_, TxpausequantSpec> {
        Quantp1W::new(self, 16)
    }
}
#[doc = "Transmit Pause Quantum Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txpausequant::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txpausequant::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxpausequantSpec;
impl crate::RegisterSpec for TxpausequantSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txpausequant::R`](R) reader structure"]
impl crate::Readable for TxpausequantSpec {}
#[doc = "`write(|w| ..)` method takes [`txpausequant::W`](W) writer structure"]
impl crate::Writable for TxpausequantSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXPAUSEQUANT to value 0xffff_ffff"]
impl crate::Resettable for TxpausequantSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
