#[doc = "Register `LFXOCTRL` reader"]
pub type R = crate::R<LfxoctrlSpec>;
#[doc = "Register `LFXOCTRL` writer"]
pub type W = crate::W<LfxoctrlSpec>;
#[doc = "Field `TUNING` reader - LFXO Internal Capacitor Array Tuning Value"]
pub type TuningR = crate::FieldReader;
#[doc = "Field `TUNING` writer - LFXO Internal Capacitor Array Tuning Value"]
pub type TuningW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "LFXO Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: 32768 Hz crystal oscillator"]
    Xtal = 0,
    #[doc = "1: An AC coupled buffer is coupled in series with LFXTAL_N pin, suitable for external sinus wave (32768 Hz)."]
    Bufextclk = 1,
    #[doc = "2: Digital external clock on LFXTAL_N pin. Oscillator is effectively bypassed."]
    Digextclk = 2,
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
#[doc = "Field `MODE` reader - LFXO Mode"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mode> {
        match self.bits {
            0 => Some(Mode::Xtal),
            1 => Some(Mode::Bufextclk),
            2 => Some(Mode::Digextclk),
            _ => None,
        }
    }
    #[doc = "32768 Hz crystal oscillator"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == Mode::Xtal
    }
    #[doc = "An AC coupled buffer is coupled in series with LFXTAL_N pin, suitable for external sinus wave (32768 Hz)."]
    #[inline(always)]
    pub fn is_bufextclk(&self) -> bool {
        *self == Mode::Bufextclk
    }
    #[doc = "Digital external clock on LFXTAL_N pin. Oscillator is effectively bypassed."]
    #[inline(always)]
    pub fn is_digextclk(&self) -> bool {
        *self == Mode::Digextclk
    }
}
#[doc = "Field `MODE` writer - LFXO Mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32768 Hz crystal oscillator"]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Xtal)
    }
    #[doc = "An AC coupled buffer is coupled in series with LFXTAL_N pin, suitable for external sinus wave (32768 Hz)."]
    #[inline(always)]
    pub fn bufextclk(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Bufextclk)
    }
    #[doc = "Digital external clock on LFXTAL_N pin. Oscillator is effectively bypassed."]
    #[inline(always)]
    pub fn digextclk(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Digextclk)
    }
}
#[doc = "Field `GAIN` reader - LFXO Startup Gain"]
pub type GainR = crate::FieldReader;
#[doc = "Field `GAIN` writer - LFXO Startup Gain"]
pub type GainW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HIGHAMPL` reader - LFXO High XTAL Oscillation Amplitude Enable"]
pub type HighamplR = crate::BitReader;
#[doc = "Field `HIGHAMPL` writer - LFXO High XTAL Oscillation Amplitude Enable"]
pub type HighamplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AGC` reader - LFXO AGC Enable"]
pub type AgcR = crate::BitReader;
#[doc = "Field `AGC` writer - LFXO AGC Enable"]
pub type AgcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CUR` reader - LFXO Current Trim"]
pub type CurR = crate::FieldReader;
#[doc = "Field `CUR` writer - LFXO Current Trim"]
pub type CurW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BUFCUR` reader - LFXO Buffer Bias Current"]
pub type BufcurR = crate::BitReader;
#[doc = "Field `BUFCUR` writer - LFXO Buffer Bias Current"]
pub type BufcurW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "LFXO Timeout\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Timeout {
    #[doc = "0: Timeout period of 2 cycles"]
    _2cycles = 0,
    #[doc = "1: Timeout period of 256 cycles"]
    _256cycles = 1,
    #[doc = "2: Timeout period of 1024 cycles"]
    _1kcycles = 2,
    #[doc = "3: Timeout period of 2048 cycles"]
    _2kcycles = 3,
    #[doc = "4: Timeout period of 4096 cycles"]
    _4kcycles = 4,
    #[doc = "5: Timeout period of 8192 cycles"]
    _8kcycles = 5,
    #[doc = "6: Timeout period of 16384 cycles"]
    _16kcycles = 6,
    #[doc = "7: Timeout period of 32768 cycles"]
    _32kcycles = 7,
}
impl From<Timeout> for u8 {
    #[inline(always)]
    fn from(variant: Timeout) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Timeout {
    type Ux = u8;
}
impl crate::IsEnum for Timeout {}
#[doc = "Field `TIMEOUT` reader - LFXO Timeout"]
pub type TimeoutR = crate::FieldReader<Timeout>;
impl TimeoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timeout {
        match self.bits {
            0 => Timeout::_2cycles,
            1 => Timeout::_256cycles,
            2 => Timeout::_1kcycles,
            3 => Timeout::_2kcycles,
            4 => Timeout::_4kcycles,
            5 => Timeout::_8kcycles,
            6 => Timeout::_16kcycles,
            7 => Timeout::_32kcycles,
            _ => unreachable!(),
        }
    }
    #[doc = "Timeout period of 2 cycles"]
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == Timeout::_2cycles
    }
    #[doc = "Timeout period of 256 cycles"]
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == Timeout::_256cycles
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline(always)]
    pub fn is_1kcycles(&self) -> bool {
        *self == Timeout::_1kcycles
    }
    #[doc = "Timeout period of 2048 cycles"]
    #[inline(always)]
    pub fn is_2kcycles(&self) -> bool {
        *self == Timeout::_2kcycles
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline(always)]
    pub fn is_4kcycles(&self) -> bool {
        *self == Timeout::_4kcycles
    }
    #[doc = "Timeout period of 8192 cycles"]
    #[inline(always)]
    pub fn is_8kcycles(&self) -> bool {
        *self == Timeout::_8kcycles
    }
    #[doc = "Timeout period of 16384 cycles"]
    #[inline(always)]
    pub fn is_16kcycles(&self) -> bool {
        *self == Timeout::_16kcycles
    }
    #[doc = "Timeout period of 32768 cycles"]
    #[inline(always)]
    pub fn is_32kcycles(&self) -> bool {
        *self == Timeout::_32kcycles
    }
}
#[doc = "Field `TIMEOUT` writer - LFXO Timeout"]
pub type TimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 3, Timeout, crate::Safe>;
impl<'a, REG> TimeoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timeout period of 2 cycles"]
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Timeout::_2cycles)
    }
    #[doc = "Timeout period of 256 cycles"]
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Timeout::_256cycles)
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline(always)]
    pub fn _1kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(Timeout::_1kcycles)
    }
    #[doc = "Timeout period of 2048 cycles"]
    #[inline(always)]
    pub fn _2kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(Timeout::_2kcycles)
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline(always)]
    pub fn _4kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(Timeout::_4kcycles)
    }
    #[doc = "Timeout period of 8192 cycles"]
    #[inline(always)]
    pub fn _8kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(Timeout::_8kcycles)
    }
    #[doc = "Timeout period of 16384 cycles"]
    #[inline(always)]
    pub fn _16kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(Timeout::_16kcycles)
    }
    #[doc = "Timeout period of 32768 cycles"]
    #[inline(always)]
    pub fn _32kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(Timeout::_32kcycles)
    }
}
impl R {
    #[doc = "Bits 0:6 - LFXO Internal Capacitor Array Tuning Value"]
    #[inline(always)]
    pub fn tuning(&self) -> TuningR {
        TuningR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - LFXO Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 11:12 - LFXO Startup Gain"]
    #[inline(always)]
    pub fn gain(&self) -> GainR {
        GainR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 14 - LFXO High XTAL Oscillation Amplitude Enable"]
    #[inline(always)]
    pub fn highampl(&self) -> HighamplR {
        HighamplR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LFXO AGC Enable"]
    #[inline(always)]
    pub fn agc(&self) -> AgcR {
        AgcR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - LFXO Current Trim"]
    #[inline(always)]
    pub fn cur(&self) -> CurR {
        CurR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - LFXO Buffer Bias Current"]
    #[inline(always)]
    pub fn bufcur(&self) -> BufcurR {
        BufcurR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:26 - LFXO Timeout"]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - LFXO Internal Capacitor Array Tuning Value"]
    #[inline(always)]
    pub fn tuning(&mut self) -> TuningW<'_, LfxoctrlSpec> {
        TuningW::new(self, 0)
    }
    #[doc = "Bits 8:9 - LFXO Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, LfxoctrlSpec> {
        ModeW::new(self, 8)
    }
    #[doc = "Bits 11:12 - LFXO Startup Gain"]
    #[inline(always)]
    pub fn gain(&mut self) -> GainW<'_, LfxoctrlSpec> {
        GainW::new(self, 11)
    }
    #[doc = "Bit 14 - LFXO High XTAL Oscillation Amplitude Enable"]
    #[inline(always)]
    pub fn highampl(&mut self) -> HighamplW<'_, LfxoctrlSpec> {
        HighamplW::new(self, 14)
    }
    #[doc = "Bit 15 - LFXO AGC Enable"]
    #[inline(always)]
    pub fn agc(&mut self) -> AgcW<'_, LfxoctrlSpec> {
        AgcW::new(self, 15)
    }
    #[doc = "Bits 16:17 - LFXO Current Trim"]
    #[inline(always)]
    pub fn cur(&mut self) -> CurW<'_, LfxoctrlSpec> {
        CurW::new(self, 16)
    }
    #[doc = "Bit 20 - LFXO Buffer Bias Current"]
    #[inline(always)]
    pub fn bufcur(&mut self) -> BufcurW<'_, LfxoctrlSpec> {
        BufcurW::new(self, 20)
    }
    #[doc = "Bits 24:26 - LFXO Timeout"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TimeoutW<'_, LfxoctrlSpec> {
        TimeoutW::new(self, 24)
    }
}
#[doc = "LFXO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfxoctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfxoctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfxoctrlSpec;
impl crate::RegisterSpec for LfxoctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfxoctrl::R`](R) reader structure"]
impl crate::Readable for LfxoctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`lfxoctrl::W`](W) writer structure"]
impl crate::Writable for LfxoctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFXOCTRL to value 0x0700_9000"]
impl crate::Resettable for LfxoctrlSpec {
    const RESET_VALUE: u32 = 0x0700_9000;
}
