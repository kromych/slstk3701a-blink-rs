#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Field `CH0CD` reader - Channel 0 Conversion Done Interrupt Flag"]
pub type Ch0cdR = crate::BitReader;
#[doc = "Field `CH1CD` reader - Channel 1 Conversion Done Interrupt Flag"]
pub type Ch1cdR = crate::BitReader;
#[doc = "Field `CH0OF` reader - Channel 0 Data Overflow Interrupt Flag"]
pub type Ch0ofR = crate::BitReader;
#[doc = "Field `CH1OF` reader - Channel 1 Data Overflow Interrupt Flag"]
pub type Ch1ofR = crate::BitReader;
#[doc = "Field `CH0UF` reader - Channel 0 Data Underflow Interrupt Flag"]
pub type Ch0ufR = crate::BitReader;
#[doc = "Field `CH1UF` reader - Channel 1 Data Underflow Interrupt Flag"]
pub type Ch1ufR = crate::BitReader;
#[doc = "Field `CH0BL` reader - Channel 0 Buffer Level Interrupt Flag"]
pub type Ch0blR = crate::BitReader;
#[doc = "Field `CH1BL` reader - Channel 1 Buffer Level Interrupt Flag"]
pub type Ch1blR = crate::BitReader;
#[doc = "Field `EM23ERR` reader - EM2/3 Entry Error Flag"]
pub type Em23errR = crate::BitReader;
#[doc = "Field `OPA0APORTCONFLICT` reader - OPA0 Bus Conflict Output Interrupt Flag"]
pub type Opa0aportconflictR = crate::BitReader;
#[doc = "Field `OPA1APORTCONFLICT` reader - OPA1 Bus Conflict Output Interrupt Flag"]
pub type Opa1aportconflictR = crate::BitReader;
#[doc = "Field `OPA2APORTCONFLICT` reader - OPA2 Bus Conflict Output Interrupt Flag"]
pub type Opa2aportconflictR = crate::BitReader;
#[doc = "Field `OPA3APORTCONFLICT` reader - OPA3 Bus Conflict Output Interrupt Flag"]
pub type Opa3aportconflictR = crate::BitReader;
#[doc = "Field `OPA0PRSTIMEDERR` reader - OPA0 PRS Trigger Mode Error Interrupt Flag"]
pub type Opa0prstimederrR = crate::BitReader;
#[doc = "Field `OPA1PRSTIMEDERR` reader - OPA1 PRS Trigger Mode Error Interrupt Flag"]
pub type Opa1prstimederrR = crate::BitReader;
#[doc = "Field `OPA2PRSTIMEDERR` reader - OPA2 PRS Trigger Mode Error Interrupt Flag"]
pub type Opa2prstimederrR = crate::BitReader;
#[doc = "Field `OPA3PRSTIMEDERR` reader - OPA3 PRS Trigger Mode Error Interrupt Flag"]
pub type Opa3prstimederrR = crate::BitReader;
#[doc = "Field `OPA0OUTVALID` reader - OPA0 Output Valid Interrupt Flag"]
pub type Opa0outvalidR = crate::BitReader;
#[doc = "Field `OPA1OUTVALID` reader - OPA1 Output Valid Interrupt Flag"]
pub type Opa1outvalidR = crate::BitReader;
#[doc = "Field `OPA2OUTVALID` reader - OPA3 Output Valid Interrupt Flag"]
pub type Opa2outvalidR = crate::BitReader;
#[doc = "Field `OPA3OUTVALID` reader - OPA3 Output Valid Interrupt Flag"]
pub type Opa3outvalidR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel 0 Conversion Done Interrupt Flag"]
    #[inline(always)]
    pub fn ch0cd(&self) -> Ch0cdR {
        Ch0cdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Conversion Done Interrupt Flag"]
    #[inline(always)]
    pub fn ch1cd(&self) -> Ch1cdR {
        Ch1cdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 0 Data Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn ch0of(&self) -> Ch0ofR {
        Ch0ofR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 1 Data Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn ch1of(&self) -> Ch1ofR {
        Ch1ofR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 0 Data Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn ch0uf(&self) -> Ch0ufR {
        Ch0ufR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 1 Data Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn ch1uf(&self) -> Ch1ufR {
        Ch1ufR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 0 Buffer Level Interrupt Flag"]
    #[inline(always)]
    pub fn ch0bl(&self) -> Ch0blR {
        Ch0blR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 1 Buffer Level Interrupt Flag"]
    #[inline(always)]
    pub fn ch1bl(&self) -> Ch1blR {
        Ch1blR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - EM2/3 Entry Error Flag"]
    #[inline(always)]
    pub fn em23err(&self) -> Em23errR {
        Em23errR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - OPA0 Bus Conflict Output Interrupt Flag"]
    #[inline(always)]
    pub fn opa0aportconflict(&self) -> Opa0aportconflictR {
        Opa0aportconflictR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - OPA1 Bus Conflict Output Interrupt Flag"]
    #[inline(always)]
    pub fn opa1aportconflict(&self) -> Opa1aportconflictR {
        Opa1aportconflictR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - OPA2 Bus Conflict Output Interrupt Flag"]
    #[inline(always)]
    pub fn opa2aportconflict(&self) -> Opa2aportconflictR {
        Opa2aportconflictR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OPA3 Bus Conflict Output Interrupt Flag"]
    #[inline(always)]
    pub fn opa3aportconflict(&self) -> Opa3aportconflictR {
        Opa3aportconflictR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - OPA0 PRS Trigger Mode Error Interrupt Flag"]
    #[inline(always)]
    pub fn opa0prstimederr(&self) -> Opa0prstimederrR {
        Opa0prstimederrR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - OPA1 PRS Trigger Mode Error Interrupt Flag"]
    #[inline(always)]
    pub fn opa1prstimederr(&self) -> Opa1prstimederrR {
        Opa1prstimederrR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - OPA2 PRS Trigger Mode Error Interrupt Flag"]
    #[inline(always)]
    pub fn opa2prstimederr(&self) -> Opa2prstimederrR {
        Opa2prstimederrR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - OPA3 PRS Trigger Mode Error Interrupt Flag"]
    #[inline(always)]
    pub fn opa3prstimederr(&self) -> Opa3prstimederrR {
        Opa3prstimederrR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 28 - OPA0 Output Valid Interrupt Flag"]
    #[inline(always)]
    pub fn opa0outvalid(&self) -> Opa0outvalidR {
        Opa0outvalidR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - OPA1 Output Valid Interrupt Flag"]
    #[inline(always)]
    pub fn opa1outvalid(&self) -> Opa1outvalidR {
        Opa1outvalidR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - OPA3 Output Valid Interrupt Flag"]
    #[inline(always)]
    pub fn opa2outvalid(&self) -> Opa2outvalidR {
        Opa2outvalidR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - OPA3 Output Valid Interrupt Flag"]
    #[inline(always)]
    pub fn opa3outvalid(&self) -> Opa3outvalidR {
        Opa3outvalidR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfSpec;
impl crate::RegisterSpec for IfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IfSpec {}
#[doc = "`reset()` method sets IF to value 0xc0"]
impl crate::Resettable for IfSpec {
    const RESET_VALUE: u32 = 0xc0;
}
