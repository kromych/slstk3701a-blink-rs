#[doc = "Register `HCFG` reader"]
pub type R = crate::R<HCFG_SPEC>;
#[doc = "Register `HCFG` writer"]
pub type W = crate::W<HCFG_SPEC>;
#[doc = "Field `FSLSPCLKSEL` reader - FS/LS PHY Clock Select"]
pub type FSLSPCLKSEL_R = crate::FieldReader<FSLSPCLKSEL_A>;
#[doc = "FS/LS PHY Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSLSPCLKSEL_A {
    #[doc = "1: Internal PHY clock is running at 48 MHz (undivided)."]
    DIV1 = 1,
    #[doc = "2: Internal PHY clock is running at 6 MHz (48 MHz divided by 8)."]
    DIV8 = 2,
}
impl From<FSLSPCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FSLSPCLKSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSLSPCLKSEL_A {
    type Ux = u8;
}
impl FSLSPCLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FSLSPCLKSEL_A> {
        match self.bits {
            1 => Some(FSLSPCLKSEL_A::DIV1),
            2 => Some(FSLSPCLKSEL_A::DIV8),
            _ => None,
        }
    }
    #[doc = "Internal PHY clock is running at 48 MHz (undivided)."]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == FSLSPCLKSEL_A::DIV1
    }
    #[doc = "Internal PHY clock is running at 6 MHz (48 MHz divided by 8)."]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == FSLSPCLKSEL_A::DIV8
    }
}
#[doc = "Field `FSLSPCLKSEL` writer - FS/LS PHY Clock Select"]
pub type FSLSPCLKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, FSLSPCLKSEL_A>;
impl<'a, REG> FSLSPCLKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal PHY clock is running at 48 MHz (undivided)."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(FSLSPCLKSEL_A::DIV1)
    }
    #[doc = "Internal PHY clock is running at 6 MHz (48 MHz divided by 8)."]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(FSLSPCLKSEL_A::DIV8)
    }
}
#[doc = "Field `FSLSSUPP` reader - FS- and LS-Only Support"]
pub type FSLSSUPP_R = crate::BitReader;
#[doc = "Field `FSLSSUPP` writer - FS- and LS-Only Support"]
pub type FSLSSUPP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENA32KHZS` reader - Enable 32 kHz Suspend Mode"]
pub type ENA32KHZS_R = crate::BitReader;
#[doc = "Field `ENA32KHZS` writer - Enable 32 kHz Suspend Mode"]
pub type ENA32KHZS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESVALID` reader - Resume Validation Period"]
pub type RESVALID_R = crate::FieldReader;
#[doc = "Field `RESVALID` writer - Resume Validation Period"]
pub type RESVALID_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MODECHTIMEN` reader - Mode Change Time"]
pub type MODECHTIMEN_R = crate::BitReader;
#[doc = "Field `MODECHTIMEN` writer - Mode Change Time"]
pub type MODECHTIMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - FS/LS PHY Clock Select"]
    #[inline(always)]
    pub fn fslspclksel(&self) -> FSLSPCLKSEL_R {
        FSLSPCLKSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - FS- and LS-Only Support"]
    #[inline(always)]
    pub fn fslssupp(&self) -> FSLSSUPP_R {
        FSLSSUPP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable 32 kHz Suspend Mode"]
    #[inline(always)]
    pub fn ena32khzs(&self) -> ENA32KHZS_R {
        ENA32KHZS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Resume Validation Period"]
    #[inline(always)]
    pub fn resvalid(&self) -> RESVALID_R {
        RESVALID_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - Mode Change Time"]
    #[inline(always)]
    pub fn modechtimen(&self) -> MODECHTIMEN_R {
        MODECHTIMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - FS/LS PHY Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn fslspclksel(&mut self) -> FSLSPCLKSEL_W<HCFG_SPEC> {
        FSLSPCLKSEL_W::new(self, 0)
    }
    #[doc = "Bit 2 - FS- and LS-Only Support"]
    #[inline(always)]
    #[must_use]
    pub fn fslssupp(&mut self) -> FSLSSUPP_W<HCFG_SPEC> {
        FSLSSUPP_W::new(self, 2)
    }
    #[doc = "Bit 7 - Enable 32 kHz Suspend Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ena32khzs(&mut self) -> ENA32KHZS_W<HCFG_SPEC> {
        ENA32KHZS_W::new(self, 7)
    }
    #[doc = "Bits 8:15 - Resume Validation Period"]
    #[inline(always)]
    #[must_use]
    pub fn resvalid(&mut self) -> RESVALID_W<HCFG_SPEC> {
        RESVALID_W::new(self, 8)
    }
    #[doc = "Bit 31 - Mode Change Time"]
    #[inline(always)]
    #[must_use]
    pub fn modechtimen(&mut self) -> MODECHTIMEN_W<HCFG_SPEC> {
        MODECHTIMEN_W::new(self, 31)
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
#[doc = "Host Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCFG_SPEC;
impl crate::RegisterSpec for HCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcfg::R`](R) reader structure"]
impl crate::Readable for HCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcfg::W`](W) writer structure"]
impl crate::Writable for HCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCFG to value 0x0200"]
impl crate::Resettable for HCFG_SPEC {
    const RESET_VALUE: u32 = 0x0200;
}
