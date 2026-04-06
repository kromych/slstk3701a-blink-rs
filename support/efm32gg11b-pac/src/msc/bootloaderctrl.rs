#[doc = "Register `BOOTLOADERCTRL` reader"]
pub type R = crate::R<BootloaderctrlSpec>;
#[doc = "Register `BOOTLOADERCTRL` writer"]
pub type W = crate::W<BootloaderctrlSpec>;
#[doc = "Field `BLRDIS` reader - Flash Bootloader Read Disable"]
pub type BlrdisR = crate::BitReader;
#[doc = "Field `BLRDIS` writer - Flash Bootloader Read Disable"]
pub type BlrdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLWDIS` reader - Flash Bootloader Write/Erase Disable"]
pub type BlwdisR = crate::BitReader;
#[doc = "Field `BLWDIS` writer - Flash Bootloader Write/Erase Disable"]
pub type BlwdisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Flash Bootloader Read Disable"]
    #[inline(always)]
    pub fn blrdis(&self) -> BlrdisR {
        BlrdisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Flash Bootloader Write/Erase Disable"]
    #[inline(always)]
    pub fn blwdis(&self) -> BlwdisR {
        BlwdisR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flash Bootloader Read Disable"]
    #[inline(always)]
    pub fn blrdis(&mut self) -> BlrdisW<'_, BootloaderctrlSpec> {
        BlrdisW::new(self, 0)
    }
    #[doc = "Bit 1 - Flash Bootloader Write/Erase Disable"]
    #[inline(always)]
    pub fn blwdis(&mut self) -> BlwdisW<'_, BootloaderctrlSpec> {
        BlwdisW::new(self, 1)
    }
}
#[doc = "Bootloader Read and Write Enable, Write Once Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bootloaderctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootloaderctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BootloaderctrlSpec;
impl crate::RegisterSpec for BootloaderctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bootloaderctrl::R`](R) reader structure"]
impl crate::Readable for BootloaderctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`bootloaderctrl::W`](W) writer structure"]
impl crate::Writable for BootloaderctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BOOTLOADERCTRL to value 0"]
impl crate::Resettable for BootloaderctrlSpec {}
