#[doc = "Register `IEN` reader"]
pub type R = crate::R<IEN_SPEC>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IEN_SPEC>;
#[doc = "Field `VSYNC` reader - Vertical Sync Interrupt Enable"]
pub type VSYNC_R = crate::BitReader;
#[doc = "Field `VSYNC` writer - Vertical Sync Interrupt Enable"]
pub type VSYNC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSYNC` reader - Horizontal Sync Interrupt Enable"]
pub type HSYNC_R = crate::BitReader;
#[doc = "Field `HSYNC` writer - Horizontal Sync Interrupt Enable"]
pub type HSYNC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VBPORCH` reader - Vertical Back Porch Interrupt Enable"]
pub type VBPORCH_R = crate::BitReader;
#[doc = "Field `VBPORCH` writer - Vertical Back Porch Interrupt Enable"]
pub type VBPORCH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VFPORCH` reader - Vertical Front Porch Interrupt Enable"]
pub type VFPORCH_R = crate::BitReader;
#[doc = "Field `VFPORCH` writer - Vertical Front Porch Interrupt Enable"]
pub type VFPORCH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DDEMPTY` reader - Direct Drive Data Empty Interrupt Enable"]
pub type DDEMPTY_R = crate::BitReader;
#[doc = "Field `DDEMPTY` writer - Direct Drive Data Empty Interrupt Enable"]
pub type DDEMPTY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DDJIT` reader - Direct Drive Jitter Interrupt Enable"]
pub type DDJIT_R = crate::BitReader;
#[doc = "Field `DDJIT` writer - Direct Drive Jitter Interrupt Enable"]
pub type DDJIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TFTPIXEL0EMPTY` reader - EBI_TFTPIXEL0 Empty Interrupt Enable"]
pub type TFTPIXEL0EMPTY_R = crate::BitReader;
#[doc = "Field `TFTPIXEL0EMPTY` writer - EBI_TFTPIXEL0 Empty Interrupt Enable"]
pub type TFTPIXEL0EMPTY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TFTPIXEL1EMPTY` reader - EBI_TFTPIXEL1 Empty Interrupt Enable"]
pub type TFTPIXEL1EMPTY_R = crate::BitReader;
#[doc = "Field `TFTPIXEL1EMPTY` writer - EBI_TFTPIXEL1 Empty Interrupt Enable"]
pub type TFTPIXEL1EMPTY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TFTPIXELFULL` reader - EBI_TFTPIXEL Full Interrupt Enable"]
pub type TFTPIXELFULL_R = crate::BitReader;
#[doc = "Field `TFTPIXELFULL` writer - EBI_TFTPIXEL Full Interrupt Enable"]
pub type TFTPIXELFULL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TFTPIXELOF` reader - EBI_TFTPIXEL Overflow Interrupt Enable"]
pub type TFTPIXELOF_R = crate::BitReader;
#[doc = "Field `TFTPIXELOF` writer - EBI_TFTPIXEL Overflow Interrupt Enable"]
pub type TFTPIXELOF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Vertical Sync Interrupt Enable"]
    #[inline(always)]
    pub fn vsync(&self) -> VSYNC_R {
        VSYNC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Horizontal Sync Interrupt Enable"]
    #[inline(always)]
    pub fn hsync(&self) -> HSYNC_R {
        HSYNC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Vertical Back Porch Interrupt Enable"]
    #[inline(always)]
    pub fn vbporch(&self) -> VBPORCH_R {
        VBPORCH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Vertical Front Porch Interrupt Enable"]
    #[inline(always)]
    pub fn vfporch(&self) -> VFPORCH_R {
        VFPORCH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Direct Drive Data Empty Interrupt Enable"]
    #[inline(always)]
    pub fn ddempty(&self) -> DDEMPTY_R {
        DDEMPTY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Direct Drive Jitter Interrupt Enable"]
    #[inline(always)]
    pub fn ddjit(&self) -> DDJIT_R {
        DDJIT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EBI_TFTPIXEL0 Empty Interrupt Enable"]
    #[inline(always)]
    pub fn tftpixel0empty(&self) -> TFTPIXEL0EMPTY_R {
        TFTPIXEL0EMPTY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EBI_TFTPIXEL1 Empty Interrupt Enable"]
    #[inline(always)]
    pub fn tftpixel1empty(&self) -> TFTPIXEL1EMPTY_R {
        TFTPIXEL1EMPTY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - EBI_TFTPIXEL Full Interrupt Enable"]
    #[inline(always)]
    pub fn tftpixelfull(&self) -> TFTPIXELFULL_R {
        TFTPIXELFULL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EBI_TFTPIXEL Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn tftpixelof(&self) -> TFTPIXELOF_R {
        TFTPIXELOF_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Vertical Sync Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vsync(&mut self) -> VSYNC_W<IEN_SPEC, 0> {
        VSYNC_W::new(self)
    }
    #[doc = "Bit 1 - Horizontal Sync Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsync(&mut self) -> HSYNC_W<IEN_SPEC, 1> {
        HSYNC_W::new(self)
    }
    #[doc = "Bit 2 - Vertical Back Porch Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vbporch(&mut self) -> VBPORCH_W<IEN_SPEC, 2> {
        VBPORCH_W::new(self)
    }
    #[doc = "Bit 3 - Vertical Front Porch Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vfporch(&mut self) -> VFPORCH_W<IEN_SPEC, 3> {
        VFPORCH_W::new(self)
    }
    #[doc = "Bit 4 - Direct Drive Data Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ddempty(&mut self) -> DDEMPTY_W<IEN_SPEC, 4> {
        DDEMPTY_W::new(self)
    }
    #[doc = "Bit 5 - Direct Drive Jitter Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ddjit(&mut self) -> DDJIT_W<IEN_SPEC, 5> {
        DDJIT_W::new(self)
    }
    #[doc = "Bit 6 - EBI_TFTPIXEL0 Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tftpixel0empty(&mut self) -> TFTPIXEL0EMPTY_W<IEN_SPEC, 6> {
        TFTPIXEL0EMPTY_W::new(self)
    }
    #[doc = "Bit 7 - EBI_TFTPIXEL1 Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tftpixel1empty(&mut self) -> TFTPIXEL1EMPTY_W<IEN_SPEC, 7> {
        TFTPIXEL1EMPTY_W::new(self)
    }
    #[doc = "Bit 8 - EBI_TFTPIXEL Full Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tftpixelfull(&mut self) -> TFTPIXELFULL_W<IEN_SPEC, 8> {
        TFTPIXELFULL_W::new(self)
    }
    #[doc = "Bit 9 - EBI_TFTPIXEL Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tftpixelof(&mut self) -> TFTPIXELOF_W<IEN_SPEC, 9> {
        TFTPIXELOF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ien::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
