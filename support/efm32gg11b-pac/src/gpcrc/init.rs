#[doc = "Register `INIT` reader"]
pub type R = crate::R<INIT_SPEC>;
#[doc = "Register `INIT` writer"]
pub type W = crate::W<INIT_SPEC>;
#[doc = "Field `INIT` reader - CRC Initialization Value"]
pub type INIT_R = crate::FieldReader<u32>;
#[doc = "Field `INIT` writer - CRC Initialization Value"]
pub type INIT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC Initialization Value"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC Initialization Value"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> INIT_W<INIT_SPEC, 0> {
        INIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "CRC Init Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`init::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`init::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INIT_SPEC;
impl crate::RegisterSpec for INIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`init::R`](R) reader structure"]
impl crate::Readable for INIT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`init::W`](W) writer structure"]
impl crate::Writable for INIT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INIT to value 0"]
impl crate::Resettable for INIT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
