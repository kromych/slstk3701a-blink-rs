#[doc = "Register `DCDCMISCCTRL` reader"]
pub type R = crate::R<DcdcmiscctrlSpec>;
#[doc = "Register `DCDCMISCCTRL` writer"]
pub type W = crate::W<DcdcmiscctrlSpec>;
#[doc = "Field `LNFORCECCM` reader - Force DCDC Into CCM Mode in Low Noise Operation"]
pub type LnforceccmR = crate::BitReader;
#[doc = "Field `LNFORCECCM` writer - Force DCDC Into CCM Mode in Low Noise Operation"]
pub type LnforceccmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPCMPHYSDIS` reader - Disable LP Mode Hysteresis in the State Machine Control"]
pub type LpcmphysdisR = crate::BitReader;
#[doc = "Field `LPCMPHYSDIS` writer - Disable LP Mode Hysteresis in the State Machine Control"]
pub type LpcmphysdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPCMPHYSHI` reader - Comparator Threshold on the High Side"]
pub type LpcmphyshiR = crate::BitReader;
#[doc = "Field `LPCMPHYSHI` writer - Comparator Threshold on the High Side"]
pub type LpcmphyshiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LNFORCECCMIMM` reader - Force DCDC Into CCM Mode Immediately, Based on LNFORCECCM"]
pub type LnforceccmimmR = crate::BitReader;
#[doc = "Field `LNFORCECCMIMM` writer - Force DCDC Into CCM Mode Immediately, Based on LNFORCECCM"]
pub type LnforceccmimmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFETCNT` reader - PFET Switch Number Selection"]
pub type PfetcntR = crate::FieldReader;
#[doc = "Field `PFETCNT` writer - PFET Switch Number Selection"]
pub type PfetcntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NFETCNT` reader - NFET Switch Number Selection"]
pub type NfetcntR = crate::FieldReader;
#[doc = "Field `NFETCNT` writer - NFET Switch Number Selection"]
pub type NfetcntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BYPLIMSEL` reader - Current Limit in Bypass Mode"]
pub type ByplimselR = crate::FieldReader;
#[doc = "Field `BYPLIMSEL` writer - Current Limit in Bypass Mode"]
pub type ByplimselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPCLIMILIMSEL` reader - Current Limit Level Selection for Current Limiter in LP Mode"]
pub type LpclimilimselR = crate::FieldReader;
#[doc = "Field `LPCLIMILIMSEL` writer - Current Limit Level Selection for Current Limiter in LP Mode"]
pub type LpclimilimselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LNCLIMILIMSEL` reader - Current Limit Level Selection for Current Limiter in LN Mode"]
pub type LnclimilimselR = crate::FieldReader;
#[doc = "Field `LNCLIMILIMSEL` writer - Current Limit Level Selection for Current Limiter in LN Mode"]
pub type LnclimilimselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "LP Mode Comparator Bias Selection for EM23 or EM4H\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lpcmpbiasem234h {
    #[doc = "0: Maximum load current less than 75uA."]
    Bias0 = 0,
    #[doc = "1: Maximum load current less than 500uA."]
    Bias1 = 1,
    #[doc = "2: Maximum load current less than 2.5mA."]
    Bias2 = 2,
    #[doc = "3: Maximum load current less than 10mA."]
    Bias3 = 3,
}
impl From<Lpcmpbiasem234h> for u8 {
    #[inline(always)]
    fn from(variant: Lpcmpbiasem234h) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lpcmpbiasem234h {
    type Ux = u8;
}
impl crate::IsEnum for Lpcmpbiasem234h {}
#[doc = "Field `LPCMPBIASEM234H` reader - LP Mode Comparator Bias Selection for EM23 or EM4H"]
pub type Lpcmpbiasem234hR = crate::FieldReader<Lpcmpbiasem234h>;
impl Lpcmpbiasem234hR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpcmpbiasem234h {
        match self.bits {
            0 => Lpcmpbiasem234h::Bias0,
            1 => Lpcmpbiasem234h::Bias1,
            2 => Lpcmpbiasem234h::Bias2,
            3 => Lpcmpbiasem234h::Bias3,
            _ => unreachable!(),
        }
    }
    #[doc = "Maximum load current less than 75uA."]
    #[inline(always)]
    pub fn is_bias0(&self) -> bool {
        *self == Lpcmpbiasem234h::Bias0
    }
    #[doc = "Maximum load current less than 500uA."]
    #[inline(always)]
    pub fn is_bias1(&self) -> bool {
        *self == Lpcmpbiasem234h::Bias1
    }
    #[doc = "Maximum load current less than 2.5mA."]
    #[inline(always)]
    pub fn is_bias2(&self) -> bool {
        *self == Lpcmpbiasem234h::Bias2
    }
    #[doc = "Maximum load current less than 10mA."]
    #[inline(always)]
    pub fn is_bias3(&self) -> bool {
        *self == Lpcmpbiasem234h::Bias3
    }
}
#[doc = "Field `LPCMPBIASEM234H` writer - LP Mode Comparator Bias Selection for EM23 or EM4H"]
pub type Lpcmpbiasem234hW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lpcmpbiasem234h, crate::Safe>;
impl<'a, REG> Lpcmpbiasem234hW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Maximum load current less than 75uA."]
    #[inline(always)]
    pub fn bias0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpcmpbiasem234h::Bias0)
    }
    #[doc = "Maximum load current less than 500uA."]
    #[inline(always)]
    pub fn bias1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpcmpbiasem234h::Bias1)
    }
    #[doc = "Maximum load current less than 2.5mA."]
    #[inline(always)]
    pub fn bias2(self) -> &'a mut crate::W<REG> {
        self.variant(Lpcmpbiasem234h::Bias2)
    }
    #[doc = "Maximum load current less than 10mA."]
    #[inline(always)]
    pub fn bias3(self) -> &'a mut crate::W<REG> {
        self.variant(Lpcmpbiasem234h::Bias3)
    }
}
impl R {
    #[doc = "Bit 0 - Force DCDC Into CCM Mode in Low Noise Operation"]
    #[inline(always)]
    pub fn lnforceccm(&self) -> LnforceccmR {
        LnforceccmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable LP Mode Hysteresis in the State Machine Control"]
    #[inline(always)]
    pub fn lpcmphysdis(&self) -> LpcmphysdisR {
        LpcmphysdisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comparator Threshold on the High Side"]
    #[inline(always)]
    pub fn lpcmphyshi(&self) -> LpcmphyshiR {
        LpcmphyshiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Force DCDC Into CCM Mode Immediately, Based on LNFORCECCM"]
    #[inline(always)]
    pub fn lnforceccmimm(&self) -> LnforceccmimmR {
        LnforceccmimmR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - PFET Switch Number Selection"]
    #[inline(always)]
    pub fn pfetcnt(&self) -> PfetcntR {
        PfetcntR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - NFET Switch Number Selection"]
    #[inline(always)]
    pub fn nfetcnt(&self) -> NfetcntR {
        NfetcntR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Current Limit in Bypass Mode"]
    #[inline(always)]
    pub fn byplimsel(&self) -> ByplimselR {
        ByplimselR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - Current Limit Level Selection for Current Limiter in LP Mode"]
    #[inline(always)]
    pub fn lpclimilimsel(&self) -> LpclimilimselR {
        LpclimilimselR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Current Limit Level Selection for Current Limiter in LN Mode"]
    #[inline(always)]
    pub fn lnclimilimsel(&self) -> LnclimilimselR {
        LnclimilimselR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:29 - LP Mode Comparator Bias Selection for EM23 or EM4H"]
    #[inline(always)]
    pub fn lpcmpbiasem234h(&self) -> Lpcmpbiasem234hR {
        Lpcmpbiasem234hR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Force DCDC Into CCM Mode in Low Noise Operation"]
    #[inline(always)]
    pub fn lnforceccm(&mut self) -> LnforceccmW<'_, DcdcmiscctrlSpec> {
        LnforceccmW::new(self, 0)
    }
    #[doc = "Bit 1 - Disable LP Mode Hysteresis in the State Machine Control"]
    #[inline(always)]
    pub fn lpcmphysdis(&mut self) -> LpcmphysdisW<'_, DcdcmiscctrlSpec> {
        LpcmphysdisW::new(self, 1)
    }
    #[doc = "Bit 2 - Comparator Threshold on the High Side"]
    #[inline(always)]
    pub fn lpcmphyshi(&mut self) -> LpcmphyshiW<'_, DcdcmiscctrlSpec> {
        LpcmphyshiW::new(self, 2)
    }
    #[doc = "Bit 5 - Force DCDC Into CCM Mode Immediately, Based on LNFORCECCM"]
    #[inline(always)]
    pub fn lnforceccmimm(&mut self) -> LnforceccmimmW<'_, DcdcmiscctrlSpec> {
        LnforceccmimmW::new(self, 5)
    }
    #[doc = "Bits 8:11 - PFET Switch Number Selection"]
    #[inline(always)]
    pub fn pfetcnt(&mut self) -> PfetcntW<'_, DcdcmiscctrlSpec> {
        PfetcntW::new(self, 8)
    }
    #[doc = "Bits 12:15 - NFET Switch Number Selection"]
    #[inline(always)]
    pub fn nfetcnt(&mut self) -> NfetcntW<'_, DcdcmiscctrlSpec> {
        NfetcntW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Current Limit in Bypass Mode"]
    #[inline(always)]
    pub fn byplimsel(&mut self) -> ByplimselW<'_, DcdcmiscctrlSpec> {
        ByplimselW::new(self, 16)
    }
    #[doc = "Bits 20:22 - Current Limit Level Selection for Current Limiter in LP Mode"]
    #[inline(always)]
    pub fn lpclimilimsel(&mut self) -> LpclimilimselW<'_, DcdcmiscctrlSpec> {
        LpclimilimselW::new(self, 20)
    }
    #[doc = "Bits 24:26 - Current Limit Level Selection for Current Limiter in LN Mode"]
    #[inline(always)]
    pub fn lnclimilimsel(&mut self) -> LnclimilimselW<'_, DcdcmiscctrlSpec> {
        LnclimilimselW::new(self, 24)
    }
    #[doc = "Bits 28:29 - LP Mode Comparator Bias Selection for EM23 or EM4H"]
    #[inline(always)]
    pub fn lpcmpbiasem234h(&mut self) -> Lpcmpbiasem234hW<'_, DcdcmiscctrlSpec> {
        Lpcmpbiasem234hW::new(self, 28)
    }
}
#[doc = "DCDC Miscellaneous Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdcmiscctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdcmiscctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcdcmiscctrlSpec;
impl crate::RegisterSpec for DcdcmiscctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdcmiscctrl::R`](R) reader structure"]
impl crate::Readable for DcdcmiscctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dcdcmiscctrl::W`](W) writer structure"]
impl crate::Writable for DcdcmiscctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCDCMISCCTRL to value 0x0310_7706"]
impl crate::Resettable for DcdcmiscctrlSpec {
    const RESET_VALUE: u32 = 0x0310_7706;
}
