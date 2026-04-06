#[doc = "Register `ETMTECR1` reader"]
pub type R = crate::R<Etmtecr1Spec>;
#[doc = "Register `ETMTECR1` writer"]
pub type W = crate::W<Etmtecr1Spec>;
#[doc = "Field `ADRCMP` reader - Address Comparator"]
pub type AdrcmpR = crate::FieldReader;
#[doc = "Field `ADRCMP` writer - Address Comparator"]
pub type AdrcmpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MEMMAP` reader - Memmap"]
pub type MemmapR = crate::FieldReader<u16>;
#[doc = "Field `MEMMAP` writer - Memmap"]
pub type MemmapW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `INCEXCTL` reader - Trace Include/Exclude Flag"]
pub type IncexctlR = crate::BitReader;
#[doc = "Field `INCEXCTL` writer - Trace Include/Exclude Flag"]
pub type IncexctlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCE` reader - Trace Control Enable"]
pub type TceR = crate::BitReader;
#[doc = "Field `TCE` writer - Trace Control Enable"]
pub type TceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Address Comparator"]
    #[inline(always)]
    pub fn adrcmp(&self) -> AdrcmpR {
        AdrcmpR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:23 - Memmap"]
    #[inline(always)]
    pub fn memmap(&self) -> MemmapR {
        MemmapR::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bit 24 - Trace Include/Exclude Flag"]
    #[inline(always)]
    pub fn incexctl(&self) -> IncexctlR {
        IncexctlR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Trace Control Enable"]
    #[inline(always)]
    pub fn tce(&self) -> TceR {
        TceR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Address Comparator"]
    #[inline(always)]
    pub fn adrcmp(&mut self) -> AdrcmpW<'_, Etmtecr1Spec> {
        AdrcmpW::new(self, 0)
    }
    #[doc = "Bits 8:23 - Memmap"]
    #[inline(always)]
    pub fn memmap(&mut self) -> MemmapW<'_, Etmtecr1Spec> {
        MemmapW::new(self, 8)
    }
    #[doc = "Bit 24 - Trace Include/Exclude Flag"]
    #[inline(always)]
    pub fn incexctl(&mut self) -> IncexctlW<'_, Etmtecr1Spec> {
        IncexctlW::new(self, 24)
    }
    #[doc = "Bit 25 - Trace Control Enable"]
    #[inline(always)]
    pub fn tce(&mut self) -> TceW<'_, Etmtecr1Spec> {
        TceW::new(self, 25)
    }
}
#[doc = "ETM Trace control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmtecr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etmtecr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Etmtecr1Spec;
impl crate::RegisterSpec for Etmtecr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmtecr1::R`](R) reader structure"]
impl crate::Readable for Etmtecr1Spec {}
#[doc = "`write(|w| ..)` method takes [`etmtecr1::W`](W) writer structure"]
impl crate::Writable for Etmtecr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETMTECR1 to value 0"]
impl crate::Resettable for Etmtecr1Spec {}
