#[doc = "Register `TFTPIXEL1` reader"]
pub type R = crate::R<TFTPIXEL1_SPEC>;
#[doc = "Register `TFTPIXEL1` writer"]
pub type W = crate::W<TFTPIXEL1_SPEC>;
#[doc = "Field `DATA` reader - RGB Data"]
pub type DATA_R = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - RGB Data"]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - RGB Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - RGB Data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<TFTPIXEL1_SPEC> {
        DATA_W::new(self, 0)
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
#[doc = "TFT Pixel 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tftpixel1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tftpixel1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TFTPIXEL1_SPEC;
impl crate::RegisterSpec for TFTPIXEL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tftpixel1::R`](R) reader structure"]
impl crate::Readable for TFTPIXEL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tftpixel1::W`](W) writer structure"]
impl crate::Writable for TFTPIXEL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TFTPIXEL1 to value 0"]
impl crate::Resettable for TFTPIXEL1_SPEC {
    const RESET_VALUE: u32 = 0;
}
