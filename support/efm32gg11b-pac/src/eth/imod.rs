#[doc = "Register `IMOD` reader"]
pub type R = crate::R<ImodSpec>;
#[doc = "Register `IMOD` writer"]
pub type W = crate::W<ImodSpec>;
#[doc = "Field `RXINTMOD` reader - Count of 800ns periods before bit 1 is set in the interrupt status register after a frame is received"]
pub type RxintmodR = crate::FieldReader;
#[doc = "Field `RXINTMOD` writer - Count of 800ns periods before bit 1 is set in the interrupt status register after a frame is received"]
pub type RxintmodW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TXINTMOD` reader - Count of 800ns periods before bit 7 is set in the interrupt status register after a frame is transmitted"]
pub type TxintmodR = crate::FieldReader;
#[doc = "Field `TXINTMOD` writer - Count of 800ns periods before bit 7 is set in the interrupt status register after a frame is transmitted"]
pub type TxintmodW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Count of 800ns periods before bit 1 is set in the interrupt status register after a frame is received"]
    #[inline(always)]
    pub fn rxintmod(&self) -> RxintmodR {
        RxintmodR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Count of 800ns periods before bit 7 is set in the interrupt status register after a frame is transmitted"]
    #[inline(always)]
    pub fn txintmod(&self) -> TxintmodR {
        TxintmodR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Count of 800ns periods before bit 1 is set in the interrupt status register after a frame is received"]
    #[inline(always)]
    pub fn rxintmod(&mut self) -> RxintmodW<'_, ImodSpec> {
        RxintmodW::new(self, 0)
    }
    #[doc = "Bits 16:23 - Count of 800ns periods before bit 7 is set in the interrupt status register after a frame is transmitted"]
    #[inline(always)]
    pub fn txintmod(&mut self) -> TxintmodW<'_, ImodSpec> {
        TxintmodW::new(self, 16)
    }
}
#[doc = "Interrupt moderation register\n\nYou can [`read`](crate::Reg::read) this register and get [`imod::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imod::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImodSpec;
impl crate::RegisterSpec for ImodSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imod::R`](R) reader structure"]
impl crate::Readable for ImodSpec {}
#[doc = "`write(|w| ..)` method takes [`imod::W`](W) writer structure"]
impl crate::Writable for ImodSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IMOD to value 0"]
impl crate::Resettable for ImodSpec {}
