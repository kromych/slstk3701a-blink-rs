#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `VMONRDY` reader - VMON Ready"]
pub type VmonrdyR = crate::BitReader;
#[doc = "Field `VMONAVDD` reader - VMON AVDD Channel"]
pub type VmonavddR = crate::BitReader;
#[doc = "Field `VMONALTAVDD` reader - Alternate VMON AVDD Channel"]
pub type VmonaltavddR = crate::BitReader;
#[doc = "Field `VMONDVDD` reader - VMON DVDD Channel"]
pub type VmondvddR = crate::BitReader;
#[doc = "Field `VMONIO0` reader - VMON IOVDD0 Channel"]
pub type Vmonio0R = crate::BitReader;
#[doc = "Field `VMONIO1` reader - VMON IOVDD1 Channel"]
pub type Vmonio1R = crate::BitReader;
#[doc = "Field `VMONBUVDD` reader - VMON BUVDD Channel"]
pub type VmonbuvddR = crate::BitReader;
#[doc = "Field `BURDY` reader - Backup Mode Ready"]
pub type BurdyR = crate::BitReader;
#[doc = "Current Voltage Scale Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vscale {
    #[doc = "0: Voltage Scale Level 2"]
    Vscale2 = 0,
    #[doc = "2: Voltage Scale Level 0"]
    Vscale0 = 2,
    #[doc = "3: RESV"]
    Resv = 3,
}
impl From<Vscale> for u8 {
    #[inline(always)]
    fn from(variant: Vscale) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vscale {
    type Ux = u8;
}
impl crate::IsEnum for Vscale {}
#[doc = "Field `VSCALE` reader - Current Voltage Scale Value"]
pub type VscaleR = crate::FieldReader<Vscale>;
impl VscaleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Vscale> {
        match self.bits {
            0 => Some(Vscale::Vscale2),
            2 => Some(Vscale::Vscale0),
            3 => Some(Vscale::Resv),
            _ => None,
        }
    }
    #[doc = "Voltage Scale Level 2"]
    #[inline(always)]
    pub fn is_vscale2(&self) -> bool {
        *self == Vscale::Vscale2
    }
    #[doc = "Voltage Scale Level 0"]
    #[inline(always)]
    pub fn is_vscale0(&self) -> bool {
        *self == Vscale::Vscale0
    }
    #[doc = "RESV"]
    #[inline(always)]
    pub fn is_resv(&self) -> bool {
        *self == Vscale::Resv
    }
}
#[doc = "Field `VSCALEBUSY` reader - System is Busy Scaling Voltage"]
pub type VscalebusyR = crate::BitReader;
#[doc = "Field `EM4IORET` reader - IO Retention Status"]
pub type Em4ioretR = crate::BitReader;
#[doc = "Field `TEMPACTIVE` reader - Temperature Measurement Active"]
pub type TempactiveR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - VMON Ready"]
    #[inline(always)]
    pub fn vmonrdy(&self) -> VmonrdyR {
        VmonrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VMON AVDD Channel"]
    #[inline(always)]
    pub fn vmonavdd(&self) -> VmonavddR {
        VmonavddR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Alternate VMON AVDD Channel"]
    #[inline(always)]
    pub fn vmonaltavdd(&self) -> VmonaltavddR {
        VmonaltavddR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VMON DVDD Channel"]
    #[inline(always)]
    pub fn vmondvdd(&self) -> VmondvddR {
        VmondvddR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VMON IOVDD0 Channel"]
    #[inline(always)]
    pub fn vmonio0(&self) -> Vmonio0R {
        Vmonio0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - VMON IOVDD1 Channel"]
    #[inline(always)]
    pub fn vmonio1(&self) -> Vmonio1R {
        Vmonio1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - VMON BUVDD Channel"]
    #[inline(always)]
    pub fn vmonbuvdd(&self) -> VmonbuvddR {
        VmonbuvddR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - Backup Mode Ready"]
    #[inline(always)]
    pub fn burdy(&self) -> BurdyR {
        BurdyR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Current Voltage Scale Value"]
    #[inline(always)]
    pub fn vscale(&self) -> VscaleR {
        VscaleR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - System is Busy Scaling Voltage"]
    #[inline(always)]
    pub fn vscalebusy(&self) -> VscalebusyR {
        VscalebusyR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - IO Retention Status"]
    #[inline(always)]
    pub fn em4ioret(&self) -> Em4ioretR {
        Em4ioretR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 26 - Temperature Measurement Active"]
    #[inline(always)]
    pub fn tempactive(&self) -> TempactiveR {
        TempactiveR::new(((self.bits >> 26) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
