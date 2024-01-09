#[doc = "Register `DCDCLPEM01CFG` reader"]
pub type R = crate::R<DCDCLPEM01CFG_SPEC>;
#[doc = "Register `DCDCLPEM01CFG` writer"]
pub type W = crate::W<DCDCLPEM01CFG_SPEC>;
#[doc = "Field `LPCMPBIASEM01` reader - LP Mode Comparator Bias Selection for EM01"]
pub type LPCMPBIASEM01_R = crate::FieldReader<LPCMPBIASEM01_A>;
#[doc = "LP Mode Comparator Bias Selection for EM01\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPCMPBIASEM01_A {
    #[doc = "0: Maximum load current less than 75uA."]
    BIAS0 = 0,
    #[doc = "1: Maximum load current less than 500uA."]
    BIAS1 = 1,
    #[doc = "2: Maximum load current less than 2.5mA."]
    BIAS2 = 2,
    #[doc = "3: Maximum load current less than 10mA."]
    BIAS3 = 3,
}
impl From<LPCMPBIASEM01_A> for u8 {
    #[inline(always)]
    fn from(variant: LPCMPBIASEM01_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LPCMPBIASEM01_A {
    type Ux = u8;
}
impl LPCMPBIASEM01_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPCMPBIASEM01_A {
        match self.bits {
            0 => LPCMPBIASEM01_A::BIAS0,
            1 => LPCMPBIASEM01_A::BIAS1,
            2 => LPCMPBIASEM01_A::BIAS2,
            3 => LPCMPBIASEM01_A::BIAS3,
            _ => unreachable!(),
        }
    }
    #[doc = "Maximum load current less than 75uA."]
    #[inline(always)]
    pub fn is_bias0(&self) -> bool {
        *self == LPCMPBIASEM01_A::BIAS0
    }
    #[doc = "Maximum load current less than 500uA."]
    #[inline(always)]
    pub fn is_bias1(&self) -> bool {
        *self == LPCMPBIASEM01_A::BIAS1
    }
    #[doc = "Maximum load current less than 2.5mA."]
    #[inline(always)]
    pub fn is_bias2(&self) -> bool {
        *self == LPCMPBIASEM01_A::BIAS2
    }
    #[doc = "Maximum load current less than 10mA."]
    #[inline(always)]
    pub fn is_bias3(&self) -> bool {
        *self == LPCMPBIASEM01_A::BIAS3
    }
}
#[doc = "Field `LPCMPBIASEM01` writer - LP Mode Comparator Bias Selection for EM01"]
pub type LPCMPBIASEM01_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, LPCMPBIASEM01_A>;
impl<'a, REG> LPCMPBIASEM01_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Maximum load current less than 75uA."]
    #[inline(always)]
    pub fn bias0(self) -> &'a mut crate::W<REG> {
        self.variant(LPCMPBIASEM01_A::BIAS0)
    }
    #[doc = "Maximum load current less than 500uA."]
    #[inline(always)]
    pub fn bias1(self) -> &'a mut crate::W<REG> {
        self.variant(LPCMPBIASEM01_A::BIAS1)
    }
    #[doc = "Maximum load current less than 2.5mA."]
    #[inline(always)]
    pub fn bias2(self) -> &'a mut crate::W<REG> {
        self.variant(LPCMPBIASEM01_A::BIAS2)
    }
    #[doc = "Maximum load current less than 10mA."]
    #[inline(always)]
    pub fn bias3(self) -> &'a mut crate::W<REG> {
        self.variant(LPCMPBIASEM01_A::BIAS3)
    }
}
#[doc = "Field `LPCMPHYSSELEM01` reader - LP Mode Hysteresis Selection for EM01"]
pub type LPCMPHYSSELEM01_R = crate::FieldReader;
#[doc = "Field `LPCMPHYSSELEM01` writer - LP Mode Hysteresis Selection for EM01"]
pub type LPCMPHYSSELEM01_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 8:9 - LP Mode Comparator Bias Selection for EM01"]
    #[inline(always)]
    pub fn lpcmpbiasem01(&self) -> LPCMPBIASEM01_R {
        LPCMPBIASEM01_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:15 - LP Mode Hysteresis Selection for EM01"]
    #[inline(always)]
    pub fn lpcmphysselem01(&self) -> LPCMPHYSSELEM01_R {
        LPCMPHYSSELEM01_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:9 - LP Mode Comparator Bias Selection for EM01"]
    #[inline(always)]
    #[must_use]
    pub fn lpcmpbiasem01(&mut self) -> LPCMPBIASEM01_W<DCDCLPEM01CFG_SPEC> {
        LPCMPBIASEM01_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - LP Mode Hysteresis Selection for EM01"]
    #[inline(always)]
    #[must_use]
    pub fn lpcmphysselem01(&mut self) -> LPCMPHYSSELEM01_W<DCDCLPEM01CFG_SPEC> {
        LPCMPHYSSELEM01_W::new(self, 12)
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
#[doc = "Configuration Bits for Low Power Mode to Be Applied During EM01, This Field is Only Relevant If LP Mode is Used in EM01\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdclpem01cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdclpem01cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCDCLPEM01CFG_SPEC;
impl crate::RegisterSpec for DCDCLPEM01CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdclpem01cfg::R`](R) reader structure"]
impl crate::Readable for DCDCLPEM01CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcdclpem01cfg::W`](W) writer structure"]
impl crate::Writable for DCDCLPEM01CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCDCLPEM01CFG to value 0x0300"]
impl crate::Resettable for DCDCLPEM01CFG_SPEC {
    const RESET_VALUE: u32 = 0x0300;
}
