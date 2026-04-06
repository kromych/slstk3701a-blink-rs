#[doc = "Register `BACTRL` reader"]
pub type R = crate::R<BactrlSpec>;
#[doc = "Register `BACTRL` writer"]
pub type W = crate::W<BactrlSpec>;
#[doc = "Field `BLINKEN` reader - Blink Enable"]
pub type BlinkenR = crate::BitReader;
#[doc = "Field `BLINKEN` writer - Blink Enable"]
pub type BlinkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLANK` reader - Blank Display"]
pub type BlankR = crate::BitReader;
#[doc = "Field `BLANK` writer - Blank Display"]
pub type BlankW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AEN` reader - Animation Enable"]
pub type AenR = crate::BitReader;
#[doc = "Field `AEN` writer - Animation Enable"]
pub type AenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Animate Register a Shift Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Aregasc {
    #[doc = "0: No Shift operation on Animation Register A"]
    Noshift = 0,
    #[doc = "1: Animation Register A is shifted left"]
    Shiftleft = 1,
    #[doc = "2: Animation Register A is shifted right"]
    Shiftright = 2,
}
impl From<Aregasc> for u8 {
    #[inline(always)]
    fn from(variant: Aregasc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Aregasc {
    type Ux = u8;
}
impl crate::IsEnum for Aregasc {}
#[doc = "Field `AREGASC` reader - Animate Register a Shift Control"]
pub type AregascR = crate::FieldReader<Aregasc>;
impl AregascR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Aregasc> {
        match self.bits {
            0 => Some(Aregasc::Noshift),
            1 => Some(Aregasc::Shiftleft),
            2 => Some(Aregasc::Shiftright),
            _ => None,
        }
    }
    #[doc = "No Shift operation on Animation Register A"]
    #[inline(always)]
    pub fn is_noshift(&self) -> bool {
        *self == Aregasc::Noshift
    }
    #[doc = "Animation Register A is shifted left"]
    #[inline(always)]
    pub fn is_shiftleft(&self) -> bool {
        *self == Aregasc::Shiftleft
    }
    #[doc = "Animation Register A is shifted right"]
    #[inline(always)]
    pub fn is_shiftright(&self) -> bool {
        *self == Aregasc::Shiftright
    }
}
#[doc = "Field `AREGASC` writer - Animate Register a Shift Control"]
pub type AregascW<'a, REG> = crate::FieldWriter<'a, REG, 2, Aregasc>;
impl<'a, REG> AregascW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Shift operation on Animation Register A"]
    #[inline(always)]
    pub fn noshift(self) -> &'a mut crate::W<REG> {
        self.variant(Aregasc::Noshift)
    }
    #[doc = "Animation Register A is shifted left"]
    #[inline(always)]
    pub fn shiftleft(self) -> &'a mut crate::W<REG> {
        self.variant(Aregasc::Shiftleft)
    }
    #[doc = "Animation Register A is shifted right"]
    #[inline(always)]
    pub fn shiftright(self) -> &'a mut crate::W<REG> {
        self.variant(Aregasc::Shiftright)
    }
}
#[doc = "Animate Register B Shift Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Aregbsc {
    #[doc = "0: No Shift operation on Animation Register B"]
    Noshift = 0,
    #[doc = "1: Animation Register B is shifted left"]
    Shiftleft = 1,
    #[doc = "2: Animation Register B is shifted right"]
    Shiftright = 2,
}
impl From<Aregbsc> for u8 {
    #[inline(always)]
    fn from(variant: Aregbsc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Aregbsc {
    type Ux = u8;
}
impl crate::IsEnum for Aregbsc {}
#[doc = "Field `AREGBSC` reader - Animate Register B Shift Control"]
pub type AregbscR = crate::FieldReader<Aregbsc>;
impl AregbscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Aregbsc> {
        match self.bits {
            0 => Some(Aregbsc::Noshift),
            1 => Some(Aregbsc::Shiftleft),
            2 => Some(Aregbsc::Shiftright),
            _ => None,
        }
    }
    #[doc = "No Shift operation on Animation Register B"]
    #[inline(always)]
    pub fn is_noshift(&self) -> bool {
        *self == Aregbsc::Noshift
    }
    #[doc = "Animation Register B is shifted left"]
    #[inline(always)]
    pub fn is_shiftleft(&self) -> bool {
        *self == Aregbsc::Shiftleft
    }
    #[doc = "Animation Register B is shifted right"]
    #[inline(always)]
    pub fn is_shiftright(&self) -> bool {
        *self == Aregbsc::Shiftright
    }
}
#[doc = "Field `AREGBSC` writer - Animate Register B Shift Control"]
pub type AregbscW<'a, REG> = crate::FieldWriter<'a, REG, 2, Aregbsc>;
impl<'a, REG> AregbscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Shift operation on Animation Register B"]
    #[inline(always)]
    pub fn noshift(self) -> &'a mut crate::W<REG> {
        self.variant(Aregbsc::Noshift)
    }
    #[doc = "Animation Register B is shifted left"]
    #[inline(always)]
    pub fn shiftleft(self) -> &'a mut crate::W<REG> {
        self.variant(Aregbsc::Shiftleft)
    }
    #[doc = "Animation Register B is shifted right"]
    #[inline(always)]
    pub fn shiftright(self) -> &'a mut crate::W<REG> {
        self.variant(Aregbsc::Shiftright)
    }
}
#[doc = "Field `ALOGSEL` reader - Animate Logic Function Select"]
pub type AlogselR = crate::BitReader;
#[doc = "Field `ALOGSEL` writer - Animate Logic Function Select"]
pub type AlogselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCEN` reader - Frame Counter Enable"]
pub type FcenR = crate::BitReader;
#[doc = "Field `FCEN` writer - Frame Counter Enable"]
pub type FcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Frame Counter Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fcpresc {
    #[doc = "0: CLKFC = CLKFRAME / 1"]
    Div1 = 0,
    #[doc = "1: CLKFC = CLKFRAME / 2"]
    Div2 = 1,
    #[doc = "2: CLKFC = CLKFRAME / 4"]
    Div4 = 2,
    #[doc = "3: CLKFC = CLKFRAME / 8"]
    Div8 = 3,
}
impl From<Fcpresc> for u8 {
    #[inline(always)]
    fn from(variant: Fcpresc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fcpresc {
    type Ux = u8;
}
impl crate::IsEnum for Fcpresc {}
#[doc = "Field `FCPRESC` reader - Frame Counter Prescaler"]
pub type FcprescR = crate::FieldReader<Fcpresc>;
impl FcprescR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fcpresc {
        match self.bits {
            0 => Fcpresc::Div1,
            1 => Fcpresc::Div2,
            2 => Fcpresc::Div4,
            3 => Fcpresc::Div8,
            _ => unreachable!(),
        }
    }
    #[doc = "CLKFC = CLKFRAME / 1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Fcpresc::Div1
    }
    #[doc = "CLKFC = CLKFRAME / 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Fcpresc::Div2
    }
    #[doc = "CLKFC = CLKFRAME / 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Fcpresc::Div4
    }
    #[doc = "CLKFC = CLKFRAME / 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Fcpresc::Div8
    }
}
#[doc = "Field `FCPRESC` writer - Frame Counter Prescaler"]
pub type FcprescW<'a, REG> = crate::FieldWriter<'a, REG, 2, Fcpresc, crate::Safe>;
impl<'a, REG> FcprescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CLKFC = CLKFRAME / 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Fcpresc::Div1)
    }
    #[doc = "CLKFC = CLKFRAME / 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Fcpresc::Div2)
    }
    #[doc = "CLKFC = CLKFRAME / 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Fcpresc::Div4)
    }
    #[doc = "CLKFC = CLKFRAME / 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Fcpresc::Div8)
    }
}
#[doc = "Field `FCTOP` reader - Frame Counter Top Value"]
pub type FctopR = crate::FieldReader;
#[doc = "Field `FCTOP` writer - Frame Counter Top Value"]
pub type FctopW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ALOC` reader - Animation Location"]
pub type AlocR = crate::BitReader;
#[doc = "Field `ALOC` writer - Animation Location"]
pub type AlocW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Blink Enable"]
    #[inline(always)]
    pub fn blinken(&self) -> BlinkenR {
        BlinkenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Blank Display"]
    #[inline(always)]
    pub fn blank(&self) -> BlankR {
        BlankR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Animation Enable"]
    #[inline(always)]
    pub fn aen(&self) -> AenR {
        AenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Animate Register a Shift Control"]
    #[inline(always)]
    pub fn aregasc(&self) -> AregascR {
        AregascR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Animate Register B Shift Control"]
    #[inline(always)]
    pub fn aregbsc(&self) -> AregbscR {
        AregbscR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Animate Logic Function Select"]
    #[inline(always)]
    pub fn alogsel(&self) -> AlogselR {
        AlogselR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Frame Counter Enable"]
    #[inline(always)]
    pub fn fcen(&self) -> FcenR {
        FcenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Frame Counter Prescaler"]
    #[inline(always)]
    pub fn fcpresc(&self) -> FcprescR {
        FcprescR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:23 - Frame Counter Top Value"]
    #[inline(always)]
    pub fn fctop(&self) -> FctopR {
        FctopR::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bit 28 - Animation Location"]
    #[inline(always)]
    pub fn aloc(&self) -> AlocR {
        AlocR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Blink Enable"]
    #[inline(always)]
    pub fn blinken(&mut self) -> BlinkenW<'_, BactrlSpec> {
        BlinkenW::new(self, 0)
    }
    #[doc = "Bit 1 - Blank Display"]
    #[inline(always)]
    pub fn blank(&mut self) -> BlankW<'_, BactrlSpec> {
        BlankW::new(self, 1)
    }
    #[doc = "Bit 2 - Animation Enable"]
    #[inline(always)]
    pub fn aen(&mut self) -> AenW<'_, BactrlSpec> {
        AenW::new(self, 2)
    }
    #[doc = "Bits 3:4 - Animate Register a Shift Control"]
    #[inline(always)]
    pub fn aregasc(&mut self) -> AregascW<'_, BactrlSpec> {
        AregascW::new(self, 3)
    }
    #[doc = "Bits 5:6 - Animate Register B Shift Control"]
    #[inline(always)]
    pub fn aregbsc(&mut self) -> AregbscW<'_, BactrlSpec> {
        AregbscW::new(self, 5)
    }
    #[doc = "Bit 7 - Animate Logic Function Select"]
    #[inline(always)]
    pub fn alogsel(&mut self) -> AlogselW<'_, BactrlSpec> {
        AlogselW::new(self, 7)
    }
    #[doc = "Bit 8 - Frame Counter Enable"]
    #[inline(always)]
    pub fn fcen(&mut self) -> FcenW<'_, BactrlSpec> {
        FcenW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Frame Counter Prescaler"]
    #[inline(always)]
    pub fn fcpresc(&mut self) -> FcprescW<'_, BactrlSpec> {
        FcprescW::new(self, 16)
    }
    #[doc = "Bits 18:23 - Frame Counter Top Value"]
    #[inline(always)]
    pub fn fctop(&mut self) -> FctopW<'_, BactrlSpec> {
        FctopW::new(self, 18)
    }
    #[doc = "Bit 28 - Animation Location"]
    #[inline(always)]
    pub fn aloc(&mut self) -> AlocW<'_, BactrlSpec> {
        AlocW::new(self, 28)
    }
}
#[doc = "Blink and Animation Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bactrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bactrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BactrlSpec;
impl crate::RegisterSpec for BactrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bactrl::R`](R) reader structure"]
impl crate::Readable for BactrlSpec {}
#[doc = "`write(|w| ..)` method takes [`bactrl::W`](W) writer structure"]
impl crate::Writable for BactrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BACTRL to value 0"]
impl crate::Resettable for BactrlSpec {}
