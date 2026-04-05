#[doc = "Register `WRITECOMPLETIONCTRL` reader"]
pub type R = crate::R<WritecompletionctrlSpec>;
#[doc = "Register `WRITECOMPLETIONCTRL` writer"]
pub type W = crate::W<WritecompletionctrlSpec>;
#[doc = "Field `OPCODE` reader - Opcode"]
pub type OpcodeR = crate::FieldReader;
#[doc = "Field `OPCODE` writer - Opcode"]
pub type OpcodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `POLLINGBITINDEX` reader - Polling Bit Index"]
pub type PollingbitindexR = crate::FieldReader;
#[doc = "Field `POLLINGBITINDEX` writer - Polling Bit Index"]
pub type PollingbitindexW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `POLLINGPOLARITY` reader - Polling Polarity"]
pub type PollingpolarityR = crate::BitReader;
#[doc = "Field `POLLINGPOLARITY` writer - Polling Polarity"]
pub type PollingpolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLEPOLLING` reader - Disable Polling"]
pub type DisablepollingR = crate::BitReader;
#[doc = "Field `DISABLEPOLLING` writer - Disable Polling"]
pub type DisablepollingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLEPOLLINGEXP` reader - Enable Polling Expiration"]
pub type EnablepollingexpR = crate::BitReader;
#[doc = "Field `ENABLEPOLLINGEXP` writer - Enable Polling Expiration"]
pub type EnablepollingexpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLLCOUNT` reader - Poll Count"]
pub type PollcountR = crate::FieldReader;
#[doc = "Field `POLLCOUNT` writer - Poll Count"]
pub type PollcountW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `POLLREPDELAY` reader - Poll Repetition Delay"]
pub type PollrepdelayR = crate::FieldReader;
#[doc = "Field `POLLREPDELAY` writer - Poll Repetition Delay"]
pub type PollrepdelayW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Opcode"]
    #[inline(always)]
    pub fn opcode(&self) -> OpcodeR {
        OpcodeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Polling Bit Index"]
    #[inline(always)]
    pub fn pollingbitindex(&self) -> PollingbitindexR {
        PollingbitindexR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 13 - Polling Polarity"]
    #[inline(always)]
    pub fn pollingpolarity(&self) -> PollingpolarityR {
        PollingpolarityR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Disable Polling"]
    #[inline(always)]
    pub fn disablepolling(&self) -> DisablepollingR {
        DisablepollingR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Polling Expiration"]
    #[inline(always)]
    pub fn enablepollingexp(&self) -> EnablepollingexpR {
        EnablepollingexpR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Poll Count"]
    #[inline(always)]
    pub fn pollcount(&self) -> PollcountR {
        PollcountR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Poll Repetition Delay"]
    #[inline(always)]
    pub fn pollrepdelay(&self) -> PollrepdelayR {
        PollrepdelayR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Opcode"]
    #[inline(always)]
    pub fn opcode(&mut self) -> OpcodeW<'_, WritecompletionctrlSpec> {
        OpcodeW::new(self, 0)
    }
    #[doc = "Bits 8:10 - Polling Bit Index"]
    #[inline(always)]
    pub fn pollingbitindex(&mut self) -> PollingbitindexW<'_, WritecompletionctrlSpec> {
        PollingbitindexW::new(self, 8)
    }
    #[doc = "Bit 13 - Polling Polarity"]
    #[inline(always)]
    pub fn pollingpolarity(&mut self) -> PollingpolarityW<'_, WritecompletionctrlSpec> {
        PollingpolarityW::new(self, 13)
    }
    #[doc = "Bit 14 - Disable Polling"]
    #[inline(always)]
    pub fn disablepolling(&mut self) -> DisablepollingW<'_, WritecompletionctrlSpec> {
        DisablepollingW::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Polling Expiration"]
    #[inline(always)]
    pub fn enablepollingexp(&mut self) -> EnablepollingexpW<'_, WritecompletionctrlSpec> {
        EnablepollingexpW::new(self, 15)
    }
    #[doc = "Bits 16:23 - Poll Count"]
    #[inline(always)]
    pub fn pollcount(&mut self) -> PollcountW<'_, WritecompletionctrlSpec> {
        PollcountW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Poll Repetition Delay"]
    #[inline(always)]
    pub fn pollrepdelay(&mut self) -> PollrepdelayW<'_, WritecompletionctrlSpec> {
        PollrepdelayW::new(self, 24)
    }
}
#[doc = "Write Completion Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`writecompletionctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`writecompletionctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WritecompletionctrlSpec;
impl crate::RegisterSpec for WritecompletionctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`writecompletionctrl::R`](R) reader structure"]
impl crate::Readable for WritecompletionctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`writecompletionctrl::W`](W) writer structure"]
impl crate::Writable for WritecompletionctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WRITECOMPLETIONCTRL to value 0x0001_0005"]
impl crate::Resettable for WritecompletionctrlSpec {
    const RESET_VALUE: u32 = 0x0001_0005;
}
