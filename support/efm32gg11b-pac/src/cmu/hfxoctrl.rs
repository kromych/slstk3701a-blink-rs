#[doc = "Register `HFXOCTRL` reader"]
pub type R = crate::R<HFXOCTRL_SPEC>;
#[doc = "Register `HFXOCTRL` writer"]
pub type W = crate::W<HFXOCTRL_SPEC>;
#[doc = "Field `MODE` reader - HFXO Mode"]
pub type MODE_R = crate::FieldReader<MODE_A>;
#[doc = "HFXO Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: 4 MHz - 50 MHz crystal oscillator"]
    XTAL = 0,
    #[doc = "1: An AC coupled buffer is coupled in series with HFXTAL_N pin, suitable for external sinus wave."]
    ACBUFEXTCLK = 1,
    #[doc = "2: A DC coupled buffer is coupled in series with HFXTAL_N pin, suitable for external sinus wave."]
    DCBUFEXTCLK = 2,
    #[doc = "3: Digital external clock can be supplied on HFXTAL_N pin."]
    DIGEXTCLK = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE_A {
    type Ux = u8;
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::XTAL,
            1 => MODE_A::ACBUFEXTCLK,
            2 => MODE_A::DCBUFEXTCLK,
            3 => MODE_A::DIGEXTCLK,
            _ => unreachable!(),
        }
    }
    #[doc = "4 MHz - 50 MHz crystal oscillator"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == MODE_A::XTAL
    }
    #[doc = "An AC coupled buffer is coupled in series with HFXTAL_N pin, suitable for external sinus wave."]
    #[inline(always)]
    pub fn is_acbufextclk(&self) -> bool {
        *self == MODE_A::ACBUFEXTCLK
    }
    #[doc = "A DC coupled buffer is coupled in series with HFXTAL_N pin, suitable for external sinus wave."]
    #[inline(always)]
    pub fn is_dcbufextclk(&self) -> bool {
        *self == MODE_A::DCBUFEXTCLK
    }
    #[doc = "Digital external clock can be supplied on HFXTAL_N pin."]
    #[inline(always)]
    pub fn is_digextclk(&self) -> bool {
        *self == MODE_A::DIGEXTCLK
    }
}
#[doc = "Field `MODE` writer - HFXO Mode"]
pub type MODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, MODE_A>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4 MHz - 50 MHz crystal oscillator"]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::XTAL)
    }
    #[doc = "An AC coupled buffer is coupled in series with HFXTAL_N pin, suitable for external sinus wave."]
    #[inline(always)]
    pub fn acbufextclk(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::ACBUFEXTCLK)
    }
    #[doc = "A DC coupled buffer is coupled in series with HFXTAL_N pin, suitable for external sinus wave."]
    #[inline(always)]
    pub fn dcbufextclk(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::DCBUFEXTCLK)
    }
    #[doc = "Digital external clock can be supplied on HFXTAL_N pin."]
    #[inline(always)]
    pub fn digextclk(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::DIGEXTCLK)
    }
}
#[doc = "Field `HFXOX2EN` reader - Enable Double Frequency on HFXOX2 Clock (compared to HFXO Clock)"]
pub type HFXOX2EN_R = crate::BitReader;
#[doc = "Field `HFXOX2EN` writer - Enable Double Frequency on HFXOX2 Clock (compared to HFXO Clock)"]
pub type HFXOX2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEAKDETMODE` reader - HFXO Automatic Peak Detection Mode"]
pub type PEAKDETMODE_R = crate::FieldReader<PEAKDETMODE_A>;
#[doc = "HFXO Automatic Peak Detection Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PEAKDETMODE_A {
    #[doc = "0: Automatic control of HFXO peak detection sequence. Only performs peak detection on initial HFXO startup. CMU_CMD HFXOPEAKDETSTART allowed to be used after HFXORDY=1."]
    ONCECMD = 0,
    #[doc = "1: Automatic control of HFXO peak detection sequence. CMU_CMD HFXOPEAKDETSTART allowed to be used after HFXORDY=1."]
    AUTOCMD = 1,
    #[doc = "2: CMU_CMD HFXOPEAKDETSTART can be used to trigger the peak detection sequence after HFXORDY=1."]
    CMD = 2,
    #[doc = "3: CMU_HFXOSTEADYSTATECTRL IBTRIMXOCORE and PEAKDETEN are under full software control and are allowed to be changed once HFXO is ready."]
    MANUAL = 3,
}
impl From<PEAKDETMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PEAKDETMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PEAKDETMODE_A {
    type Ux = u8;
}
impl PEAKDETMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PEAKDETMODE_A {
        match self.bits {
            0 => PEAKDETMODE_A::ONCECMD,
            1 => PEAKDETMODE_A::AUTOCMD,
            2 => PEAKDETMODE_A::CMD,
            3 => PEAKDETMODE_A::MANUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Automatic control of HFXO peak detection sequence. Only performs peak detection on initial HFXO startup. CMU_CMD HFXOPEAKDETSTART allowed to be used after HFXORDY=1."]
    #[inline(always)]
    pub fn is_oncecmd(&self) -> bool {
        *self == PEAKDETMODE_A::ONCECMD
    }
    #[doc = "Automatic control of HFXO peak detection sequence. CMU_CMD HFXOPEAKDETSTART allowed to be used after HFXORDY=1."]
    #[inline(always)]
    pub fn is_autocmd(&self) -> bool {
        *self == PEAKDETMODE_A::AUTOCMD
    }
    #[doc = "CMU_CMD HFXOPEAKDETSTART can be used to trigger the peak detection sequence after HFXORDY=1."]
    #[inline(always)]
    pub fn is_cmd(&self) -> bool {
        *self == PEAKDETMODE_A::CMD
    }
    #[doc = "CMU_HFXOSTEADYSTATECTRL IBTRIMXOCORE and PEAKDETEN are under full software control and are allowed to be changed once HFXO is ready."]
    #[inline(always)]
    pub fn is_manual(&self) -> bool {
        *self == PEAKDETMODE_A::MANUAL
    }
}
#[doc = "Field `PEAKDETMODE` writer - HFXO Automatic Peak Detection Mode"]
pub type PEAKDETMODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PEAKDETMODE_A>;
impl<'a, REG> PEAKDETMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Automatic control of HFXO peak detection sequence. Only performs peak detection on initial HFXO startup. CMU_CMD HFXOPEAKDETSTART allowed to be used after HFXORDY=1."]
    #[inline(always)]
    pub fn oncecmd(self) -> &'a mut crate::W<REG> {
        self.variant(PEAKDETMODE_A::ONCECMD)
    }
    #[doc = "Automatic control of HFXO peak detection sequence. CMU_CMD HFXOPEAKDETSTART allowed to be used after HFXORDY=1."]
    #[inline(always)]
    pub fn autocmd(self) -> &'a mut crate::W<REG> {
        self.variant(PEAKDETMODE_A::AUTOCMD)
    }
    #[doc = "CMU_CMD HFXOPEAKDETSTART can be used to trigger the peak detection sequence after HFXORDY=1."]
    #[inline(always)]
    pub fn cmd(self) -> &'a mut crate::W<REG> {
        self.variant(PEAKDETMODE_A::CMD)
    }
    #[doc = "CMU_HFXOSTEADYSTATECTRL IBTRIMXOCORE and PEAKDETEN are under full software control and are allowed to be changed once HFXO is ready."]
    #[inline(always)]
    pub fn manual(self) -> &'a mut crate::W<REG> {
        self.variant(PEAKDETMODE_A::MANUAL)
    }
}
#[doc = "Field `LFTIMEOUT` reader - HFXO Low Frequency Timeout"]
pub type LFTIMEOUT_R = crate::FieldReader<LFTIMEOUT_A>;
#[doc = "HFXO Low Frequency Timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LFTIMEOUT_A {
    #[doc = "0: Timeout period of 0 cycles (disabled)"]
    _0CYCLES = 0,
    #[doc = "1: Timeout period of 2 cycles"]
    _2CYCLES = 1,
    #[doc = "2: Timeout period of 4 cycles"]
    _4CYCLES = 2,
    #[doc = "3: Timeout period of 16 cycles"]
    _16CYCLES = 3,
    #[doc = "4: Timeout period of 32 cycles"]
    _32CYCLES = 4,
    #[doc = "5: Timeout period of 64 cycles"]
    _64CYCLES = 5,
    #[doc = "6: Timeout period of 1024 cycles"]
    _1KCYCLES = 6,
    #[doc = "7: Timeout period of 4096 cycles"]
    _4KCYCLES = 7,
}
impl From<LFTIMEOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: LFTIMEOUT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LFTIMEOUT_A {
    type Ux = u8;
}
impl LFTIMEOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LFTIMEOUT_A {
        match self.bits {
            0 => LFTIMEOUT_A::_0CYCLES,
            1 => LFTIMEOUT_A::_2CYCLES,
            2 => LFTIMEOUT_A::_4CYCLES,
            3 => LFTIMEOUT_A::_16CYCLES,
            4 => LFTIMEOUT_A::_32CYCLES,
            5 => LFTIMEOUT_A::_64CYCLES,
            6 => LFTIMEOUT_A::_1KCYCLES,
            7 => LFTIMEOUT_A::_4KCYCLES,
            _ => unreachable!(),
        }
    }
    #[doc = "Timeout period of 0 cycles (disabled)"]
    #[inline(always)]
    pub fn is_0cycles(&self) -> bool {
        *self == LFTIMEOUT_A::_0CYCLES
    }
    #[doc = "Timeout period of 2 cycles"]
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == LFTIMEOUT_A::_2CYCLES
    }
    #[doc = "Timeout period of 4 cycles"]
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == LFTIMEOUT_A::_4CYCLES
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == LFTIMEOUT_A::_16CYCLES
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == LFTIMEOUT_A::_32CYCLES
    }
    #[doc = "Timeout period of 64 cycles"]
    #[inline(always)]
    pub fn is_64cycles(&self) -> bool {
        *self == LFTIMEOUT_A::_64CYCLES
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline(always)]
    pub fn is_1kcycles(&self) -> bool {
        *self == LFTIMEOUT_A::_1KCYCLES
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline(always)]
    pub fn is_4kcycles(&self) -> bool {
        *self == LFTIMEOUT_A::_4KCYCLES
    }
}
#[doc = "Field `LFTIMEOUT` writer - HFXO Low Frequency Timeout"]
pub type LFTIMEOUT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, LFTIMEOUT_A>;
impl<'a, REG> LFTIMEOUT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timeout period of 0 cycles (disabled)"]
    #[inline(always)]
    pub fn _0cycles(self) -> &'a mut crate::W<REG> {
        self.variant(LFTIMEOUT_A::_0CYCLES)
    }
    #[doc = "Timeout period of 2 cycles"]
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut crate::W<REG> {
        self.variant(LFTIMEOUT_A::_2CYCLES)
    }
    #[doc = "Timeout period of 4 cycles"]
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut crate::W<REG> {
        self.variant(LFTIMEOUT_A::_4CYCLES)
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut crate::W<REG> {
        self.variant(LFTIMEOUT_A::_16CYCLES)
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut crate::W<REG> {
        self.variant(LFTIMEOUT_A::_32CYCLES)
    }
    #[doc = "Timeout period of 64 cycles"]
    #[inline(always)]
    pub fn _64cycles(self) -> &'a mut crate::W<REG> {
        self.variant(LFTIMEOUT_A::_64CYCLES)
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline(always)]
    pub fn _1kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(LFTIMEOUT_A::_1KCYCLES)
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline(always)]
    pub fn _4kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(LFTIMEOUT_A::_4KCYCLES)
    }
}
#[doc = "Field `AUTOSTARTEM0EM1` reader - Automatically Start of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
pub type AUTOSTARTEM0EM1_R = crate::BitReader;
#[doc = "Field `AUTOSTARTEM0EM1` writer - Automatically Start of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
pub type AUTOSTARTEM0EM1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOSTARTSELEM0EM1` reader - Automatically Start and Select of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
pub type AUTOSTARTSELEM0EM1_R = crate::BitReader;
#[doc = "Field `AUTOSTARTSELEM0EM1` writer - Automatically Start and Select of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
pub type AUTOSTARTSELEM0EM1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - HFXO Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Enable Double Frequency on HFXOX2 Clock (compared to HFXO Clock)"]
    #[inline(always)]
    pub fn hfxox2en(&self) -> HFXOX2EN_R {
        HFXOX2EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - HFXO Automatic Peak Detection Mode"]
    #[inline(always)]
    pub fn peakdetmode(&self) -> PEAKDETMODE_R {
        PEAKDETMODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 24:26 - HFXO Low Frequency Timeout"]
    #[inline(always)]
    pub fn lftimeout(&self) -> LFTIMEOUT_R {
        LFTIMEOUT_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 28 - Automatically Start of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
    #[inline(always)]
    pub fn autostartem0em1(&self) -> AUTOSTARTEM0EM1_R {
        AUTOSTARTEM0EM1_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Automatically Start and Select of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
    #[inline(always)]
    pub fn autostartselem0em1(&self) -> AUTOSTARTSELEM0EM1_R {
        AUTOSTARTSELEM0EM1_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - HFXO Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<HFXOCTRL_SPEC> {
        MODE_W::new(self, 0)
    }
    #[doc = "Bit 3 - Enable Double Frequency on HFXOX2 Clock (compared to HFXO Clock)"]
    #[inline(always)]
    #[must_use]
    pub fn hfxox2en(&mut self) -> HFXOX2EN_W<HFXOCTRL_SPEC> {
        HFXOX2EN_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - HFXO Automatic Peak Detection Mode"]
    #[inline(always)]
    #[must_use]
    pub fn peakdetmode(&mut self) -> PEAKDETMODE_W<HFXOCTRL_SPEC> {
        PEAKDETMODE_W::new(self, 4)
    }
    #[doc = "Bits 24:26 - HFXO Low Frequency Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn lftimeout(&mut self) -> LFTIMEOUT_W<HFXOCTRL_SPEC> {
        LFTIMEOUT_W::new(self, 24)
    }
    #[doc = "Bit 28 - Automatically Start of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
    #[inline(always)]
    #[must_use]
    pub fn autostartem0em1(&mut self) -> AUTOSTARTEM0EM1_W<HFXOCTRL_SPEC> {
        AUTOSTARTEM0EM1_W::new(self, 28)
    }
    #[doc = "Bit 29 - Automatically Start and Select of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
    #[inline(always)]
    #[must_use]
    pub fn autostartselem0em1(&mut self) -> AUTOSTARTSELEM0EM1_W<HFXOCTRL_SPEC> {
        AUTOSTARTSELEM0EM1_W::new(self, 29)
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
#[doc = "HFXO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfxoctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfxoctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFXOCTRL_SPEC;
impl crate::RegisterSpec for HFXOCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfxoctrl::R`](R) reader structure"]
impl crate::Readable for HFXOCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hfxoctrl::W`](W) writer structure"]
impl crate::Writable for HFXOCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HFXOCTRL to value 0x08"]
impl crate::Resettable for HFXOCTRL_SPEC {
    const RESET_VALUE: u32 = 0x08;
}
