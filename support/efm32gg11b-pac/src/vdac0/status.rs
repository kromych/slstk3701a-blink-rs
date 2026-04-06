#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `CH0ENS` reader - Channel 0 Enabled Status"]
pub type Ch0ensR = crate::BitReader;
#[doc = "Field `CH1ENS` reader - Channel 1 Enabled Status"]
pub type Ch1ensR = crate::BitReader;
#[doc = "Field `CH0BL` reader - Channel 0 Buffer Level"]
pub type Ch0blR = crate::BitReader;
#[doc = "Field `CH1BL` reader - Channel 1 Buffer Level"]
pub type Ch1blR = crate::BitReader;
#[doc = "Field `CH0WARM` reader - Channel 0 Warm"]
pub type Ch0warmR = crate::BitReader;
#[doc = "Field `CH1WARM` reader - Channel 1 Warm"]
pub type Ch1warmR = crate::BitReader;
#[doc = "Field `OPA0APORTCONFLICT` reader - OPA0 Bus Conflict Output"]
pub type Opa0aportconflictR = crate::BitReader;
#[doc = "Field `OPA1APORTCONFLICT` reader - OPA1 Bus Conflict Output"]
pub type Opa1aportconflictR = crate::BitReader;
#[doc = "Field `OPA2APORTCONFLICT` reader - OPA2 Bus Conflict Output"]
pub type Opa2aportconflictR = crate::BitReader;
#[doc = "Field `OPA3APORTCONFLICT` reader - OPA3 Bus Conflict Output"]
pub type Opa3aportconflictR = crate::BitReader;
#[doc = "Field `OPA0ENS` reader - OPA0 Enabled Status"]
pub type Opa0ensR = crate::BitReader;
#[doc = "Field `OPA1ENS` reader - OPA1 Enabled Status"]
pub type Opa1ensR = crate::BitReader;
#[doc = "Field `OPA2ENS` reader - OPA2 Enabled Status"]
pub type Opa2ensR = crate::BitReader;
#[doc = "Field `OPA3ENS` reader - OPA3 Enabled Status"]
pub type Opa3ensR = crate::BitReader;
#[doc = "Field `OPA0WARM` reader - OPA0 Warm Status"]
pub type Opa0warmR = crate::BitReader;
#[doc = "Field `OPA1WARM` reader - OPA1 Warm Status"]
pub type Opa1warmR = crate::BitReader;
#[doc = "Field `OPA2WARM` reader - OPA2 Warm Status"]
pub type Opa2warmR = crate::BitReader;
#[doc = "Field `OPA3WARM` reader - OPA3 Warm Status"]
pub type Opa3warmR = crate::BitReader;
#[doc = "Field `OPA0OUTVALID` reader - OPA0 Output Valid Status"]
pub type Opa0outvalidR = crate::BitReader;
#[doc = "Field `OPA1OUTVALID` reader - OPA1 Output Valid Status"]
pub type Opa1outvalidR = crate::BitReader;
#[doc = "Field `OPA2OUTVALID` reader - OPA2 Output Valid Status"]
pub type Opa2outvalidR = crate::BitReader;
#[doc = "Field `OPA3OUTVALID` reader - OPA3 Output Valid Status"]
pub type Opa3outvalidR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel 0 Enabled Status"]
    #[inline(always)]
    pub fn ch0ens(&self) -> Ch0ensR {
        Ch0ensR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Enabled Status"]
    #[inline(always)]
    pub fn ch1ens(&self) -> Ch1ensR {
        Ch1ensR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 0 Buffer Level"]
    #[inline(always)]
    pub fn ch0bl(&self) -> Ch0blR {
        Ch0blR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 1 Buffer Level"]
    #[inline(always)]
    pub fn ch1bl(&self) -> Ch1blR {
        Ch1blR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 0 Warm"]
    #[inline(always)]
    pub fn ch0warm(&self) -> Ch0warmR {
        Ch0warmR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 1 Warm"]
    #[inline(always)]
    pub fn ch1warm(&self) -> Ch1warmR {
        Ch1warmR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - OPA0 Bus Conflict Output"]
    #[inline(always)]
    pub fn opa0aportconflict(&self) -> Opa0aportconflictR {
        Opa0aportconflictR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - OPA1 Bus Conflict Output"]
    #[inline(always)]
    pub fn opa1aportconflict(&self) -> Opa1aportconflictR {
        Opa1aportconflictR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - OPA2 Bus Conflict Output"]
    #[inline(always)]
    pub fn opa2aportconflict(&self) -> Opa2aportconflictR {
        Opa2aportconflictR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OPA3 Bus Conflict Output"]
    #[inline(always)]
    pub fn opa3aportconflict(&self) -> Opa3aportconflictR {
        Opa3aportconflictR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - OPA0 Enabled Status"]
    #[inline(always)]
    pub fn opa0ens(&self) -> Opa0ensR {
        Opa0ensR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - OPA1 Enabled Status"]
    #[inline(always)]
    pub fn opa1ens(&self) -> Opa1ensR {
        Opa1ensR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - OPA2 Enabled Status"]
    #[inline(always)]
    pub fn opa2ens(&self) -> Opa2ensR {
        Opa2ensR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - OPA3 Enabled Status"]
    #[inline(always)]
    pub fn opa3ens(&self) -> Opa3ensR {
        Opa3ensR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - OPA0 Warm Status"]
    #[inline(always)]
    pub fn opa0warm(&self) -> Opa0warmR {
        Opa0warmR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - OPA1 Warm Status"]
    #[inline(always)]
    pub fn opa1warm(&self) -> Opa1warmR {
        Opa1warmR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - OPA2 Warm Status"]
    #[inline(always)]
    pub fn opa2warm(&self) -> Opa2warmR {
        Opa2warmR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - OPA3 Warm Status"]
    #[inline(always)]
    pub fn opa3warm(&self) -> Opa3warmR {
        Opa3warmR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - OPA0 Output Valid Status"]
    #[inline(always)]
    pub fn opa0outvalid(&self) -> Opa0outvalidR {
        Opa0outvalidR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - OPA1 Output Valid Status"]
    #[inline(always)]
    pub fn opa1outvalid(&self) -> Opa1outvalidR {
        Opa1outvalidR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - OPA2 Output Valid Status"]
    #[inline(always)]
    pub fn opa2outvalid(&self) -> Opa2outvalidR {
        Opa2outvalidR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - OPA3 Output Valid Status"]
    #[inline(always)]
    pub fn opa3outvalid(&self) -> Opa3outvalidR {
        Opa3outvalidR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0x0c"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0x0c;
}
