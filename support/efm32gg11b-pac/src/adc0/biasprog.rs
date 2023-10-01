#[doc = "Register `BIASPROG` reader"]
pub type R = crate::R<BIASPROG_SPEC>;
#[doc = "Register `BIASPROG` writer"]
pub type W = crate::W<BIASPROG_SPEC>;
#[doc = "Field `ADCBIASPROG` reader - Bias Programming Value of Analog ADC Block"]
pub type ADCBIASPROG_R = crate::FieldReader<ADCBIASPROG_A>;
#[doc = "Bias Programming Value of Analog ADC Block\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCBIASPROG_A {
    #[doc = "0: Normal power (use for 1Msps operation)"]
    NORMAL = 0,
    #[doc = "4: Scaling bias to 1/2"]
    SCALE2 = 4,
    #[doc = "8: Scaling bias to 1/4"]
    SCALE4 = 8,
    #[doc = "12: Scaling bias to 1/8"]
    SCALE8 = 12,
    #[doc = "14: Scaling bias to 1/16"]
    SCALE16 = 14,
    #[doc = "15: Scaling bias to 1/32"]
    SCALE32 = 15,
}
impl From<ADCBIASPROG_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCBIASPROG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADCBIASPROG_A {
    type Ux = u8;
}
impl ADCBIASPROG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADCBIASPROG_A> {
        match self.bits {
            0 => Some(ADCBIASPROG_A::NORMAL),
            4 => Some(ADCBIASPROG_A::SCALE2),
            8 => Some(ADCBIASPROG_A::SCALE4),
            12 => Some(ADCBIASPROG_A::SCALE8),
            14 => Some(ADCBIASPROG_A::SCALE16),
            15 => Some(ADCBIASPROG_A::SCALE32),
            _ => None,
        }
    }
    #[doc = "Normal power (use for 1Msps operation)"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == ADCBIASPROG_A::NORMAL
    }
    #[doc = "Scaling bias to 1/2"]
    #[inline(always)]
    pub fn is_scale2(&self) -> bool {
        *self == ADCBIASPROG_A::SCALE2
    }
    #[doc = "Scaling bias to 1/4"]
    #[inline(always)]
    pub fn is_scale4(&self) -> bool {
        *self == ADCBIASPROG_A::SCALE4
    }
    #[doc = "Scaling bias to 1/8"]
    #[inline(always)]
    pub fn is_scale8(&self) -> bool {
        *self == ADCBIASPROG_A::SCALE8
    }
    #[doc = "Scaling bias to 1/16"]
    #[inline(always)]
    pub fn is_scale16(&self) -> bool {
        *self == ADCBIASPROG_A::SCALE16
    }
    #[doc = "Scaling bias to 1/32"]
    #[inline(always)]
    pub fn is_scale32(&self) -> bool {
        *self == ADCBIASPROG_A::SCALE32
    }
}
#[doc = "Field `ADCBIASPROG` writer - Bias Programming Value of Analog ADC Block"]
pub type ADCBIASPROG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, ADCBIASPROG_A>;
impl<'a, REG, const O: u8> ADCBIASPROG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal power (use for 1Msps operation)"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(ADCBIASPROG_A::NORMAL)
    }
    #[doc = "Scaling bias to 1/2"]
    #[inline(always)]
    pub fn scale2(self) -> &'a mut crate::W<REG> {
        self.variant(ADCBIASPROG_A::SCALE2)
    }
    #[doc = "Scaling bias to 1/4"]
    #[inline(always)]
    pub fn scale4(self) -> &'a mut crate::W<REG> {
        self.variant(ADCBIASPROG_A::SCALE4)
    }
    #[doc = "Scaling bias to 1/8"]
    #[inline(always)]
    pub fn scale8(self) -> &'a mut crate::W<REG> {
        self.variant(ADCBIASPROG_A::SCALE8)
    }
    #[doc = "Scaling bias to 1/16"]
    #[inline(always)]
    pub fn scale16(self) -> &'a mut crate::W<REG> {
        self.variant(ADCBIASPROG_A::SCALE16)
    }
    #[doc = "Scaling bias to 1/32"]
    #[inline(always)]
    pub fn scale32(self) -> &'a mut crate::W<REG> {
        self.variant(ADCBIASPROG_A::SCALE32)
    }
}
#[doc = "Field `VFAULTCLR` reader - Clear VREFOF Flag"]
pub type VFAULTCLR_R = crate::BitReader;
#[doc = "Field `VFAULTCLR` writer - Clear VREFOF Flag"]
pub type VFAULTCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPBIASACC` reader - Accuracy Setting for the System Bias During ADC Operation"]
pub type GPBIASACC_R = crate::BitReader;
#[doc = "Field `GPBIASACC` writer - Accuracy Setting for the System Bias During ADC Operation"]
pub type GPBIASACC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:3 - Bias Programming Value of Analog ADC Block"]
    #[inline(always)]
    pub fn adcbiasprog(&self) -> ADCBIASPROG_R {
        ADCBIASPROG_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Clear VREFOF Flag"]
    #[inline(always)]
    pub fn vfaultclr(&self) -> VFAULTCLR_R {
        VFAULTCLR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Accuracy Setting for the System Bias During ADC Operation"]
    #[inline(always)]
    pub fn gpbiasacc(&self) -> GPBIASACC_R {
        GPBIASACC_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Bias Programming Value of Analog ADC Block"]
    #[inline(always)]
    #[must_use]
    pub fn adcbiasprog(&mut self) -> ADCBIASPROG_W<BIASPROG_SPEC, 0> {
        ADCBIASPROG_W::new(self)
    }
    #[doc = "Bit 12 - Clear VREFOF Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vfaultclr(&mut self) -> VFAULTCLR_W<BIASPROG_SPEC, 12> {
        VFAULTCLR_W::new(self)
    }
    #[doc = "Bit 16 - Accuracy Setting for the System Bias During ADC Operation"]
    #[inline(always)]
    #[must_use]
    pub fn gpbiasacc(&mut self) -> GPBIASACC_W<BIASPROG_SPEC, 16> {
        GPBIASACC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Bias Programming Register for Various Analog Blocks Used in ADC Operation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`biasprog::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`biasprog::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BIASPROG_SPEC;
impl crate::RegisterSpec for BIASPROG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`biasprog::R`](R) reader structure"]
impl crate::Readable for BIASPROG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`biasprog::W`](W) writer structure"]
impl crate::Writable for BIASPROG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BIASPROG to value 0"]
impl crate::Resettable for BIASPROG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
