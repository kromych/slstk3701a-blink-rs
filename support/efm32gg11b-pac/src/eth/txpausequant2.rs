#[doc = "Register `TXPAUSEQUANT2` reader"]
pub type R = crate::R<Txpausequant2Spec>;
#[doc = "Register `TXPAUSEQUANT2` writer"]
pub type W = crate::W<Txpausequant2Spec>;
#[doc = "Field `QUANTP4` reader - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 4."]
pub type Quantp4R = crate::FieldReader<u16>;
#[doc = "Field `QUANTP4` writer - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 4."]
pub type Quantp4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `QUANTP5` reader - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 5."]
pub type Quantp5R = crate::FieldReader<u16>;
#[doc = "Field `QUANTP5` writer - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 5."]
pub type Quantp5W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 4."]
    #[inline(always)]
    pub fn quantp4(&self) -> Quantp4R {
        Quantp4R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 5."]
    #[inline(always)]
    pub fn quantp5(&self) -> Quantp5R {
        Quantp5R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 4."]
    #[inline(always)]
    pub fn quantp4(&mut self) -> Quantp4W<'_, Txpausequant2Spec> {
        Quantp4W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 5."]
    #[inline(always)]
    pub fn quantp5(&mut self) -> Quantp5W<'_, Txpausequant2Spec> {
        Quantp5W::new(self, 16)
    }
}
#[doc = "Transmit Pause Quantum Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`txpausequant2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txpausequant2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txpausequant2Spec;
impl crate::RegisterSpec for Txpausequant2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txpausequant2::R`](R) reader structure"]
impl crate::Readable for Txpausequant2Spec {}
#[doc = "`write(|w| ..)` method takes [`txpausequant2::W`](W) writer structure"]
impl crate::Writable for Txpausequant2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXPAUSEQUANT2 to value 0xffff_ffff"]
impl crate::Resettable for Txpausequant2Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
