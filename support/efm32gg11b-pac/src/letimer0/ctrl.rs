#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `REPMODE` reader - Repeat Mode"]
pub type REPMODE_R = crate::FieldReader<REPMODE_A>;
#[doc = "Repeat Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REPMODE_A {
    #[doc = "0: When started, the LETIMER counts down until it is stopped by software"]
    FREE = 0,
    #[doc = "1: The counter counts REP0 times. When REP0 reaches zero, the counter stops"]
    ONESHOT = 1,
    #[doc = "2: The counter counts REP0 times. If REP1 has been written, it is loaded into REP0 when REP0 reaches zero, otherwise the counter stops"]
    BUFFERED = 2,
    #[doc = "3: Both REP0 and REP1 are decremented when the LETIMER wraps around. The LETIMER counts until both REP0 and REP1 are zero"]
    DOUBLE = 3,
}
impl From<REPMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: REPMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for REPMODE_A {
    type Ux = u8;
}
impl REPMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REPMODE_A {
        match self.bits {
            0 => REPMODE_A::FREE,
            1 => REPMODE_A::ONESHOT,
            2 => REPMODE_A::BUFFERED,
            3 => REPMODE_A::DOUBLE,
            _ => unreachable!(),
        }
    }
    #[doc = "When started, the LETIMER counts down until it is stopped by software"]
    #[inline(always)]
    pub fn is_free(&self) -> bool {
        *self == REPMODE_A::FREE
    }
    #[doc = "The counter counts REP0 times. When REP0 reaches zero, the counter stops"]
    #[inline(always)]
    pub fn is_oneshot(&self) -> bool {
        *self == REPMODE_A::ONESHOT
    }
    #[doc = "The counter counts REP0 times. If REP1 has been written, it is loaded into REP0 when REP0 reaches zero, otherwise the counter stops"]
    #[inline(always)]
    pub fn is_buffered(&self) -> bool {
        *self == REPMODE_A::BUFFERED
    }
    #[doc = "Both REP0 and REP1 are decremented when the LETIMER wraps around. The LETIMER counts until both REP0 and REP1 are zero"]
    #[inline(always)]
    pub fn is_double(&self) -> bool {
        *self == REPMODE_A::DOUBLE
    }
}
#[doc = "Field `REPMODE` writer - Repeat Mode"]
pub type REPMODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, REPMODE_A>;
impl<'a, REG> REPMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "When started, the LETIMER counts down until it is stopped by software"]
    #[inline(always)]
    pub fn free(self) -> &'a mut crate::W<REG> {
        self.variant(REPMODE_A::FREE)
    }
    #[doc = "The counter counts REP0 times. When REP0 reaches zero, the counter stops"]
    #[inline(always)]
    pub fn oneshot(self) -> &'a mut crate::W<REG> {
        self.variant(REPMODE_A::ONESHOT)
    }
    #[doc = "The counter counts REP0 times. If REP1 has been written, it is loaded into REP0 when REP0 reaches zero, otherwise the counter stops"]
    #[inline(always)]
    pub fn buffered(self) -> &'a mut crate::W<REG> {
        self.variant(REPMODE_A::BUFFERED)
    }
    #[doc = "Both REP0 and REP1 are decremented when the LETIMER wraps around. The LETIMER counts until both REP0 and REP1 are zero"]
    #[inline(always)]
    pub fn double(self) -> &'a mut crate::W<REG> {
        self.variant(REPMODE_A::DOUBLE)
    }
}
#[doc = "Field `UFOA0` reader - Underflow Output Action 0"]
pub type UFOA0_R = crate::FieldReader<UFOA0_A>;
#[doc = "Underflow Output Action 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UFOA0_A {
    #[doc = "0: LETn_O0 is held at its idle value as defined by OPOL0"]
    NONE = 0,
    #[doc = "1: LETn_O0 is toggled on CNT underflow"]
    TOGGLE = 1,
    #[doc = "2: LETn_O0 is held active for one LFACLKLETIMER0 clock cycle on CNT underflow. The output then returns to its idle value as defined by OPOL0"]
    PULSE = 2,
    #[doc = "3: LETn_O0 is set idle on CNT underflow, and active on compare match with COMP1"]
    PWM = 3,
}
impl From<UFOA0_A> for u8 {
    #[inline(always)]
    fn from(variant: UFOA0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UFOA0_A {
    type Ux = u8;
}
impl UFOA0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UFOA0_A {
        match self.bits {
            0 => UFOA0_A::NONE,
            1 => UFOA0_A::TOGGLE,
            2 => UFOA0_A::PULSE,
            3 => UFOA0_A::PWM,
            _ => unreachable!(),
        }
    }
    #[doc = "LETn_O0 is held at its idle value as defined by OPOL0"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == UFOA0_A::NONE
    }
    #[doc = "LETn_O0 is toggled on CNT underflow"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == UFOA0_A::TOGGLE
    }
    #[doc = "LETn_O0 is held active for one LFACLKLETIMER0 clock cycle on CNT underflow. The output then returns to its idle value as defined by OPOL0"]
    #[inline(always)]
    pub fn is_pulse(&self) -> bool {
        *self == UFOA0_A::PULSE
    }
    #[doc = "LETn_O0 is set idle on CNT underflow, and active on compare match with COMP1"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == UFOA0_A::PWM
    }
}
#[doc = "Field `UFOA0` writer - Underflow Output Action 0"]
pub type UFOA0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, UFOA0_A>;
impl<'a, REG> UFOA0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LETn_O0 is held at its idle value as defined by OPOL0"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(UFOA0_A::NONE)
    }
    #[doc = "LETn_O0 is toggled on CNT underflow"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(UFOA0_A::TOGGLE)
    }
    #[doc = "LETn_O0 is held active for one LFACLKLETIMER0 clock cycle on CNT underflow. The output then returns to its idle value as defined by OPOL0"]
    #[inline(always)]
    pub fn pulse(self) -> &'a mut crate::W<REG> {
        self.variant(UFOA0_A::PULSE)
    }
    #[doc = "LETn_O0 is set idle on CNT underflow, and active on compare match with COMP1"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut crate::W<REG> {
        self.variant(UFOA0_A::PWM)
    }
}
#[doc = "Field `UFOA1` reader - Underflow Output Action 1"]
pub type UFOA1_R = crate::FieldReader<UFOA1_A>;
#[doc = "Underflow Output Action 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UFOA1_A {
    #[doc = "0: LETn_O1 is held at its idle value as defined by OPOL1"]
    NONE = 0,
    #[doc = "1: LETn_O1 is toggled on CNT underflow"]
    TOGGLE = 1,
    #[doc = "2: LETn_O1 is held active for one LFACLKLETIMER0 clock cycle on CNT underflow. The output then returns to its idle value as defined by OPOL1"]
    PULSE = 2,
    #[doc = "3: LETn_O1 is set idle on CNT underflow, and active on compare match with COMP1"]
    PWM = 3,
}
impl From<UFOA1_A> for u8 {
    #[inline(always)]
    fn from(variant: UFOA1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UFOA1_A {
    type Ux = u8;
}
impl UFOA1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UFOA1_A {
        match self.bits {
            0 => UFOA1_A::NONE,
            1 => UFOA1_A::TOGGLE,
            2 => UFOA1_A::PULSE,
            3 => UFOA1_A::PWM,
            _ => unreachable!(),
        }
    }
    #[doc = "LETn_O1 is held at its idle value as defined by OPOL1"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == UFOA1_A::NONE
    }
    #[doc = "LETn_O1 is toggled on CNT underflow"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == UFOA1_A::TOGGLE
    }
    #[doc = "LETn_O1 is held active for one LFACLKLETIMER0 clock cycle on CNT underflow. The output then returns to its idle value as defined by OPOL1"]
    #[inline(always)]
    pub fn is_pulse(&self) -> bool {
        *self == UFOA1_A::PULSE
    }
    #[doc = "LETn_O1 is set idle on CNT underflow, and active on compare match with COMP1"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == UFOA1_A::PWM
    }
}
#[doc = "Field `UFOA1` writer - Underflow Output Action 1"]
pub type UFOA1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, UFOA1_A>;
impl<'a, REG> UFOA1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LETn_O1 is held at its idle value as defined by OPOL1"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(UFOA1_A::NONE)
    }
    #[doc = "LETn_O1 is toggled on CNT underflow"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(UFOA1_A::TOGGLE)
    }
    #[doc = "LETn_O1 is held active for one LFACLKLETIMER0 clock cycle on CNT underflow. The output then returns to its idle value as defined by OPOL1"]
    #[inline(always)]
    pub fn pulse(self) -> &'a mut crate::W<REG> {
        self.variant(UFOA1_A::PULSE)
    }
    #[doc = "LETn_O1 is set idle on CNT underflow, and active on compare match with COMP1"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut crate::W<REG> {
        self.variant(UFOA1_A::PWM)
    }
}
#[doc = "Field `OPOL0` reader - Output 0 Polarity"]
pub type OPOL0_R = crate::BitReader;
#[doc = "Field `OPOL0` writer - Output 0 Polarity"]
pub type OPOL0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPOL1` reader - Output 1 Polarity"]
pub type OPOL1_R = crate::BitReader;
#[doc = "Field `OPOL1` writer - Output 1 Polarity"]
pub type OPOL1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFTOP` reader - Buffered Top"]
pub type BUFTOP_R = crate::BitReader;
#[doc = "Field `BUFTOP` writer - Buffered Top"]
pub type BUFTOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP0TOP` reader - Compare Value 0 is Top Value"]
pub type COMP0TOP_R = crate::BitReader;
#[doc = "Field `COMP0TOP` writer - Compare Value 0 is Top Value"]
pub type COMP0TOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEBUGRUN` reader - Debug Mode Run Enable"]
pub type DEBUGRUN_R = crate::BitReader;
#[doc = "Field `DEBUGRUN` writer - Debug Mode Run Enable"]
pub type DEBUGRUN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Repeat Mode"]
    #[inline(always)]
    pub fn repmode(&self) -> REPMODE_R {
        REPMODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Underflow Output Action 0"]
    #[inline(always)]
    pub fn ufoa0(&self) -> UFOA0_R {
        UFOA0_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Underflow Output Action 1"]
    #[inline(always)]
    pub fn ufoa1(&self) -> UFOA1_R {
        UFOA1_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Output 0 Polarity"]
    #[inline(always)]
    pub fn opol0(&self) -> OPOL0_R {
        OPOL0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Output 1 Polarity"]
    #[inline(always)]
    pub fn opol1(&self) -> OPOL1_R {
        OPOL1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Buffered Top"]
    #[inline(always)]
    pub fn buftop(&self) -> BUFTOP_R {
        BUFTOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Compare Value 0 is Top Value"]
    #[inline(always)]
    pub fn comp0top(&self) -> COMP0TOP_R {
        COMP0TOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&self) -> DEBUGRUN_R {
        DEBUGRUN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Repeat Mode"]
    #[inline(always)]
    #[must_use]
    pub fn repmode(&mut self) -> REPMODE_W<CTRL_SPEC> {
        REPMODE_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Underflow Output Action 0"]
    #[inline(always)]
    #[must_use]
    pub fn ufoa0(&mut self) -> UFOA0_W<CTRL_SPEC> {
        UFOA0_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Underflow Output Action 1"]
    #[inline(always)]
    #[must_use]
    pub fn ufoa1(&mut self) -> UFOA1_W<CTRL_SPEC> {
        UFOA1_W::new(self, 4)
    }
    #[doc = "Bit 6 - Output 0 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn opol0(&mut self) -> OPOL0_W<CTRL_SPEC> {
        OPOL0_W::new(self, 6)
    }
    #[doc = "Bit 7 - Output 1 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn opol1(&mut self) -> OPOL1_W<CTRL_SPEC> {
        OPOL1_W::new(self, 7)
    }
    #[doc = "Bit 8 - Buffered Top"]
    #[inline(always)]
    #[must_use]
    pub fn buftop(&mut self) -> BUFTOP_W<CTRL_SPEC> {
        BUFTOP_W::new(self, 8)
    }
    #[doc = "Bit 9 - Compare Value 0 is Top Value"]
    #[inline(always)]
    #[must_use]
    pub fn comp0top(&mut self) -> COMP0TOP_W<CTRL_SPEC> {
        COMP0TOP_W::new(self, 9)
    }
    #[doc = "Bit 12 - Debug Mode Run Enable"]
    #[inline(always)]
    #[must_use]
    pub fn debugrun(&mut self) -> DEBUGRUN_W<CTRL_SPEC> {
        DEBUGRUN_W::new(self, 12)
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
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
