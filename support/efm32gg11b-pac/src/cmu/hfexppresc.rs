#[doc = "Register `HFEXPPRESC` reader"]
pub type R = crate::R<HfexpprescSpec>;
#[doc = "Register `HFEXPPRESC` writer"]
pub type W = crate::W<HfexpprescSpec>;
#[doc = "HFEXPCLK Prescaler\n\nValue on reset: 0"]
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
#[doc = "Field `PRESC` reader - HFEXPCLK Prescaler"]
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
#[doc = "Field `PRESC` writer - HFEXPCLK Prescaler"]
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
impl R {
    #[doc = "Bits 8:12 - HFEXPCLK Prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PrescR {
        PrescR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:12 - HFEXPCLK Prescaler"]
    #[inline(always)]
    pub fn presc(&mut self) -> PrescW<'_, HfexpprescSpec> {
        PrescW::new(self, 8)
    }
}
#[doc = "High Frequency Export Clock Prescaler Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hfexppresc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfexppresc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfexpprescSpec;
impl crate::RegisterSpec for HfexpprescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfexppresc::R`](R) reader structure"]
impl crate::Readable for HfexpprescSpec {}
#[doc = "`write(|w| ..)` method takes [`hfexppresc::W`](W) writer structure"]
impl crate::Writable for HfexpprescSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HFEXPPRESC to value 0"]
impl crate::Resettable for HfexpprescSpec {}
