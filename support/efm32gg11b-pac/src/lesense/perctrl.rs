#[doc = "Register `PERCTRL` reader"]
pub type R = crate::R<PERCTRL_SPEC>;
#[doc = "Register `PERCTRL` writer"]
pub type W = crate::W<PERCTRL_SPEC>;
#[doc = "Field `DACCH0EN` reader - VDAC CH0 Enable"]
pub type DACCH0EN_R = crate::BitReader;
#[doc = "Field `DACCH0EN` writer - VDAC CH0 Enable"]
pub type DACCH0EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DACCH1EN` reader - VDAC CH1 Enable"]
pub type DACCH1EN_R = crate::BitReader;
#[doc = "Field `DACCH1EN` writer - VDAC CH1 Enable"]
pub type DACCH1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DACCH0DATA` reader - VDAC CH0 Data Selection"]
pub type DACCH0DATA_R = crate::BitReader;
#[doc = "Field `DACCH0DATA` writer - VDAC CH0 Data Selection"]
pub type DACCH0DATA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DACCH1DATA` reader - VDAC CH1 Data Selection"]
pub type DACCH1DATA_R = crate::BitReader;
#[doc = "Field `DACCH1DATA` writer - VDAC CH1 Data Selection"]
pub type DACCH1DATA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DACSTARTUP` reader - VDAC Startup Configuration"]
pub type DACSTARTUP_R = crate::BitReader;
#[doc = "Field `DACSTARTUP` writer - VDAC Startup Configuration"]
pub type DACSTARTUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DACCONVTRIG` reader - VDAC Conversion Trigger Configuration"]
pub type DACCONVTRIG_R = crate::BitReader;
#[doc = "Field `DACCONVTRIG` writer - VDAC Conversion Trigger Configuration"]
pub type DACCONVTRIG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP0MODE` reader - ACMP0 Mode"]
pub type ACMP0MODE_R = crate::FieldReader<ACMP0MODE_A>;
#[doc = "ACMP0 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACMP0MODE_A {
    #[doc = "0: LESENSE does not control ACMP0"]
    DISABLE = 0,
    #[doc = "1: LESENSE controls the input mux (POSSEL) of ACMP0"]
    MUX = 1,
    #[doc = "2: LESENSE controls the input mux (POSSEL) and the threshold value (VDDLEVEL) of ACMP0"]
    MUXTHRES = 2,
}
impl From<ACMP0MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: ACMP0MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ACMP0MODE_A {
    type Ux = u8;
}
impl ACMP0MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ACMP0MODE_A> {
        match self.bits {
            0 => Some(ACMP0MODE_A::DISABLE),
            1 => Some(ACMP0MODE_A::MUX),
            2 => Some(ACMP0MODE_A::MUXTHRES),
            _ => None,
        }
    }
    #[doc = "LESENSE does not control ACMP0"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ACMP0MODE_A::DISABLE
    }
    #[doc = "LESENSE controls the input mux (POSSEL) of ACMP0"]
    #[inline(always)]
    pub fn is_mux(&self) -> bool {
        *self == ACMP0MODE_A::MUX
    }
    #[doc = "LESENSE controls the input mux (POSSEL) and the threshold value (VDDLEVEL) of ACMP0"]
    #[inline(always)]
    pub fn is_muxthres(&self) -> bool {
        *self == ACMP0MODE_A::MUXTHRES
    }
}
#[doc = "Field `ACMP0MODE` writer - ACMP0 Mode"]
pub type ACMP0MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ACMP0MODE_A>;
impl<'a, REG> ACMP0MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LESENSE does not control ACMP0"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ACMP0MODE_A::DISABLE)
    }
    #[doc = "LESENSE controls the input mux (POSSEL) of ACMP0"]
    #[inline(always)]
    pub fn mux(self) -> &'a mut crate::W<REG> {
        self.variant(ACMP0MODE_A::MUX)
    }
    #[doc = "LESENSE controls the input mux (POSSEL) and the threshold value (VDDLEVEL) of ACMP0"]
    #[inline(always)]
    pub fn muxthres(self) -> &'a mut crate::W<REG> {
        self.variant(ACMP0MODE_A::MUXTHRES)
    }
}
#[doc = "Field `ACMP1MODE` reader - ACMP1 Mode"]
pub type ACMP1MODE_R = crate::FieldReader<ACMP1MODE_A>;
#[doc = "ACMP1 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACMP1MODE_A {
    #[doc = "0: LESENSE does not control ACMP1"]
    DISABLE = 0,
    #[doc = "1: LESENSE controls the input mux (POSSEL) of ACMP1"]
    MUX = 1,
    #[doc = "2: LESENSE controls the input mux and the threshold value (VDDLEVEL) of ACMP1"]
    MUXTHRES = 2,
}
impl From<ACMP1MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: ACMP1MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ACMP1MODE_A {
    type Ux = u8;
}
impl ACMP1MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ACMP1MODE_A> {
        match self.bits {
            0 => Some(ACMP1MODE_A::DISABLE),
            1 => Some(ACMP1MODE_A::MUX),
            2 => Some(ACMP1MODE_A::MUXTHRES),
            _ => None,
        }
    }
    #[doc = "LESENSE does not control ACMP1"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ACMP1MODE_A::DISABLE
    }
    #[doc = "LESENSE controls the input mux (POSSEL) of ACMP1"]
    #[inline(always)]
    pub fn is_mux(&self) -> bool {
        *self == ACMP1MODE_A::MUX
    }
    #[doc = "LESENSE controls the input mux and the threshold value (VDDLEVEL) of ACMP1"]
    #[inline(always)]
    pub fn is_muxthres(&self) -> bool {
        *self == ACMP1MODE_A::MUXTHRES
    }
}
#[doc = "Field `ACMP1MODE` writer - ACMP1 Mode"]
pub type ACMP1MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ACMP1MODE_A>;
impl<'a, REG> ACMP1MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LESENSE does not control ACMP1"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ACMP1MODE_A::DISABLE)
    }
    #[doc = "LESENSE controls the input mux (POSSEL) of ACMP1"]
    #[inline(always)]
    pub fn mux(self) -> &'a mut crate::W<REG> {
        self.variant(ACMP1MODE_A::MUX)
    }
    #[doc = "LESENSE controls the input mux and the threshold value (VDDLEVEL) of ACMP1"]
    #[inline(always)]
    pub fn muxthres(self) -> &'a mut crate::W<REG> {
        self.variant(ACMP1MODE_A::MUXTHRES)
    }
}
#[doc = "Field `ACMP0INV` reader - Invert Analog Comparator 0 Output"]
pub type ACMP0INV_R = crate::BitReader;
#[doc = "Field `ACMP0INV` writer - Invert Analog Comparator 0 Output"]
pub type ACMP0INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP1INV` reader - Invert Analog Comparator 1 Output"]
pub type ACMP1INV_R = crate::BitReader;
#[doc = "Field `ACMP1INV` writer - Invert Analog Comparator 1 Output"]
pub type ACMP1INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP0HYSTEN` reader - ACMP0 Hysteresis Enable"]
pub type ACMP0HYSTEN_R = crate::BitReader;
#[doc = "Field `ACMP0HYSTEN` writer - ACMP0 Hysteresis Enable"]
pub type ACMP0HYSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP1HYSTEN` reader - ACMP1 Hysteresis Enable"]
pub type ACMP1HYSTEN_R = crate::BitReader;
#[doc = "Field `ACMP1HYSTEN` writer - ACMP1 Hysteresis Enable"]
pub type ACMP1HYSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WARMUPMODE` reader - ACMP and VDAC Duty Cycle Mode"]
pub type WARMUPMODE_R = crate::FieldReader<WARMUPMODE_A>;
#[doc = "ACMP and VDAC Duty Cycle Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WARMUPMODE_A {
    #[doc = "0: The analog comparators and VDAC are shut down when LESENSE is idle"]
    NORMAL = 0,
    #[doc = "1: The analog comparators are kept powered up when LESENSE is idle"]
    KEEPACMPWARM = 1,
    #[doc = "2: The VDAC is kept powered up when LESENSE is idle"]
    KEEPDACWARM = 2,
    #[doc = "3: The analog comparators and VDAC are kept powered up when LESENSE is idle"]
    KEEPACMPDACWARM = 3,
}
impl From<WARMUPMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: WARMUPMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WARMUPMODE_A {
    type Ux = u8;
}
impl WARMUPMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WARMUPMODE_A {
        match self.bits {
            0 => WARMUPMODE_A::NORMAL,
            1 => WARMUPMODE_A::KEEPACMPWARM,
            2 => WARMUPMODE_A::KEEPDACWARM,
            3 => WARMUPMODE_A::KEEPACMPDACWARM,
            _ => unreachable!(),
        }
    }
    #[doc = "The analog comparators and VDAC are shut down when LESENSE is idle"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == WARMUPMODE_A::NORMAL
    }
    #[doc = "The analog comparators are kept powered up when LESENSE is idle"]
    #[inline(always)]
    pub fn is_keepacmpwarm(&self) -> bool {
        *self == WARMUPMODE_A::KEEPACMPWARM
    }
    #[doc = "The VDAC is kept powered up when LESENSE is idle"]
    #[inline(always)]
    pub fn is_keepdacwarm(&self) -> bool {
        *self == WARMUPMODE_A::KEEPDACWARM
    }
    #[doc = "The analog comparators and VDAC are kept powered up when LESENSE is idle"]
    #[inline(always)]
    pub fn is_keepacmpdacwarm(&self) -> bool {
        *self == WARMUPMODE_A::KEEPACMPDACWARM
    }
}
#[doc = "Field `WARMUPMODE` writer - ACMP and VDAC Duty Cycle Mode"]
pub type WARMUPMODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, WARMUPMODE_A>;
impl<'a, REG> WARMUPMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The analog comparators and VDAC are shut down when LESENSE is idle"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(WARMUPMODE_A::NORMAL)
    }
    #[doc = "The analog comparators are kept powered up when LESENSE is idle"]
    #[inline(always)]
    pub fn keepacmpwarm(self) -> &'a mut crate::W<REG> {
        self.variant(WARMUPMODE_A::KEEPACMPWARM)
    }
    #[doc = "The VDAC is kept powered up when LESENSE is idle"]
    #[inline(always)]
    pub fn keepdacwarm(self) -> &'a mut crate::W<REG> {
        self.variant(WARMUPMODE_A::KEEPDACWARM)
    }
    #[doc = "The analog comparators and VDAC are kept powered up when LESENSE is idle"]
    #[inline(always)]
    pub fn keepacmpdacwarm(self) -> &'a mut crate::W<REG> {
        self.variant(WARMUPMODE_A::KEEPACMPDACWARM)
    }
}
impl R {
    #[doc = "Bit 0 - VDAC CH0 Enable"]
    #[inline(always)]
    pub fn dacch0en(&self) -> DACCH0EN_R {
        DACCH0EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VDAC CH1 Enable"]
    #[inline(always)]
    pub fn dacch1en(&self) -> DACCH1EN_R {
        DACCH1EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VDAC CH0 Data Selection"]
    #[inline(always)]
    pub fn dacch0data(&self) -> DACCH0DATA_R {
        DACCH0DATA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VDAC CH1 Data Selection"]
    #[inline(always)]
    pub fn dacch1data(&self) -> DACCH1DATA_R {
        DACCH1DATA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - VDAC Startup Configuration"]
    #[inline(always)]
    pub fn dacstartup(&self) -> DACSTARTUP_R {
        DACSTARTUP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - VDAC Conversion Trigger Configuration"]
    #[inline(always)]
    pub fn dacconvtrig(&self) -> DACCONVTRIG_R {
        DACCONVTRIG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 20:21 - ACMP0 Mode"]
    #[inline(always)]
    pub fn acmp0mode(&self) -> ACMP0MODE_R {
        ACMP0MODE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - ACMP1 Mode"]
    #[inline(always)]
    pub fn acmp1mode(&self) -> ACMP1MODE_R {
        ACMP1MODE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - Invert Analog Comparator 0 Output"]
    #[inline(always)]
    pub fn acmp0inv(&self) -> ACMP0INV_R {
        ACMP0INV_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Invert Analog Comparator 1 Output"]
    #[inline(always)]
    pub fn acmp1inv(&self) -> ACMP1INV_R {
        ACMP1INV_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - ACMP0 Hysteresis Enable"]
    #[inline(always)]
    pub fn acmp0hysten(&self) -> ACMP0HYSTEN_R {
        ACMP0HYSTEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - ACMP1 Hysteresis Enable"]
    #[inline(always)]
    pub fn acmp1hysten(&self) -> ACMP1HYSTEN_R {
        ACMP1HYSTEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - ACMP and VDAC Duty Cycle Mode"]
    #[inline(always)]
    pub fn warmupmode(&self) -> WARMUPMODE_R {
        WARMUPMODE_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - VDAC CH0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dacch0en(&mut self) -> DACCH0EN_W<PERCTRL_SPEC> {
        DACCH0EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - VDAC CH1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dacch1en(&mut self) -> DACCH1EN_W<PERCTRL_SPEC> {
        DACCH1EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - VDAC CH0 Data Selection"]
    #[inline(always)]
    #[must_use]
    pub fn dacch0data(&mut self) -> DACCH0DATA_W<PERCTRL_SPEC> {
        DACCH0DATA_W::new(self, 2)
    }
    #[doc = "Bit 3 - VDAC CH1 Data Selection"]
    #[inline(always)]
    #[must_use]
    pub fn dacch1data(&mut self) -> DACCH1DATA_W<PERCTRL_SPEC> {
        DACCH1DATA_W::new(self, 3)
    }
    #[doc = "Bit 6 - VDAC Startup Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn dacstartup(&mut self) -> DACSTARTUP_W<PERCTRL_SPEC> {
        DACSTARTUP_W::new(self, 6)
    }
    #[doc = "Bit 8 - VDAC Conversion Trigger Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn dacconvtrig(&mut self) -> DACCONVTRIG_W<PERCTRL_SPEC> {
        DACCONVTRIG_W::new(self, 8)
    }
    #[doc = "Bits 20:21 - ACMP0 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn acmp0mode(&mut self) -> ACMP0MODE_W<PERCTRL_SPEC> {
        ACMP0MODE_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - ACMP1 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn acmp1mode(&mut self) -> ACMP1MODE_W<PERCTRL_SPEC> {
        ACMP1MODE_W::new(self, 22)
    }
    #[doc = "Bit 24 - Invert Analog Comparator 0 Output"]
    #[inline(always)]
    #[must_use]
    pub fn acmp0inv(&mut self) -> ACMP0INV_W<PERCTRL_SPEC> {
        ACMP0INV_W::new(self, 24)
    }
    #[doc = "Bit 25 - Invert Analog Comparator 1 Output"]
    #[inline(always)]
    #[must_use]
    pub fn acmp1inv(&mut self) -> ACMP1INV_W<PERCTRL_SPEC> {
        ACMP1INV_W::new(self, 25)
    }
    #[doc = "Bit 26 - ACMP0 Hysteresis Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acmp0hysten(&mut self) -> ACMP0HYSTEN_W<PERCTRL_SPEC> {
        ACMP0HYSTEN_W::new(self, 26)
    }
    #[doc = "Bit 27 - ACMP1 Hysteresis Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acmp1hysten(&mut self) -> ACMP1HYSTEN_W<PERCTRL_SPEC> {
        ACMP1HYSTEN_W::new(self, 27)
    }
    #[doc = "Bits 28:29 - ACMP and VDAC Duty Cycle Mode"]
    #[inline(always)]
    #[must_use]
    pub fn warmupmode(&mut self) -> WARMUPMODE_W<PERCTRL_SPEC> {
        WARMUPMODE_W::new(self, 28)
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
#[doc = "Peripheral Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERCTRL_SPEC;
impl crate::RegisterSpec for PERCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perctrl::R`](R) reader structure"]
impl crate::Readable for PERCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`perctrl::W`](W) writer structure"]
impl crate::Writable for PERCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERCTRL to value 0"]
impl crate::Resettable for PERCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
