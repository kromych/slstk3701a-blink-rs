#[doc = "Register `ETMAUTHSTATUS` reader"]
pub type R = crate::R<EtmauthstatusSpec>;
#[doc = "Field `NONSECINVDBG` reader - Non-secure invasive Debug Status"]
pub type NonsecinvdbgR = crate::FieldReader;
#[doc = "Non-secure non-invasive Debug Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nonsecnoninvdbg {
    #[doc = "2: Non-secure non-invasive debug disable"]
    Disable = 2,
    #[doc = "3: Non-secure non-invasive debug enable"]
    Enable = 3,
}
impl From<Nonsecnoninvdbg> for u8 {
    #[inline(always)]
    fn from(variant: Nonsecnoninvdbg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nonsecnoninvdbg {
    type Ux = u8;
}
impl crate::IsEnum for Nonsecnoninvdbg {}
#[doc = "Field `NONSECNONINVDBG` reader - Non-secure non-invasive Debug Status"]
pub type NonsecnoninvdbgR = crate::FieldReader<Nonsecnoninvdbg>;
impl NonsecnoninvdbgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Nonsecnoninvdbg> {
        match self.bits {
            2 => Some(Nonsecnoninvdbg::Disable),
            3 => Some(Nonsecnoninvdbg::Enable),
            _ => None,
        }
    }
    #[doc = "Non-secure non-invasive debug disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Nonsecnoninvdbg::Disable
    }
    #[doc = "Non-secure non-invasive debug enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Nonsecnoninvdbg::Enable
    }
}
#[doc = "Field `SECINVDBG` reader - Secure invasive Debug Status"]
pub type SecinvdbgR = crate::FieldReader;
#[doc = "Field `SECNONINVDBG` reader - Secure non-invasive Debug Status"]
pub type SecnoninvdbgR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Non-secure invasive Debug Status"]
    #[inline(always)]
    pub fn nonsecinvdbg(&self) -> NonsecinvdbgR {
        NonsecinvdbgR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Non-secure non-invasive Debug Status"]
    #[inline(always)]
    pub fn nonsecnoninvdbg(&self) -> NonsecnoninvdbgR {
        NonsecnoninvdbgR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Secure invasive Debug Status"]
    #[inline(always)]
    pub fn secinvdbg(&self) -> SecinvdbgR {
        SecinvdbgR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Secure non-invasive Debug Status"]
    #[inline(always)]
    pub fn secnoninvdbg(&self) -> SecnoninvdbgR {
        SecnoninvdbgR::new(((self.bits >> 6) & 3) as u8)
    }
}
#[doc = "ETM Authentication Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmauthstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtmauthstatusSpec;
impl crate::RegisterSpec for EtmauthstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmauthstatus::R`](R) reader structure"]
impl crate::Readable for EtmauthstatusSpec {}
#[doc = "`reset()` method sets ETMAUTHSTATUS to value 0xc0"]
impl crate::Resettable for EtmauthstatusSpec {
    const RESET_VALUE: u32 = 0xc0;
}
