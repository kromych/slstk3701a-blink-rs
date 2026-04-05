#[doc = "Register `FLASHCMDCTRL` reader"]
pub type R = crate::R<FlashcmdctrlSpec>;
#[doc = "Register `FLASHCMDCTRL` writer"]
pub type W = crate::W<FlashcmdctrlSpec>;
#[doc = "Field `CMDEXEC` writer - Execute the Command"]
pub type CmdexecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDEXECSTATUS` reader - Command Execution in Progress"]
pub type CmdexecstatusR = crate::BitReader;
#[doc = "Field `STIGMEMBANKEN` reader - STIG Memory Bank Enable Bit"]
pub type StigmembankenR = crate::BitReader;
#[doc = "Field `STIGMEMBANKEN` writer - STIG Memory Bank Enable Bit"]
pub type StigmembankenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NUMDUMMYCYCLES` reader - Number of Dummy Cycles"]
pub type NumdummycyclesR = crate::FieldReader;
#[doc = "Field `NUMDUMMYCYCLES` writer - Number of Dummy Cycles"]
pub type NumdummycyclesW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `NUMWRDATABYTES` reader - Number of Write Data Bytes"]
pub type NumwrdatabytesR = crate::FieldReader;
#[doc = "Field `NUMWRDATABYTES` writer - Number of Write Data Bytes"]
pub type NumwrdatabytesW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ENBWRITEDATA` reader - Write Data Enable"]
pub type EnbwritedataR = crate::BitReader;
#[doc = "Field `ENBWRITEDATA` writer - Write Data Enable"]
pub type EnbwritedataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NUMADDRBYTES` reader - Number of Address Bytes"]
pub type NumaddrbytesR = crate::FieldReader;
#[doc = "Field `NUMADDRBYTES` writer - Number of Address Bytes"]
pub type NumaddrbytesW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ENBMODEBIT` reader - Mode Bit Enable"]
pub type EnbmodebitR = crate::BitReader;
#[doc = "Field `ENBMODEBIT` writer - Mode Bit Enable"]
pub type EnbmodebitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENBCOMDADDR` reader - Command Address Enable"]
pub type EnbcomdaddrR = crate::BitReader;
#[doc = "Field `ENBCOMDADDR` writer - Command Address Enable"]
pub type EnbcomdaddrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NUMRDDATABYTES` reader - Number of Read Data Bytes"]
pub type NumrddatabytesR = crate::FieldReader;
#[doc = "Field `NUMRDDATABYTES` writer - Number of Read Data Bytes"]
pub type NumrddatabytesW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ENBREADDATA` reader - Read Data Enable"]
pub type EnbreaddataR = crate::BitReader;
#[doc = "Field `ENBREADDATA` writer - Read Data Enable"]
pub type EnbreaddataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDOPCODE` reader - Command Opcode"]
pub type CmdopcodeR = crate::FieldReader;
#[doc = "Field `CMDOPCODE` writer - Command Opcode"]
pub type CmdopcodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 1 - Command Execution in Progress"]
    #[inline(always)]
    pub fn cmdexecstatus(&self) -> CmdexecstatusR {
        CmdexecstatusR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - STIG Memory Bank Enable Bit"]
    #[inline(always)]
    pub fn stigmembanken(&self) -> StigmembankenR {
        StigmembankenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 7:11 - Number of Dummy Cycles"]
    #[inline(always)]
    pub fn numdummycycles(&self) -> NumdummycyclesR {
        NumdummycyclesR::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:14 - Number of Write Data Bytes"]
    #[inline(always)]
    pub fn numwrdatabytes(&self) -> NumwrdatabytesR {
        NumwrdatabytesR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Write Data Enable"]
    #[inline(always)]
    pub fn enbwritedata(&self) -> EnbwritedataR {
        EnbwritedataR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Number of Address Bytes"]
    #[inline(always)]
    pub fn numaddrbytes(&self) -> NumaddrbytesR {
        NumaddrbytesR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Mode Bit Enable"]
    #[inline(always)]
    pub fn enbmodebit(&self) -> EnbmodebitR {
        EnbmodebitR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Command Address Enable"]
    #[inline(always)]
    pub fn enbcomdaddr(&self) -> EnbcomdaddrR {
        EnbcomdaddrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - Number of Read Data Bytes"]
    #[inline(always)]
    pub fn numrddatabytes(&self) -> NumrddatabytesR {
        NumrddatabytesR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Read Data Enable"]
    #[inline(always)]
    pub fn enbreaddata(&self) -> EnbreaddataR {
        EnbreaddataR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Command Opcode"]
    #[inline(always)]
    pub fn cmdopcode(&self) -> CmdopcodeR {
        CmdopcodeR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Execute the Command"]
    #[inline(always)]
    pub fn cmdexec(&mut self) -> CmdexecW<'_, FlashcmdctrlSpec> {
        CmdexecW::new(self, 0)
    }
    #[doc = "Bit 2 - STIG Memory Bank Enable Bit"]
    #[inline(always)]
    pub fn stigmembanken(&mut self) -> StigmembankenW<'_, FlashcmdctrlSpec> {
        StigmembankenW::new(self, 2)
    }
    #[doc = "Bits 7:11 - Number of Dummy Cycles"]
    #[inline(always)]
    pub fn numdummycycles(&mut self) -> NumdummycyclesW<'_, FlashcmdctrlSpec> {
        NumdummycyclesW::new(self, 7)
    }
    #[doc = "Bits 12:14 - Number of Write Data Bytes"]
    #[inline(always)]
    pub fn numwrdatabytes(&mut self) -> NumwrdatabytesW<'_, FlashcmdctrlSpec> {
        NumwrdatabytesW::new(self, 12)
    }
    #[doc = "Bit 15 - Write Data Enable"]
    #[inline(always)]
    pub fn enbwritedata(&mut self) -> EnbwritedataW<'_, FlashcmdctrlSpec> {
        EnbwritedataW::new(self, 15)
    }
    #[doc = "Bits 16:17 - Number of Address Bytes"]
    #[inline(always)]
    pub fn numaddrbytes(&mut self) -> NumaddrbytesW<'_, FlashcmdctrlSpec> {
        NumaddrbytesW::new(self, 16)
    }
    #[doc = "Bit 18 - Mode Bit Enable"]
    #[inline(always)]
    pub fn enbmodebit(&mut self) -> EnbmodebitW<'_, FlashcmdctrlSpec> {
        EnbmodebitW::new(self, 18)
    }
    #[doc = "Bit 19 - Command Address Enable"]
    #[inline(always)]
    pub fn enbcomdaddr(&mut self) -> EnbcomdaddrW<'_, FlashcmdctrlSpec> {
        EnbcomdaddrW::new(self, 19)
    }
    #[doc = "Bits 20:22 - Number of Read Data Bytes"]
    #[inline(always)]
    pub fn numrddatabytes(&mut self) -> NumrddatabytesW<'_, FlashcmdctrlSpec> {
        NumrddatabytesW::new(self, 20)
    }
    #[doc = "Bit 23 - Read Data Enable"]
    #[inline(always)]
    pub fn enbreaddata(&mut self) -> EnbreaddataW<'_, FlashcmdctrlSpec> {
        EnbreaddataW::new(self, 23)
    }
    #[doc = "Bits 24:31 - Command Opcode"]
    #[inline(always)]
    pub fn cmdopcode(&mut self) -> CmdopcodeW<'_, FlashcmdctrlSpec> {
        CmdopcodeW::new(self, 24)
    }
}
#[doc = "Flash Command Control Register (STIG)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashcmdctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashcmdctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashcmdctrlSpec;
impl crate::RegisterSpec for FlashcmdctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashcmdctrl::R`](R) reader structure"]
impl crate::Readable for FlashcmdctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`flashcmdctrl::W`](W) writer structure"]
impl crate::Writable for FlashcmdctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASHCMDCTRL to value 0"]
impl crate::Resettable for FlashcmdctrlSpec {}
