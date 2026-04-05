#[doc = "Register `TXTHRESH` reader"]
pub type R = crate::R<TxthreshSpec>;
#[doc = "Register `TXTHRESH` writer"]
pub type W = crate::W<TxthreshSpec>;
#[doc = "Field `LEVEL` reader - Threshold Level"]
pub type LevelR = crate::FieldReader;
#[doc = "Field `LEVEL` writer - Threshold Level"]
pub type LevelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Threshold Level"]
    #[inline(always)]
    pub fn level(&self) -> LevelR {
        LevelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Threshold Level"]
    #[inline(always)]
    pub fn level(&mut self) -> LevelW<'_, TxthreshSpec> {
        LevelW::new(self, 0)
    }
}
#[doc = "TX Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txthresh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txthresh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxthreshSpec;
impl crate::RegisterSpec for TxthreshSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txthresh::R`](R) reader structure"]
impl crate::Readable for TxthreshSpec {}
#[doc = "`write(|w| ..)` method takes [`txthresh::W`](W) writer structure"]
impl crate::Writable for TxthreshSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXTHRESH to value 0x01"]
impl crate::Resettable for TxthreshSpec {
    const RESET_VALUE: u32 = 0x01;
}
