#[doc = "Register `USBCRCTRL` reader"]
pub type R = crate::R<UsbcrctrlSpec>;
#[doc = "Register `USBCRCTRL` writer"]
pub type W = crate::W<UsbcrctrlSpec>;
#[doc = "Field `USBCREN` reader - Clock Recovery Enable"]
pub type UsbcrenR = crate::BitReader;
#[doc = "Field `USBCREN` writer - Clock Recovery Enable"]
pub type UsbcrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBLSCRMD` reader - Low Speed Clock Recovery Mode"]
pub type UsblscrmdR = crate::BitReader;
#[doc = "Field `USBLSCRMD` writer - Low Speed Clock Recovery Mode"]
pub type UsblscrmdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clock Recovery Enable"]
    #[inline(always)]
    pub fn usbcren(&self) -> UsbcrenR {
        UsbcrenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low Speed Clock Recovery Mode"]
    #[inline(always)]
    pub fn usblscrmd(&self) -> UsblscrmdR {
        UsblscrmdR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Recovery Enable"]
    #[inline(always)]
    pub fn usbcren(&mut self) -> UsbcrenW<'_, UsbcrctrlSpec> {
        UsbcrenW::new(self, 0)
    }
    #[doc = "Bit 1 - Low Speed Clock Recovery Mode"]
    #[inline(always)]
    pub fn usblscrmd(&mut self) -> UsblscrmdW<'_, UsbcrctrlSpec> {
        UsblscrmdW::new(self, 1)
    }
}
#[doc = "USB Clock Recovery Control\n\nYou can [`read`](crate::Reg::read) this register and get [`usbcrctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbcrctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbcrctrlSpec;
impl crate::RegisterSpec for UsbcrctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbcrctrl::R`](R) reader structure"]
impl crate::Readable for UsbcrctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`usbcrctrl::W`](W) writer structure"]
impl crate::Writable for UsbcrctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USBCRCTRL to value 0"]
impl crate::Resettable for UsbcrctrlSpec {}
