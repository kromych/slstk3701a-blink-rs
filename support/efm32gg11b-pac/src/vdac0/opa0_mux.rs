#[doc = "Register `OPA0_MUX` reader"]
pub type R = crate::R<Opa0MuxSpec>;
#[doc = "Register `OPA0_MUX` writer"]
pub type W = crate::W<Opa0MuxSpec>;
#[doc = "Field `POSSEL` reader - OPAx Non-inverting Input Mux"]
pub type PosselR = crate::FieldReader;
#[doc = "Field `POSSEL` writer - OPAx Non-inverting Input Mux"]
pub type PosselW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NEGSEL` reader - OPAx Inverting Input Mux"]
pub type NegselR = crate::FieldReader;
#[doc = "Field `NEGSEL` writer - OPAx Inverting Input Mux"]
pub type NegselW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "OPAx Resistor Ladder Input Mux\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Resinmux {
    #[doc = "0: Set for Unity Gain"]
    Disable = 0,
    #[doc = "1: Set for NEXTOUT(x-1) input"]
    Opanext = 1,
    #[doc = "2: NEG pad connected"]
    Negpad = 2,
    #[doc = "3: POS pad connected"]
    Pospad = 3,
    #[doc = "4: Neg pad of OPA0 connected. Direct input to support common reference."]
    Compad = 4,
    #[doc = "5: OPA0 and OPA1 Resmux connected to form fully differential instrumentation amplifier."]
    Center = 5,
    #[doc = "6: VSS connected"]
    Vss = 6,
}
impl From<Resinmux> for u8 {
    #[inline(always)]
    fn from(variant: Resinmux) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Resinmux {
    type Ux = u8;
}
impl crate::IsEnum for Resinmux {}
#[doc = "Field `RESINMUX` reader - OPAx Resistor Ladder Input Mux"]
pub type ResinmuxR = crate::FieldReader<Resinmux>;
impl ResinmuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Resinmux> {
        match self.bits {
            0 => Some(Resinmux::Disable),
            1 => Some(Resinmux::Opanext),
            2 => Some(Resinmux::Negpad),
            3 => Some(Resinmux::Pospad),
            4 => Some(Resinmux::Compad),
            5 => Some(Resinmux::Center),
            6 => Some(Resinmux::Vss),
            _ => None,
        }
    }
    #[doc = "Set for Unity Gain"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Resinmux::Disable
    }
    #[doc = "Set for NEXTOUT(x-1) input"]
    #[inline(always)]
    pub fn is_opanext(&self) -> bool {
        *self == Resinmux::Opanext
    }
    #[doc = "NEG pad connected"]
    #[inline(always)]
    pub fn is_negpad(&self) -> bool {
        *self == Resinmux::Negpad
    }
    #[doc = "POS pad connected"]
    #[inline(always)]
    pub fn is_pospad(&self) -> bool {
        *self == Resinmux::Pospad
    }
    #[doc = "Neg pad of OPA0 connected. Direct input to support common reference."]
    #[inline(always)]
    pub fn is_compad(&self) -> bool {
        *self == Resinmux::Compad
    }
    #[doc = "OPA0 and OPA1 Resmux connected to form fully differential instrumentation amplifier."]
    #[inline(always)]
    pub fn is_center(&self) -> bool {
        *self == Resinmux::Center
    }
    #[doc = "VSS connected"]
    #[inline(always)]
    pub fn is_vss(&self) -> bool {
        *self == Resinmux::Vss
    }
}
#[doc = "Field `RESINMUX` writer - OPAx Resistor Ladder Input Mux"]
pub type ResinmuxW<'a, REG> = crate::FieldWriter<'a, REG, 3, Resinmux>;
impl<'a, REG> ResinmuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Set for Unity Gain"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Resinmux::Disable)
    }
    #[doc = "Set for NEXTOUT(x-1) input"]
    #[inline(always)]
    pub fn opanext(self) -> &'a mut crate::W<REG> {
        self.variant(Resinmux::Opanext)
    }
    #[doc = "NEG pad connected"]
    #[inline(always)]
    pub fn negpad(self) -> &'a mut crate::W<REG> {
        self.variant(Resinmux::Negpad)
    }
    #[doc = "POS pad connected"]
    #[inline(always)]
    pub fn pospad(self) -> &'a mut crate::W<REG> {
        self.variant(Resinmux::Pospad)
    }
    #[doc = "Neg pad of OPA0 connected. Direct input to support common reference."]
    #[inline(always)]
    pub fn compad(self) -> &'a mut crate::W<REG> {
        self.variant(Resinmux::Compad)
    }
    #[doc = "OPA0 and OPA1 Resmux connected to form fully differential instrumentation amplifier."]
    #[inline(always)]
    pub fn center(self) -> &'a mut crate::W<REG> {
        self.variant(Resinmux::Center)
    }
    #[doc = "VSS connected"]
    #[inline(always)]
    pub fn vss(self) -> &'a mut crate::W<REG> {
        self.variant(Resinmux::Vss)
    }
}
#[doc = "Field `GAIN3X` reader - OPAx Dedicated 3x Gain Resistor Ladder"]
pub type Gain3xR = crate::BitReader;
#[doc = "Field `GAIN3X` writer - OPAx Dedicated 3x Gain Resistor Ladder"]
pub type Gain3xW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "OPAx Resistor Ladder Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ressel {
    #[doc = "0: Gain of 1/3"]
    Res0 = 0,
    #[doc = "1: Gain of 1"]
    Res1 = 1,
    #[doc = "2: Gain of 1 2/3"]
    Res2 = 2,
    #[doc = "3: Gain of 2 1/5"]
    Res3 = 3,
    #[doc = "4: Gain of 3"]
    Res4 = 4,
    #[doc = "5: Gain of 4 1/3"]
    Res5 = 5,
    #[doc = "6: Gain of 7"]
    Res6 = 6,
    #[doc = "7: Gain of 15"]
    Res7 = 7,
}
impl From<Ressel> for u8 {
    #[inline(always)]
    fn from(variant: Ressel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ressel {
    type Ux = u8;
}
impl crate::IsEnum for Ressel {}
#[doc = "Field `RESSEL` reader - OPAx Resistor Ladder Select"]
pub type ResselR = crate::FieldReader<Ressel>;
impl ResselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ressel {
        match self.bits {
            0 => Ressel::Res0,
            1 => Ressel::Res1,
            2 => Ressel::Res2,
            3 => Ressel::Res3,
            4 => Ressel::Res4,
            5 => Ressel::Res5,
            6 => Ressel::Res6,
            7 => Ressel::Res7,
            _ => unreachable!(),
        }
    }
    #[doc = "Gain of 1/3"]
    #[inline(always)]
    pub fn is_res0(&self) -> bool {
        *self == Ressel::Res0
    }
    #[doc = "Gain of 1"]
    #[inline(always)]
    pub fn is_res1(&self) -> bool {
        *self == Ressel::Res1
    }
    #[doc = "Gain of 1 2/3"]
    #[inline(always)]
    pub fn is_res2(&self) -> bool {
        *self == Ressel::Res2
    }
    #[doc = "Gain of 2 1/5"]
    #[inline(always)]
    pub fn is_res3(&self) -> bool {
        *self == Ressel::Res3
    }
    #[doc = "Gain of 3"]
    #[inline(always)]
    pub fn is_res4(&self) -> bool {
        *self == Ressel::Res4
    }
    #[doc = "Gain of 4 1/3"]
    #[inline(always)]
    pub fn is_res5(&self) -> bool {
        *self == Ressel::Res5
    }
    #[doc = "Gain of 7"]
    #[inline(always)]
    pub fn is_res6(&self) -> bool {
        *self == Ressel::Res6
    }
    #[doc = "Gain of 15"]
    #[inline(always)]
    pub fn is_res7(&self) -> bool {
        *self == Ressel::Res7
    }
}
#[doc = "Field `RESSEL` writer - OPAx Resistor Ladder Select"]
pub type ResselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ressel, crate::Safe>;
impl<'a, REG> ResselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Gain of 1/3"]
    #[inline(always)]
    pub fn res0(self) -> &'a mut crate::W<REG> {
        self.variant(Ressel::Res0)
    }
    #[doc = "Gain of 1"]
    #[inline(always)]
    pub fn res1(self) -> &'a mut crate::W<REG> {
        self.variant(Ressel::Res1)
    }
    #[doc = "Gain of 1 2/3"]
    #[inline(always)]
    pub fn res2(self) -> &'a mut crate::W<REG> {
        self.variant(Ressel::Res2)
    }
    #[doc = "Gain of 2 1/5"]
    #[inline(always)]
    pub fn res3(self) -> &'a mut crate::W<REG> {
        self.variant(Ressel::Res3)
    }
    #[doc = "Gain of 3"]
    #[inline(always)]
    pub fn res4(self) -> &'a mut crate::W<REG> {
        self.variant(Ressel::Res4)
    }
    #[doc = "Gain of 4 1/3"]
    #[inline(always)]
    pub fn res5(self) -> &'a mut crate::W<REG> {
        self.variant(Ressel::Res5)
    }
    #[doc = "Gain of 7"]
    #[inline(always)]
    pub fn res6(self) -> &'a mut crate::W<REG> {
        self.variant(Ressel::Res6)
    }
    #[doc = "Gain of 15"]
    #[inline(always)]
    pub fn res7(self) -> &'a mut crate::W<REG> {
        self.variant(Ressel::Res7)
    }
}
impl R {
    #[doc = "Bits 0:7 - OPAx Non-inverting Input Mux"]
    #[inline(always)]
    pub fn possel(&self) -> PosselR {
        PosselR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - OPAx Inverting Input Mux"]
    #[inline(always)]
    pub fn negsel(&self) -> NegselR {
        NegselR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - OPAx Resistor Ladder Input Mux"]
    #[inline(always)]
    pub fn resinmux(&self) -> ResinmuxR {
        ResinmuxR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - OPAx Dedicated 3x Gain Resistor Ladder"]
    #[inline(always)]
    pub fn gain3x(&self) -> Gain3xR {
        Gain3xR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:26 - OPAx Resistor Ladder Select"]
    #[inline(always)]
    pub fn ressel(&self) -> ResselR {
        ResselR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - OPAx Non-inverting Input Mux"]
    #[inline(always)]
    pub fn possel(&mut self) -> PosselW<'_, Opa0MuxSpec> {
        PosselW::new(self, 0)
    }
    #[doc = "Bits 8:15 - OPAx Inverting Input Mux"]
    #[inline(always)]
    pub fn negsel(&mut self) -> NegselW<'_, Opa0MuxSpec> {
        NegselW::new(self, 8)
    }
    #[doc = "Bits 16:18 - OPAx Resistor Ladder Input Mux"]
    #[inline(always)]
    pub fn resinmux(&mut self) -> ResinmuxW<'_, Opa0MuxSpec> {
        ResinmuxW::new(self, 16)
    }
    #[doc = "Bit 20 - OPAx Dedicated 3x Gain Resistor Ladder"]
    #[inline(always)]
    pub fn gain3x(&mut self) -> Gain3xW<'_, Opa0MuxSpec> {
        Gain3xW::new(self, 20)
    }
    #[doc = "Bits 24:26 - OPAx Resistor Ladder Select"]
    #[inline(always)]
    pub fn ressel(&mut self) -> ResselW<'_, Opa0MuxSpec> {
        ResselW::new(self, 24)
    }
}
#[doc = "Operational Amplifier Mux Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`opa0_mux::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opa0_mux::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Opa0MuxSpec;
impl crate::RegisterSpec for Opa0MuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opa0_mux::R`](R) reader structure"]
impl crate::Readable for Opa0MuxSpec {}
#[doc = "`write(|w| ..)` method takes [`opa0_mux::W`](W) writer structure"]
impl crate::Writable for Opa0MuxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OPA0_MUX to value 0x0016_f2f1"]
impl crate::Resettable for Opa0MuxSpec {
    const RESET_VALUE: u32 = 0x0016_f2f1;
}
