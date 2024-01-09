#[doc = "Register `DECSTATE` reader"]
pub type R = crate::R<DECSTATE_SPEC>;
#[doc = "Register `DECSTATE` writer"]
pub type W = crate::W<DECSTATE_SPEC>;
#[doc = "Field `DECSTATE` reader - Current Decoder State"]
pub type DECSTATE_R = crate::FieldReader;
#[doc = "Field `DECSTATE` writer - Current Decoder State"]
pub type DECSTATE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Current Decoder State"]
    #[inline(always)]
    pub fn decstate(&self) -> DECSTATE_R {
        DECSTATE_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Current Decoder State"]
    #[inline(always)]
    #[must_use]
    pub fn decstate(&mut self) -> DECSTATE_W<DECSTATE_SPEC> {
        DECSTATE_W::new(self, 0)
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
#[doc = "Current Decoder State\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`decstate::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`decstate::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DECSTATE_SPEC;
impl crate::RegisterSpec for DECSTATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`decstate::R`](R) reader structure"]
impl crate::Readable for DECSTATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`decstate::W`](W) writer structure"]
impl crate::Writable for DECSTATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DECSTATE to value 0"]
impl crate::Resettable for DECSTATE_SPEC {
    const RESET_VALUE: u32 = 0;
}
