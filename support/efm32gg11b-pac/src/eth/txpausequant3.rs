#[doc = "Register `TXPAUSEQUANT3` reader"]
pub type R = crate::R<Txpausequant3Spec>;
#[doc = "Register `TXPAUSEQUANT3` writer"]
pub type W = crate::W<Txpausequant3Spec>;
#[doc = "Field `QUANTP6` reader - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 6."]
pub type Quantp6R = crate::FieldReader<u16>;
#[doc = "Field `QUANTP6` writer - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 6."]
pub type Quantp6W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `QUANTP7` reader - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 7."]
pub type Quantp7R = crate::FieldReader<u16>;
#[doc = "Field `QUANTP7` writer - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 7."]
pub type Quantp7W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 6."]
    #[inline(always)]
    pub fn quantp6(&self) -> Quantp6R {
        Quantp6R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 7."]
    #[inline(always)]
    pub fn quantp7(&self) -> Quantp7R {
        Quantp7R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 6."]
    #[inline(always)]
    pub fn quantp6(&mut self) -> Quantp6W<'_, Txpausequant3Spec> {
        Quantp6W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 7."]
    #[inline(always)]
    pub fn quantp7(&mut self) -> Quantp7W<'_, Txpausequant3Spec> {
        Quantp7W::new(self, 16)
    }
}
#[doc = "Transmit Pause Quantum Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`txpausequant3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txpausequant3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txpausequant3Spec;
impl crate::RegisterSpec for Txpausequant3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txpausequant3::R`](R) reader structure"]
impl crate::Readable for Txpausequant3Spec {}
#[doc = "`write(|w| ..)` method takes [`txpausequant3::W`](W) writer structure"]
impl crate::Writable for Txpausequant3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXPAUSEQUANT3 to value 0xffff_ffff"]
impl crate::Resettable for Txpausequant3Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
