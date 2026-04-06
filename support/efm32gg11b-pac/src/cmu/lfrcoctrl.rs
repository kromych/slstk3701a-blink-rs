#[doc = "Register `LFRCOCTRL` reader"]
pub type R = crate::R<LfrcoctrlSpec>;
#[doc = "Register `LFRCOCTRL` writer"]
pub type W = crate::W<LfrcoctrlSpec>;
#[doc = "Field `TUNING` reader - LFRCO Tuning Value"]
pub type TuningR = crate::FieldReader<u16>;
#[doc = "Field `TUNING` writer - LFRCO Tuning Value"]
pub type TuningW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `ENVREF` reader - Enable Duty Cycling of Vref"]
pub type EnvrefR = crate::BitReader;
#[doc = "Field `ENVREF` writer - Enable Duty Cycling of Vref"]
pub type EnvrefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENCHOP` reader - Enable Comparator Chopping"]
pub type EnchopR = crate::BitReader;
#[doc = "Field `ENCHOP` writer - Enable Comparator Chopping"]
pub type EnchopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDEM` reader - Enable Dynamic Element Matching"]
pub type EndemR = crate::BitReader;
#[doc = "Field `ENDEM` writer - Enable Dynamic Element Matching"]
pub type EndemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Control Vref Update Rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vrefupdate {
    #[doc = "0: 32 clocks."]
    _32cycles = 0,
    #[doc = "1: 64 clocks."]
    _64cycles = 1,
    #[doc = "2: 128 clocks."]
    _128cycles = 2,
    #[doc = "3: 256 clocks."]
    _256cycles = 3,
}
impl From<Vrefupdate> for u8 {
    #[inline(always)]
    fn from(variant: Vrefupdate) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vrefupdate {
    type Ux = u8;
}
impl crate::IsEnum for Vrefupdate {}
#[doc = "Field `VREFUPDATE` reader - Control Vref Update Rate"]
pub type VrefupdateR = crate::FieldReader<Vrefupdate>;
impl VrefupdateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vrefupdate {
        match self.bits {
            0 => Vrefupdate::_32cycles,
            1 => Vrefupdate::_64cycles,
            2 => Vrefupdate::_128cycles,
            3 => Vrefupdate::_256cycles,
            _ => unreachable!(),
        }
    }
    #[doc = "32 clocks."]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == Vrefupdate::_32cycles
    }
    #[doc = "64 clocks."]
    #[inline(always)]
    pub fn is_64cycles(&self) -> bool {
        *self == Vrefupdate::_64cycles
    }
    #[doc = "128 clocks."]
    #[inline(always)]
    pub fn is_128cycles(&self) -> bool {
        *self == Vrefupdate::_128cycles
    }
    #[doc = "256 clocks."]
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == Vrefupdate::_256cycles
    }
}
#[doc = "Field `VREFUPDATE` writer - Control Vref Update Rate"]
pub type VrefupdateW<'a, REG> = crate::FieldWriter<'a, REG, 2, Vrefupdate, crate::Safe>;
impl<'a, REG> VrefupdateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32 clocks."]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Vrefupdate::_32cycles)
    }
    #[doc = "64 clocks."]
    #[inline(always)]
    pub fn _64cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Vrefupdate::_64cycles)
    }
    #[doc = "128 clocks."]
    #[inline(always)]
    pub fn _128cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Vrefupdate::_128cycles)
    }
    #[doc = "256 clocks."]
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Vrefupdate::_256cycles)
    }
}
#[doc = "LFRCO Timeout\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Timeout {
    #[doc = "0: Timeout period of 2 cycles"]
    _2cycles = 0,
    #[doc = "1: Timeout period of 16 cycles"]
    _16cycles = 1,
    #[doc = "2: Timeout period of 32 cycles"]
    _32cycles = 2,
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
#[doc = "Field `TIMEOUT` reader - LFRCO Timeout"]
pub type TimeoutR = crate::FieldReader<Timeout>;
impl TimeoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Timeout> {
        match self.bits {
            0 => Some(Timeout::_2cycles),
            1 => Some(Timeout::_16cycles),
            2 => Some(Timeout::_32cycles),
            _ => None,
        }
    }
    #[doc = "Timeout period of 2 cycles"]
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == Timeout::_2cycles
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == Timeout::_16cycles
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == Timeout::_32cycles
    }
}
#[doc = "Field `TIMEOUT` writer - LFRCO Timeout"]
pub type TimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 2, Timeout>;
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
    #[doc = "Timeout period of 16 cycles"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Timeout::_16cycles)
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Timeout::_32cycles)
    }
}
#[doc = "Field `GMCCURTUNE` reader - Tuning of Gmc Current"]
pub type GmccurtuneR = crate::FieldReader;
#[doc = "Field `GMCCURTUNE` writer - Tuning of Gmc Current"]
pub type GmccurtuneW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:8 - LFRCO Tuning Value"]
    #[inline(always)]
    pub fn tuning(&self) -> TuningR {
        TuningR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 16 - Enable Duty Cycling of Vref"]
    #[inline(always)]
    pub fn envref(&self) -> EnvrefR {
        EnvrefR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Comparator Chopping"]
    #[inline(always)]
    pub fn enchop(&self) -> EnchopR {
        EnchopR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Dynamic Element Matching"]
    #[inline(always)]
    pub fn endem(&self) -> EndemR {
        EndemR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Control Vref Update Rate"]
    #[inline(always)]
    pub fn vrefupdate(&self) -> VrefupdateR {
        VrefupdateR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - LFRCO Timeout"]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:31 - Tuning of Gmc Current"]
    #[inline(always)]
    pub fn gmccurtune(&self) -> GmccurtuneR {
        GmccurtuneR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - LFRCO Tuning Value"]
    #[inline(always)]
    pub fn tuning(&mut self) -> TuningW<'_, LfrcoctrlSpec> {
        TuningW::new(self, 0)
    }
    #[doc = "Bit 16 - Enable Duty Cycling of Vref"]
    #[inline(always)]
    pub fn envref(&mut self) -> EnvrefW<'_, LfrcoctrlSpec> {
        EnvrefW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Comparator Chopping"]
    #[inline(always)]
    pub fn enchop(&mut self) -> EnchopW<'_, LfrcoctrlSpec> {
        EnchopW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Dynamic Element Matching"]
    #[inline(always)]
    pub fn endem(&mut self) -> EndemW<'_, LfrcoctrlSpec> {
        EndemW::new(self, 18)
    }
    #[doc = "Bits 20:21 - Control Vref Update Rate"]
    #[inline(always)]
    pub fn vrefupdate(&mut self) -> VrefupdateW<'_, LfrcoctrlSpec> {
        VrefupdateW::new(self, 20)
    }
    #[doc = "Bits 24:25 - LFRCO Timeout"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TimeoutW<'_, LfrcoctrlSpec> {
        TimeoutW::new(self, 24)
    }
    #[doc = "Bits 28:31 - Tuning of Gmc Current"]
    #[inline(always)]
    pub fn gmccurtune(&mut self) -> GmccurtuneW<'_, LfrcoctrlSpec> {
        GmccurtuneW::new(self, 28)
    }
}
#[doc = "LFRCO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfrcoctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfrcoctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfrcoctrlSpec;
impl crate::RegisterSpec for LfrcoctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfrcoctrl::R`](R) reader structure"]
impl crate::Readable for LfrcoctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`lfrcoctrl::W`](W) writer structure"]
impl crate::Writable for LfrcoctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFRCOCTRL to value 0x8106_0100"]
impl crate::Resettable for LfrcoctrlSpec {
    const RESET_VALUE: u32 = 0x8106_0100;
}
