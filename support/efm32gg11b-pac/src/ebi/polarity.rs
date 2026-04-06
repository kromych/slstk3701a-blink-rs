#[doc = "Register `POLARITY` reader"]
pub type R = crate::R<PolaritySpec>;
#[doc = "Register `POLARITY` writer"]
pub type W = crate::W<PolaritySpec>;
#[doc = "Field `CSPOL` reader - Chip Select Polarity"]
pub type CspolR = crate::BitReader;
#[doc = "Field `CSPOL` writer - Chip Select Polarity"]
pub type CspolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REPOL` reader - Read Enable Polarity"]
pub type RepolR = crate::BitReader;
#[doc = "Field `REPOL` writer - Read Enable Polarity"]
pub type RepolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WEPOL` reader - Write Enable Polarity"]
pub type WepolR = crate::BitReader;
#[doc = "Field `WEPOL` writer - Write Enable Polarity"]
pub type WepolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALEPOL` reader - Address Latch Polarity"]
pub type AlepolR = crate::BitReader;
#[doc = "Field `ALEPOL` writer - Address Latch Polarity"]
pub type AlepolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARDYPOL` reader - ARDY Polarity"]
pub type ArdypolR = crate::BitReader;
#[doc = "Field `ARDYPOL` writer - ARDY Polarity"]
pub type ArdypolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLPOL` reader - BL Polarity"]
pub type BlpolR = crate::BitReader;
#[doc = "Field `BLPOL` writer - BL Polarity"]
pub type BlpolW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Chip Select Polarity"]
    #[inline(always)]
    pub fn cspol(&self) -> CspolR {
        CspolR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read Enable Polarity"]
    #[inline(always)]
    pub fn repol(&self) -> RepolR {
        RepolR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write Enable Polarity"]
    #[inline(always)]
    pub fn wepol(&self) -> WepolR {
        WepolR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Address Latch Polarity"]
    #[inline(always)]
    pub fn alepol(&self) -> AlepolR {
        AlepolR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ARDY Polarity"]
    #[inline(always)]
    pub fn ardypol(&self) -> ArdypolR {
        ArdypolR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BL Polarity"]
    #[inline(always)]
    pub fn blpol(&self) -> BlpolR {
        BlpolR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Chip Select Polarity"]
    #[inline(always)]
    pub fn cspol(&mut self) -> CspolW<'_, PolaritySpec> {
        CspolW::new(self, 0)
    }
    #[doc = "Bit 1 - Read Enable Polarity"]
    #[inline(always)]
    pub fn repol(&mut self) -> RepolW<'_, PolaritySpec> {
        RepolW::new(self, 1)
    }
    #[doc = "Bit 2 - Write Enable Polarity"]
    #[inline(always)]
    pub fn wepol(&mut self) -> WepolW<'_, PolaritySpec> {
        WepolW::new(self, 2)
    }
    #[doc = "Bit 3 - Address Latch Polarity"]
    #[inline(always)]
    pub fn alepol(&mut self) -> AlepolW<'_, PolaritySpec> {
        AlepolW::new(self, 3)
    }
    #[doc = "Bit 4 - ARDY Polarity"]
    #[inline(always)]
    pub fn ardypol(&mut self) -> ArdypolW<'_, PolaritySpec> {
        ArdypolW::new(self, 4)
    }
    #[doc = "Bit 5 - BL Polarity"]
    #[inline(always)]
    pub fn blpol(&mut self) -> BlpolW<'_, PolaritySpec> {
        BlpolW::new(self, 5)
    }
}
#[doc = "Polarity Register\n\nYou can [`read`](crate::Reg::read) this register and get [`polarity::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`polarity::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PolaritySpec;
impl crate::RegisterSpec for PolaritySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`polarity::R`](R) reader structure"]
impl crate::Readable for PolaritySpec {}
#[doc = "`write(|w| ..)` method takes [`polarity::W`](W) writer structure"]
impl crate::Writable for PolaritySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POLARITY to value 0"]
impl crate::Resettable for PolaritySpec {}
