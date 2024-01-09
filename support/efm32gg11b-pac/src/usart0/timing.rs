#[doc = "Register `TIMING` reader"]
pub type R = crate::R<TIMING_SPEC>;
#[doc = "Register `TIMING` writer"]
pub type W = crate::W<TIMING_SPEC>;
#[doc = "Field `TXDELAY` reader - TX Frame Start Delay"]
pub type TXDELAY_R = crate::FieldReader<TXDELAY_A>;
#[doc = "TX Frame Start Delay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXDELAY_A {
    #[doc = "0: Disable - TXDELAY in USARTn_CTRL can be used for legacy"]
    DISABLE = 0,
    #[doc = "1: Start of transmission is delayed for 1 baud-times"]
    ONE = 1,
    #[doc = "2: Start of transmission is delayed for 2 baud-times"]
    TWO = 2,
    #[doc = "3: Start of transmission is delayed for 3 baud-times"]
    THREE = 3,
    #[doc = "4: Start of transmission is delayed for 7 baud-times"]
    SEVEN = 4,
    #[doc = "5: Start of transmission is delayed for TCMPVAL0 baud-times"]
    TCMP0 = 5,
    #[doc = "6: Start of transmission is delayed for TCMPVAL1 baud-times"]
    TCMP1 = 6,
    #[doc = "7: Start of transmission is delayed for TCMPVAL2 baud-times"]
    TCMP2 = 7,
}
impl From<TXDELAY_A> for u8 {
    #[inline(always)]
    fn from(variant: TXDELAY_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TXDELAY_A {
    type Ux = u8;
}
impl TXDELAY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXDELAY_A {
        match self.bits {
            0 => TXDELAY_A::DISABLE,
            1 => TXDELAY_A::ONE,
            2 => TXDELAY_A::TWO,
            3 => TXDELAY_A::THREE,
            4 => TXDELAY_A::SEVEN,
            5 => TXDELAY_A::TCMP0,
            6 => TXDELAY_A::TCMP1,
            7 => TXDELAY_A::TCMP2,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable - TXDELAY in USARTn_CTRL can be used for legacy"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TXDELAY_A::DISABLE
    }
    #[doc = "Start of transmission is delayed for 1 baud-times"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == TXDELAY_A::ONE
    }
    #[doc = "Start of transmission is delayed for 2 baud-times"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == TXDELAY_A::TWO
    }
    #[doc = "Start of transmission is delayed for 3 baud-times"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == TXDELAY_A::THREE
    }
    #[doc = "Start of transmission is delayed for 7 baud-times"]
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == TXDELAY_A::SEVEN
    }
    #[doc = "Start of transmission is delayed for TCMPVAL0 baud-times"]
    #[inline(always)]
    pub fn is_tcmp0(&self) -> bool {
        *self == TXDELAY_A::TCMP0
    }
    #[doc = "Start of transmission is delayed for TCMPVAL1 baud-times"]
    #[inline(always)]
    pub fn is_tcmp1(&self) -> bool {
        *self == TXDELAY_A::TCMP1
    }
    #[doc = "Start of transmission is delayed for TCMPVAL2 baud-times"]
    #[inline(always)]
    pub fn is_tcmp2(&self) -> bool {
        *self == TXDELAY_A::TCMP2
    }
}
#[doc = "Field `TXDELAY` writer - TX Frame Start Delay"]
pub type TXDELAY_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, TXDELAY_A>;
impl<'a, REG> TXDELAY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable - TXDELAY in USARTn_CTRL can be used for legacy"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TXDELAY_A::DISABLE)
    }
    #[doc = "Start of transmission is delayed for 1 baud-times"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(TXDELAY_A::ONE)
    }
    #[doc = "Start of transmission is delayed for 2 baud-times"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(TXDELAY_A::TWO)
    }
    #[doc = "Start of transmission is delayed for 3 baud-times"]
    #[inline(always)]
    pub fn three(self) -> &'a mut crate::W<REG> {
        self.variant(TXDELAY_A::THREE)
    }
    #[doc = "Start of transmission is delayed for 7 baud-times"]
    #[inline(always)]
    pub fn seven(self) -> &'a mut crate::W<REG> {
        self.variant(TXDELAY_A::SEVEN)
    }
    #[doc = "Start of transmission is delayed for TCMPVAL0 baud-times"]
    #[inline(always)]
    pub fn tcmp0(self) -> &'a mut crate::W<REG> {
        self.variant(TXDELAY_A::TCMP0)
    }
    #[doc = "Start of transmission is delayed for TCMPVAL1 baud-times"]
    #[inline(always)]
    pub fn tcmp1(self) -> &'a mut crate::W<REG> {
        self.variant(TXDELAY_A::TCMP1)
    }
    #[doc = "Start of transmission is delayed for TCMPVAL2 baud-times"]
    #[inline(always)]
    pub fn tcmp2(self) -> &'a mut crate::W<REG> {
        self.variant(TXDELAY_A::TCMP2)
    }
}
#[doc = "Field `CSSETUP` reader - Chip Select Setup"]
pub type CSSETUP_R = crate::FieldReader<CSSETUP_A>;
#[doc = "Chip Select Setup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSSETUP_A {
    #[doc = "0: CS is not asserted before start of transmission"]
    ZERO = 0,
    #[doc = "1: CS is asserted for 1 baud-times before start of transmission"]
    ONE = 1,
    #[doc = "2: CS is asserted for 2 baud-times before start of transmission"]
    TWO = 2,
    #[doc = "3: CS is asserted for 3 baud-times before start of transmission"]
    THREE = 3,
    #[doc = "4: CS is asserted for 7 baud-times before start of transmission"]
    SEVEN = 4,
    #[doc = "5: CS is asserted before the start of transmission for TCMPVAL0 baud-times"]
    TCMP0 = 5,
    #[doc = "6: CS is asserted before the start of transmission for TCMPVAL1 baud-times"]
    TCMP1 = 6,
    #[doc = "7: CS is asserted before the start of transmission for TCMPVAL2 baud-times"]
    TCMP2 = 7,
}
impl From<CSSETUP_A> for u8 {
    #[inline(always)]
    fn from(variant: CSSETUP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CSSETUP_A {
    type Ux = u8;
}
impl CSSETUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSSETUP_A {
        match self.bits {
            0 => CSSETUP_A::ZERO,
            1 => CSSETUP_A::ONE,
            2 => CSSETUP_A::TWO,
            3 => CSSETUP_A::THREE,
            4 => CSSETUP_A::SEVEN,
            5 => CSSETUP_A::TCMP0,
            6 => CSSETUP_A::TCMP1,
            7 => CSSETUP_A::TCMP2,
            _ => unreachable!(),
        }
    }
    #[doc = "CS is not asserted before start of transmission"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == CSSETUP_A::ZERO
    }
    #[doc = "CS is asserted for 1 baud-times before start of transmission"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == CSSETUP_A::ONE
    }
    #[doc = "CS is asserted for 2 baud-times before start of transmission"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == CSSETUP_A::TWO
    }
    #[doc = "CS is asserted for 3 baud-times before start of transmission"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == CSSETUP_A::THREE
    }
    #[doc = "CS is asserted for 7 baud-times before start of transmission"]
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == CSSETUP_A::SEVEN
    }
    #[doc = "CS is asserted before the start of transmission for TCMPVAL0 baud-times"]
    #[inline(always)]
    pub fn is_tcmp0(&self) -> bool {
        *self == CSSETUP_A::TCMP0
    }
    #[doc = "CS is asserted before the start of transmission for TCMPVAL1 baud-times"]
    #[inline(always)]
    pub fn is_tcmp1(&self) -> bool {
        *self == CSSETUP_A::TCMP1
    }
    #[doc = "CS is asserted before the start of transmission for TCMPVAL2 baud-times"]
    #[inline(always)]
    pub fn is_tcmp2(&self) -> bool {
        *self == CSSETUP_A::TCMP2
    }
}
#[doc = "Field `CSSETUP` writer - Chip Select Setup"]
pub type CSSETUP_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, CSSETUP_A>;
impl<'a, REG> CSSETUP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CS is not asserted before start of transmission"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(CSSETUP_A::ZERO)
    }
    #[doc = "CS is asserted for 1 baud-times before start of transmission"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(CSSETUP_A::ONE)
    }
    #[doc = "CS is asserted for 2 baud-times before start of transmission"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(CSSETUP_A::TWO)
    }
    #[doc = "CS is asserted for 3 baud-times before start of transmission"]
    #[inline(always)]
    pub fn three(self) -> &'a mut crate::W<REG> {
        self.variant(CSSETUP_A::THREE)
    }
    #[doc = "CS is asserted for 7 baud-times before start of transmission"]
    #[inline(always)]
    pub fn seven(self) -> &'a mut crate::W<REG> {
        self.variant(CSSETUP_A::SEVEN)
    }
    #[doc = "CS is asserted before the start of transmission for TCMPVAL0 baud-times"]
    #[inline(always)]
    pub fn tcmp0(self) -> &'a mut crate::W<REG> {
        self.variant(CSSETUP_A::TCMP0)
    }
    #[doc = "CS is asserted before the start of transmission for TCMPVAL1 baud-times"]
    #[inline(always)]
    pub fn tcmp1(self) -> &'a mut crate::W<REG> {
        self.variant(CSSETUP_A::TCMP1)
    }
    #[doc = "CS is asserted before the start of transmission for TCMPVAL2 baud-times"]
    #[inline(always)]
    pub fn tcmp2(self) -> &'a mut crate::W<REG> {
        self.variant(CSSETUP_A::TCMP2)
    }
}
#[doc = "Field `ICS` reader - Inter-character Spacing"]
pub type ICS_R = crate::FieldReader<ICS_A>;
#[doc = "Inter-character Spacing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICS_A {
    #[doc = "0: There is no space between charcters"]
    ZERO = 0,
    #[doc = "1: Create a space of 1 baud-times before start of transmission"]
    ONE = 1,
    #[doc = "2: Create a space of 2 baud-times before start of transmission"]
    TWO = 2,
    #[doc = "3: Create a space of 3 baud-times before start of transmission"]
    THREE = 3,
    #[doc = "4: Create a space of 7 baud-times before start of transmission"]
    SEVEN = 4,
    #[doc = "5: Create a space of before the start of transmission for TCMPVAL0 baud-times"]
    TCMP0 = 5,
    #[doc = "6: Create a space of before the start of transmission for TCMPVAL1 baud-times"]
    TCMP1 = 6,
    #[doc = "7: Create a space of before the start of transmission for TCMPVAL2 baud-times"]
    TCMP2 = 7,
}
impl From<ICS_A> for u8 {
    #[inline(always)]
    fn from(variant: ICS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ICS_A {
    type Ux = u8;
}
impl ICS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ICS_A {
        match self.bits {
            0 => ICS_A::ZERO,
            1 => ICS_A::ONE,
            2 => ICS_A::TWO,
            3 => ICS_A::THREE,
            4 => ICS_A::SEVEN,
            5 => ICS_A::TCMP0,
            6 => ICS_A::TCMP1,
            7 => ICS_A::TCMP2,
            _ => unreachable!(),
        }
    }
    #[doc = "There is no space between charcters"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == ICS_A::ZERO
    }
    #[doc = "Create a space of 1 baud-times before start of transmission"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == ICS_A::ONE
    }
    #[doc = "Create a space of 2 baud-times before start of transmission"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == ICS_A::TWO
    }
    #[doc = "Create a space of 3 baud-times before start of transmission"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == ICS_A::THREE
    }
    #[doc = "Create a space of 7 baud-times before start of transmission"]
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == ICS_A::SEVEN
    }
    #[doc = "Create a space of before the start of transmission for TCMPVAL0 baud-times"]
    #[inline(always)]
    pub fn is_tcmp0(&self) -> bool {
        *self == ICS_A::TCMP0
    }
    #[doc = "Create a space of before the start of transmission for TCMPVAL1 baud-times"]
    #[inline(always)]
    pub fn is_tcmp1(&self) -> bool {
        *self == ICS_A::TCMP1
    }
    #[doc = "Create a space of before the start of transmission for TCMPVAL2 baud-times"]
    #[inline(always)]
    pub fn is_tcmp2(&self) -> bool {
        *self == ICS_A::TCMP2
    }
}
#[doc = "Field `ICS` writer - Inter-character Spacing"]
pub type ICS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, ICS_A>;
impl<'a, REG> ICS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "There is no space between charcters"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(ICS_A::ZERO)
    }
    #[doc = "Create a space of 1 baud-times before start of transmission"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(ICS_A::ONE)
    }
    #[doc = "Create a space of 2 baud-times before start of transmission"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(ICS_A::TWO)
    }
    #[doc = "Create a space of 3 baud-times before start of transmission"]
    #[inline(always)]
    pub fn three(self) -> &'a mut crate::W<REG> {
        self.variant(ICS_A::THREE)
    }
    #[doc = "Create a space of 7 baud-times before start of transmission"]
    #[inline(always)]
    pub fn seven(self) -> &'a mut crate::W<REG> {
        self.variant(ICS_A::SEVEN)
    }
    #[doc = "Create a space of before the start of transmission for TCMPVAL0 baud-times"]
    #[inline(always)]
    pub fn tcmp0(self) -> &'a mut crate::W<REG> {
        self.variant(ICS_A::TCMP0)
    }
    #[doc = "Create a space of before the start of transmission for TCMPVAL1 baud-times"]
    #[inline(always)]
    pub fn tcmp1(self) -> &'a mut crate::W<REG> {
        self.variant(ICS_A::TCMP1)
    }
    #[doc = "Create a space of before the start of transmission for TCMPVAL2 baud-times"]
    #[inline(always)]
    pub fn tcmp2(self) -> &'a mut crate::W<REG> {
        self.variant(ICS_A::TCMP2)
    }
}
#[doc = "Field `CSHOLD` reader - Chip Select Hold"]
pub type CSHOLD_R = crate::FieldReader<CSHOLD_A>;
#[doc = "Chip Select Hold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSHOLD_A {
    #[doc = "0: Disable CS being asserted after the end of transmission"]
    ZERO = 0,
    #[doc = "1: CS is asserted for 1 baud-times after the end of transmission"]
    ONE = 1,
    #[doc = "2: CS is asserted for 2 baud-times after the end of transmission"]
    TWO = 2,
    #[doc = "3: CS is asserted for 3 baud-times after the end of transmission"]
    THREE = 3,
    #[doc = "4: CS is asserted for 7 baud-times after the end of transmission"]
    SEVEN = 4,
    #[doc = "5: CS is asserted after the end of transmission for TCMPVAL0 baud-times"]
    TCMP0 = 5,
    #[doc = "6: CS is asserted after the end of transmission for TCMPVAL1 baud-times"]
    TCMP1 = 6,
    #[doc = "7: CS is asserted after the end of transmission for TCMPVAL2 baud-times"]
    TCMP2 = 7,
}
impl From<CSHOLD_A> for u8 {
    #[inline(always)]
    fn from(variant: CSHOLD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CSHOLD_A {
    type Ux = u8;
}
impl CSHOLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSHOLD_A {
        match self.bits {
            0 => CSHOLD_A::ZERO,
            1 => CSHOLD_A::ONE,
            2 => CSHOLD_A::TWO,
            3 => CSHOLD_A::THREE,
            4 => CSHOLD_A::SEVEN,
            5 => CSHOLD_A::TCMP0,
            6 => CSHOLD_A::TCMP1,
            7 => CSHOLD_A::TCMP2,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable CS being asserted after the end of transmission"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == CSHOLD_A::ZERO
    }
    #[doc = "CS is asserted for 1 baud-times after the end of transmission"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == CSHOLD_A::ONE
    }
    #[doc = "CS is asserted for 2 baud-times after the end of transmission"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == CSHOLD_A::TWO
    }
    #[doc = "CS is asserted for 3 baud-times after the end of transmission"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == CSHOLD_A::THREE
    }
    #[doc = "CS is asserted for 7 baud-times after the end of transmission"]
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == CSHOLD_A::SEVEN
    }
    #[doc = "CS is asserted after the end of transmission for TCMPVAL0 baud-times"]
    #[inline(always)]
    pub fn is_tcmp0(&self) -> bool {
        *self == CSHOLD_A::TCMP0
    }
    #[doc = "CS is asserted after the end of transmission for TCMPVAL1 baud-times"]
    #[inline(always)]
    pub fn is_tcmp1(&self) -> bool {
        *self == CSHOLD_A::TCMP1
    }
    #[doc = "CS is asserted after the end of transmission for TCMPVAL2 baud-times"]
    #[inline(always)]
    pub fn is_tcmp2(&self) -> bool {
        *self == CSHOLD_A::TCMP2
    }
}
#[doc = "Field `CSHOLD` writer - Chip Select Hold"]
pub type CSHOLD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, CSHOLD_A>;
impl<'a, REG> CSHOLD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable CS being asserted after the end of transmission"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(CSHOLD_A::ZERO)
    }
    #[doc = "CS is asserted for 1 baud-times after the end of transmission"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(CSHOLD_A::ONE)
    }
    #[doc = "CS is asserted for 2 baud-times after the end of transmission"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(CSHOLD_A::TWO)
    }
    #[doc = "CS is asserted for 3 baud-times after the end of transmission"]
    #[inline(always)]
    pub fn three(self) -> &'a mut crate::W<REG> {
        self.variant(CSHOLD_A::THREE)
    }
    #[doc = "CS is asserted for 7 baud-times after the end of transmission"]
    #[inline(always)]
    pub fn seven(self) -> &'a mut crate::W<REG> {
        self.variant(CSHOLD_A::SEVEN)
    }
    #[doc = "CS is asserted after the end of transmission for TCMPVAL0 baud-times"]
    #[inline(always)]
    pub fn tcmp0(self) -> &'a mut crate::W<REG> {
        self.variant(CSHOLD_A::TCMP0)
    }
    #[doc = "CS is asserted after the end of transmission for TCMPVAL1 baud-times"]
    #[inline(always)]
    pub fn tcmp1(self) -> &'a mut crate::W<REG> {
        self.variant(CSHOLD_A::TCMP1)
    }
    #[doc = "CS is asserted after the end of transmission for TCMPVAL2 baud-times"]
    #[inline(always)]
    pub fn tcmp2(self) -> &'a mut crate::W<REG> {
        self.variant(CSHOLD_A::TCMP2)
    }
}
impl R {
    #[doc = "Bits 16:18 - TX Frame Start Delay"]
    #[inline(always)]
    pub fn txdelay(&self) -> TXDELAY_R {
        TXDELAY_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Chip Select Setup"]
    #[inline(always)]
    pub fn cssetup(&self) -> CSSETUP_R {
        CSSETUP_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Inter-character Spacing"]
    #[inline(always)]
    pub fn ics(&self) -> ICS_R {
        ICS_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Chip Select Hold"]
    #[inline(always)]
    pub fn cshold(&self) -> CSHOLD_R {
        CSHOLD_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 16:18 - TX Frame Start Delay"]
    #[inline(always)]
    #[must_use]
    pub fn txdelay(&mut self) -> TXDELAY_W<TIMING_SPEC> {
        TXDELAY_W::new(self, 16)
    }
    #[doc = "Bits 20:22 - Chip Select Setup"]
    #[inline(always)]
    #[must_use]
    pub fn cssetup(&mut self) -> CSSETUP_W<TIMING_SPEC> {
        CSSETUP_W::new(self, 20)
    }
    #[doc = "Bits 24:26 - Inter-character Spacing"]
    #[inline(always)]
    #[must_use]
    pub fn ics(&mut self) -> ICS_W<TIMING_SPEC> {
        ICS_W::new(self, 24)
    }
    #[doc = "Bits 28:30 - Chip Select Hold"]
    #[inline(always)]
    #[must_use]
    pub fn cshold(&mut self) -> CSHOLD_W<TIMING_SPEC> {
        CSHOLD_W::new(self, 28)
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
#[doc = "Timing Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timing::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timing::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMING_SPEC;
impl crate::RegisterSpec for TIMING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timing::R`](R) reader structure"]
impl crate::Readable for TIMING_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timing::W`](W) writer structure"]
impl crate::Writable for TIMING_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMING to value 0"]
impl crate::Resettable for TIMING_SPEC {
    const RESET_VALUE: u32 = 0;
}
