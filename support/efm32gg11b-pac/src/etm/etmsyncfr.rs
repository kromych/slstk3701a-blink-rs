#[doc = "Register `ETMSYNCFR` reader"]
pub type R = crate::R<EtmsyncfrSpec>;
#[doc = "Register `ETMSYNCFR` writer"]
pub type W = crate::W<EtmsyncfrSpec>;
#[doc = "Field `FREQ` reader - Synchronisation Frequency Value"]
pub type FreqR = crate::FieldReader<u16>;
#[doc = "Field `FREQ` writer - Synchronisation Frequency Value"]
pub type FreqW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Synchronisation Frequency Value"]
    #[inline(always)]
    pub fn freq(&self) -> FreqR {
        FreqR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Synchronisation Frequency Value"]
    #[inline(always)]
    pub fn freq(&mut self) -> FreqW<'_, EtmsyncfrSpec> {
        FreqW::new(self, 0)
    }
}
#[doc = "Synchronisation Frequency Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmsyncfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etmsyncfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtmsyncfrSpec;
impl crate::RegisterSpec for EtmsyncfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmsyncfr::R`](R) reader structure"]
impl crate::Readable for EtmsyncfrSpec {}
#[doc = "`write(|w| ..)` method takes [`etmsyncfr::W`](W) writer structure"]
impl crate::Writable for EtmsyncfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETMSYNCFR to value 0x0400"]
impl crate::Resettable for EtmsyncfrSpec {
    const RESET_VALUE: u32 = 0x0400;
}
