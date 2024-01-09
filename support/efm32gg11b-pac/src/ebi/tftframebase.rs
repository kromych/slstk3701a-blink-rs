#[doc = "Register `TFTFRAMEBASE` reader"]
pub type R = crate::R<TFTFRAMEBASE_SPEC>;
#[doc = "Register `TFTFRAMEBASE` writer"]
pub type W = crate::W<TFTFRAMEBASE_SPEC>;
#[doc = "Field `FRAMEBASE` reader - Frame Base Address"]
pub type FRAMEBASE_R = crate::FieldReader<u32>;
#[doc = "Field `FRAMEBASE` writer - Frame Base Address"]
pub type FRAMEBASE_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - Frame Base Address"]
    #[inline(always)]
    pub fn framebase(&self) -> FRAMEBASE_R {
        FRAMEBASE_R::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - Frame Base Address"]
    #[inline(always)]
    #[must_use]
    pub fn framebase(&mut self) -> FRAMEBASE_W<TFTFRAMEBASE_SPEC> {
        FRAMEBASE_W::new(self, 0)
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
#[doc = "TFT Frame Base Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tftframebase::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tftframebase::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TFTFRAMEBASE_SPEC;
impl crate::RegisterSpec for TFTFRAMEBASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tftframebase::R`](R) reader structure"]
impl crate::Readable for TFTFRAMEBASE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tftframebase::W`](W) writer structure"]
impl crate::Writable for TFTFRAMEBASE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TFTFRAMEBASE to value 0"]
impl crate::Resettable for TFTFRAMEBASE_SPEC {
    const RESET_VALUE: u32 = 0;
}
