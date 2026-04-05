#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `VSYNC` reader - Vertical Sync Interrupt Enable"]
pub type VsyncR = crate::BitReader;
#[doc = "Field `VSYNC` writer - Vertical Sync Interrupt Enable"]
pub type VsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSYNC` reader - Horizontal Sync Interrupt Enable"]
pub type HsyncR = crate::BitReader;
#[doc = "Field `HSYNC` writer - Horizontal Sync Interrupt Enable"]
pub type HsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBPORCH` reader - Vertical Back Porch Interrupt Enable"]
pub type VbporchR = crate::BitReader;
#[doc = "Field `VBPORCH` writer - Vertical Back Porch Interrupt Enable"]
pub type VbporchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VFPORCH` reader - Vertical Front Porch Interrupt Enable"]
pub type VfporchR = crate::BitReader;
#[doc = "Field `VFPORCH` writer - Vertical Front Porch Interrupt Enable"]
pub type VfporchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDEMPTY` reader - Direct Drive Data Empty Interrupt Enable"]
pub type DdemptyR = crate::BitReader;
#[doc = "Field `DDEMPTY` writer - Direct Drive Data Empty Interrupt Enable"]
pub type DdemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDJIT` reader - Direct Drive Jitter Interrupt Enable"]
pub type DdjitR = crate::BitReader;
#[doc = "Field `DDJIT` writer - Direct Drive Jitter Interrupt Enable"]
pub type DdjitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFTPIXEL0EMPTY` reader - EBI_TFTPIXEL0 Empty Interrupt Enable"]
pub type Tftpixel0emptyR = crate::BitReader;
#[doc = "Field `TFTPIXEL0EMPTY` writer - EBI_TFTPIXEL0 Empty Interrupt Enable"]
pub type Tftpixel0emptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFTPIXEL1EMPTY` reader - EBI_TFTPIXEL1 Empty Interrupt Enable"]
pub type Tftpixel1emptyR = crate::BitReader;
#[doc = "Field `TFTPIXEL1EMPTY` writer - EBI_TFTPIXEL1 Empty Interrupt Enable"]
pub type Tftpixel1emptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFTPIXELFULL` reader - EBI_TFTPIXEL Full Interrupt Enable"]
pub type TftpixelfullR = crate::BitReader;
#[doc = "Field `TFTPIXELFULL` writer - EBI_TFTPIXEL Full Interrupt Enable"]
pub type TftpixelfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFTPIXELOF` reader - EBI_TFTPIXEL Overflow Interrupt Enable"]
pub type TftpixelofR = crate::BitReader;
#[doc = "Field `TFTPIXELOF` writer - EBI_TFTPIXEL Overflow Interrupt Enable"]
pub type TftpixelofW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Vertical Sync Interrupt Enable"]
    #[inline(always)]
    pub fn vsync(&self) -> VsyncR {
        VsyncR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Horizontal Sync Interrupt Enable"]
    #[inline(always)]
    pub fn hsync(&self) -> HsyncR {
        HsyncR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Vertical Back Porch Interrupt Enable"]
    #[inline(always)]
    pub fn vbporch(&self) -> VbporchR {
        VbporchR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Vertical Front Porch Interrupt Enable"]
    #[inline(always)]
    pub fn vfporch(&self) -> VfporchR {
        VfporchR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Direct Drive Data Empty Interrupt Enable"]
    #[inline(always)]
    pub fn ddempty(&self) -> DdemptyR {
        DdemptyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Direct Drive Jitter Interrupt Enable"]
    #[inline(always)]
    pub fn ddjit(&self) -> DdjitR {
        DdjitR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EBI_TFTPIXEL0 Empty Interrupt Enable"]
    #[inline(always)]
    pub fn tftpixel0empty(&self) -> Tftpixel0emptyR {
        Tftpixel0emptyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EBI_TFTPIXEL1 Empty Interrupt Enable"]
    #[inline(always)]
    pub fn tftpixel1empty(&self) -> Tftpixel1emptyR {
        Tftpixel1emptyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - EBI_TFTPIXEL Full Interrupt Enable"]
    #[inline(always)]
    pub fn tftpixelfull(&self) -> TftpixelfullR {
        TftpixelfullR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EBI_TFTPIXEL Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn tftpixelof(&self) -> TftpixelofR {
        TftpixelofR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Vertical Sync Interrupt Enable"]
    #[inline(always)]
    pub fn vsync(&mut self) -> VsyncW<'_, IenSpec> {
        VsyncW::new(self, 0)
    }
    #[doc = "Bit 1 - Horizontal Sync Interrupt Enable"]
    #[inline(always)]
    pub fn hsync(&mut self) -> HsyncW<'_, IenSpec> {
        HsyncW::new(self, 1)
    }
    #[doc = "Bit 2 - Vertical Back Porch Interrupt Enable"]
    #[inline(always)]
    pub fn vbporch(&mut self) -> VbporchW<'_, IenSpec> {
        VbporchW::new(self, 2)
    }
    #[doc = "Bit 3 - Vertical Front Porch Interrupt Enable"]
    #[inline(always)]
    pub fn vfporch(&mut self) -> VfporchW<'_, IenSpec> {
        VfporchW::new(self, 3)
    }
    #[doc = "Bit 4 - Direct Drive Data Empty Interrupt Enable"]
    #[inline(always)]
    pub fn ddempty(&mut self) -> DdemptyW<'_, IenSpec> {
        DdemptyW::new(self, 4)
    }
    #[doc = "Bit 5 - Direct Drive Jitter Interrupt Enable"]
    #[inline(always)]
    pub fn ddjit(&mut self) -> DdjitW<'_, IenSpec> {
        DdjitW::new(self, 5)
    }
    #[doc = "Bit 6 - EBI_TFTPIXEL0 Empty Interrupt Enable"]
    #[inline(always)]
    pub fn tftpixel0empty(&mut self) -> Tftpixel0emptyW<'_, IenSpec> {
        Tftpixel0emptyW::new(self, 6)
    }
    #[doc = "Bit 7 - EBI_TFTPIXEL1 Empty Interrupt Enable"]
    #[inline(always)]
    pub fn tftpixel1empty(&mut self) -> Tftpixel1emptyW<'_, IenSpec> {
        Tftpixel1emptyW::new(self, 7)
    }
    #[doc = "Bit 8 - EBI_TFTPIXEL Full Interrupt Enable"]
    #[inline(always)]
    pub fn tftpixelfull(&mut self) -> TftpixelfullW<'_, IenSpec> {
        TftpixelfullW::new(self, 8)
    }
    #[doc = "Bit 9 - EBI_TFTPIXEL Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn tftpixelof(&mut self) -> TftpixelofW<'_, IenSpec> {
        TftpixelofW::new(self, 9)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IenSpec;
impl crate::RegisterSpec for IenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IenSpec {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IenSpec {}
