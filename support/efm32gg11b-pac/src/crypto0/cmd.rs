#[doc = "Register `CMD` reader"]
pub type R = crate::R<CmdSpec>;
#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Field `INSTR` reader - Execute Instruction"]
pub type InstrR = crate::FieldReader;
#[doc = "Field `INSTR` writer - Execute Instruction"]
pub type InstrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SEQSTART` writer - Encryption/Decryption SEQUENCE Start"]
pub type SeqstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQSTOP` writer - Sequence Stop"]
pub type SeqstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQSTEP` writer - Sequence Step"]
pub type SeqstepW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Execute Instruction"]
    #[inline(always)]
    pub fn instr(&self) -> InstrR {
        InstrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Execute Instruction"]
    #[inline(always)]
    pub fn instr(&mut self) -> InstrW<'_, CmdSpec> {
        InstrW::new(self, 0)
    }
    #[doc = "Bit 9 - Encryption/Decryption SEQUENCE Start"]
    #[inline(always)]
    pub fn seqstart(&mut self) -> SeqstartW<'_, CmdSpec> {
        SeqstartW::new(self, 9)
    }
    #[doc = "Bit 10 - Sequence Stop"]
    #[inline(always)]
    pub fn seqstop(&mut self) -> SeqstopW<'_, CmdSpec> {
        SeqstopW::new(self, 10)
    }
    #[doc = "Bit 11 - Sequence Step"]
    #[inline(always)]
    pub fn seqstep(&mut self) -> SeqstepW<'_, CmdSpec> {
        SeqstepW::new(self, 11)
    }
}
#[doc = "Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdSpec;
impl crate::RegisterSpec for CmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd::R`](R) reader structure"]
impl crate::Readable for CmdSpec {}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CmdSpec {}
