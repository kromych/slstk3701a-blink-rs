#[doc = "Register `PRSSTAT` reader"]
pub type R = crate::R<PrsstatSpec>;
#[doc = "Field `CMDINHIBITCMD` reader - Command Inhibit (CMD)"]
pub type CmdinhibitcmdR = crate::BitReader;
#[doc = "Field `CMDINHIBITDAT` reader - Command Inhibit (DAT)"]
pub type CmdinhibitdatR = crate::BitReader;
#[doc = "Field `DATLINEACTIVE` reader - DAT Line Active"]
pub type DatlineactiveR = crate::BitReader;
#[doc = "Field `RETUNINGREQ` reader - Re-Tuning Request"]
pub type RetuningreqR = crate::BitReader;
#[doc = "Field `WRTRANACT` reader - Write Transfer Active"]
pub type WrtranactR = crate::BitReader;
#[doc = "Field `RDTRANACT` reader - Read Transfer Active"]
pub type RdtranactR = crate::BitReader;
#[doc = "Field `BUFFERWRITEENABLE` reader - Buffer Write Enable"]
pub type BufferwriteenableR = crate::BitReader;
#[doc = "Field `BUFRDEN` reader - Buffer Read Enable"]
pub type BufrdenR = crate::BitReader;
#[doc = "Field `CARDINS` reader - Card Inserted Status"]
pub type CardinsR = crate::BitReader;
#[doc = "Field `CARDSTATESTABLE` reader - Card State Stable Status"]
pub type CardstatestableR = crate::BitReader;
#[doc = "Field `CARDDETPINLVL` reader - Card Detect Pin Level"]
pub type CarddetpinlvlR = crate::BitReader;
#[doc = "Field `WRPROTSWPINLVL` reader - Write Protect Switch Pin Level"]
pub type WrprotswpinlvlR = crate::BitReader;
#[doc = "Field `DAT3TO0SIGLVL` reader - DAT\\[3:0\\] Line Signal Level"]
pub type Dat3to0siglvlR = crate::FieldReader;
#[doc = "Field `CMDSIGLVL` reader - Command Line Signal Level"]
pub type CmdsiglvlR = crate::BitReader;
#[doc = "Field `DAT7TO4SIGLVL` reader - DAT\\[7:4\\] Line Signal Level"]
pub type Dat7to4siglvlR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Command Inhibit (CMD)"]
    #[inline(always)]
    pub fn cmdinhibitcmd(&self) -> CmdinhibitcmdR {
        CmdinhibitcmdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command Inhibit (DAT)"]
    #[inline(always)]
    pub fn cmdinhibitdat(&self) -> CmdinhibitdatR {
        CmdinhibitdatR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DAT Line Active"]
    #[inline(always)]
    pub fn datlineactive(&self) -> DatlineactiveR {
        DatlineactiveR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Re-Tuning Request"]
    #[inline(always)]
    pub fn retuningreq(&self) -> RetuningreqR {
        RetuningreqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Write Transfer Active"]
    #[inline(always)]
    pub fn wrtranact(&self) -> WrtranactR {
        WrtranactR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Read Transfer Active"]
    #[inline(always)]
    pub fn rdtranact(&self) -> RdtranactR {
        RdtranactR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Buffer Write Enable"]
    #[inline(always)]
    pub fn bufferwriteenable(&self) -> BufferwriteenableR {
        BufferwriteenableR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Buffer Read Enable"]
    #[inline(always)]
    pub fn bufrden(&self) -> BufrdenR {
        BufrdenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Card Inserted Status"]
    #[inline(always)]
    pub fn cardins(&self) -> CardinsR {
        CardinsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Card State Stable Status"]
    #[inline(always)]
    pub fn cardstatestable(&self) -> CardstatestableR {
        CardstatestableR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Card Detect Pin Level"]
    #[inline(always)]
    pub fn carddetpinlvl(&self) -> CarddetpinlvlR {
        CarddetpinlvlR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Write Protect Switch Pin Level"]
    #[inline(always)]
    pub fn wrprotswpinlvl(&self) -> WrprotswpinlvlR {
        WrprotswpinlvlR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - DAT\\[3:0\\] Line Signal Level"]
    #[inline(always)]
    pub fn dat3to0siglvl(&self) -> Dat3to0siglvlR {
        Dat3to0siglvlR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Command Line Signal Level"]
    #[inline(always)]
    pub fn cmdsiglvl(&self) -> CmdsiglvlR {
        CmdsiglvlR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:28 - DAT\\[7:4\\] Line Signal Level"]
    #[inline(always)]
    pub fn dat7to4siglvl(&self) -> Dat7to4siglvlR {
        Dat7to4siglvlR::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
#[doc = "Present State Register\n\nYou can [`read`](crate::Reg::read) this register and get [`prsstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrsstatSpec;
impl crate::RegisterSpec for PrsstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prsstat::R`](R) reader structure"]
impl crate::Readable for PrsstatSpec {}
#[doc = "`reset()` method sets PRSSTAT to value 0"]
impl crate::Resettable for PrsstatSpec {}
