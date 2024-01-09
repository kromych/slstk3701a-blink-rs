#[doc = "Register `PRSSTAT` reader"]
pub type R = crate::R<PRSSTAT_SPEC>;
#[doc = "Field `CMDINHIBITCMD` reader - Command Inhibit (CMD)"]
pub type CMDINHIBITCMD_R = crate::BitReader;
#[doc = "Field `CMDINHIBITDAT` reader - Command Inhibit (DAT)"]
pub type CMDINHIBITDAT_R = crate::BitReader;
#[doc = "Field `DATLINEACTIVE` reader - DAT Line Active"]
pub type DATLINEACTIVE_R = crate::BitReader;
#[doc = "Field `RETUNINGREQ` reader - Re-Tuning Request"]
pub type RETUNINGREQ_R = crate::BitReader;
#[doc = "Field `WRTRANACT` reader - Write Transfer Active"]
pub type WRTRANACT_R = crate::BitReader;
#[doc = "Field `RDTRANACT` reader - Read Transfer Active"]
pub type RDTRANACT_R = crate::BitReader;
#[doc = "Field `BUFFERWRITEENABLE` reader - Buffer Write Enable"]
pub type BUFFERWRITEENABLE_R = crate::BitReader;
#[doc = "Field `BUFRDEN` reader - Buffer Read Enable"]
pub type BUFRDEN_R = crate::BitReader;
#[doc = "Field `CARDINS` reader - Card Inserted Status"]
pub type CARDINS_R = crate::BitReader;
#[doc = "Field `CARDSTATESTABLE` reader - Card State Stable Status"]
pub type CARDSTATESTABLE_R = crate::BitReader;
#[doc = "Field `CARDDETPINLVL` reader - Card Detect Pin Level"]
pub type CARDDETPINLVL_R = crate::BitReader;
#[doc = "Field `WRPROTSWPINLVL` reader - Write Protect Switch Pin Level"]
pub type WRPROTSWPINLVL_R = crate::BitReader;
#[doc = "Field `DAT3TO0SIGLVL` reader - DAT\\[3:0\\]
Line Signal Level"]
pub type DAT3TO0SIGLVL_R = crate::FieldReader;
#[doc = "Field `CMDSIGLVL` reader - Command Line Signal Level"]
pub type CMDSIGLVL_R = crate::BitReader;
#[doc = "Field `DAT7TO4SIGLVL` reader - DAT\\[7:4\\]
Line Signal Level"]
pub type DAT7TO4SIGLVL_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Command Inhibit (CMD)"]
    #[inline(always)]
    pub fn cmdinhibitcmd(&self) -> CMDINHIBITCMD_R {
        CMDINHIBITCMD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command Inhibit (DAT)"]
    #[inline(always)]
    pub fn cmdinhibitdat(&self) -> CMDINHIBITDAT_R {
        CMDINHIBITDAT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DAT Line Active"]
    #[inline(always)]
    pub fn datlineactive(&self) -> DATLINEACTIVE_R {
        DATLINEACTIVE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Re-Tuning Request"]
    #[inline(always)]
    pub fn retuningreq(&self) -> RETUNINGREQ_R {
        RETUNINGREQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Write Transfer Active"]
    #[inline(always)]
    pub fn wrtranact(&self) -> WRTRANACT_R {
        WRTRANACT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Read Transfer Active"]
    #[inline(always)]
    pub fn rdtranact(&self) -> RDTRANACT_R {
        RDTRANACT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Buffer Write Enable"]
    #[inline(always)]
    pub fn bufferwriteenable(&self) -> BUFFERWRITEENABLE_R {
        BUFFERWRITEENABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Buffer Read Enable"]
    #[inline(always)]
    pub fn bufrden(&self) -> BUFRDEN_R {
        BUFRDEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Card Inserted Status"]
    #[inline(always)]
    pub fn cardins(&self) -> CARDINS_R {
        CARDINS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Card State Stable Status"]
    #[inline(always)]
    pub fn cardstatestable(&self) -> CARDSTATESTABLE_R {
        CARDSTATESTABLE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Card Detect Pin Level"]
    #[inline(always)]
    pub fn carddetpinlvl(&self) -> CARDDETPINLVL_R {
        CARDDETPINLVL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Write Protect Switch Pin Level"]
    #[inline(always)]
    pub fn wrprotswpinlvl(&self) -> WRPROTSWPINLVL_R {
        WRPROTSWPINLVL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - DAT\\[3:0\\]
Line Signal Level"]
    #[inline(always)]
    pub fn dat3to0siglvl(&self) -> DAT3TO0SIGLVL_R {
        DAT3TO0SIGLVL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Command Line Signal Level"]
    #[inline(always)]
    pub fn cmdsiglvl(&self) -> CMDSIGLVL_R {
        CMDSIGLVL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:28 - DAT\\[7:4\\]
Line Signal Level"]
    #[inline(always)]
    pub fn dat7to4siglvl(&self) -> DAT7TO4SIGLVL_R {
        DAT7TO4SIGLVL_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
#[doc = "Present State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prsstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRSSTAT_SPEC;
impl crate::RegisterSpec for PRSSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prsstat::R`](R) reader structure"]
impl crate::Readable for PRSSTAT_SPEC {}
#[doc = "`reset()` method sets PRSSTAT to value 0"]
impl crate::Resettable for PRSSTAT_SPEC {
    const RESET_VALUE: u32 = 0;
}
