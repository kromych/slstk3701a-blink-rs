#[doc = "Register `STARTUP` reader"]
pub type R = crate::R<STARTUP_SPEC>;
#[doc = "Register `STARTUP` writer"]
pub type W = crate::W<STARTUP_SPEC>;
#[doc = "Field `STDLY0` reader - Startup Delay 0"]
pub type STDLY0_R = crate::FieldReader<u16>;
#[doc = "Field `STDLY0` writer - Startup Delay 0"]
pub type STDLY0_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `STDLY1` reader - Startup Delay 0"]
pub type STDLY1_R = crate::FieldReader<u16>;
#[doc = "Field `STDLY1` writer - Startup Delay 0"]
pub type STDLY1_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `ASTWAIT` reader - Active Startup Wait"]
pub type ASTWAIT_R = crate::BitReader;
#[doc = "Field `ASTWAIT` writer - Active Startup Wait"]
pub type ASTWAIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STWSEN` reader - Startup Waitstates Enable"]
pub type STWSEN_R = crate::BitReader;
#[doc = "Field `STWSEN` writer - Startup Waitstates Enable"]
pub type STWSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STWSAEN` reader - Startup Waitstates Always Enable"]
pub type STWSAEN_R = crate::BitReader;
#[doc = "Field `STWSAEN` writer - Startup Waitstates Always Enable"]
pub type STWSAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STWS` reader - Startup Waitstates"]
pub type STWS_R = crate::FieldReader;
#[doc = "Field `STWS` writer - Startup Waitstates"]
pub type STWS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:9 - Startup Delay 0"]
    #[inline(always)]
    pub fn stdly0(&self) -> STDLY0_R {
        STDLY0_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 12:21 - Startup Delay 0"]
    #[inline(always)]
    pub fn stdly1(&self) -> STDLY1_R {
        STDLY1_R::new(((self.bits >> 12) & 0x03ff) as u16)
    }
    #[doc = "Bit 24 - Active Startup Wait"]
    #[inline(always)]
    pub fn astwait(&self) -> ASTWAIT_R {
        ASTWAIT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Startup Waitstates Enable"]
    #[inline(always)]
    pub fn stwsen(&self) -> STWSEN_R {
        STWSEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Startup Waitstates Always Enable"]
    #[inline(always)]
    pub fn stwsaen(&self) -> STWSAEN_R {
        STWSAEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Startup Waitstates"]
    #[inline(always)]
    pub fn stws(&self) -> STWS_R {
        STWS_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Startup Delay 0"]
    #[inline(always)]
    #[must_use]
    pub fn stdly0(&mut self) -> STDLY0_W<STARTUP_SPEC> {
        STDLY0_W::new(self, 0)
    }
    #[doc = "Bits 12:21 - Startup Delay 0"]
    #[inline(always)]
    #[must_use]
    pub fn stdly1(&mut self) -> STDLY1_W<STARTUP_SPEC> {
        STDLY1_W::new(self, 12)
    }
    #[doc = "Bit 24 - Active Startup Wait"]
    #[inline(always)]
    #[must_use]
    pub fn astwait(&mut self) -> ASTWAIT_W<STARTUP_SPEC> {
        ASTWAIT_W::new(self, 24)
    }
    #[doc = "Bit 25 - Startup Waitstates Enable"]
    #[inline(always)]
    #[must_use]
    pub fn stwsen(&mut self) -> STWSEN_W<STARTUP_SPEC> {
        STWSEN_W::new(self, 25)
    }
    #[doc = "Bit 26 - Startup Waitstates Always Enable"]
    #[inline(always)]
    #[must_use]
    pub fn stwsaen(&mut self) -> STWSAEN_W<STARTUP_SPEC> {
        STWSAEN_W::new(self, 26)
    }
    #[doc = "Bits 28:30 - Startup Waitstates"]
    #[inline(always)]
    #[must_use]
    pub fn stws(&mut self) -> STWS_W<STARTUP_SPEC> {
        STWS_W::new(self, 28)
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
#[doc = "Startup Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`startup::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`startup::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STARTUP_SPEC;
impl crate::RegisterSpec for STARTUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`startup::R`](R) reader structure"]
impl crate::Readable for STARTUP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`startup::W`](W) writer structure"]
impl crate::Writable for STARTUP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STARTUP to value 0x1300_1054"]
impl crate::Resettable for STARTUP_SPEC {
    const RESET_VALUE: u32 = 0x1300_1054;
}
