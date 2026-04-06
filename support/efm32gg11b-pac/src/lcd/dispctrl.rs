#[doc = "Register `DISPCTRL` reader"]
pub type R = crate::R<DispctrlSpec>;
#[doc = "Register `DISPCTRL` writer"]
pub type W = crate::W<DispctrlSpec>;
#[doc = "Mux Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mux {
    #[doc = "0: Static"]
    Static = 0,
    #[doc = "1: Duplex"]
    Duplex = 1,
    #[doc = "2: Triplex"]
    Triplex = 2,
    #[doc = "3: Quadruplex"]
    Quadruplex = 3,
    #[doc = "5: Sextaplex"]
    Sextaplex = 5,
    #[doc = "7: Octaplex"]
    Octaplex = 7,
}
impl From<Mux> for u8 {
    #[inline(always)]
    fn from(variant: Mux) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mux {
    type Ux = u8;
}
impl crate::IsEnum for Mux {}
#[doc = "Field `MUX` reader - Mux Configuration"]
pub type MuxR = crate::FieldReader<Mux>;
impl MuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mux> {
        match self.bits {
            0 => Some(Mux::Static),
            1 => Some(Mux::Duplex),
            2 => Some(Mux::Triplex),
            3 => Some(Mux::Quadruplex),
            5 => Some(Mux::Sextaplex),
            7 => Some(Mux::Octaplex),
            _ => None,
        }
    }
    #[doc = "Static"]
    #[inline(always)]
    pub fn is_static(&self) -> bool {
        *self == Mux::Static
    }
    #[doc = "Duplex"]
    #[inline(always)]
    pub fn is_duplex(&self) -> bool {
        *self == Mux::Duplex
    }
    #[doc = "Triplex"]
    #[inline(always)]
    pub fn is_triplex(&self) -> bool {
        *self == Mux::Triplex
    }
    #[doc = "Quadruplex"]
    #[inline(always)]
    pub fn is_quadruplex(&self) -> bool {
        *self == Mux::Quadruplex
    }
    #[doc = "Sextaplex"]
    #[inline(always)]
    pub fn is_sextaplex(&self) -> bool {
        *self == Mux::Sextaplex
    }
    #[doc = "Octaplex"]
    #[inline(always)]
    pub fn is_octaplex(&self) -> bool {
        *self == Mux::Octaplex
    }
}
#[doc = "Field `MUX` writer - Mux Configuration"]
pub type MuxW<'a, REG> = crate::FieldWriter<'a, REG, 3, Mux>;
impl<'a, REG> MuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Static"]
    #[inline(always)]
    pub fn static_(self) -> &'a mut crate::W<REG> {
        self.variant(Mux::Static)
    }
    #[doc = "Duplex"]
    #[inline(always)]
    pub fn duplex(self) -> &'a mut crate::W<REG> {
        self.variant(Mux::Duplex)
    }
    #[doc = "Triplex"]
    #[inline(always)]
    pub fn triplex(self) -> &'a mut crate::W<REG> {
        self.variant(Mux::Triplex)
    }
    #[doc = "Quadruplex"]
    #[inline(always)]
    pub fn quadruplex(self) -> &'a mut crate::W<REG> {
        self.variant(Mux::Quadruplex)
    }
    #[doc = "Sextaplex"]
    #[inline(always)]
    pub fn sextaplex(self) -> &'a mut crate::W<REG> {
        self.variant(Mux::Sextaplex)
    }
    #[doc = "Octaplex"]
    #[inline(always)]
    pub fn octaplex(self) -> &'a mut crate::W<REG> {
        self.variant(Mux::Octaplex)
    }
}
#[doc = "Field `WAVE` reader - Waveform Selection"]
pub type WaveR = crate::BitReader;
#[doc = "Field `WAVE` writer - Waveform Selection"]
pub type WaveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONTRAST` reader - Contrast Control"]
pub type ContrastR = crate::FieldReader;
#[doc = "Field `CONTRAST` writer - Contrast Control"]
pub type ContrastW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Charge Redistribution Cycles\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chgrdst {
    #[doc = "0: Disable charge redistribution."]
    Disable = 0,
    #[doc = "1: Use 1 prescaled low frequency clock cycle for charge redistribution."]
    One = 1,
    #[doc = "2: Use 2 prescaled low frequency clock cycles for charge redistribution."]
    Two = 2,
    #[doc = "3: Use 3 prescaled low frequency clock cycles for charge redistribution."]
    Three = 3,
    #[doc = "4: Use 4 prescaled low frequency clock cycles for charge redistribution."]
    Four = 4,
}
impl From<Chgrdst> for u8 {
    #[inline(always)]
    fn from(variant: Chgrdst) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chgrdst {
    type Ux = u8;
}
impl crate::IsEnum for Chgrdst {}
#[doc = "Field `CHGRDST` reader - Charge Redistribution Cycles"]
pub type ChgrdstR = crate::FieldReader<Chgrdst>;
impl ChgrdstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Chgrdst> {
        match self.bits {
            0 => Some(Chgrdst::Disable),
            1 => Some(Chgrdst::One),
            2 => Some(Chgrdst::Two),
            3 => Some(Chgrdst::Three),
            4 => Some(Chgrdst::Four),
            _ => None,
        }
    }
    #[doc = "Disable charge redistribution."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chgrdst::Disable
    }
    #[doc = "Use 1 prescaled low frequency clock cycle for charge redistribution."]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Chgrdst::One
    }
    #[doc = "Use 2 prescaled low frequency clock cycles for charge redistribution."]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == Chgrdst::Two
    }
    #[doc = "Use 3 prescaled low frequency clock cycles for charge redistribution."]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == Chgrdst::Three
    }
    #[doc = "Use 4 prescaled low frequency clock cycles for charge redistribution."]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == Chgrdst::Four
    }
}
#[doc = "Field `CHGRDST` writer - Charge Redistribution Cycles"]
pub type ChgrdstW<'a, REG> = crate::FieldWriter<'a, REG, 3, Chgrdst>;
impl<'a, REG> ChgrdstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable charge redistribution."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chgrdst::Disable)
    }
    #[doc = "Use 1 prescaled low frequency clock cycle for charge redistribution."]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Chgrdst::One)
    }
    #[doc = "Use 2 prescaled low frequency clock cycles for charge redistribution."]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(Chgrdst::Two)
    }
    #[doc = "Use 3 prescaled low frequency clock cycles for charge redistribution."]
    #[inline(always)]
    pub fn three(self) -> &'a mut crate::W<REG> {
        self.variant(Chgrdst::Three)
    }
    #[doc = "Use 4 prescaled low frequency clock cycles for charge redistribution."]
    #[inline(always)]
    pub fn four(self) -> &'a mut crate::W<REG> {
        self.variant(Chgrdst::Four)
    }
}
#[doc = "Bias Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bias {
    #[doc = "0: Static"]
    Static = 0,
    #[doc = "1: 1/2 Bias"]
    Onehalf = 1,
    #[doc = "2: 1/3 Bias"]
    Onethird = 2,
    #[doc = "3: 1/4 Bias"]
    Onefourth = 3,
}
impl From<Bias> for u8 {
    #[inline(always)]
    fn from(variant: Bias) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bias {
    type Ux = u8;
}
impl crate::IsEnum for Bias {}
#[doc = "Field `BIAS` reader - Bias Configuration"]
pub type BiasR = crate::FieldReader<Bias>;
impl BiasR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bias {
        match self.bits {
            0 => Bias::Static,
            1 => Bias::Onehalf,
            2 => Bias::Onethird,
            3 => Bias::Onefourth,
            _ => unreachable!(),
        }
    }
    #[doc = "Static"]
    #[inline(always)]
    pub fn is_static(&self) -> bool {
        *self == Bias::Static
    }
    #[doc = "1/2 Bias"]
    #[inline(always)]
    pub fn is_onehalf(&self) -> bool {
        *self == Bias::Onehalf
    }
    #[doc = "1/3 Bias"]
    #[inline(always)]
    pub fn is_onethird(&self) -> bool {
        *self == Bias::Onethird
    }
    #[doc = "1/4 Bias"]
    #[inline(always)]
    pub fn is_onefourth(&self) -> bool {
        *self == Bias::Onefourth
    }
}
#[doc = "Field `BIAS` writer - Bias Configuration"]
pub type BiasW<'a, REG> = crate::FieldWriter<'a, REG, 2, Bias, crate::Safe>;
impl<'a, REG> BiasW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Static"]
    #[inline(always)]
    pub fn static_(self) -> &'a mut crate::W<REG> {
        self.variant(Bias::Static)
    }
    #[doc = "1/2 Bias"]
    #[inline(always)]
    pub fn onehalf(self) -> &'a mut crate::W<REG> {
        self.variant(Bias::Onehalf)
    }
    #[doc = "1/3 Bias"]
    #[inline(always)]
    pub fn onethird(self) -> &'a mut crate::W<REG> {
        self.variant(Bias::Onethird)
    }
    #[doc = "1/4 Bias"]
    #[inline(always)]
    pub fn onefourth(self) -> &'a mut crate::W<REG> {
        self.variant(Bias::Onefourth)
    }
}
#[doc = "Mode Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: No External Cap. Uses an internal current source to generate VLCD. Use CONTRAST\\[4:0\\] to control VLCD."]
    Noextcap = 0,
    #[doc = "1: Use step down control with VLCD less than VDD. Use CONTRAST\\[5:0\\] to control VLCD level, and use SPEED to adjust VLCD drive strength."]
    Stepdown = 1,
    #[doc = "2: Charge pump used with internal oscillator. Use CONTRAST\\[5:0\\] to control VLCD level, and use SPEED to adjust oscillator frequency."]
    Cpintosc = 2,
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
#[doc = "Field `MODE` reader - Mode Setting"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mode> {
        match self.bits {
            0 => Some(Mode::Noextcap),
            1 => Some(Mode::Stepdown),
            2 => Some(Mode::Cpintosc),
            _ => None,
        }
    }
    #[doc = "No External Cap. Uses an internal current source to generate VLCD. Use CONTRAST\\[4:0\\] to control VLCD."]
    #[inline(always)]
    pub fn is_noextcap(&self) -> bool {
        *self == Mode::Noextcap
    }
    #[doc = "Use step down control with VLCD less than VDD. Use CONTRAST\\[5:0\\] to control VLCD level, and use SPEED to adjust VLCD drive strength."]
    #[inline(always)]
    pub fn is_stepdown(&self) -> bool {
        *self == Mode::Stepdown
    }
    #[doc = "Charge pump used with internal oscillator. Use CONTRAST\\[5:0\\] to control VLCD level, and use SPEED to adjust oscillator frequency."]
    #[inline(always)]
    pub fn is_cpintosc(&self) -> bool {
        *self == Mode::Cpintosc
    }
}
#[doc = "Field `MODE` writer - Mode Setting"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No External Cap. Uses an internal current source to generate VLCD. Use CONTRAST\\[4:0\\] to control VLCD."]
    #[inline(always)]
    pub fn noextcap(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Noextcap)
    }
    #[doc = "Use step down control with VLCD less than VDD. Use CONTRAST\\[5:0\\] to control VLCD level, and use SPEED to adjust VLCD drive strength."]
    #[inline(always)]
    pub fn stepdown(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Stepdown)
    }
    #[doc = "Charge pump used with internal oscillator. Use CONTRAST\\[5:0\\] to control VLCD level, and use SPEED to adjust oscillator frequency."]
    #[inline(always)]
    pub fn cpintosc(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Cpintosc)
    }
}
impl R {
    #[doc = "Bits 0:2 - Mux Configuration"]
    #[inline(always)]
    pub fn mux(&self) -> MuxR {
        MuxR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Waveform Selection"]
    #[inline(always)]
    pub fn wave(&self) -> WaveR {
        WaveR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:13 - Contrast Control"]
    #[inline(always)]
    pub fn contrast(&self) -> ContrastR {
        ContrastR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 20:22 - Charge Redistribution Cycles"]
    #[inline(always)]
    pub fn chgrdst(&self) -> ChgrdstR {
        ChgrdstR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:25 - Bias Configuration"]
    #[inline(always)]
    pub fn bias(&self) -> BiasR {
        BiasR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Mode Setting"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Mux Configuration"]
    #[inline(always)]
    pub fn mux(&mut self) -> MuxW<'_, DispctrlSpec> {
        MuxW::new(self, 0)
    }
    #[doc = "Bit 4 - Waveform Selection"]
    #[inline(always)]
    pub fn wave(&mut self) -> WaveW<'_, DispctrlSpec> {
        WaveW::new(self, 4)
    }
    #[doc = "Bits 8:13 - Contrast Control"]
    #[inline(always)]
    pub fn contrast(&mut self) -> ContrastW<'_, DispctrlSpec> {
        ContrastW::new(self, 8)
    }
    #[doc = "Bits 20:22 - Charge Redistribution Cycles"]
    #[inline(always)]
    pub fn chgrdst(&mut self) -> ChgrdstW<'_, DispctrlSpec> {
        ChgrdstW::new(self, 20)
    }
    #[doc = "Bits 24:25 - Bias Configuration"]
    #[inline(always)]
    pub fn bias(&mut self) -> BiasW<'_, DispctrlSpec> {
        BiasW::new(self, 24)
    }
    #[doc = "Bits 28:29 - Mode Setting"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, DispctrlSpec> {
        ModeW::new(self, 28)
    }
}
#[doc = "Display Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dispctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dispctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DispctrlSpec;
impl crate::RegisterSpec for DispctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dispctrl::R`](R) reader structure"]
impl crate::Readable for DispctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dispctrl::W`](W) writer structure"]
impl crate::Writable for DispctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DISPCTRL to value 0x0010_3f00"]
impl crate::Resettable for DispctrlSpec {
    const RESET_VALUE: u32 = 0x0010_3f00;
}
