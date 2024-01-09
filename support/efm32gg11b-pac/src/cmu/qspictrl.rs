#[doc = "Register `QSPICTRL` reader"]
pub type R = crate::R<QSPICTRL_SPEC>;
#[doc = "Register `QSPICTRL` writer"]
pub type W = crate::W<QSPICTRL_SPEC>;
#[doc = "Field `QSPI0CLKSEL` reader - QSPI0 Reference Clock Select"]
pub type QSPI0CLKSEL_R = crate::FieldReader<QSPI0CLKSEL_A>;
#[doc = "QSPI0 Reference Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum QSPI0CLKSEL_A {
    #[doc = "0: HFRCO clock is used to clock QSPI0"]
    HFRCO = 0,
    #[doc = "1: HFXO clock is used to clock QSPI0"]
    HFXO = 1,
    #[doc = "2: AUXHFRCO is used to clock QSPI0"]
    AUXHFRCO = 2,
    #[doc = "3: USHFRCO is used to clock QSPI0"]
    USHFRCO = 3,
}
impl From<QSPI0CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: QSPI0CLKSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for QSPI0CLKSEL_A {
    type Ux = u8;
}
impl QSPI0CLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> QSPI0CLKSEL_A {
        match self.bits {
            0 => QSPI0CLKSEL_A::HFRCO,
            1 => QSPI0CLKSEL_A::HFXO,
            2 => QSPI0CLKSEL_A::AUXHFRCO,
            3 => QSPI0CLKSEL_A::USHFRCO,
            _ => unreachable!(),
        }
    }
    #[doc = "HFRCO clock is used to clock QSPI0"]
    #[inline(always)]
    pub fn is_hfrco(&self) -> bool {
        *self == QSPI0CLKSEL_A::HFRCO
    }
    #[doc = "HFXO clock is used to clock QSPI0"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == QSPI0CLKSEL_A::HFXO
    }
    #[doc = "AUXHFRCO is used to clock QSPI0"]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == QSPI0CLKSEL_A::AUXHFRCO
    }
    #[doc = "USHFRCO is used to clock QSPI0"]
    #[inline(always)]
    pub fn is_ushfrco(&self) -> bool {
        *self == QSPI0CLKSEL_A::USHFRCO
    }
}
#[doc = "Field `QSPI0CLKSEL` writer - QSPI0 Reference Clock Select"]
pub type QSPI0CLKSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, QSPI0CLKSEL_A>;
impl<'a, REG> QSPI0CLKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HFRCO clock is used to clock QSPI0"]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut crate::W<REG> {
        self.variant(QSPI0CLKSEL_A::HFRCO)
    }
    #[doc = "HFXO clock is used to clock QSPI0"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(QSPI0CLKSEL_A::HFXO)
    }
    #[doc = "AUXHFRCO is used to clock QSPI0"]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut crate::W<REG> {
        self.variant(QSPI0CLKSEL_A::AUXHFRCO)
    }
    #[doc = "USHFRCO is used to clock QSPI0"]
    #[inline(always)]
    pub fn ushfrco(self) -> &'a mut crate::W<REG> {
        self.variant(QSPI0CLKSEL_A::USHFRCO)
    }
}
#[doc = "Field `QSPI0CLKDIS` reader - QSPI0 Reference Clock Disable"]
pub type QSPI0CLKDIS_R = crate::BitReader;
#[doc = "Field `QSPI0CLKDIS` writer - QSPI0 Reference Clock Disable"]
pub type QSPI0CLKDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - QSPI0 Reference Clock Select"]
    #[inline(always)]
    pub fn qspi0clksel(&self) -> QSPI0CLKSEL_R {
        QSPI0CLKSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 7 - QSPI0 Reference Clock Disable"]
    #[inline(always)]
    pub fn qspi0clkdis(&self) -> QSPI0CLKDIS_R {
        QSPI0CLKDIS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - QSPI0 Reference Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn qspi0clksel(&mut self) -> QSPI0CLKSEL_W<QSPICTRL_SPEC> {
        QSPI0CLKSEL_W::new(self, 0)
    }
    #[doc = "Bit 7 - QSPI0 Reference Clock Disable"]
    #[inline(always)]
    #[must_use]
    pub fn qspi0clkdis(&mut self) -> QSPI0CLKDIS_W<QSPICTRL_SPEC> {
        QSPI0CLKDIS_W::new(self, 7)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "QSPI Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qspictrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qspictrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QSPICTRL_SPEC;
impl crate::RegisterSpec for QSPICTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qspictrl::R`](R) reader structure"]
impl crate::Readable for QSPICTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`qspictrl::W`](W) writer structure"]
impl crate::Writable for QSPICTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QSPICTRL to value 0"]
impl crate::Resettable for QSPICTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
