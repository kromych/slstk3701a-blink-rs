#[doc = "Register `DPLLCTRL` reader"]
pub type R = crate::R<DpllctrlSpec>;
#[doc = "Register `DPLLCTRL` writer"]
pub type W = crate::W<DpllctrlSpec>;
#[doc = "Field `MODE` reader - Operating Mode Control"]
pub type ModeR = crate::BitReader;
#[doc = "Field `MODE` writer - Operating Mode Control"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDGESEL` reader - Reference Edge Select"]
pub type EdgeselR = crate::BitReader;
#[doc = "Field `EDGESEL` writer - Reference Edge Select"]
pub type EdgeselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTORECOVER` reader - Automatic Recovery Ctrl"]
pub type AutorecoverR = crate::BitReader;
#[doc = "Field `AUTORECOVER` writer - Automatic Recovery Ctrl"]
pub type AutorecoverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Reference Clock Selection Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Refsel {
    #[doc = "0: HFXO selected"]
    Hfxo = 0,
    #[doc = "1: LFXO selected"]
    Lfxo = 1,
    #[doc = "2: USHFRCO selected"]
    Ushfrco = 2,
    #[doc = "3: CLKIN0 selected"]
    Clkin0 = 3,
}
impl From<Refsel> for u8 {
    #[inline(always)]
    fn from(variant: Refsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Refsel {
    type Ux = u8;
}
impl crate::IsEnum for Refsel {}
#[doc = "Field `REFSEL` reader - Reference Clock Selection Control"]
pub type RefselR = crate::FieldReader<Refsel>;
impl RefselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refsel {
        match self.bits {
            0 => Refsel::Hfxo,
            1 => Refsel::Lfxo,
            2 => Refsel::Ushfrco,
            3 => Refsel::Clkin0,
            _ => unreachable!(),
        }
    }
    #[doc = "HFXO selected"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == Refsel::Hfxo
    }
    #[doc = "LFXO selected"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == Refsel::Lfxo
    }
    #[doc = "USHFRCO selected"]
    #[inline(always)]
    pub fn is_ushfrco(&self) -> bool {
        *self == Refsel::Ushfrco
    }
    #[doc = "CLKIN0 selected"]
    #[inline(always)]
    pub fn is_clkin0(&self) -> bool {
        *self == Refsel::Clkin0
    }
}
#[doc = "Field `REFSEL` writer - Reference Clock Selection Control"]
pub type RefselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Refsel, crate::Safe>;
impl<'a, REG> RefselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HFXO selected"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Hfxo)
    }
    #[doc = "LFXO selected"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Lfxo)
    }
    #[doc = "USHFRCO selected"]
    #[inline(always)]
    pub fn ushfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Ushfrco)
    }
    #[doc = "CLKIN0 selected"]
    #[inline(always)]
    pub fn clkin0(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Clkin0)
    }
}
#[doc = "Field `DITHEN` reader - Dither Enable Control"]
pub type DithenR = crate::BitReader;
#[doc = "Field `DITHEN` writer - Dither Enable Control"]
pub type DithenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Operating Mode Control"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reference Edge Select"]
    #[inline(always)]
    pub fn edgesel(&self) -> EdgeselR {
        EdgeselR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Automatic Recovery Ctrl"]
    #[inline(always)]
    pub fn autorecover(&self) -> AutorecoverR {
        AutorecoverR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Reference Clock Selection Control"]
    #[inline(always)]
    pub fn refsel(&self) -> RefselR {
        RefselR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 6 - Dither Enable Control"]
    #[inline(always)]
    pub fn dithen(&self) -> DithenR {
        DithenR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Operating Mode Control"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, DpllctrlSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bit 1 - Reference Edge Select"]
    #[inline(always)]
    pub fn edgesel(&mut self) -> EdgeselW<'_, DpllctrlSpec> {
        EdgeselW::new(self, 1)
    }
    #[doc = "Bit 2 - Automatic Recovery Ctrl"]
    #[inline(always)]
    pub fn autorecover(&mut self) -> AutorecoverW<'_, DpllctrlSpec> {
        AutorecoverW::new(self, 2)
    }
    #[doc = "Bits 3:4 - Reference Clock Selection Control"]
    #[inline(always)]
    pub fn refsel(&mut self) -> RefselW<'_, DpllctrlSpec> {
        RefselW::new(self, 3)
    }
    #[doc = "Bit 6 - Dither Enable Control"]
    #[inline(always)]
    pub fn dithen(&mut self) -> DithenW<'_, DpllctrlSpec> {
        DithenW::new(self, 6)
    }
}
#[doc = "DPLL Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dpllctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpllctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpllctrlSpec;
impl crate::RegisterSpec for DpllctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpllctrl::R`](R) reader structure"]
impl crate::Readable for DpllctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dpllctrl::W`](W) writer structure"]
impl crate::Writable for DpllctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPLLCTRL to value 0"]
impl crate::Resettable for DpllctrlSpec {}
