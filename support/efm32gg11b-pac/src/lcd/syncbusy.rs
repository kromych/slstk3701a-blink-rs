#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SyncbusySpec>;
#[doc = "Field `CTRL` reader - CTRL Register Busy"]
pub type CtrlR = crate::BitReader;
#[doc = "Field `BACTRL` reader - BACTRL Register Busy"]
pub type BactrlR = crate::BitReader;
#[doc = "Field `AREGA` reader - AREGA Register Busy"]
pub type AregaR = crate::BitReader;
#[doc = "Field `AREGB` reader - AREGB Register Busy"]
pub type AregbR = crate::BitReader;
#[doc = "Field `SEGD0L` reader - SEGD0L Register Busy"]
pub type Segd0lR = crate::BitReader;
#[doc = "Field `SEGD1L` reader - SEGD1L Register Busy"]
pub type Segd1lR = crate::BitReader;
#[doc = "Field `SEGD2L` reader - SEGD2L Register Busy"]
pub type Segd2lR = crate::BitReader;
#[doc = "Field `SEGD3L` reader - SEGD3L Register Busy"]
pub type Segd3lR = crate::BitReader;
#[doc = "Field `SEGD0H` reader - SEGD0H Register Busy"]
pub type Segd0hR = crate::BitReader;
#[doc = "Field `SEGD1H` reader - SEGD1H Register Busy"]
pub type Segd1hR = crate::BitReader;
#[doc = "Field `SEGD2H` reader - SEGD2H Register Busy"]
pub type Segd2hR = crate::BitReader;
#[doc = "Field `SEGD3H` reader - SEGD3H Register Busy"]
pub type Segd3hR = crate::BitReader;
#[doc = "Field `SEGD4L` reader - SEGD4L Register Busy"]
pub type Segd4lR = crate::BitReader;
#[doc = "Field `SEGD5L` reader - SEGD5L Register Busy"]
pub type Segd5lR = crate::BitReader;
#[doc = "Field `SEGD6L` reader - SEGD6L Register Busy"]
pub type Segd6lR = crate::BitReader;
#[doc = "Field `SEGD7L` reader - SEGD7L Register Busy"]
pub type Segd7lR = crate::BitReader;
#[doc = "Field `SEGD4H` reader - SEGD4H Register Busy"]
pub type Segd4hR = crate::BitReader;
#[doc = "Field `SEGD5H` reader - SEGD5H Register Busy"]
pub type Segd5hR = crate::BitReader;
#[doc = "Field `SEGD6H` reader - SEGD6H Register Busy"]
pub type Segd6hR = crate::BitReader;
#[doc = "Field `SEGD7H` reader - SEGD7H Register Busy"]
pub type Segd7hR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CTRL Register Busy"]
    #[inline(always)]
    pub fn ctrl(&self) -> CtrlR {
        CtrlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BACTRL Register Busy"]
    #[inline(always)]
    pub fn bactrl(&self) -> BactrlR {
        BactrlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AREGA Register Busy"]
    #[inline(always)]
    pub fn arega(&self) -> AregaR {
        AregaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AREGB Register Busy"]
    #[inline(always)]
    pub fn aregb(&self) -> AregbR {
        AregbR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SEGD0L Register Busy"]
    #[inline(always)]
    pub fn segd0l(&self) -> Segd0lR {
        Segd0lR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SEGD1L Register Busy"]
    #[inline(always)]
    pub fn segd1l(&self) -> Segd1lR {
        Segd1lR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SEGD2L Register Busy"]
    #[inline(always)]
    pub fn segd2l(&self) -> Segd2lR {
        Segd2lR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SEGD3L Register Busy"]
    #[inline(always)]
    pub fn segd3l(&self) -> Segd3lR {
        Segd3lR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SEGD0H Register Busy"]
    #[inline(always)]
    pub fn segd0h(&self) -> Segd0hR {
        Segd0hR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SEGD1H Register Busy"]
    #[inline(always)]
    pub fn segd1h(&self) -> Segd1hR {
        Segd1hR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SEGD2H Register Busy"]
    #[inline(always)]
    pub fn segd2h(&self) -> Segd2hR {
        Segd2hR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SEGD3H Register Busy"]
    #[inline(always)]
    pub fn segd3h(&self) -> Segd3hR {
        Segd3hR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SEGD4L Register Busy"]
    #[inline(always)]
    pub fn segd4l(&self) -> Segd4lR {
        Segd4lR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SEGD5L Register Busy"]
    #[inline(always)]
    pub fn segd5l(&self) -> Segd5lR {
        Segd5lR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SEGD6L Register Busy"]
    #[inline(always)]
    pub fn segd6l(&self) -> Segd6lR {
        Segd6lR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SEGD7L Register Busy"]
    #[inline(always)]
    pub fn segd7l(&self) -> Segd7lR {
        Segd7lR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SEGD4H Register Busy"]
    #[inline(always)]
    pub fn segd4h(&self) -> Segd4hR {
        Segd4hR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SEGD5H Register Busy"]
    #[inline(always)]
    pub fn segd5h(&self) -> Segd5hR {
        Segd5hR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SEGD6H Register Busy"]
    #[inline(always)]
    pub fn segd6h(&self) -> Segd6hR {
        Segd6hR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SEGD7H Register Busy"]
    #[inline(always)]
    pub fn segd7h(&self) -> Segd7hR {
        Segd7hR::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "Synchronization Busy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncbusySpec;
impl crate::RegisterSpec for SyncbusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncbusy::R`](R) reader structure"]
impl crate::Readable for SyncbusySpec {}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SyncbusySpec {}
