#[doc = "Register `TFTDD` reader"]
pub type R = crate::R<TFTDD_SPEC>;
#[doc = "Register `TFTDD` writer"]
pub type W = crate::W<TFTDD_SPEC>;
#[doc = "Field `DATA` reader - TFT Direct Drive Data From Internal Memory"]
pub type DATA_R = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - TFT Direct Drive Data From Internal Memory"]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - TFT Direct Drive Data From Internal Memory"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - TFT Direct Drive Data From Internal Memory"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<TFTDD_SPEC> {
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
#[doc = "TFT Direct Drive Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tftdd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tftdd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TFTDD_SPEC;
impl crate::RegisterSpec for TFTDD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tftdd::R`](R) reader structure"]
impl crate::Readable for TFTDD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tftdd::W`](W) writer structure"]
impl crate::Writable for TFTDD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TFTDD to value 0"]
impl crate::Resettable for TFTDD_SPEC {
    const RESET_VALUE: u32 = 0;
}
