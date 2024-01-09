#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMD_SPEC>;
#[doc = "Field `EM4UNLATCH` writer - EM4 Unlatch"]
pub type EM4UNLATCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM01VSCALE0` writer - EM01 Voltage Scale Command to Scale to Voltage Scale Level 0"]
pub type EM01VSCALE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM01VSCALE2` writer - EM01 Voltage Scale Command to Scale to Voltage Scale Level 2"]
pub type EM01VSCALE2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - EM4 Unlatch"]
    #[inline(always)]
    #[must_use]
    pub fn em4unlatch(&mut self) -> EM4UNLATCH_W<CMD_SPEC> {
        EM4UNLATCH_W::new(self, 0)
    }
    #[doc = "Bit 4 - EM01 Voltage Scale Command to Scale to Voltage Scale Level 0"]
    #[inline(always)]
    #[must_use]
    pub fn em01vscale0(&mut self) -> EM01VSCALE0_W<CMD_SPEC> {
        EM01VSCALE0_W::new(self, 4)
    }
    #[doc = "Bit 6 - EM01 Voltage Scale Command to Scale to Voltage Scale Level 2"]
    #[inline(always)]
    #[must_use]
    pub fn em01vscale2(&mut self) -> EM01VSCALE2_W<CMD_SPEC> {
        EM01VSCALE2_W::new(self, 6)
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
