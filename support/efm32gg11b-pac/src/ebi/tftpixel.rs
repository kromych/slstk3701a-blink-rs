#[doc = "Register `TFTPIXEL` reader"]
pub type R = crate::R<TFTPIXEL_SPEC>;
#[doc = "Field `DATA` reader - Alpha Blending Result"]
pub type DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Alpha Blending Result"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "TFT Alpha Blending Result Pixel Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tftpixel::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TFTPIXEL_SPEC;
impl crate::RegisterSpec for TFTPIXEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tftpixel::R`](R) reader structure"]
impl crate::Readable for TFTPIXEL_SPEC {}
#[doc = "`reset()` method sets TFTPIXEL to value 0"]
impl crate::Resettable for TFTPIXEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
