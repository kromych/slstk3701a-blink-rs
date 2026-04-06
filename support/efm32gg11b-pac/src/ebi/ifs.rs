#[doc = "Register `IFS` writer"]
pub type W = crate::W<IfsSpec>;
#[doc = "Field `VSYNC` writer - Vertical Sync Interrupt Flag Set"]
pub type VsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSYNC` writer - Horizontal Sync Interrupt Flag Set"]
pub type HsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBPORCH` writer - Vertical Back Porch Interrupt Flag Set"]
pub type VbporchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VFPORCH` writer - Vertical Front Porch Interrupt Flag Set"]
pub type VfporchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDEMPTY` writer - Direct Drive Data Empty Interrupt Flag Set"]
pub type DdemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDJIT` writer - Direct Drive Jitter Interrupt Flag Set"]
pub type DdjitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFTPIXEL0EMPTY` writer - EBI_TFTPIXEL0 Empty Interrupt Flag Set"]
pub type Tftpixel0emptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFTPIXEL1EMPTY` writer - EBI_TFTPIXEL1 Empty Interrupt Flag Set"]
pub type Tftpixel1emptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFTPIXELFULL` writer - EBI_TFTPIXEL Full Interrupt Flag Set"]
pub type TftpixelfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFTPIXELOF` writer - EBI_TFTPIXEL Overflow Interrupt Flag Set"]
pub type TftpixelofW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Vertical Sync Interrupt Flag Set"]
    #[inline(always)]
    pub fn vsync(&mut self) -> VsyncW<'_, IfsSpec> {
        VsyncW::new(self, 0)
    }
    #[doc = "Bit 1 - Horizontal Sync Interrupt Flag Set"]
    #[inline(always)]
    pub fn hsync(&mut self) -> HsyncW<'_, IfsSpec> {
        HsyncW::new(self, 1)
    }
    #[doc = "Bit 2 - Vertical Back Porch Interrupt Flag Set"]
    #[inline(always)]
    pub fn vbporch(&mut self) -> VbporchW<'_, IfsSpec> {
        VbporchW::new(self, 2)
    }
    #[doc = "Bit 3 - Vertical Front Porch Interrupt Flag Set"]
    #[inline(always)]
    pub fn vfporch(&mut self) -> VfporchW<'_, IfsSpec> {
        VfporchW::new(self, 3)
    }
    #[doc = "Bit 4 - Direct Drive Data Empty Interrupt Flag Set"]
    #[inline(always)]
    pub fn ddempty(&mut self) -> DdemptyW<'_, IfsSpec> {
        DdemptyW::new(self, 4)
    }
    #[doc = "Bit 5 - Direct Drive Jitter Interrupt Flag Set"]
    #[inline(always)]
    pub fn ddjit(&mut self) -> DdjitW<'_, IfsSpec> {
        DdjitW::new(self, 5)
    }
    #[doc = "Bit 6 - EBI_TFTPIXEL0 Empty Interrupt Flag Set"]
    #[inline(always)]
    pub fn tftpixel0empty(&mut self) -> Tftpixel0emptyW<'_, IfsSpec> {
        Tftpixel0emptyW::new(self, 6)
    }
    #[doc = "Bit 7 - EBI_TFTPIXEL1 Empty Interrupt Flag Set"]
    #[inline(always)]
    pub fn tftpixel1empty(&mut self) -> Tftpixel1emptyW<'_, IfsSpec> {
        Tftpixel1emptyW::new(self, 7)
    }
    #[doc = "Bit 8 - EBI_TFTPIXEL Full Interrupt Flag Set"]
    #[inline(always)]
    pub fn tftpixelfull(&mut self) -> TftpixelfullW<'_, IfsSpec> {
        TftpixelfullW::new(self, 8)
    }
    #[doc = "Bit 9 - EBI_TFTPIXEL Overflow Interrupt Flag Set"]
    #[inline(always)]
    pub fn tftpixelof(&mut self) -> TftpixelofW<'_, IfsSpec> {
        TftpixelofW::new(self, 9)
    }
}
#[doc = "Interrupt Flag Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfsSpec;
impl crate::RegisterSpec for IfsSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifs::W`](W) writer structure"]
impl crate::Writable for IfsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IfsSpec {}
