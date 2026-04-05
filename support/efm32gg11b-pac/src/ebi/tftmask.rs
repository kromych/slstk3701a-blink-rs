#[doc = "Register `TFTMASK` reader"]
pub type R = crate::R<TftmaskSpec>;
#[doc = "Register `TFTMASK` writer"]
pub type W = crate::W<TftmaskSpec>;
#[doc = "Field `TFTMASK` reader - TFT Mask Value"]
pub type TftmaskR = crate::FieldReader<u32>;
#[doc = "Field `TFTMASK` writer - TFT Mask Value"]
pub type TftmaskW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - TFT Mask Value"]
    #[inline(always)]
    pub fn tftmask(&self) -> TftmaskR {
        TftmaskR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - TFT Mask Value"]
    #[inline(always)]
    pub fn tftmask(&mut self) -> TftmaskW<'_, TftmaskSpec> {
        TftmaskW::new(self, 0)
    }
}
#[doc = "TFT Masking Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tftmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tftmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TftmaskSpec;
impl crate::RegisterSpec for TftmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tftmask::R`](R) reader structure"]
impl crate::Readable for TftmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`tftmask::W`](W) writer structure"]
impl crate::Writable for TftmaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TFTMASK to value 0"]
impl crate::Resettable for TftmaskSpec {}
