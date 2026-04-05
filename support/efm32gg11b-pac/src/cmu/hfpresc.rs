#[doc = "Register `HFPRESC` reader"]
pub type R = crate::R<HfprescSpec>;
#[doc = "Register `HFPRESC` writer"]
pub type W = crate::W<HfprescSpec>;
#[doc = "HFCLK Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Presc {
    #[doc = "0: `0`"]
    Nodivision = 0,
}
impl From<Presc> for u8 {
    #[inline(always)]
    fn from(variant: Presc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Presc {
    type Ux = u8;
}
impl crate::IsEnum for Presc {}
#[doc = "Field `PRESC` reader - HFCLK Prescaler"]
pub type PrescR = crate::FieldReader<Presc>;
impl PrescR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Presc> {
        match self.bits {
            0 => Some(Presc::Nodivision),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_nodivision(&self) -> bool {
        *self == Presc::Nodivision
    }
}
#[doc = "Field `PRESC` writer - HFCLK Prescaler"]
pub type PrescW<'a, REG> = crate::FieldWriter<'a, REG, 5, Presc>;
impl<'a, REG> PrescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn nodivision(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Nodivision)
    }
}
#[doc = "HFCLKLE Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hfclklepresc {
    #[doc = "0: HFCLKLE is HFBUSCLKLE divided by 2."]
    Div2 = 0,
    #[doc = "1: HFCLKLE is HFBUSCLKLE divided by 4."]
    Div4 = 1,
    #[doc = "2: HFCLKLE is HFBUSCLKLE divided by 8."]
    Div8 = 2,
}
impl From<Hfclklepresc> for u8 {
    #[inline(always)]
    fn from(variant: Hfclklepresc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hfclklepresc {
    type Ux = u8;
}
impl crate::IsEnum for Hfclklepresc {}
#[doc = "Field `HFCLKLEPRESC` reader - HFCLKLE Prescaler"]
pub type HfclkleprescR = crate::FieldReader<Hfclklepresc>;
impl HfclkleprescR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hfclklepresc> {
        match self.bits {
            0 => Some(Hfclklepresc::Div2),
            1 => Some(Hfclklepresc::Div4),
            2 => Some(Hfclklepresc::Div8),
            _ => None,
        }
    }
    #[doc = "HFCLKLE is HFBUSCLKLE divided by 2."]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Hfclklepresc::Div2
    }
    #[doc = "HFCLKLE is HFBUSCLKLE divided by 4."]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Hfclklepresc::Div4
    }
    #[doc = "HFCLKLE is HFBUSCLKLE divided by 8."]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Hfclklepresc::Div8
    }
}
#[doc = "Field `HFCLKLEPRESC` writer - HFCLKLE Prescaler"]
pub type HfclkleprescW<'a, REG> = crate::FieldWriter<'a, REG, 2, Hfclklepresc>;
impl<'a, REG> HfclkleprescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HFCLKLE is HFBUSCLKLE divided by 2."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Hfclklepresc::Div2)
    }
    #[doc = "HFCLKLE is HFBUSCLKLE divided by 4."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Hfclklepresc::Div4)
    }
    #[doc = "HFCLKLE is HFBUSCLKLE divided by 8."]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Hfclklepresc::Div8)
    }
}
impl R {
    #[doc = "Bits 8:12 - HFCLK Prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PrescR {
        PrescR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 24:25 - HFCLKLE Prescaler"]
    #[inline(always)]
    pub fn hfclklepresc(&self) -> HfclkleprescR {
        HfclkleprescR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 8:12 - HFCLK Prescaler"]
    #[inline(always)]
    pub fn presc(&mut self) -> PrescW<'_, HfprescSpec> {
        PrescW::new(self, 8)
    }
    #[doc = "Bits 24:25 - HFCLKLE Prescaler"]
    #[inline(always)]
    pub fn hfclklepresc(&mut self) -> HfclkleprescW<'_, HfprescSpec> {
        HfclkleprescW::new(self, 24)
    }
}
#[doc = "High Frequency Clock Prescaler Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hfpresc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfpresc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfprescSpec;
impl crate::RegisterSpec for HfprescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfpresc::R`](R) reader structure"]
impl crate::Readable for HfprescSpec {}
#[doc = "`write(|w| ..)` method takes [`hfpresc::W`](W) writer structure"]
impl crate::Writable for HfprescSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HFPRESC to value 0"]
impl crate::Resettable for HfprescSpec {}
