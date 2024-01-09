#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMD_SPEC>;
#[doc = "Field `ECCSTART` writer - Error Correction Code Generation Start"]
pub type ECCSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCSTOP` writer - Error Correction Code Generation Stop"]
pub type ECCSTOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCCLEAR` writer - Error Correction Code Clear"]
pub type ECCCLEAR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Error Correction Code Generation Start"]
    #[inline(always)]
    #[must_use]
    pub fn eccstart(&mut self) -> ECCSTART_W<CMD_SPEC> {
        ECCSTART_W::new(self, 0)
    }
    #[doc = "Bit 1 - Error Correction Code Generation Stop"]
    #[inline(always)]
    #[must_use]
    pub fn eccstop(&mut self) -> ECCSTOP_W<CMD_SPEC> {
        ECCSTOP_W::new(self, 1)
    }
    #[doc = "Bit 2 - Error Correction Code Clear"]
    #[inline(always)]
    #[must_use]
    pub fn eccclear(&mut self) -> ECCCLEAR_W<CMD_SPEC> {
        ECCCLEAR_W::new(self, 2)
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
#[doc = "Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: u32 = 0;
}
