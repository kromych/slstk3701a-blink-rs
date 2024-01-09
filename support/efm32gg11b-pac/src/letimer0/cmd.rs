#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMD_SPEC>;
#[doc = "Field `START` writer - Start LETIMER"]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` writer - Stop LETIMER"]
pub type STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR` writer - Clear LETIMER"]
pub type CLEAR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTO0` writer - Clear Toggle Output 0"]
pub type CTO0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTO1` writer - Clear Toggle Output 1"]
pub type CTO1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Start LETIMER"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<CMD_SPEC> {
        START_W::new(self, 0)
    }
    #[doc = "Bit 1 - Stop LETIMER"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<CMD_SPEC> {
        STOP_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear LETIMER"]
    #[inline(always)]
    #[must_use]
    pub fn clear(&mut self) -> CLEAR_W<CMD_SPEC> {
        CLEAR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear Toggle Output 0"]
    #[inline(always)]
    #[must_use]
    pub fn cto0(&mut self) -> CTO0_W<CMD_SPEC> {
        CTO0_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear Toggle Output 1"]
    #[inline(always)]
    #[must_use]
    pub fn cto1(&mut self) -> CTO1_W<CMD_SPEC> {
        CTO1_W::new(self, 4)
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
