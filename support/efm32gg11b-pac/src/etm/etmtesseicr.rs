#[doc = "Register `ETMTESSEICR` reader"]
pub type R = crate::R<EtmtesseicrSpec>;
#[doc = "Register `ETMTESSEICR` writer"]
pub type W = crate::W<EtmtesseicrSpec>;
#[doc = "Field `STARTRSEL` reader - Stop Resource Selection"]
pub type StartrselR = crate::FieldReader;
#[doc = "Field `STARTRSEL` writer - Stop Resource Selection"]
pub type StartrselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `STOPRSEL` reader - Stop Resource Selection"]
pub type StoprselR = crate::FieldReader;
#[doc = "Field `STOPRSEL` writer - Stop Resource Selection"]
pub type StoprselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Stop Resource Selection"]
    #[inline(always)]
    pub fn startrsel(&self) -> StartrselR {
        StartrselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Stop Resource Selection"]
    #[inline(always)]
    pub fn stoprsel(&self) -> StoprselR {
        StoprselR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Stop Resource Selection"]
    #[inline(always)]
    pub fn startrsel(&mut self) -> StartrselW<'_, EtmtesseicrSpec> {
        StartrselW::new(self, 0)
    }
    #[doc = "Bits 16:19 - Stop Resource Selection"]
    #[inline(always)]
    pub fn stoprsel(&mut self) -> StoprselW<'_, EtmtesseicrSpec> {
        StoprselW::new(self, 16)
    }
}
#[doc = "TraceEnable Start/Stop EmbeddedICE Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmtesseicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etmtesseicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtmtesseicrSpec;
impl crate::RegisterSpec for EtmtesseicrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmtesseicr::R`](R) reader structure"]
impl crate::Readable for EtmtesseicrSpec {}
#[doc = "`write(|w| ..)` method takes [`etmtesseicr::W`](W) writer structure"]
impl crate::Writable for EtmtesseicrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETMTESSEICR to value 0"]
impl crate::Resettable for EtmtesseicrSpec {}
