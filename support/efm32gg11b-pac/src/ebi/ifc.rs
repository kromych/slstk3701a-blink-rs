#[doc = "Register `IFC` writer"]
pub type W = crate::W<IFC_SPEC>;
#[doc = "Field `VSYNC` writer - Vertical Sync Interrupt Flag Clear"]
pub type VSYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSYNC` writer - Horizontal Sync Interrupt Flag Clear"]
pub type HSYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBPORCH` writer - Vertical Back Porch Interrupt Flag Clear"]
pub type VBPORCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VFPORCH` writer - Vertical Front Porch Interrupt Flag Clear"]
pub type VFPORCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDEMPTY` writer - Direct Drive Data Empty Interrupt Flag Clear"]
pub type DDEMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDJIT` writer - Direct Drive Jitter Interrupt Flag Clear"]
pub type DDJIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFTPIXEL0EMPTY` writer - EBI_TFTPIXEL0 Empty Interrupt Flag Clear"]
pub type TFTPIXEL0EMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFTPIXEL1EMPTY` writer - EBI_TFTPIXEL1 Empty Interrupt Flag Clear"]
pub type TFTPIXEL1EMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFTPIXELFULL` writer - EBI_TFTPIXEL Full Interrupt Flag Clear"]
pub type TFTPIXELFULL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFTPIXELOF` writer - EBI_TFTPIXEL Overflow Interrupt Flag Clear"]
pub type TFTPIXELOF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Vertical Sync Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn vsync(&mut self) -> VSYNC_W<IFC_SPEC> {
        VSYNC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Horizontal Sync Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsync(&mut self) -> HSYNC_W<IFC_SPEC> {
        HSYNC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Vertical Back Porch Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn vbporch(&mut self) -> VBPORCH_W<IFC_SPEC> {
        VBPORCH_W::new(self, 2)
    }
    #[doc = "Bit 3 - Vertical Front Porch Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn vfporch(&mut self) -> VFPORCH_W<IFC_SPEC> {
        VFPORCH_W::new(self, 3)
    }
    #[doc = "Bit 4 - Direct Drive Data Empty Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ddempty(&mut self) -> DDEMPTY_W<IFC_SPEC> {
        DDEMPTY_W::new(self, 4)
    }
    #[doc = "Bit 5 - Direct Drive Jitter Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ddjit(&mut self) -> DDJIT_W<IFC_SPEC> {
        DDJIT_W::new(self, 5)
    }
    #[doc = "Bit 6 - EBI_TFTPIXEL0 Empty Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn tftpixel0empty(&mut self) -> TFTPIXEL0EMPTY_W<IFC_SPEC> {
        TFTPIXEL0EMPTY_W::new(self, 6)
    }
    #[doc = "Bit 7 - EBI_TFTPIXEL1 Empty Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn tftpixel1empty(&mut self) -> TFTPIXEL1EMPTY_W<IFC_SPEC> {
        TFTPIXEL1EMPTY_W::new(self, 7)
    }
    #[doc = "Bit 8 - EBI_TFTPIXEL Full Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn tftpixelfull(&mut self) -> TFTPIXELFULL_W<IFC_SPEC> {
        TFTPIXELFULL_W::new(self, 8)
    }
    #[doc = "Bit 9 - EBI_TFTPIXEL Overflow Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn tftpixelof(&mut self) -> TFTPIXELOF_W<IFC_SPEC> {
        TFTPIXELOF_W::new(self, 9)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFC_SPEC;
impl crate::RegisterSpec for IFC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifc::W`](W) writer structure"]
impl crate::Writable for IFC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IFC_SPEC {
    const RESET_VALUE: u32 = 0;
}
