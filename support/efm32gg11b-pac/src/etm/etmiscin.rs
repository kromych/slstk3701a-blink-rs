#[doc = "Register `ETMISCIN` reader"]
pub type R = crate::R<EtmiscinSpec>;
#[doc = "Register `ETMISCIN` writer"]
pub type W = crate::W<EtmiscinSpec>;
#[doc = "Field `EXTIN` reader - EXTIN Value"]
pub type ExtinR = crate::FieldReader;
#[doc = "Field `EXTIN` writer - EXTIN Value"]
pub type ExtinW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COREHALT` reader - Core Halt"]
pub type CorehaltR = crate::BitReader;
#[doc = "Field `COREHALT` writer - Core Halt"]
pub type CorehaltW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - EXTIN Value"]
    #[inline(always)]
    pub fn extin(&self) -> ExtinR {
        ExtinR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Core Halt"]
    #[inline(always)]
    pub fn corehalt(&self) -> CorehaltR {
        CorehaltR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - EXTIN Value"]
    #[inline(always)]
    pub fn extin(&mut self) -> ExtinW<'_, EtmiscinSpec> {
        ExtinW::new(self, 0)
    }
    #[doc = "Bit 4 - Core Halt"]
    #[inline(always)]
    pub fn corehalt(&mut self) -> CorehaltW<'_, EtmiscinSpec> {
        CorehaltW::new(self, 4)
    }
}
#[doc = "Integration Test Miscellaneous Inputs Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmiscin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etmiscin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtmiscinSpec;
impl crate::RegisterSpec for EtmiscinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmiscin::R`](R) reader structure"]
impl crate::Readable for EtmiscinSpec {}
#[doc = "`write(|w| ..)` method takes [`etmiscin::W`](W) writer structure"]
impl crate::Writable for EtmiscinSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETMISCIN to value 0"]
impl crate::Resettable for EtmiscinSpec {}
