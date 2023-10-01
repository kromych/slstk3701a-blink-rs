#[doc = "Register `IF` reader"]
pub type R = crate::R<IF_SPEC>;
#[doc = "Field `VSYNC` reader - Vertical Sync Interrupt Flag"]
pub type VSYNC_R = crate::BitReader;
#[doc = "Field `HSYNC` reader - Horizontal Sync Interrupt Flag"]
pub type HSYNC_R = crate::BitReader;
#[doc = "Field `VBPORCH` reader - Vertical Back Porch Interrupt Flag"]
pub type VBPORCH_R = crate::BitReader;
#[doc = "Field `VFPORCH` reader - Vertical Front Porch Interrupt Flag"]
pub type VFPORCH_R = crate::BitReader;
#[doc = "Field `DDEMPTY` reader - Direct Drive Data Empty Interrupt Flag"]
pub type DDEMPTY_R = crate::BitReader;
#[doc = "Field `DDJIT` reader - Direct Drive Jitter Interrupt Flag"]
pub type DDJIT_R = crate::BitReader;
#[doc = "Field `TFTPIXEL0EMPTY` reader - EBI_TFTPIXEL0 is Empty Interrupt Flag"]
pub type TFTPIXEL0EMPTY_R = crate::BitReader;
#[doc = "Field `TFTPIXEL1EMPTY` reader - EBI_TFTPIXEL1 is Empty Interrupt Flag"]
pub type TFTPIXEL1EMPTY_R = crate::BitReader;
#[doc = "Field `TFTPIXELFULL` reader - EBI_TFTPIXEL is Full Interrupt Flag"]
pub type TFTPIXELFULL_R = crate::BitReader;
#[doc = "Field `TFTPIXELOF` reader - EBI_TFTPIXEL Register Overflow Interrupt Flag"]
pub type TFTPIXELOF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Vertical Sync Interrupt Flag"]
    #[inline(always)]
    pub fn vsync(&self) -> VSYNC_R {
        VSYNC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Horizontal Sync Interrupt Flag"]
    #[inline(always)]
    pub fn hsync(&self) -> HSYNC_R {
        HSYNC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Vertical Back Porch Interrupt Flag"]
    #[inline(always)]
    pub fn vbporch(&self) -> VBPORCH_R {
        VBPORCH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Vertical Front Porch Interrupt Flag"]
    #[inline(always)]
    pub fn vfporch(&self) -> VFPORCH_R {
        VFPORCH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Direct Drive Data Empty Interrupt Flag"]
    #[inline(always)]
    pub fn ddempty(&self) -> DDEMPTY_R {
        DDEMPTY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Direct Drive Jitter Interrupt Flag"]
    #[inline(always)]
    pub fn ddjit(&self) -> DDJIT_R {
        DDJIT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EBI_TFTPIXEL0 is Empty Interrupt Flag"]
    #[inline(always)]
    pub fn tftpixel0empty(&self) -> TFTPIXEL0EMPTY_R {
        TFTPIXEL0EMPTY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EBI_TFTPIXEL1 is Empty Interrupt Flag"]
    #[inline(always)]
    pub fn tftpixel1empty(&self) -> TFTPIXEL1EMPTY_R {
        TFTPIXEL1EMPTY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - EBI_TFTPIXEL is Full Interrupt Flag"]
    #[inline(always)]
    pub fn tftpixelfull(&self) -> TFTPIXELFULL_R {
        TFTPIXELFULL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EBI_TFTPIXEL Register Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn tftpixelof(&self) -> TFTPIXELOF_R {
        TFTPIXELOF_R::new(((self.bits >> 9) & 1) != 0)
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
    const RESET_VALUE: Self::Ux = 0;
}
