#[doc = "Register `SDIOCTRL` reader"]
pub type R = crate::R<SdioctrlSpec>;
#[doc = "Register `SDIOCTRL` writer"]
pub type W = crate::W<SdioctrlSpec>;
#[doc = "SDIO Reference Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sdioclksel {
    #[doc = "0: HFRCO clock is used to clock SDIO"]
    Hfrco = 0,
    #[doc = "1: HFXO clock is used to clock SDIO"]
    Hfxo = 1,
    #[doc = "2: AUXHFRCO is used to clock SDIO"]
    Auxhfrco = 2,
    #[doc = "3: USHFRCO is used to clock SDIO"]
    Ushfrco = 3,
}
impl From<Sdioclksel> for u8 {
    #[inline(always)]
    fn from(variant: Sdioclksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sdioclksel {
    type Ux = u8;
}
impl crate::IsEnum for Sdioclksel {}
#[doc = "Field `SDIOCLKSEL` reader - SDIO Reference Clock Select"]
pub type SdioclkselR = crate::FieldReader<Sdioclksel>;
impl SdioclkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdioclksel {
        match self.bits {
            0 => Sdioclksel::Hfrco,
            1 => Sdioclksel::Hfxo,
            2 => Sdioclksel::Auxhfrco,
            3 => Sdioclksel::Ushfrco,
            _ => unreachable!(),
        }
    }
    #[doc = "HFRCO clock is used to clock SDIO"]
    #[inline(always)]
    pub fn is_hfrco(&self) -> bool {
        *self == Sdioclksel::Hfrco
    }
    #[doc = "HFXO clock is used to clock SDIO"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == Sdioclksel::Hfxo
    }
    #[doc = "AUXHFRCO is used to clock SDIO"]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == Sdioclksel::Auxhfrco
    }
    #[doc = "USHFRCO is used to clock SDIO"]
    #[inline(always)]
    pub fn is_ushfrco(&self) -> bool {
        *self == Sdioclksel::Ushfrco
    }
}
#[doc = "Field `SDIOCLKSEL` writer - SDIO Reference Clock Select"]
pub type SdioclkselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sdioclksel, crate::Safe>;
impl<'a, REG> SdioclkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HFRCO clock is used to clock SDIO"]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Sdioclksel::Hfrco)
    }
    #[doc = "HFXO clock is used to clock SDIO"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Sdioclksel::Hfxo)
    }
    #[doc = "AUXHFRCO is used to clock SDIO"]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Sdioclksel::Auxhfrco)
    }
    #[doc = "USHFRCO is used to clock SDIO"]
    #[inline(always)]
    pub fn ushfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Sdioclksel::Ushfrco)
    }
}
#[doc = "Field `SDIOCLKDIS` reader - SDIO Reference Clock Disable"]
pub type SdioclkdisR = crate::BitReader;
#[doc = "Field `SDIOCLKDIS` writer - SDIO Reference Clock Disable"]
pub type SdioclkdisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - SDIO Reference Clock Select"]
    #[inline(always)]
    pub fn sdioclksel(&self) -> SdioclkselR {
        SdioclkselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 7 - SDIO Reference Clock Disable"]
    #[inline(always)]
    pub fn sdioclkdis(&self) -> SdioclkdisR {
        SdioclkdisR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SDIO Reference Clock Select"]
    #[inline(always)]
    pub fn sdioclksel(&mut self) -> SdioclkselW<'_, SdioctrlSpec> {
        SdioclkselW::new(self, 0)
    }
    #[doc = "Bit 7 - SDIO Reference Clock Disable"]
    #[inline(always)]
    pub fn sdioclkdis(&mut self) -> SdioclkdisW<'_, SdioctrlSpec> {
        SdioclkdisW::new(self, 7)
    }
}
#[doc = "SDIO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdioctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdioctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdioctrlSpec;
impl crate::RegisterSpec for SdioctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdioctrl::R`](R) reader structure"]
impl crate::Readable for SdioctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`sdioctrl::W`](W) writer structure"]
impl crate::Writable for SdioctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDIOCTRL to value 0"]
impl crate::Resettable for SdioctrlSpec {}
