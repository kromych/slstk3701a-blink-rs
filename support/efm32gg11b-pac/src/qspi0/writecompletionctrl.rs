#[doc = "Register `WRITECOMPLETIONCTRL` reader"]
pub type R = crate::R<WRITECOMPLETIONCTRL_SPEC>;
#[doc = "Register `WRITECOMPLETIONCTRL` writer"]
pub type W = crate::W<WRITECOMPLETIONCTRL_SPEC>;
#[doc = "Field `OPCODE` reader - Opcode"]
pub type OPCODE_R = crate::FieldReader;
#[doc = "Field `OPCODE` writer - Opcode"]
pub type OPCODE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `POLLINGBITINDEX` reader - Polling Bit Index"]
pub type POLLINGBITINDEX_R = crate::FieldReader;
#[doc = "Field `POLLINGBITINDEX` writer - Polling Bit Index"]
pub type POLLINGBITINDEX_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `POLLINGPOLARITY` reader - Polling Polarity"]
pub type POLLINGPOLARITY_R = crate::BitReader;
#[doc = "Field `POLLINGPOLARITY` writer - Polling Polarity"]
pub type POLLINGPOLARITY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLEPOLLING` reader - Disable Polling"]
pub type DISABLEPOLLING_R = crate::BitReader;
#[doc = "Field `DISABLEPOLLING` writer - Disable Polling"]
pub type DISABLEPOLLING_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLEPOLLINGEXP` reader - Enable Polling Expiration"]
pub type ENABLEPOLLINGEXP_R = crate::BitReader;
#[doc = "Field `ENABLEPOLLINGEXP` writer - Enable Polling Expiration"]
pub type ENABLEPOLLINGEXP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLLCOUNT` reader - Poll Count"]
pub type POLLCOUNT_R = crate::FieldReader;
#[doc = "Field `POLLCOUNT` writer - Poll Count"]
pub type POLLCOUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `POLLREPDELAY` reader - Poll Repetition Delay"]
pub type POLLREPDELAY_R = crate::FieldReader;
#[doc = "Field `POLLREPDELAY` writer - Poll Repetition Delay"]
pub type POLLREPDELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Opcode"]
    #[inline(always)]
    pub fn opcode(&self) -> OPCODE_R {
        OPCODE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Polling Bit Index"]
    #[inline(always)]
    pub fn pollingbitindex(&self) -> POLLINGBITINDEX_R {
        POLLINGBITINDEX_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 13 - Polling Polarity"]
    #[inline(always)]
    pub fn pollingpolarity(&self) -> POLLINGPOLARITY_R {
        POLLINGPOLARITY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Disable Polling"]
    #[inline(always)]
    pub fn disablepolling(&self) -> DISABLEPOLLING_R {
        DISABLEPOLLING_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Polling Expiration"]
    #[inline(always)]
    pub fn enablepollingexp(&self) -> ENABLEPOLLINGEXP_R {
        ENABLEPOLLINGEXP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Poll Count"]
    #[inline(always)]
    pub fn pollcount(&self) -> POLLCOUNT_R {
        POLLCOUNT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Poll Repetition Delay"]
    #[inline(always)]
    pub fn pollrepdelay(&self) -> POLLREPDELAY_R {
        POLLREPDELAY_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Opcode"]
    #[inline(always)]
    #[must_use]
    pub fn opcode(&mut self) -> OPCODE_W<WRITECOMPLETIONCTRL_SPEC> {
        OPCODE_W::new(self, 0)
    }
    #[doc = "Bits 8:10 - Polling Bit Index"]
    #[inline(always)]
    #[must_use]
    pub fn pollingbitindex(&mut self) -> POLLINGBITINDEX_W<WRITECOMPLETIONCTRL_SPEC> {
        POLLINGBITINDEX_W::new(self, 8)
    }
    #[doc = "Bit 13 - Polling Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pollingpolarity(&mut self) -> POLLINGPOLARITY_W<WRITECOMPLETIONCTRL_SPEC> {
        POLLINGPOLARITY_W::new(self, 13)
    }
    #[doc = "Bit 14 - Disable Polling"]
    #[inline(always)]
    #[must_use]
    pub fn disablepolling(&mut self) -> DISABLEPOLLING_W<WRITECOMPLETIONCTRL_SPEC> {
        DISABLEPOLLING_W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Polling Expiration"]
    #[inline(always)]
    #[must_use]
    pub fn enablepollingexp(&mut self) -> ENABLEPOLLINGEXP_W<WRITECOMPLETIONCTRL_SPEC> {
        ENABLEPOLLINGEXP_W::new(self, 15)
    }
    #[doc = "Bits 16:23 - Poll Count"]
    #[inline(always)]
    #[must_use]
    pub fn pollcount(&mut self) -> POLLCOUNT_W<WRITECOMPLETIONCTRL_SPEC> {
        POLLCOUNT_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Poll Repetition Delay"]
    #[inline(always)]
    #[must_use]
    pub fn pollrepdelay(&mut self) -> POLLREPDELAY_W<WRITECOMPLETIONCTRL_SPEC> {
        POLLREPDELAY_W::new(self, 24)
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
#[doc = "Write Completion Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`writecompletionctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`writecompletionctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRITECOMPLETIONCTRL_SPEC;
impl crate::RegisterSpec for WRITECOMPLETIONCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`writecompletionctrl::R`](R) reader structure"]
impl crate::Readable for WRITECOMPLETIONCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`writecompletionctrl::W`](W) writer structure"]
impl crate::Writable for WRITECOMPLETIONCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WRITECOMPLETIONCTRL to value 0x0001_0005"]
impl crate::Resettable for WRITECOMPLETIONCTRL_SPEC {
    const RESET_VALUE: u32 = 0x0001_0005;
}
