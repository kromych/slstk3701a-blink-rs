#[doc = "Register `IF` reader"]
pub type R = crate::R<IF_SPEC>;
#[doc = "Field `VMONAVDDFALL` reader - VMON AVDD Channel Fall"]
pub type VMONAVDDFALL_R = crate::BitReader;
#[doc = "Field `VMONAVDDRISE` reader - VMON AVDD Channel Rise"]
pub type VMONAVDDRISE_R = crate::BitReader;
#[doc = "Field `VMONALTAVDDFALL` reader - Alternate VMON AVDD Channel Fall"]
pub type VMONALTAVDDFALL_R = crate::BitReader;
#[doc = "Field `VMONALTAVDDRISE` reader - Alternate VMON AVDD Channel Rise"]
pub type VMONALTAVDDRISE_R = crate::BitReader;
#[doc = "Field `VMONDVDDFALL` reader - VMON DVDD Channel Fall"]
pub type VMONDVDDFALL_R = crate::BitReader;
#[doc = "Field `VMONDVDDRISE` reader - VMON DVDD Channel Rise"]
pub type VMONDVDDRISE_R = crate::BitReader;
#[doc = "Field `VMONIO0FALL` reader - VMON IOVDD0 Channel Fall"]
pub type VMONIO0FALL_R = crate::BitReader;
#[doc = "Field `VMONIO0RISE` reader - VMON IOVDD0 Channel Rise"]
pub type VMONIO0RISE_R = crate::BitReader;
#[doc = "Field `VMONIO1FALL` reader - VMON IOVDD1 Channel Fall"]
pub type VMONIO1FALL_R = crate::BitReader;
#[doc = "Field `VMONIO1RISE` reader - VMON IOVDD1 Channel Rise"]
pub type VMONIO1RISE_R = crate::BitReader;
#[doc = "Field `R5VREADY` reader - 5V Regulator is Ready to Use"]
pub type R5VREADY_R = crate::BitReader;
#[doc = "Field `VMONBUVDDFALL` reader - VMON BACKUP Channel Fall"]
pub type VMONBUVDDFALL_R = crate::BitReader;
#[doc = "Field `VMONBUVDDRISE` reader - VMON BUVDD Channel Rise"]
pub type VMONBUVDDRISE_R = crate::BitReader;
#[doc = "Field `PFETOVERCURRENTLIMIT` reader - PFET Current Limit Hit"]
pub type PFETOVERCURRENTLIMIT_R = crate::BitReader;
#[doc = "Field `NFETOVERCURRENTLIMIT` reader - NFET Current Limit Hit"]
pub type NFETOVERCURRENTLIMIT_R = crate::BitReader;
#[doc = "Field `DCDCLPRUNNING` reader - LP Mode is Running"]
pub type DCDCLPRUNNING_R = crate::BitReader;
#[doc = "Field `DCDCLNRUNNING` reader - LN Mode is Running"]
pub type DCDCLNRUNNING_R = crate::BitReader;
#[doc = "Field `DCDCINBYPASS` reader - DCDC is in Bypass"]
pub type DCDCINBYPASS_R = crate::BitReader;
#[doc = "Field `BURDY` reader - Backup Functionality Ready Interrupt Flag"]
pub type BURDY_R = crate::BitReader;
#[doc = "Field `R5VVSINT` reader - 5V Regulator Voltage Update Done"]
pub type R5VVSINT_R = crate::BitReader;
#[doc = "Field `EM23WAKEUP` reader - Wakeup IRQ From EM2 and EM3"]
pub type EM23WAKEUP_R = crate::BitReader;
#[doc = "Field `VSCALEDONE` reader - Voltage Scale Steps Done IRQ"]
pub type VSCALEDONE_R = crate::BitReader;
#[doc = "Field `TEMP` reader - New Temperature Measurement Valid"]
pub type TEMP_R = crate::BitReader;
#[doc = "Field `TEMPLOW` reader - Temperature Low Limit Reached"]
pub type TEMPLOW_R = crate::BitReader;
#[doc = "Field `TEMPHIGH` reader - Temperature High Limit Reached"]
pub type TEMPHIGH_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - VMON AVDD Channel Fall"]
    #[inline(always)]
    pub fn vmonavddfall(&self) -> VMONAVDDFALL_R {
        VMONAVDDFALL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VMON AVDD Channel Rise"]
    #[inline(always)]
    pub fn vmonavddrise(&self) -> VMONAVDDRISE_R {
        VMONAVDDRISE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Alternate VMON AVDD Channel Fall"]
    #[inline(always)]
    pub fn vmonaltavddfall(&self) -> VMONALTAVDDFALL_R {
        VMONALTAVDDFALL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Alternate VMON AVDD Channel Rise"]
    #[inline(always)]
    pub fn vmonaltavddrise(&self) -> VMONALTAVDDRISE_R {
        VMONALTAVDDRISE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VMON DVDD Channel Fall"]
    #[inline(always)]
    pub fn vmondvddfall(&self) -> VMONDVDDFALL_R {
        VMONDVDDFALL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - VMON DVDD Channel Rise"]
    #[inline(always)]
    pub fn vmondvddrise(&self) -> VMONDVDDRISE_R {
        VMONDVDDRISE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - VMON IOVDD0 Channel Fall"]
    #[inline(always)]
    pub fn vmonio0fall(&self) -> VMONIO0FALL_R {
        VMONIO0FALL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - VMON IOVDD0 Channel Rise"]
    #[inline(always)]
    pub fn vmonio0rise(&self) -> VMONIO0RISE_R {
        VMONIO0RISE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - VMON IOVDD1 Channel Fall"]
    #[inline(always)]
    pub fn vmonio1fall(&self) -> VMONIO1FALL_R {
        VMONIO1FALL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - VMON IOVDD1 Channel Rise"]
    #[inline(always)]
    pub fn vmonio1rise(&self) -> VMONIO1RISE_R {
        VMONIO1RISE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 5V Regulator is Ready to Use"]
    #[inline(always)]
    pub fn r5vready(&self) -> R5VREADY_R {
        R5VREADY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - VMON BACKUP Channel Fall"]
    #[inline(always)]
    pub fn vmonbuvddfall(&self) -> VMONBUVDDFALL_R {
        VMONBUVDDFALL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - VMON BUVDD Channel Rise"]
    #[inline(always)]
    pub fn vmonbuvddrise(&self) -> VMONBUVDDRISE_R {
        VMONBUVDDRISE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - PFET Current Limit Hit"]
    #[inline(always)]
    pub fn pfetovercurrentlimit(&self) -> PFETOVERCURRENTLIMIT_R {
        PFETOVERCURRENTLIMIT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - NFET Current Limit Hit"]
    #[inline(always)]
    pub fn nfetovercurrentlimit(&self) -> NFETOVERCURRENTLIMIT_R {
        NFETOVERCURRENTLIMIT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - LP Mode is Running"]
    #[inline(always)]
    pub fn dcdclprunning(&self) -> DCDCLPRUNNING_R {
        DCDCLPRUNNING_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - LN Mode is Running"]
    #[inline(always)]
    pub fn dcdclnrunning(&self) -> DCDCLNRUNNING_R {
        DCDCLNRUNNING_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DCDC is in Bypass"]
    #[inline(always)]
    pub fn dcdcinbypass(&self) -> DCDCINBYPASS_R {
        DCDCINBYPASS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Backup Functionality Ready Interrupt Flag"]
    #[inline(always)]
    pub fn burdy(&self) -> BURDY_R {
        BURDY_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 5V Regulator Voltage Update Done"]
    #[inline(always)]
    pub fn r5vvsint(&self) -> R5VVSINT_R {
        R5VVSINT_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Wakeup IRQ From EM2 and EM3"]
    #[inline(always)]
    pub fn em23wakeup(&self) -> EM23WAKEUP_R {
        EM23WAKEUP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Voltage Scale Steps Done IRQ"]
    #[inline(always)]
    pub fn vscaledone(&self) -> VSCALEDONE_R {
        VSCALEDONE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 29 - New Temperature Measurement Valid"]
    #[inline(always)]
    pub fn temp(&self) -> TEMP_R {
        TEMP_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Temperature Low Limit Reached"]
    #[inline(always)]
    pub fn templow(&self) -> TEMPLOW_R {
        TEMPLOW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Temperature High Limit Reached"]
    #[inline(always)]
    pub fn temphigh(&self) -> TEMPHIGH_R {
        TEMPHIGH_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if_::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IF_SPEC;
impl crate::RegisterSpec for IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IF_SPEC {}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IF_SPEC {
    const RESET_VALUE: u32 = 0;
}
