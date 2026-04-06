#[doc = "Register `HFCOREPRESC` reader"]
pub type R = crate::R<HfcoreprescSpec>;
#[doc = "Register `HFCOREPRESC` writer"]
pub type W = crate::W<HfcoreprescSpec>;
#[doc = "HFCORECLK Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Presc {
    #[doc = "0: `0`"]
    Nodivision = 0,
}
impl From<Presc> for u16 {
    #[inline(always)]
    fn from(variant: Presc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Presc {
    type Ux = u16;
}
impl crate::IsEnum for Presc {}
#[doc = "Field `PRESC` reader - HFCORECLK Prescaler"]
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
#[doc = "Field `PRESC` writer - HFCORECLK Prescaler"]
pub type PrescW<'a, REG> = crate::FieldWriter<'a, REG, 9, Presc>;
impl<'a, REG> PrescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn nodivision(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Nodivision)
    }
}
impl R {
    #[doc = "Bits 8:16 - HFCORECLK Prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PrescR {
        PrescR::new(((self.bits >> 8) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:16 - HFCORECLK Prescaler"]
    #[inline(always)]
    pub fn presc(&mut self) -> PrescW<'_, HfcoreprescSpec> {
        PrescW::new(self, 8)
    }
}
#[doc = "High Frequency Core Clock Prescaler Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hfcorepresc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfcorepresc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfcoreprescSpec;
impl crate::RegisterSpec for HfcoreprescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfcorepresc::R`](R) reader structure"]
impl crate::Readable for HfcoreprescSpec {}
#[doc = "`write(|w| ..)` method takes [`hfcorepresc::W`](W) writer structure"]
impl crate::Writable for HfcoreprescSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HFCOREPRESC to value 0"]
impl crate::Resettable for HfcoreprescSpec {}
