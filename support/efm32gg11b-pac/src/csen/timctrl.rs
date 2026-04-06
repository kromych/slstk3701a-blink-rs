#[doc = "Register `TIMCTRL` reader"]
pub type R = crate::R<TimctrlSpec>;
#[doc = "Register `TIMCTRL` writer"]
pub type W = crate::W<TimctrlSpec>;
#[doc = "Period Counter Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pcpresc {
    #[doc = "0: The period counter clock frequency is LFBCLKCSEN/1"]
    Div1 = 0,
    #[doc = "1: The period counter clock frequency is LFBCLKCSEN/2"]
    Div2 = 1,
    #[doc = "2: The period counter clock frequency is LFBCLKCSEN/4"]
    Div4 = 2,
    #[doc = "3: The period counter clock frequency is LFBCLKCSEN/8"]
    Div8 = 3,
    #[doc = "4: The period counter clock frequency is LFBCLKCSEN/16"]
    Div16 = 4,
    #[doc = "5: The period counter clock frequency is LFBCLKCSEN/32"]
    Div32 = 5,
    #[doc = "6: The period counter clock frequency is LFBCLKCSEN/64"]
    Div64 = 6,
    #[doc = "7: The period counter clock frequency is LFBCLKCSEN/128"]
    Div128 = 7,
}
impl From<Pcpresc> for u8 {
    #[inline(always)]
    fn from(variant: Pcpresc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pcpresc {
    type Ux = u8;
}
impl crate::IsEnum for Pcpresc {}
#[doc = "Field `PCPRESC` reader - Period Counter Prescaler"]
pub type PcprescR = crate::FieldReader<Pcpresc>;
impl PcprescR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pcpresc {
        match self.bits {
            0 => Pcpresc::Div1,
            1 => Pcpresc::Div2,
            2 => Pcpresc::Div4,
            3 => Pcpresc::Div8,
            4 => Pcpresc::Div16,
            5 => Pcpresc::Div32,
            6 => Pcpresc::Div64,
            7 => Pcpresc::Div128,
            _ => unreachable!(),
        }
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Pcpresc::Div1
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Pcpresc::Div2
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Pcpresc::Div4
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Pcpresc::Div8
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Pcpresc::Div16
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Pcpresc::Div32
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Pcpresc::Div64
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Pcpresc::Div128
    }
}
#[doc = "Field `PCPRESC` writer - Period Counter Prescaler"]
pub type PcprescW<'a, REG> = crate::FieldWriter<'a, REG, 3, Pcpresc, crate::Safe>;
impl<'a, REG> PcprescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The period counter clock frequency is LFBCLKCSEN/1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Pcpresc::Div1)
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Pcpresc::Div2)
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Pcpresc::Div4)
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Pcpresc::Div8)
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Pcpresc::Div16)
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Pcpresc::Div32)
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Pcpresc::Div64)
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Pcpresc::Div128)
    }
}
#[doc = "Field `PCTOP` reader - Period Counter Top Value"]
pub type PctopR = crate::FieldReader;
#[doc = "Field `PCTOP` writer - Period Counter Top Value"]
pub type PctopW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WARMUPCNT` reader - Warmup Period Counter"]
pub type WarmupcntR = crate::FieldReader;
#[doc = "Field `WARMUPCNT` writer - Warmup Period Counter"]
pub type WarmupcntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - Period Counter Prescaler"]
    #[inline(always)]
    pub fn pcpresc(&self) -> PcprescR {
        PcprescR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:15 - Period Counter Top Value"]
    #[inline(always)]
    pub fn pctop(&self) -> PctopR {
        PctopR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Warmup Period Counter"]
    #[inline(always)]
    pub fn warmupcnt(&self) -> WarmupcntR {
        WarmupcntR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Period Counter Prescaler"]
    #[inline(always)]
    pub fn pcpresc(&mut self) -> PcprescW<'_, TimctrlSpec> {
        PcprescW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Period Counter Top Value"]
    #[inline(always)]
    pub fn pctop(&mut self) -> PctopW<'_, TimctrlSpec> {
        PctopW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Warmup Period Counter"]
    #[inline(always)]
    pub fn warmupcnt(&mut self) -> WarmupcntW<'_, TimctrlSpec> {
        WarmupcntW::new(self, 16)
    }
}
#[doc = "Timing Control\n\nYou can [`read`](crate::Reg::read) this register and get [`timctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimctrlSpec;
impl crate::RegisterSpec for TimctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timctrl::R`](R) reader structure"]
impl crate::Readable for TimctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`timctrl::W`](W) writer structure"]
impl crate::Writable for TimctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMCTRL to value 0"]
impl crate::Resettable for TimctrlSpec {}
