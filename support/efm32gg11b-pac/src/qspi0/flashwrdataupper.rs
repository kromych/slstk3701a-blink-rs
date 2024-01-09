#[doc = "Register `FLASHWRDATAUPPER` reader"]
pub type R = crate::R<FLASHWRDATAUPPER_SPEC>;
#[doc = "Register `FLASHWRDATAUPPER` writer"]
pub type W = crate::W<FLASHWRDATAUPPER_SPEC>;
#[doc = "Field `DATA` reader - Command Write Data Upper Byte"]
pub type DATA_R = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Command Write Data Upper Byte"]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command Write Data Upper Byte"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command Write Data Upper Byte"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<FLASHWRDATAUPPER_SPEC> {
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
#[doc = "Flash Command Write Data Register (Upper) (STIG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flashwrdataupper::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flashwrdataupper::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASHWRDATAUPPER_SPEC;
impl crate::RegisterSpec for FLASHWRDATAUPPER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashwrdataupper::R`](R) reader structure"]
impl crate::Readable for FLASHWRDATAUPPER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`flashwrdataupper::W`](W) writer structure"]
impl crate::Writable for FLASHWRDATAUPPER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASHWRDATAUPPER to value 0"]
impl crate::Resettable for FLASHWRDATAUPPER_SPEC {
    const RESET_VALUE: u32 = 0;
}
