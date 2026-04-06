#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Field `VSYNC` reader - Vertical Sync Interrupt Flag"]
pub type VsyncR = crate::BitReader;
#[doc = "Field `HSYNC` reader - Horizontal Sync Interrupt Flag"]
pub type HsyncR = crate::BitReader;
#[doc = "Field `VBPORCH` reader - Vertical Back Porch Interrupt Flag"]
pub type VbporchR = crate::BitReader;
#[doc = "Field `VFPORCH` reader - Vertical Front Porch Interrupt Flag"]
pub type VfporchR = crate::BitReader;
#[doc = "Field `DDEMPTY` reader - Direct Drive Data Empty Interrupt Flag"]
pub type DdemptyR = crate::BitReader;
#[doc = "Field `DDJIT` reader - Direct Drive Jitter Interrupt Flag"]
pub type DdjitR = crate::BitReader;
#[doc = "Field `TFTPIXEL0EMPTY` reader - EBI_TFTPIXEL0 is Empty Interrupt Flag"]
pub type Tftpixel0emptyR = crate::BitReader;
#[doc = "Field `TFTPIXEL1EMPTY` reader - EBI_TFTPIXEL1 is Empty Interrupt Flag"]
pub type Tftpixel1emptyR = crate::BitReader;
#[doc = "Field `TFTPIXELFULL` reader - EBI_TFTPIXEL is Full Interrupt Flag"]
pub type TftpixelfullR = crate::BitReader;
#[doc = "Field `TFTPIXELOF` reader - EBI_TFTPIXEL Register Overflow Interrupt Flag"]
pub type TftpixelofR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Vertical Sync Interrupt Flag"]
    #[inline(always)]
    pub fn vsync(&self) -> VsyncR {
        VsyncR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Horizontal Sync Interrupt Flag"]
    #[inline(always)]
    pub fn hsync(&self) -> HsyncR {
        HsyncR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Vertical Back Porch Interrupt Flag"]
    #[inline(always)]
    pub fn vbporch(&self) -> VbporchR {
        VbporchR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Vertical Front Porch Interrupt Flag"]
    #[inline(always)]
    pub fn vfporch(&self) -> VfporchR {
        VfporchR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Direct Drive Data Empty Interrupt Flag"]
    #[inline(always)]
    pub fn ddempty(&self) -> DdemptyR {
        DdemptyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Direct Drive Jitter Interrupt Flag"]
    #[inline(always)]
    pub fn ddjit(&self) -> DdjitR {
        DdjitR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EBI_TFTPIXEL0 is Empty Interrupt Flag"]
    #[inline(always)]
    pub fn tftpixel0empty(&self) -> Tftpixel0emptyR {
        Tftpixel0emptyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EBI_TFTPIXEL1 is Empty Interrupt Flag"]
    #[inline(always)]
    pub fn tftpixel1empty(&self) -> Tftpixel1emptyR {
        Tftpixel1emptyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - EBI_TFTPIXEL is Full Interrupt Flag"]
    #[inline(always)]
    pub fn tftpixelfull(&self) -> TftpixelfullR {
        TftpixelfullR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EBI_TFTPIXEL Register Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn tftpixelof(&self) -> TftpixelofR {
        TftpixelofR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfSpec;
impl crate::RegisterSpec for IfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IfSpec {}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IfSpec {}
