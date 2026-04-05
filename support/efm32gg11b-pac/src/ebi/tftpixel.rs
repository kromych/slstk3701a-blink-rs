#[doc = "Register `TFTPIXEL` reader"]
pub type R = crate::R<TftpixelSpec>;
#[doc = "Field `DATA` reader - Alpha Blending Result"]
pub type DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Alpha Blending Result"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "TFT Alpha Blending Result Pixel Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tftpixel::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TftpixelSpec;
impl crate::RegisterSpec for TftpixelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tftpixel::R`](R) reader structure"]
impl crate::Readable for TftpixelSpec {}
#[doc = "`reset()` method sets TFTPIXEL to value 0"]
impl crate::Resettable for TftpixelSpec {}
