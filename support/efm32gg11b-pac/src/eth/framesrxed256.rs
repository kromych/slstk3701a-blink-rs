#[doc = "Register `FRAMESRXED256` reader"]
pub type R = crate::R<FRAMESRXED256_SPEC>;
#[doc = "Register `FRAMESRXED256` writer"]
pub type W = crate::W<FRAMESRXED256_SPEC>;
#[doc = "Field `COUNT` reader - 256 to 511 byte frames received without error"]
pub type COUNT_R = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - 256 to 511 byte frames received without error"]
pub type COUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 256 to 511 byte frames received without error"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 256 to 511 byte frames received without error"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<FRAMESRXED256_SPEC> {
        COUNT_W::new(self, 0)
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
#[doc = "256 to 511 Byte Frames Received\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`framesrxed256::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`framesrxed256::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRAMESRXED256_SPEC;
impl crate::RegisterSpec for FRAMESRXED256_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`framesrxed256::R`](R) reader structure"]
impl crate::Readable for FRAMESRXED256_SPEC {}
#[doc = "`write(|w| ..)` method takes [`framesrxed256::W`](W) writer structure"]
impl crate::Writable for FRAMESRXED256_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FRAMESRXED256 to value 0"]
impl crate::Resettable for FRAMESRXED256_SPEC {
    const RESET_VALUE: u32 = 0;
}
