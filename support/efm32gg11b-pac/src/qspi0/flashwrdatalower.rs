#[doc = "Register `FLASHWRDATALOWER` reader"]
pub type R = crate::R<FLASHWRDATALOWER_SPEC>;
#[doc = "Register `FLASHWRDATALOWER` writer"]
pub type W = crate::W<FLASHWRDATALOWER_SPEC>;
#[doc = "Field `DATA` reader - Command Write Data Lower Byte"]
pub type DATA_R = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Command Write Data Lower Byte"]
pub type DATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Command Write Data Lower Byte"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command Write Data Lower Byte"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<FLASHWRDATALOWER_SPEC, 0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Flash Command Write Data Register (Lower) (STIG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flashwrdatalower::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flashwrdatalower::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASHWRDATALOWER_SPEC;
impl crate::RegisterSpec for FLASHWRDATALOWER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashwrdatalower::R`](R) reader structure"]
impl crate::Readable for FLASHWRDATALOWER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`flashwrdatalower::W`](W) writer structure"]
impl crate::Writable for FLASHWRDATALOWER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLASHWRDATALOWER to value 0"]
impl crate::Resettable for FLASHWRDATALOWER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
