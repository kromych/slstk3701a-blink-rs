#[doc = "Register `ETMSCR` reader"]
pub type R = crate::R<EtmscrSpec>;
#[doc = "Field `MAXPORTSIZE` reader - Maximum Port Size"]
pub type MaxportsizeR = crate::FieldReader;
#[doc = "Field `FIFOFULL` reader - FIFO FULL Supported"]
pub type FifofullR = crate::BitReader;
#[doc = "Field `MAXPORTSIZE3` reader - Max Port Size\\[3\\]"]
pub type Maxportsize3R = crate::BitReader;
#[doc = "Field `PORTSIZE` reader - Port Size Supported"]
pub type PortsizeR = crate::BitReader;
#[doc = "Field `PORTMODE` reader - Port Mode Supported"]
pub type PortmodeR = crate::BitReader;
#[doc = "Field `PROCNUM` reader - Number of Supported Processros"]
pub type ProcnumR = crate::FieldReader;
#[doc = "Field `NOFETCHCOMP` reader - No Fetch Comparison"]
pub type NofetchcompR = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Maximum Port Size"]
    #[inline(always)]
    pub fn maxportsize(&self) -> MaxportsizeR {
        MaxportsizeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - FIFO FULL Supported"]
    #[inline(always)]
    pub fn fifofull(&self) -> FifofullR {
        FifofullR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Max Port Size\\[3\\]"]
    #[inline(always)]
    pub fn maxportsize3(&self) -> Maxportsize3R {
        Maxportsize3R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port Size Supported"]
    #[inline(always)]
    pub fn portsize(&self) -> PortsizeR {
        PortsizeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port Mode Supported"]
    #[inline(always)]
    pub fn portmode(&self) -> PortmodeR {
        PortmodeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Number of Supported Processros"]
    #[inline(always)]
    pub fn procnum(&self) -> ProcnumR {
        ProcnumR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 17 - No Fetch Comparison"]
    #[inline(always)]
    pub fn nofetchcomp(&self) -> NofetchcompR {
        NofetchcompR::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "ETM System Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmscr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtmscrSpec;
impl crate::RegisterSpec for EtmscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmscr::R`](R) reader structure"]
impl crate::Readable for EtmscrSpec {}
#[doc = "`reset()` method sets ETMSCR to value 0x0002_0d09"]
impl crate::Resettable for EtmscrSpec {
    const RESET_VALUE: u32 = 0x0002_0d09;
}
