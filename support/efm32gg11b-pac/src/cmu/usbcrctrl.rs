#[doc = "Register `USBCRCTRL` reader"]
pub type R = crate::R<USBCRCTRL_SPEC>;
#[doc = "Register `USBCRCTRL` writer"]
pub type W = crate::W<USBCRCTRL_SPEC>;
#[doc = "Field `USBCREN` reader - Clock Recovery Enable"]
pub type USBCREN_R = crate::BitReader;
#[doc = "Field `USBCREN` writer - Clock Recovery Enable"]
pub type USBCREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBLSCRMD` reader - Low Speed Clock Recovery Mode"]
pub type USBLSCRMD_R = crate::BitReader;
#[doc = "Field `USBLSCRMD` writer - Low Speed Clock Recovery Mode"]
pub type USBLSCRMD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clock Recovery Enable"]
    #[inline(always)]
    pub fn usbcren(&self) -> USBCREN_R {
        USBCREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low Speed Clock Recovery Mode"]
    #[inline(always)]
    pub fn usblscrmd(&self) -> USBLSCRMD_R {
        USBLSCRMD_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Recovery Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbcren(&mut self) -> USBCREN_W<USBCRCTRL_SPEC> {
        USBCREN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Low Speed Clock Recovery Mode"]
    #[inline(always)]
    #[must_use]
    pub fn usblscrmd(&mut self) -> USBLSCRMD_W<USBCRCTRL_SPEC> {
        USBLSCRMD_W::new(self, 1)
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
#[doc = "USB Clock Recovery Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbcrctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbcrctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBCRCTRL_SPEC;
impl crate::RegisterSpec for USBCRCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbcrctrl::R`](R) reader structure"]
impl crate::Readable for USBCRCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbcrctrl::W`](W) writer structure"]
impl crate::Writable for USBCRCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USBCRCTRL to value 0"]
impl crate::Resettable for USBCRCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
