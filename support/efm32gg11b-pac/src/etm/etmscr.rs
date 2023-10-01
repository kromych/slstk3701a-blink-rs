#[doc = "Register `ETMSCR` reader"]
pub type R = crate::R<ETMSCR_SPEC>;
#[doc = "Field `MAXPORTSIZE` reader - Maximum Port Size"]
pub type MAXPORTSIZE_R = crate::FieldReader;
#[doc = "Field `FIFOFULL` reader - FIFO FULL Supported"]
pub type FIFOFULL_R = crate::BitReader;
#[doc = "Field `MAXPORTSIZE3` reader - Max Port Size\\[3\\]"]
pub type MAXPORTSIZE3_R = crate::BitReader;
#[doc = "Field `PORTSIZE` reader - Port Size Supported"]
pub type PORTSIZE_R = crate::BitReader;
#[doc = "Field `PORTMODE` reader - Port Mode Supported"]
pub type PORTMODE_R = crate::BitReader;
#[doc = "Field `PROCNUM` reader - Number of Supported Processros"]
pub type PROCNUM_R = crate::FieldReader;
#[doc = "Field `NOFETCHCOMP` reader - No Fetch Comparison"]
pub type NOFETCHCOMP_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Maximum Port Size"]
    #[inline(always)]
    pub fn maxportsize(&self) -> MAXPORTSIZE_R {
        MAXPORTSIZE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - FIFO FULL Supported"]
    #[inline(always)]
    pub fn fifofull(&self) -> FIFOFULL_R {
        FIFOFULL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Max Port Size\\[3\\]"]
    #[inline(always)]
    pub fn maxportsize3(&self) -> MAXPORTSIZE3_R {
        MAXPORTSIZE3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port Size Supported"]
    #[inline(always)]
    pub fn portsize(&self) -> PORTSIZE_R {
        PORTSIZE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port Mode Supported"]
    #[inline(always)]
    pub fn portmode(&self) -> PORTMODE_R {
        PORTMODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Number of Supported Processros"]
    #[inline(always)]
    pub fn procnum(&self) -> PROCNUM_R {
        PROCNUM_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 17 - No Fetch Comparison"]
    #[inline(always)]
    pub fn nofetchcomp(&self) -> NOFETCHCOMP_R {
        NOFETCHCOMP_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "ETM System Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmscr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETMSCR_SPEC;
impl crate::RegisterSpec for ETMSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmscr::R`](R) reader structure"]
impl crate::Readable for ETMSCR_SPEC {}
#[doc = "`reset()` method sets ETMSCR to value 0x0002_0d09"]
impl crate::Resettable for ETMSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0d09;
}