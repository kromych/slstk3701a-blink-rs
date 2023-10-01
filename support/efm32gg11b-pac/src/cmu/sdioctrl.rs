#[doc = "Register `SDIOCTRL` reader"]
pub type R = crate::R<SDIOCTRL_SPEC>;
#[doc = "Register `SDIOCTRL` writer"]
pub type W = crate::W<SDIOCTRL_SPEC>;
#[doc = "Field `SDIOCLKSEL` reader - SDIO Reference Clock Select"]
pub type SDIOCLKSEL_R = crate::FieldReader<SDIOCLKSEL_A>;
#[doc = "SDIO Reference Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SDIOCLKSEL_A {
    #[doc = "0: HFRCO clock is used to clock SDIO"]
    HFRCO = 0,
    #[doc = "1: HFXO clock is used to clock SDIO"]
    HFXO = 1,
    #[doc = "2: AUXHFRCO is used to clock SDIO"]
    AUXHFRCO = 2,
    #[doc = "3: USHFRCO is used to clock SDIO"]
    USHFRCO = 3,
}
impl From<SDIOCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SDIOCLKSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SDIOCLKSEL_A {
    type Ux = u8;
}
impl SDIOCLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIOCLKSEL_A {
        match self.bits {
            0 => SDIOCLKSEL_A::HFRCO,
            1 => SDIOCLKSEL_A::HFXO,
            2 => SDIOCLKSEL_A::AUXHFRCO,
            3 => SDIOCLKSEL_A::USHFRCO,
            _ => unreachable!(),
        }
    }
    #[doc = "HFRCO clock is used to clock SDIO"]
    #[inline(always)]
    pub fn is_hfrco(&self) -> bool {
        *self == SDIOCLKSEL_A::HFRCO
    }
    #[doc = "HFXO clock is used to clock SDIO"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == SDIOCLKSEL_A::HFXO
    }
    #[doc = "AUXHFRCO is used to clock SDIO"]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == SDIOCLKSEL_A::AUXHFRCO
    }
    #[doc = "USHFRCO is used to clock SDIO"]
    #[inline(always)]
    pub fn is_ushfrco(&self) -> bool {
        *self == SDIOCLKSEL_A::USHFRCO
    }
}
#[doc = "Field `SDIOCLKSEL` writer - SDIO Reference Clock Select"]
pub type SDIOCLKSEL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, SDIOCLKSEL_A>;
impl<'a, REG, const O: u8> SDIOCLKSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HFRCO clock is used to clock SDIO"]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut crate::W<REG> {
        self.variant(SDIOCLKSEL_A::HFRCO)
    }
    #[doc = "HFXO clock is used to clock SDIO"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(SDIOCLKSEL_A::HFXO)
    }
    #[doc = "AUXHFRCO is used to clock SDIO"]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut crate::W<REG> {
        self.variant(SDIOCLKSEL_A::AUXHFRCO)
    }
    #[doc = "USHFRCO is used to clock SDIO"]
    #[inline(always)]
    pub fn ushfrco(self) -> &'a mut crate::W<REG> {
        self.variant(SDIOCLKSEL_A::USHFRCO)
    }
}
#[doc = "Field `SDIOCLKDIS` reader - SDIO Reference Clock Disable"]
pub type SDIOCLKDIS_R = crate::BitReader;
#[doc = "Field `SDIOCLKDIS` writer - SDIO Reference Clock Disable"]
pub type SDIOCLKDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - SDIO Reference Clock Select"]
    #[inline(always)]
    pub fn sdioclksel(&self) -> SDIOCLKSEL_R {
        SDIOCLKSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 7 - SDIO Reference Clock Disable"]
    #[inline(always)]
    pub fn sdioclkdis(&self) -> SDIOCLKDIS_R {
        SDIOCLKDIS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SDIO Reference Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn sdioclksel(&mut self) -> SDIOCLKSEL_W<SDIOCTRL_SPEC, 0> {
        SDIOCLKSEL_W::new(self)
    }
    #[doc = "Bit 7 - SDIO Reference Clock Disable"]
    #[inline(always)]
    #[must_use]
    pub fn sdioclkdis(&mut self) -> SDIOCLKDIS_W<SDIOCTRL_SPEC, 7> {
        SDIOCLKDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SDIO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdioctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdioctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDIOCTRL_SPEC;
impl crate::RegisterSpec for SDIOCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdioctrl::R`](R) reader structure"]
impl crate::Readable for SDIOCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdioctrl::W`](W) writer structure"]
impl crate::Writable for SDIOCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDIOCTRL to value 0"]
impl crate::Resettable for SDIOCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
