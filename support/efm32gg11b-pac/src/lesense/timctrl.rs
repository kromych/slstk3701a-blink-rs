#[doc = "Register `TIMCTRL` reader"]
pub type R = crate::R<TIMCTRL_SPEC>;
#[doc = "Register `TIMCTRL` writer"]
pub type W = crate::W<TIMCTRL_SPEC>;
#[doc = "Field `AUXPRESC` reader - Prescaling Factor for High Frequency Timer"]
pub type AUXPRESC_R = crate::FieldReader<AUXPRESC_A>;
#[doc = "Prescaling Factor for High Frequency Timer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AUXPRESC_A {
    #[doc = "0: High frequency timer is clocked with AUXHFRCO/1"]
    DIV1 = 0,
    #[doc = "1: High frequency timer is clocked with AUXHFRCO/2"]
    DIV2 = 1,
    #[doc = "2: High frequency timer is clocked with AUXHFRCO/4"]
    DIV4 = 2,
    #[doc = "3: High frequency timer is clocked with AUXHFRCO/8"]
    DIV8 = 3,
}
impl From<AUXPRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: AUXPRESC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AUXPRESC_A {
    type Ux = u8;
}
impl AUXPRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AUXPRESC_A {
        match self.bits {
            0 => AUXPRESC_A::DIV1,
            1 => AUXPRESC_A::DIV2,
            2 => AUXPRESC_A::DIV4,
            3 => AUXPRESC_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "High frequency timer is clocked with AUXHFRCO/1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == AUXPRESC_A::DIV1
    }
    #[doc = "High frequency timer is clocked with AUXHFRCO/2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == AUXPRESC_A::DIV2
    }
    #[doc = "High frequency timer is clocked with AUXHFRCO/4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == AUXPRESC_A::DIV4
    }
    #[doc = "High frequency timer is clocked with AUXHFRCO/8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == AUXPRESC_A::DIV8
    }
}
#[doc = "Field `AUXPRESC` writer - Prescaling Factor for High Frequency Timer"]
pub type AUXPRESC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, AUXPRESC_A>;
impl<'a, REG> AUXPRESC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "High frequency timer is clocked with AUXHFRCO/1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(AUXPRESC_A::DIV1)
    }
    #[doc = "High frequency timer is clocked with AUXHFRCO/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(AUXPRESC_A::DIV2)
    }
    #[doc = "High frequency timer is clocked with AUXHFRCO/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(AUXPRESC_A::DIV4)
    }
    #[doc = "High frequency timer is clocked with AUXHFRCO/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(AUXPRESC_A::DIV8)
    }
}
#[doc = "Field `LFPRESC` reader - Prescaling Factor for Low Frequency Timer"]
pub type LFPRESC_R = crate::FieldReader<LFPRESC_A>;
#[doc = "Prescaling Factor for Low Frequency Timer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LFPRESC_A {
    #[doc = "0: Low frequency timer is clocked with LFACLKLESENSE/1"]
    DIV1 = 0,
    #[doc = "1: Low frequency timer is clocked with LFACLKLESENSE/2"]
    DIV2 = 1,
    #[doc = "2: Low frequency timer is clocked with LFACLKLESENSE/4"]
    DIV4 = 2,
    #[doc = "3: Low frequency timer is clocked with LFACLKLESENSE/8"]
    DIV8 = 3,
    #[doc = "4: Low frequency timer is clocked with LFACLKLESENSE/16"]
    DIV16 = 4,
    #[doc = "5: Low frequency timer is clocked with LFACLKLESENSE/32"]
    DIV32 = 5,
    #[doc = "6: Low frequency timer is clocked with LFACLKLESENSE/64"]
    DIV64 = 6,
    #[doc = "7: Low frequency timer is clocked with LFACLKLESENSE/128"]
    DIV128 = 7,
}
impl From<LFPRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: LFPRESC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LFPRESC_A {
    type Ux = u8;
}
impl LFPRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LFPRESC_A {
        match self.bits {
            0 => LFPRESC_A::DIV1,
            1 => LFPRESC_A::DIV2,
            2 => LFPRESC_A::DIV4,
            3 => LFPRESC_A::DIV8,
            4 => LFPRESC_A::DIV16,
            5 => LFPRESC_A::DIV32,
            6 => LFPRESC_A::DIV64,
            7 => LFPRESC_A::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == LFPRESC_A::DIV1
    }
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == LFPRESC_A::DIV2
    }
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == LFPRESC_A::DIV4
    }
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == LFPRESC_A::DIV8
    }
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == LFPRESC_A::DIV16
    }
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == LFPRESC_A::DIV32
    }
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == LFPRESC_A::DIV64
    }
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == LFPRESC_A::DIV128
    }
}
#[doc = "Field `LFPRESC` writer - Prescaling Factor for Low Frequency Timer"]
pub type LFPRESC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, LFPRESC_A>;
impl<'a, REG> LFPRESC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(LFPRESC_A::DIV1)
    }
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(LFPRESC_A::DIV2)
    }
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(LFPRESC_A::DIV4)
    }
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(LFPRESC_A::DIV8)
    }
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(LFPRESC_A::DIV16)
    }
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(LFPRESC_A::DIV32)
    }
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(LFPRESC_A::DIV64)
    }
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(LFPRESC_A::DIV128)
    }
}
#[doc = "Field `PCPRESC` reader - Period Counter Prescaling"]
pub type PCPRESC_R = crate::FieldReader<PCPRESC_A>;
#[doc = "Period Counter Prescaling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCPRESC_A {
    #[doc = "0: The period counter clock frequency is LFACLKLESENSE/1"]
    DIV1 = 0,
    #[doc = "1: The period counter clock frequency is LFACLKLESENSE/2"]
    DIV2 = 1,
    #[doc = "2: The period counter clock frequency is LFACLKLESENSE/4"]
    DIV4 = 2,
    #[doc = "3: The period counter clock frequency is LFACLKLESENSE/8"]
    DIV8 = 3,
    #[doc = "4: The period counter clock frequency is LFACLKLESENSE/16"]
    DIV16 = 4,
    #[doc = "5: The period counter clock frequency is LFACLKLESENSE/32"]
    DIV32 = 5,
    #[doc = "6: The period counter clock frequency is LFACLKLESENSE/64"]
    DIV64 = 6,
    #[doc = "7: The period counter clock frequency is LFACLKLESENSE/128"]
    DIV128 = 7,
}
impl From<PCPRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: PCPRESC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCPRESC_A {
    type Ux = u8;
}
impl PCPRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCPRESC_A {
        match self.bits {
            0 => PCPRESC_A::DIV1,
            1 => PCPRESC_A::DIV2,
            2 => PCPRESC_A::DIV4,
            3 => PCPRESC_A::DIV8,
            4 => PCPRESC_A::DIV16,
            5 => PCPRESC_A::DIV32,
            6 => PCPRESC_A::DIV64,
            7 => PCPRESC_A::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "The period counter clock frequency is LFACLKLESENSE/1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PCPRESC_A::DIV1
    }
    #[doc = "The period counter clock frequency is LFACLKLESENSE/2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PCPRESC_A::DIV2
    }
    #[doc = "The period counter clock frequency is LFACLKLESENSE/4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PCPRESC_A::DIV4
    }
    #[doc = "The period counter clock frequency is LFACLKLESENSE/8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PCPRESC_A::DIV8
    }
    #[doc = "The period counter clock frequency is LFACLKLESENSE/16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PCPRESC_A::DIV16
    }
    #[doc = "The period counter clock frequency is LFACLKLESENSE/32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PCPRESC_A::DIV32
    }
    #[doc = "The period counter clock frequency is LFACLKLESENSE/64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PCPRESC_A::DIV64
    }
    #[doc = "The period counter clock frequency is LFACLKLESENSE/128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PCPRESC_A::DIV128
    }
}
#[doc = "Field `PCPRESC` writer - Period Counter Prescaling"]
pub type PCPRESC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, PCPRESC_A>;
impl<'a, REG> PCPRESC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The period counter clock frequency is LFACLKLESENSE/1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PCPRESC_A::DIV1)
    }
    #[doc = "The period counter clock frequency is LFACLKLESENSE/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PCPRESC_A::DIV2)
    }
    #[doc = "The period counter clock frequency is LFACLKLESENSE/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PCPRESC_A::DIV4)
    }
    #[doc = "The period counter clock frequency is LFACLKLESENSE/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PCPRESC_A::DIV8)
    }
    #[doc = "The period counter clock frequency is LFACLKLESENSE/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PCPRESC_A::DIV16)
    }
    #[doc = "The period counter clock frequency is LFACLKLESENSE/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(PCPRESC_A::DIV32)
    }
    #[doc = "The period counter clock frequency is LFACLKLESENSE/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(PCPRESC_A::DIV64)
    }
    #[doc = "The period counter clock frequency is LFACLKLESENSE/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(PCPRESC_A::DIV128)
    }
}
#[doc = "Field `PCTOP` reader - Period Counter Top Value"]
pub type PCTOP_R = crate::FieldReader;
#[doc = "Field `PCTOP` writer - Period Counter Top Value"]
pub type PCTOP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `STARTDLY` reader - Start Delay Configuration"]
pub type STARTDLY_R = crate::FieldReader;
#[doc = "Field `STARTDLY` writer - Start Delay Configuration"]
pub type STARTDLY_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AUXSTARTUP` reader - AUXHFRCO Startup Configuration"]
pub type AUXSTARTUP_R = crate::BitReader;
#[doc = "Field `AUXSTARTUP` writer - AUXHFRCO Startup Configuration"]
pub type AUXSTARTUP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Prescaling Factor for High Frequency Timer"]
    #[inline(always)]
    pub fn auxpresc(&self) -> AUXPRESC_R {
        AUXPRESC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6 - Prescaling Factor for Low Frequency Timer"]
    #[inline(always)]
    pub fn lfpresc(&self) -> LFPRESC_R {
        LFPRESC_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Period Counter Prescaling"]
    #[inline(always)]
    pub fn pcpresc(&self) -> PCPRESC_R {
        PCPRESC_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:19 - Period Counter Top Value"]
    #[inline(always)]
    pub fn pctop(&self) -> PCTOP_R {
        PCTOP_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 22:23 - Start Delay Configuration"]
    #[inline(always)]
    pub fn startdly(&self) -> STARTDLY_R {
        STARTDLY_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 28 - AUXHFRCO Startup Configuration"]
    #[inline(always)]
    pub fn auxstartup(&self) -> AUXSTARTUP_R {
        AUXSTARTUP_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Prescaling Factor for High Frequency Timer"]
    #[inline(always)]
    #[must_use]
    pub fn auxpresc(&mut self) -> AUXPRESC_W<TIMCTRL_SPEC> {
        AUXPRESC_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Prescaling Factor for Low Frequency Timer"]
    #[inline(always)]
    #[must_use]
    pub fn lfpresc(&mut self) -> LFPRESC_W<TIMCTRL_SPEC> {
        LFPRESC_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - Period Counter Prescaling"]
    #[inline(always)]
    #[must_use]
    pub fn pcpresc(&mut self) -> PCPRESC_W<TIMCTRL_SPEC> {
        PCPRESC_W::new(self, 8)
    }
    #[doc = "Bits 12:19 - Period Counter Top Value"]
    #[inline(always)]
    #[must_use]
    pub fn pctop(&mut self) -> PCTOP_W<TIMCTRL_SPEC> {
        PCTOP_W::new(self, 12)
    }
    #[doc = "Bits 22:23 - Start Delay Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn startdly(&mut self) -> STARTDLY_W<TIMCTRL_SPEC> {
        STARTDLY_W::new(self, 22)
    }
    #[doc = "Bit 28 - AUXHFRCO Startup Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn auxstartup(&mut self) -> AUXSTARTUP_W<TIMCTRL_SPEC> {
        AUXSTARTUP_W::new(self, 28)
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
#[doc = "Timing Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMCTRL_SPEC;
impl crate::RegisterSpec for TIMCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timctrl::R`](R) reader structure"]
impl crate::Readable for TIMCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timctrl::W`](W) writer structure"]
impl crate::Writable for TIMCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMCTRL to value 0"]
impl crate::Resettable for TIMCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
