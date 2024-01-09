#[doc = "Register `LFCCLKEN0` reader"]
pub type R = crate::R<LFCCLKEN0_SPEC>;
#[doc = "Register `LFCCLKEN0` writer"]
pub type W = crate::W<LFCCLKEN0_SPEC>;
#[doc = "Field `USB` reader - Universal Serial Bus Interface Clock Enable"]
pub type USB_R = crate::BitReader;
#[doc = "Field `USB` writer - Universal Serial Bus Interface Clock Enable"]
pub type USB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Universal Serial Bus Interface Clock Enable"]
    #[inline(always)]
    pub fn usb(&self) -> USB_R {
        USB_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Universal Serial Bus Interface Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usb(&mut self) -> USB_W<LFCCLKEN0_SPEC> {
        USB_W::new(self, 0)
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
#[doc = "Low Frequency C Clock Enable Register 0 (Async Reg)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfcclken0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfcclken0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LFCCLKEN0_SPEC;
impl crate::RegisterSpec for LFCCLKEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfcclken0::R`](R) reader structure"]
impl crate::Readable for LFCCLKEN0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lfcclken0::W`](W) writer structure"]
impl crate::Writable for LFCCLKEN0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LFCCLKEN0 to value 0"]
impl crate::Resettable for LFCCLKEN0_SPEC {
    const RESET_VALUE: u32 = 0;
}
