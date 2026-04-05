#[doc = "Register `R5VADCCTRL` reader"]
pub type R = crate::R<R5vadcctrlSpec>;
#[doc = "Register `R5VADCCTRL` writer"]
pub type W = crate::W<R5vadcctrlSpec>;
#[doc = "Field `ENAMUX` reader - Enable the 5V Subsystem ADC MUX"]
pub type EnamuxR = crate::BitReader;
#[doc = "Field `ENAMUX` writer - Enable the 5V Subsystem ADC MUX"]
pub type EnamuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "ADC Mux Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Amuxsel {
    #[doc = "0: VBUS divided by 10"]
    Vbusdiv10 = 0,
    #[doc = "1: VREGI divided by 10"]
    Vregidiv10 = 1,
    #[doc = "2: VREGO divided by 6"]
    Vregodiv6 = 2,
    #[doc = "3: VREGI current monitor"]
    Vregiimon = 3,
    #[doc = "4: VBUS current monitor"]
    Vbusimon = 4,
}
impl From<Amuxsel> for u8 {
    #[inline(always)]
    fn from(variant: Amuxsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Amuxsel {
    type Ux = u8;
}
impl crate::IsEnum for Amuxsel {}
#[doc = "Field `AMUXSEL` reader - ADC Mux Selection"]
pub type AmuxselR = crate::FieldReader<Amuxsel>;
impl AmuxselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Amuxsel> {
        match self.bits {
            0 => Some(Amuxsel::Vbusdiv10),
            1 => Some(Amuxsel::Vregidiv10),
            2 => Some(Amuxsel::Vregodiv6),
            3 => Some(Amuxsel::Vregiimon),
            4 => Some(Amuxsel::Vbusimon),
            _ => None,
        }
    }
    #[doc = "VBUS divided by 10"]
    #[inline(always)]
    pub fn is_vbusdiv10(&self) -> bool {
        *self == Amuxsel::Vbusdiv10
    }
    #[doc = "VREGI divided by 10"]
    #[inline(always)]
    pub fn is_vregidiv10(&self) -> bool {
        *self == Amuxsel::Vregidiv10
    }
    #[doc = "VREGO divided by 6"]
    #[inline(always)]
    pub fn is_vregodiv6(&self) -> bool {
        *self == Amuxsel::Vregodiv6
    }
    #[doc = "VREGI current monitor"]
    #[inline(always)]
    pub fn is_vregiimon(&self) -> bool {
        *self == Amuxsel::Vregiimon
    }
    #[doc = "VBUS current monitor"]
    #[inline(always)]
    pub fn is_vbusimon(&self) -> bool {
        *self == Amuxsel::Vbusimon
    }
}
#[doc = "Field `AMUXSEL` writer - ADC Mux Selection"]
pub type AmuxselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Amuxsel>;
impl<'a, REG> AmuxselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VBUS divided by 10"]
    #[inline(always)]
    pub fn vbusdiv10(self) -> &'a mut crate::W<REG> {
        self.variant(Amuxsel::Vbusdiv10)
    }
    #[doc = "VREGI divided by 10"]
    #[inline(always)]
    pub fn vregidiv10(self) -> &'a mut crate::W<REG> {
        self.variant(Amuxsel::Vregidiv10)
    }
    #[doc = "VREGO divided by 6"]
    #[inline(always)]
    pub fn vregodiv6(self) -> &'a mut crate::W<REG> {
        self.variant(Amuxsel::Vregodiv6)
    }
    #[doc = "VREGI current monitor"]
    #[inline(always)]
    pub fn vregiimon(self) -> &'a mut crate::W<REG> {
        self.variant(Amuxsel::Vregiimon)
    }
    #[doc = "VBUS current monitor"]
    #[inline(always)]
    pub fn vbusimon(self) -> &'a mut crate::W<REG> {
        self.variant(Amuxsel::Vbusimon)
    }
}
impl R {
    #[doc = "Bit 0 - Enable the 5V Subsystem ADC MUX"]
    #[inline(always)]
    pub fn enamux(&self) -> EnamuxR {
        EnamuxR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 12:15 - ADC Mux Selection"]
    #[inline(always)]
    pub fn amuxsel(&self) -> AmuxselR {
        AmuxselR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the 5V Subsystem ADC MUX"]
    #[inline(always)]
    pub fn enamux(&mut self) -> EnamuxW<'_, R5vadcctrlSpec> {
        EnamuxW::new(self, 0)
    }
    #[doc = "Bits 12:15 - ADC Mux Selection"]
    #[inline(always)]
    pub fn amuxsel(&mut self) -> AmuxselW<'_, R5vadcctrlSpec> {
        AmuxselW::new(self, 12)
    }
}
#[doc = "5V Regulator Control\n\nYou can [`read`](crate::Reg::read) this register and get [`r5vadcctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5vadcctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R5vadcctrlSpec;
impl crate::RegisterSpec for R5vadcctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r5vadcctrl::R`](R) reader structure"]
impl crate::Readable for R5vadcctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`r5vadcctrl::W`](W) writer structure"]
impl crate::Writable for R5vadcctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R5VADCCTRL to value 0"]
impl crate::Resettable for R5vadcctrlSpec {}
