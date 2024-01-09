#[doc = "Register `FLASHCMDCTRL` reader"]
pub type R = crate::R<FLASHCMDCTRL_SPEC>;
#[doc = "Register `FLASHCMDCTRL` writer"]
pub type W = crate::W<FLASHCMDCTRL_SPEC>;
#[doc = "Field `CMDEXEC` writer - Execute the Command"]
pub type CMDEXEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDEXECSTATUS` reader - Command Execution in Progress"]
pub type CMDEXECSTATUS_R = crate::BitReader;
#[doc = "Field `STIGMEMBANKEN` reader - STIG Memory Bank Enable Bit"]
pub type STIGMEMBANKEN_R = crate::BitReader;
#[doc = "Field `STIGMEMBANKEN` writer - STIG Memory Bank Enable Bit"]
pub type STIGMEMBANKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NUMDUMMYCYCLES` reader - Number of Dummy Cycles"]
pub type NUMDUMMYCYCLES_R = crate::FieldReader;
#[doc = "Field `NUMDUMMYCYCLES` writer - Number of Dummy Cycles"]
pub type NUMDUMMYCYCLES_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `NUMWRDATABYTES` reader - Number of Write Data Bytes"]
pub type NUMWRDATABYTES_R = crate::FieldReader;
#[doc = "Field `NUMWRDATABYTES` writer - Number of Write Data Bytes"]
pub type NUMWRDATABYTES_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ENBWRITEDATA` reader - Write Data Enable"]
pub type ENBWRITEDATA_R = crate::BitReader;
#[doc = "Field `ENBWRITEDATA` writer - Write Data Enable"]
pub type ENBWRITEDATA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NUMADDRBYTES` reader - Number of Address Bytes"]
pub type NUMADDRBYTES_R = crate::FieldReader;
#[doc = "Field `NUMADDRBYTES` writer - Number of Address Bytes"]
pub type NUMADDRBYTES_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ENBMODEBIT` reader - Mode Bit Enable"]
pub type ENBMODEBIT_R = crate::BitReader;
#[doc = "Field `ENBMODEBIT` writer - Mode Bit Enable"]
pub type ENBMODEBIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENBCOMDADDR` reader - Command Address Enable"]
pub type ENBCOMDADDR_R = crate::BitReader;
#[doc = "Field `ENBCOMDADDR` writer - Command Address Enable"]
pub type ENBCOMDADDR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NUMRDDATABYTES` reader - Number of Read Data Bytes"]
pub type NUMRDDATABYTES_R = crate::FieldReader;
#[doc = "Field `NUMRDDATABYTES` writer - Number of Read Data Bytes"]
pub type NUMRDDATABYTES_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ENBREADDATA` reader - Read Data Enable"]
pub type ENBREADDATA_R = crate::BitReader;
#[doc = "Field `ENBREADDATA` writer - Read Data Enable"]
pub type ENBREADDATA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDOPCODE` reader - Command Opcode"]
pub type CMDOPCODE_R = crate::FieldReader;
#[doc = "Field `CMDOPCODE` writer - Command Opcode"]
pub type CMDOPCODE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 1 - Command Execution in Progress"]
    #[inline(always)]
    pub fn cmdexecstatus(&self) -> CMDEXECSTATUS_R {
        CMDEXECSTATUS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - STIG Memory Bank Enable Bit"]
    #[inline(always)]
    pub fn stigmembanken(&self) -> STIGMEMBANKEN_R {
        STIGMEMBANKEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 7:11 - Number of Dummy Cycles"]
    #[inline(always)]
    pub fn numdummycycles(&self) -> NUMDUMMYCYCLES_R {
        NUMDUMMYCYCLES_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:14 - Number of Write Data Bytes"]
    #[inline(always)]
    pub fn numwrdatabytes(&self) -> NUMWRDATABYTES_R {
        NUMWRDATABYTES_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Write Data Enable"]
    #[inline(always)]
    pub fn enbwritedata(&self) -> ENBWRITEDATA_R {
        ENBWRITEDATA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Number of Address Bytes"]
    #[inline(always)]
    pub fn numaddrbytes(&self) -> NUMADDRBYTES_R {
        NUMADDRBYTES_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Mode Bit Enable"]
    #[inline(always)]
    pub fn enbmodebit(&self) -> ENBMODEBIT_R {
        ENBMODEBIT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Command Address Enable"]
    #[inline(always)]
    pub fn enbcomdaddr(&self) -> ENBCOMDADDR_R {
        ENBCOMDADDR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - Number of Read Data Bytes"]
    #[inline(always)]
    pub fn numrddatabytes(&self) -> NUMRDDATABYTES_R {
        NUMRDDATABYTES_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Read Data Enable"]
    #[inline(always)]
    pub fn enbreaddata(&self) -> ENBREADDATA_R {
        ENBREADDATA_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Command Opcode"]
    #[inline(always)]
    pub fn cmdopcode(&self) -> CMDOPCODE_R {
        CMDOPCODE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Execute the Command"]
    #[inline(always)]
    #[must_use]
    pub fn cmdexec(&mut self) -> CMDEXEC_W<FLASHCMDCTRL_SPEC> {
        CMDEXEC_W::new(self, 0)
    }
    #[doc = "Bit 2 - STIG Memory Bank Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn stigmembanken(&mut self) -> STIGMEMBANKEN_W<FLASHCMDCTRL_SPEC> {
        STIGMEMBANKEN_W::new(self, 2)
    }
    #[doc = "Bits 7:11 - Number of Dummy Cycles"]
    #[inline(always)]
    #[must_use]
    pub fn numdummycycles(&mut self) -> NUMDUMMYCYCLES_W<FLASHCMDCTRL_SPEC> {
        NUMDUMMYCYCLES_W::new(self, 7)
    }
    #[doc = "Bits 12:14 - Number of Write Data Bytes"]
    #[inline(always)]
    #[must_use]
    pub fn numwrdatabytes(&mut self) -> NUMWRDATABYTES_W<FLASHCMDCTRL_SPEC> {
        NUMWRDATABYTES_W::new(self, 12)
    }
    #[doc = "Bit 15 - Write Data Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enbwritedata(&mut self) -> ENBWRITEDATA_W<FLASHCMDCTRL_SPEC> {
        ENBWRITEDATA_W::new(self, 15)
    }
    #[doc = "Bits 16:17 - Number of Address Bytes"]
    #[inline(always)]
    #[must_use]
    pub fn numaddrbytes(&mut self) -> NUMADDRBYTES_W<FLASHCMDCTRL_SPEC> {
        NUMADDRBYTES_W::new(self, 16)
    }
    #[doc = "Bit 18 - Mode Bit Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enbmodebit(&mut self) -> ENBMODEBIT_W<FLASHCMDCTRL_SPEC> {
        ENBMODEBIT_W::new(self, 18)
    }
    #[doc = "Bit 19 - Command Address Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enbcomdaddr(&mut self) -> ENBCOMDADDR_W<FLASHCMDCTRL_SPEC> {
        ENBCOMDADDR_W::new(self, 19)
    }
    #[doc = "Bits 20:22 - Number of Read Data Bytes"]
    #[inline(always)]
    #[must_use]
    pub fn numrddatabytes(&mut self) -> NUMRDDATABYTES_W<FLASHCMDCTRL_SPEC> {
        NUMRDDATABYTES_W::new(self, 20)
    }
    #[doc = "Bit 23 - Read Data Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enbreaddata(&mut self) -> ENBREADDATA_W<FLASHCMDCTRL_SPEC> {
        ENBREADDATA_W::new(self, 23)
    }
    #[doc = "Bits 24:31 - Command Opcode"]
    #[inline(always)]
    #[must_use]
    pub fn cmdopcode(&mut self) -> CMDOPCODE_W<FLASHCMDCTRL_SPEC> {
        CMDOPCODE_W::new(self, 24)
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
#[doc = "Flash Command Control Register (STIG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flashcmdctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flashcmdctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASHCMDCTRL_SPEC;
impl crate::RegisterSpec for FLASHCMDCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashcmdctrl::R`](R) reader structure"]
impl crate::Readable for FLASHCMDCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`flashcmdctrl::W`](W) writer structure"]
impl crate::Writable for FLASHCMDCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASHCMDCTRL to value 0"]
impl crate::Resettable for FLASHCMDCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
