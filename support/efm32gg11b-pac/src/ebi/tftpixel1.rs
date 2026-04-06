#[doc = "Register `TFTPIXEL1` reader"]
pub type R = crate::R<Tftpixel1Spec>;
#[doc = "Register `TFTPIXEL1` writer"]
pub type W = crate::W<Tftpixel1Spec>;
#[doc = "Field `DATA` reader - RGB Data"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - RGB Data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - RGB Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - RGB Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, Tftpixel1Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "TFT Pixel 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tftpixel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tftpixel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tftpixel1Spec;
impl crate::RegisterSpec for Tftpixel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tftpixel1::R`](R) reader structure"]
impl crate::Readable for Tftpixel1Spec {}
#[doc = "`write(|w| ..)` method takes [`tftpixel1::W`](W) writer structure"]
impl crate::Writable for Tftpixel1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TFTPIXEL1 to value 0"]
impl crate::Resettable for Tftpixel1Spec {}
