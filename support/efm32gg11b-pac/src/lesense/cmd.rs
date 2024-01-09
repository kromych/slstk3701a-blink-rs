#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMD_SPEC>;
#[doc = "Field `START` writer - Start Scanning of Sensors"]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` writer - Stop Scanning of Sensors"]
pub type STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DECODE` writer - Start Decoder"]
pub type DECODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEARBUF` writer - Clear Result Buffer"]
pub type CLEARBUF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Start Scanning of Sensors"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<CMD_SPEC> {
        START_W::new(self, 0)
    }
    #[doc = "Bit 1 - Stop Scanning of Sensors"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<CMD_SPEC> {
        STOP_W::new(self, 1)
    }
    #[doc = "Bit 2 - Start Decoder"]
    #[inline(always)]
    #[must_use]
    pub fn decode(&mut self) -> DECODE_W<CMD_SPEC> {
        DECODE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear Result Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn clearbuf(&mut self) -> CLEARBUF_W<CMD_SPEC> {
        CLEARBUF_W::new(self, 3)
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
