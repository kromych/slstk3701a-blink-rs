#[doc = "Register `OPA2_OUT` reader"]
pub type R = crate::R<OPA2_OUT_SPEC>;
#[doc = "Register `OPA2_OUT` writer"]
pub type W = crate::W<OPA2_OUT_SPEC>;
#[doc = "Field `MAINOUTEN` reader - OPAx Main Output Enable"]
pub type MAINOUTEN_R = crate::BitReader;
#[doc = "Field `MAINOUTEN` writer - OPAx Main Output Enable"]
pub type MAINOUTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALTOUTEN` reader - OPAx Alternative Output Enable"]
pub type ALTOUTEN_R = crate::BitReader;
#[doc = "Field `ALTOUTEN` writer - OPAx Alternative Output Enable"]
pub type ALTOUTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APORTOUTEN` reader - OPAx Aport Output Enable"]
pub type APORTOUTEN_R = crate::BitReader;
#[doc = "Field `APORTOUTEN` writer - OPAx Aport Output Enable"]
pub type APORTOUTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORT` reader - OPAx Main and Alternative Output Short"]
pub type SHORT_R = crate::BitReader;
#[doc = "Field `SHORT` writer - OPAx Main and Alternative Output Short"]
pub type SHORT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALTOUTPADEN` reader - OPAx Output Enable Value"]
pub type ALTOUTPADEN_R = crate::FieldReader<ALTOUTPADEN_A>;
#[doc = "OPAx Output Enable Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ALTOUTPADEN_A {
    #[doc = "1: Alternate Output 0"]
    OUT0 = 1,
    #[doc = "2: Alternate Output 1"]
    OUT1 = 2,
    #[doc = "4: Alternate Output 2"]
    OUT2 = 4,
    #[doc = "8: Alternate Output 3"]
    OUT3 = 8,
    #[doc = "16: Alternate Output 4"]
    OUT4 = 16,
}
impl From<ALTOUTPADEN_A> for u8 {
    #[inline(always)]
    fn from(variant: ALTOUTPADEN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ALTOUTPADEN_A {
    type Ux = u8;
}
impl ALTOUTPADEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ALTOUTPADEN_A> {
        match self.bits {
            1 => Some(ALTOUTPADEN_A::OUT0),
            2 => Some(ALTOUTPADEN_A::OUT1),
            4 => Some(ALTOUTPADEN_A::OUT2),
            8 => Some(ALTOUTPADEN_A::OUT3),
            16 => Some(ALTOUTPADEN_A::OUT4),
            _ => None,
        }
    }
    #[doc = "Alternate Output 0"]
    #[inline(always)]
    pub fn is_out0(&self) -> bool {
        *self == ALTOUTPADEN_A::OUT0
    }
    #[doc = "Alternate Output 1"]
    #[inline(always)]
    pub fn is_out1(&self) -> bool {
        *self == ALTOUTPADEN_A::OUT1
    }
    #[doc = "Alternate Output 2"]
    #[inline(always)]
    pub fn is_out2(&self) -> bool {
        *self == ALTOUTPADEN_A::OUT2
    }
    #[doc = "Alternate Output 3"]
    #[inline(always)]
    pub fn is_out3(&self) -> bool {
        *self == ALTOUTPADEN_A::OUT3
    }
    #[doc = "Alternate Output 4"]
    #[inline(always)]
    pub fn is_out4(&self) -> bool {
        *self == ALTOUTPADEN_A::OUT4
    }
}
#[doc = "Field `ALTOUTPADEN` writer - OPAx Output Enable Value"]
pub type ALTOUTPADEN_W<'a, REG> = crate::FieldWriter<'a, REG, 5, ALTOUTPADEN_A>;
impl<'a, REG> ALTOUTPADEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Alternate Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut crate::W<REG> {
        self.variant(ALTOUTPADEN_A::OUT0)
    }
    #[doc = "Alternate Output 1"]
    #[inline(always)]
    pub fn out1(self) -> &'a mut crate::W<REG> {
        self.variant(ALTOUTPADEN_A::OUT1)
    }
    #[doc = "Alternate Output 2"]
    #[inline(always)]
    pub fn out2(self) -> &'a mut crate::W<REG> {
        self.variant(ALTOUTPADEN_A::OUT2)
    }
    #[doc = "Alternate Output 3"]
    #[inline(always)]
    pub fn out3(self) -> &'a mut crate::W<REG> {
        self.variant(ALTOUTPADEN_A::OUT3)
    }
    #[doc = "Alternate Output 4"]
    #[inline(always)]
    pub fn out4(self) -> &'a mut crate::W<REG> {
        self.variant(ALTOUTPADEN_A::OUT4)
    }
}
#[doc = "Field `APORTOUTSEL` reader - OPAx APORT Output"]
pub type APORTOUTSEL_R = crate::FieldReader;
#[doc = "Field `APORTOUTSEL` writer - OPAx APORT Output"]
pub type APORTOUTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - OPAx Main Output Enable"]
    #[inline(always)]
    pub fn mainouten(&self) -> MAINOUTEN_R {
        MAINOUTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OPAx Alternative Output Enable"]
    #[inline(always)]
    pub fn altouten(&self) -> ALTOUTEN_R {
        ALTOUTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OPAx Aport Output Enable"]
    #[inline(always)]
    pub fn aportouten(&self) -> APORTOUTEN_R {
        APORTOUTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OPAx Main and Alternative Output Short"]
    #[inline(always)]
    pub fn short(&self) -> SHORT_R {
        SHORT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:8 - OPAx Output Enable Value"]
    #[inline(always)]
    pub fn altoutpaden(&self) -> ALTOUTPADEN_R {
        ALTOUTPADEN_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 16:23 - OPAx APORT Output"]
    #[inline(always)]
    pub fn aportoutsel(&self) -> APORTOUTSEL_R {
        APORTOUTSEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - OPAx Main Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mainouten(&mut self) -> MAINOUTEN_W<OPA2_OUT_SPEC> {
        MAINOUTEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - OPAx Alternative Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn altouten(&mut self) -> ALTOUTEN_W<OPA2_OUT_SPEC> {
        ALTOUTEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - OPAx Aport Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aportouten(&mut self) -> APORTOUTEN_W<OPA2_OUT_SPEC> {
        APORTOUTEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - OPAx Main and Alternative Output Short"]
    #[inline(always)]
    #[must_use]
    pub fn short(&mut self) -> SHORT_W<OPA2_OUT_SPEC> {
        SHORT_W::new(self, 3)
    }
    #[doc = "Bits 4:8 - OPAx Output Enable Value"]
    #[inline(always)]
    #[must_use]
    pub fn altoutpaden(&mut self) -> ALTOUTPADEN_W<OPA2_OUT_SPEC> {
        ALTOUTPADEN_W::new(self, 4)
    }
    #[doc = "Bits 16:23 - OPAx APORT Output"]
    #[inline(always)]
    #[must_use]
    pub fn aportoutsel(&mut self) -> APORTOUTSEL_W<OPA2_OUT_SPEC> {
        APORTOUTSEL_W::new(self, 16)
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
#[doc = "Operational Amplifier Output Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa2_out::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opa2_out::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPA2_OUT_SPEC;
impl crate::RegisterSpec for OPA2_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opa2_out::R`](R) reader structure"]
impl crate::Readable for OPA2_OUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`opa2_out::W`](W) writer structure"]
impl crate::Writable for OPA2_OUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPA2_OUT to value 0x01"]
impl crate::Resettable for OPA2_OUT_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
