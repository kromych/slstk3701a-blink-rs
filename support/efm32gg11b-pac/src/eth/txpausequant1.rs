#[doc = "Register `TXPAUSEQUANT1` reader"]
pub type R = crate::R<Txpausequant1Spec>;
#[doc = "Register `TXPAUSEQUANT1` writer"]
pub type W = crate::W<Txpausequant1Spec>;
#[doc = "Field `QUANTP2` reader - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 2."]
pub type Quantp2R = crate::FieldReader<u16>;
#[doc = "Field `QUANTP2` writer - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 2."]
pub type Quantp2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `QUANTP3` reader - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 3."]
pub type Quantp3R = crate::FieldReader<u16>;
#[doc = "Field `QUANTP3` writer - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 3."]
pub type Quantp3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 2."]
    #[inline(always)]
    pub fn quantp2(&self) -> Quantp2R {
        Quantp2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 3."]
    #[inline(always)]
    pub fn quantp3(&self) -> Quantp3R {
        Quantp3R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 2."]
    #[inline(always)]
    pub fn quantp2(&mut self) -> Quantp2W<'_, Txpausequant1Spec> {
        Quantp2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 3."]
    #[inline(always)]
    pub fn quantp3(&mut self) -> Quantp3W<'_, Txpausequant1Spec> {
        Quantp3W::new(self, 16)
    }
}
#[doc = "Transmit Pause Quantum Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`txpausequant1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txpausequant1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txpausequant1Spec;
impl crate::RegisterSpec for Txpausequant1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txpausequant1::R`](R) reader structure"]
impl crate::Readable for Txpausequant1Spec {}
#[doc = "`write(|w| ..)` method takes [`txpausequant1::W`](W) writer structure"]
impl crate::Writable for Txpausequant1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXPAUSEQUANT1 to value 0xffff_ffff"]
impl crate::Resettable for Txpausequant1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
