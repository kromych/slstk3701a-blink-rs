#[doc = "Register `QSPICTRL` reader"]
pub type R = crate::R<QspictrlSpec>;
#[doc = "Register `QSPICTRL` writer"]
pub type W = crate::W<QspictrlSpec>;
#[doc = "QSPI0 Reference Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Qspi0clksel {
    #[doc = "0: HFRCO clock is used to clock QSPI0"]
    Hfrco = 0,
    #[doc = "1: HFXO clock is used to clock QSPI0"]
    Hfxo = 1,
    #[doc = "2: AUXHFRCO is used to clock QSPI0"]
    Auxhfrco = 2,
    #[doc = "3: USHFRCO is used to clock QSPI0"]
    Ushfrco = 3,
}
impl From<Qspi0clksel> for u8 {
    #[inline(always)]
    fn from(variant: Qspi0clksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Qspi0clksel {
    type Ux = u8;
}
impl crate::IsEnum for Qspi0clksel {}
#[doc = "Field `QSPI0CLKSEL` reader - QSPI0 Reference Clock Select"]
pub type Qspi0clkselR = crate::FieldReader<Qspi0clksel>;
impl Qspi0clkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Qspi0clksel {
        match self.bits {
            0 => Qspi0clksel::Hfrco,
            1 => Qspi0clksel::Hfxo,
            2 => Qspi0clksel::Auxhfrco,
            3 => Qspi0clksel::Ushfrco,
            _ => unreachable!(),
        }
    }
    #[doc = "HFRCO clock is used to clock QSPI0"]
    #[inline(always)]
    pub fn is_hfrco(&self) -> bool {
        *self == Qspi0clksel::Hfrco
    }
    #[doc = "HFXO clock is used to clock QSPI0"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == Qspi0clksel::Hfxo
    }
    #[doc = "AUXHFRCO is used to clock QSPI0"]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == Qspi0clksel::Auxhfrco
    }
    #[doc = "USHFRCO is used to clock QSPI0"]
    #[inline(always)]
    pub fn is_ushfrco(&self) -> bool {
        *self == Qspi0clksel::Ushfrco
    }
}
#[doc = "Field `QSPI0CLKSEL` writer - QSPI0 Reference Clock Select"]
pub type Qspi0clkselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Qspi0clksel, crate::Safe>;
impl<'a, REG> Qspi0clkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HFRCO clock is used to clock QSPI0"]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Qspi0clksel::Hfrco)
    }
    #[doc = "HFXO clock is used to clock QSPI0"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Qspi0clksel::Hfxo)
    }
    #[doc = "AUXHFRCO is used to clock QSPI0"]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Qspi0clksel::Auxhfrco)
    }
    #[doc = "USHFRCO is used to clock QSPI0"]
    #[inline(always)]
    pub fn ushfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Qspi0clksel::Ushfrco)
    }
}
#[doc = "Field `QSPI0CLKDIS` reader - QSPI0 Reference Clock Disable"]
pub type Qspi0clkdisR = crate::BitReader;
#[doc = "Field `QSPI0CLKDIS` writer - QSPI0 Reference Clock Disable"]
pub type Qspi0clkdisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - QSPI0 Reference Clock Select"]
    #[inline(always)]
    pub fn qspi0clksel(&self) -> Qspi0clkselR {
        Qspi0clkselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 7 - QSPI0 Reference Clock Disable"]
    #[inline(always)]
    pub fn qspi0clkdis(&self) -> Qspi0clkdisR {
        Qspi0clkdisR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - QSPI0 Reference Clock Select"]
    #[inline(always)]
    pub fn qspi0clksel(&mut self) -> Qspi0clkselW<'_, QspictrlSpec> {
        Qspi0clkselW::new(self, 0)
    }
    #[doc = "Bit 7 - QSPI0 Reference Clock Disable"]
    #[inline(always)]
    pub fn qspi0clkdis(&mut self) -> Qspi0clkdisW<'_, QspictrlSpec> {
        Qspi0clkdisW::new(self, 7)
    }
}
#[doc = "QSPI Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`qspictrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qspictrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QspictrlSpec;
impl crate::RegisterSpec for QspictrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qspictrl::R`](R) reader structure"]
impl crate::Readable for QspictrlSpec {}
#[doc = "`write(|w| ..)` method takes [`qspictrl::W`](W) writer structure"]
impl crate::Writable for QspictrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets QSPICTRL to value 0"]
impl crate::Resettable for QspictrlSpec {}
