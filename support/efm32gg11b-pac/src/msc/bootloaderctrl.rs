#[doc = "Register `BOOTLOADERCTRL` reader"]
pub type R = crate::R<BOOTLOADERCTRL_SPEC>;
#[doc = "Register `BOOTLOADERCTRL` writer"]
pub type W = crate::W<BOOTLOADERCTRL_SPEC>;
#[doc = "Field `BLRDIS` reader - Flash Bootloader Read Disable"]
pub type BLRDIS_R = crate::BitReader;
#[doc = "Field `BLRDIS` writer - Flash Bootloader Read Disable"]
pub type BLRDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BLWDIS` reader - Flash Bootloader Write/Erase Disable"]
pub type BLWDIS_R = crate::BitReader;
#[doc = "Field `BLWDIS` writer - Flash Bootloader Write/Erase Disable"]
pub type BLWDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Flash Bootloader Read Disable"]
    #[inline(always)]
    pub fn blrdis(&self) -> BLRDIS_R {
        BLRDIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Flash Bootloader Write/Erase Disable"]
    #[inline(always)]
    pub fn blwdis(&self) -> BLWDIS_R {
        BLWDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flash Bootloader Read Disable"]
    #[inline(always)]
    #[must_use]
    pub fn blrdis(&mut self) -> BLRDIS_W<BOOTLOADERCTRL_SPEC, 0> {
        BLRDIS_W::new(self)
    }
    #[doc = "Bit 1 - Flash Bootloader Write/Erase Disable"]
    #[inline(always)]
    #[must_use]
    pub fn blwdis(&mut self) -> BLWDIS_W<BOOTLOADERCTRL_SPEC, 1> {
        BLWDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Bootloader Read and Write Enable, Write Once Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bootloaderctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bootloaderctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOTLOADERCTRL_SPEC;
impl crate::RegisterSpec for BOOTLOADERCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bootloaderctrl::R`](R) reader structure"]
impl crate::Readable for BOOTLOADERCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bootloaderctrl::W`](W) writer structure"]
impl crate::Writable for BOOTLOADERCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BOOTLOADERCTRL to value 0"]
impl crate::Resettable for BOOTLOADERCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
