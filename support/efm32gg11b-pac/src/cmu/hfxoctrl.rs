#[doc = "Register `HFXOCTRL` reader"]
pub type R = crate::R<HfxoctrlSpec>;
#[doc = "Register `HFXOCTRL` writer"]
pub type W = crate::W<HfxoctrlSpec>;
#[doc = "HFXO Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: 4 MHz - 50 MHz crystal oscillator"]
    Xtal = 0,
    #[doc = "1: An AC coupled buffer is coupled in series with HFXTAL_N pin, suitable for external sinus wave."]
    Acbufextclk = 1,
    #[doc = "2: A DC coupled buffer is coupled in series with HFXTAL_N pin, suitable for external sinus wave."]
    Dcbufextclk = 2,
    #[doc = "3: Digital external clock can be supplied on HFXTAL_N pin."]
    Digextclk = 3,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - HFXO Mode"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            0 => Mode::Xtal,
            1 => Mode::Acbufextclk,
            2 => Mode::Dcbufextclk,
            3 => Mode::Digextclk,
            _ => unreachable!(),
        }
    }
    #[doc = "4 MHz - 50 MHz crystal oscillator"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == Mode::Xtal
    }
    #[doc = "An AC coupled buffer is coupled in series with HFXTAL_N pin, suitable for external sinus wave."]
    #[inline(always)]
    pub fn is_acbufextclk(&self) -> bool {
        *self == Mode::Acbufextclk
    }
    #[doc = "A DC coupled buffer is coupled in series with HFXTAL_N pin, suitable for external sinus wave."]
    #[inline(always)]
    pub fn is_dcbufextclk(&self) -> bool {
        *self == Mode::Dcbufextclk
    }
    #[doc = "Digital external clock can be supplied on HFXTAL_N pin."]
    #[inline(always)]
    pub fn is_digextclk(&self) -> bool {
        *self == Mode::Digextclk
    }
}
#[doc = "Field `MODE` writer - HFXO Mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode, crate::Safe>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4 MHz - 50 MHz crystal oscillator"]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Xtal)
    }
    #[doc = "An AC coupled buffer is coupled in series with HFXTAL_N pin, suitable for external sinus wave."]
    #[inline(always)]
    pub fn acbufextclk(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Acbufextclk)
    }
    #[doc = "A DC coupled buffer is coupled in series with HFXTAL_N pin, suitable for external sinus wave."]
    #[inline(always)]
    pub fn dcbufextclk(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Dcbufextclk)
    }
    #[doc = "Digital external clock can be supplied on HFXTAL_N pin."]
    #[inline(always)]
    pub fn digextclk(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Digextclk)
    }
}
#[doc = "Field `HFXOX2EN` reader - Enable Double Frequency on HFXOX2 Clock (compared to HFXO Clock)"]
pub type Hfxox2enR = crate::BitReader;
#[doc = "Field `HFXOX2EN` writer - Enable Double Frequency on HFXOX2 Clock (compared to HFXO Clock)"]
pub type Hfxox2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "HFXO Automatic Peak Detection Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Peakdetmode {
    #[doc = "0: Automatic control of HFXO peak detection sequence. Only performs peak detection on initial HFXO startup. CMU_CMD HFXOPEAKDETSTART allowed to be used after HFXORDY=1."]
    Oncecmd = 0,
    #[doc = "1: Automatic control of HFXO peak detection sequence. CMU_CMD HFXOPEAKDETSTART allowed to be used after HFXORDY=1."]
    Autocmd = 1,
    #[doc = "2: CMU_CMD HFXOPEAKDETSTART can be used to trigger the peak detection sequence after HFXORDY=1."]
    Cmd = 2,
    #[doc = "3: CMU_HFXOSTEADYSTATECTRL IBTRIMXOCORE and PEAKDETEN are under full software control and are allowed to be changed once HFXO is ready."]
    Manual = 3,
}
impl From<Peakdetmode> for u8 {
    #[inline(always)]
    fn from(variant: Peakdetmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Peakdetmode {
    type Ux = u8;
}
impl crate::IsEnum for Peakdetmode {}
#[doc = "Field `PEAKDETMODE` reader - HFXO Automatic Peak Detection Mode"]
pub type PeakdetmodeR = crate::FieldReader<Peakdetmode>;
impl PeakdetmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Peakdetmode {
        match self.bits {
            0 => Peakdetmode::Oncecmd,
            1 => Peakdetmode::Autocmd,
            2 => Peakdetmode::Cmd,
            3 => Peakdetmode::Manual,
            _ => unreachable!(),
        }
    }
    #[doc = "Automatic control of HFXO peak detection sequence. Only performs peak detection on initial HFXO startup. CMU_CMD HFXOPEAKDETSTART allowed to be used after HFXORDY=1."]
    #[inline(always)]
    pub fn is_oncecmd(&self) -> bool {
        *self == Peakdetmode::Oncecmd
    }
    #[doc = "Automatic control of HFXO peak detection sequence. CMU_CMD HFXOPEAKDETSTART allowed to be used after HFXORDY=1."]
    #[inline(always)]
    pub fn is_autocmd(&self) -> bool {
        *self == Peakdetmode::Autocmd
    }
    #[doc = "CMU_CMD HFXOPEAKDETSTART can be used to trigger the peak detection sequence after HFXORDY=1."]
    #[inline(always)]
    pub fn is_cmd(&self) -> bool {
        *self == Peakdetmode::Cmd
    }
    #[doc = "CMU_HFXOSTEADYSTATECTRL IBTRIMXOCORE and PEAKDETEN are under full software control and are allowed to be changed once HFXO is ready."]
    #[inline(always)]
    pub fn is_manual(&self) -> bool {
        *self == Peakdetmode::Manual
    }
}
#[doc = "Field `PEAKDETMODE` writer - HFXO Automatic Peak Detection Mode"]
pub type PeakdetmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Peakdetmode, crate::Safe>;
impl<'a, REG> PeakdetmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Automatic control of HFXO peak detection sequence. Only performs peak detection on initial HFXO startup. CMU_CMD HFXOPEAKDETSTART allowed to be used after HFXORDY=1."]
    #[inline(always)]
    pub fn oncecmd(self) -> &'a mut crate::W<REG> {
        self.variant(Peakdetmode::Oncecmd)
    }
    #[doc = "Automatic control of HFXO peak detection sequence. CMU_CMD HFXOPEAKDETSTART allowed to be used after HFXORDY=1."]
    #[inline(always)]
    pub fn autocmd(self) -> &'a mut crate::W<REG> {
        self.variant(Peakdetmode::Autocmd)
    }
    #[doc = "CMU_CMD HFXOPEAKDETSTART can be used to trigger the peak detection sequence after HFXORDY=1."]
    #[inline(always)]
    pub fn cmd(self) -> &'a mut crate::W<REG> {
        self.variant(Peakdetmode::Cmd)
    }
    #[doc = "CMU_HFXOSTEADYSTATECTRL IBTRIMXOCORE and PEAKDETEN are under full software control and are allowed to be changed once HFXO is ready."]
    #[inline(always)]
    pub fn manual(self) -> &'a mut crate::W<REG> {
        self.variant(Peakdetmode::Manual)
    }
}
#[doc = "HFXO Low Frequency Timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lftimeout {
    #[doc = "0: Timeout period of 0 cycles (disabled)"]
    _0cycles = 0,
    #[doc = "1: Timeout period of 2 cycles"]
    _2cycles = 1,
    #[doc = "2: Timeout period of 4 cycles"]
    _4cycles = 2,
    #[doc = "3: Timeout period of 16 cycles"]
    _16cycles = 3,
    #[doc = "4: Timeout period of 32 cycles"]
    _32cycles = 4,
    #[doc = "5: Timeout period of 64 cycles"]
    _64cycles = 5,
    #[doc = "6: Timeout period of 1024 cycles"]
    _1kcycles = 6,
    #[doc = "7: Timeout period of 4096 cycles"]
    _4kcycles = 7,
}
impl From<Lftimeout> for u8 {
    #[inline(always)]
    fn from(variant: Lftimeout) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lftimeout {
    type Ux = u8;
}
impl crate::IsEnum for Lftimeout {}
#[doc = "Field `LFTIMEOUT` reader - HFXO Low Frequency Timeout"]
pub type LftimeoutR = crate::FieldReader<Lftimeout>;
impl LftimeoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lftimeout {
        match self.bits {
            0 => Lftimeout::_0cycles,
            1 => Lftimeout::_2cycles,
            2 => Lftimeout::_4cycles,
            3 => Lftimeout::_16cycles,
            4 => Lftimeout::_32cycles,
            5 => Lftimeout::_64cycles,
            6 => Lftimeout::_1kcycles,
            7 => Lftimeout::_4kcycles,
            _ => unreachable!(),
        }
    }
    #[doc = "Timeout period of 0 cycles (disabled)"]
    #[inline(always)]
    pub fn is_0cycles(&self) -> bool {
        *self == Lftimeout::_0cycles
    }
    #[doc = "Timeout period of 2 cycles"]
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == Lftimeout::_2cycles
    }
    #[doc = "Timeout period of 4 cycles"]
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == Lftimeout::_4cycles
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == Lftimeout::_16cycles
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == Lftimeout::_32cycles
    }
    #[doc = "Timeout period of 64 cycles"]
    #[inline(always)]
    pub fn is_64cycles(&self) -> bool {
        *self == Lftimeout::_64cycles
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline(always)]
    pub fn is_1kcycles(&self) -> bool {
        *self == Lftimeout::_1kcycles
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline(always)]
    pub fn is_4kcycles(&self) -> bool {
        *self == Lftimeout::_4kcycles
    }
}
#[doc = "Field `LFTIMEOUT` writer - HFXO Low Frequency Timeout"]
pub type LftimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 3, Lftimeout, crate::Safe>;
impl<'a, REG> LftimeoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timeout period of 0 cycles (disabled)"]
    #[inline(always)]
    pub fn _0cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Lftimeout::_0cycles)
    }
    #[doc = "Timeout period of 2 cycles"]
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Lftimeout::_2cycles)
    }
    #[doc = "Timeout period of 4 cycles"]
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Lftimeout::_4cycles)
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Lftimeout::_16cycles)
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Lftimeout::_32cycles)
    }
    #[doc = "Timeout period of 64 cycles"]
    #[inline(always)]
    pub fn _64cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Lftimeout::_64cycles)
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline(always)]
    pub fn _1kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(Lftimeout::_1kcycles)
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline(always)]
    pub fn _4kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(Lftimeout::_4kcycles)
    }
}
#[doc = "Field `AUTOSTARTEM0EM1` reader - Automatically Start of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
pub type Autostartem0em1R = crate::BitReader;
#[doc = "Field `AUTOSTARTEM0EM1` writer - Automatically Start of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
pub type Autostartem0em1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOSTARTSELEM0EM1` reader - Automatically Start and Select of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
pub type Autostartselem0em1R = crate::BitReader;
#[doc = "Field `AUTOSTARTSELEM0EM1` writer - Automatically Start and Select of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
pub type Autostartselem0em1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - HFXO Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Enable Double Frequency on HFXOX2 Clock (compared to HFXO Clock)"]
    #[inline(always)]
    pub fn hfxox2en(&self) -> Hfxox2enR {
        Hfxox2enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - HFXO Automatic Peak Detection Mode"]
    #[inline(always)]
    pub fn peakdetmode(&self) -> PeakdetmodeR {
        PeakdetmodeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 24:26 - HFXO Low Frequency Timeout"]
    #[inline(always)]
    pub fn lftimeout(&self) -> LftimeoutR {
        LftimeoutR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 28 - Automatically Start of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
    #[inline(always)]
    pub fn autostartem0em1(&self) -> Autostartem0em1R {
        Autostartem0em1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Automatically Start and Select of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
    #[inline(always)]
    pub fn autostartselem0em1(&self) -> Autostartselem0em1R {
        Autostartselem0em1R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - HFXO Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, HfxoctrlSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bit 3 - Enable Double Frequency on HFXOX2 Clock (compared to HFXO Clock)"]
    #[inline(always)]
    pub fn hfxox2en(&mut self) -> Hfxox2enW<'_, HfxoctrlSpec> {
        Hfxox2enW::new(self, 3)
    }
    #[doc = "Bits 4:5 - HFXO Automatic Peak Detection Mode"]
    #[inline(always)]
    pub fn peakdetmode(&mut self) -> PeakdetmodeW<'_, HfxoctrlSpec> {
        PeakdetmodeW::new(self, 4)
    }
    #[doc = "Bits 24:26 - HFXO Low Frequency Timeout"]
    #[inline(always)]
    pub fn lftimeout(&mut self) -> LftimeoutW<'_, HfxoctrlSpec> {
        LftimeoutW::new(self, 24)
    }
    #[doc = "Bit 28 - Automatically Start of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
    #[inline(always)]
    pub fn autostartem0em1(&mut self) -> Autostartem0em1W<'_, HfxoctrlSpec> {
        Autostartem0em1W::new(self, 28)
    }
    #[doc = "Bit 29 - Automatically Start and Select of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
    #[inline(always)]
    pub fn autostartselem0em1(&mut self) -> Autostartselem0em1W<'_, HfxoctrlSpec> {
        Autostartselem0em1W::new(self, 29)
    }
}
#[doc = "HFXO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hfxoctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfxoctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfxoctrlSpec;
impl crate::RegisterSpec for HfxoctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfxoctrl::R`](R) reader structure"]
impl crate::Readable for HfxoctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`hfxoctrl::W`](W) writer structure"]
impl crate::Writable for HfxoctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HFXOCTRL to value 0x08"]
impl crate::Resettable for HfxoctrlSpec {
    const RESET_VALUE: u32 = 0x08;
}
