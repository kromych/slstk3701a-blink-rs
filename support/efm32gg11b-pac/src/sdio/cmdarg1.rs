#[doc = "Register `CMDARG1` reader"]
pub type R = crate::R<CMDARG1_SPEC>;
#[doc = "Register `CMDARG1` writer"]
pub type W = crate::W<CMDARG1_SPEC>;
#[doc = "Field `CMDARG1` reader - Command Argument 1"]
pub type CMDARG1_R = crate::FieldReader<u32>;
#[doc = "Field `CMDARG1` writer - Command Argument 1"]
pub type CMDARG1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Command Argument 1"]
    #[inline(always)]
    pub fn cmdarg1(&self) -> CMDARG1_R {
        CMDARG1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command Argument 1"]
    #[inline(always)]
    #[must_use]
    pub fn cmdarg1(&mut self) -> CMDARG1_W<CMDARG1_SPEC, 0> {
        CMDARG1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SD Command Argument Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdarg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdarg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMDARG1_SPEC;
impl crate::RegisterSpec for CMDARG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdarg1::R`](R) reader structure"]
impl crate::Readable for CMDARG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmdarg1::W`](W) writer structure"]
impl crate::Writable for CMDARG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMDARG1 to value 0"]
impl crate::Resettable for CMDARG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
