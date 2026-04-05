#[doc = "Register `OPCODEEXTLOWER` reader"]
pub type R = crate::R<OpcodeextlowerSpec>;
#[doc = "Register `OPCODEEXTLOWER` writer"]
pub type W = crate::W<OpcodeextlowerSpec>;
#[doc = "Field `EXTSTIGOPCODE` reader - STIG Opcode Extension"]
pub type ExtstigopcodeR = crate::FieldReader;
#[doc = "Field `EXTSTIGOPCODE` writer - STIG Opcode Extension"]
pub type ExtstigopcodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EXTPOLLOPCODE` reader - Polling Opcode Extension"]
pub type ExtpollopcodeR = crate::FieldReader;
#[doc = "Field `EXTPOLLOPCODE` writer - Polling Opcode Extension"]
pub type ExtpollopcodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EXTWRITEOPCODE` reader - Write Opcode Extension"]
pub type ExtwriteopcodeR = crate::FieldReader;
#[doc = "Field `EXTWRITEOPCODE` writer - Write Opcode Extension"]
pub type ExtwriteopcodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EXTREADOPCODE` reader - Read Opcode Extension"]
pub type ExtreadopcodeR = crate::FieldReader;
#[doc = "Field `EXTREADOPCODE` writer - Read Opcode Extension"]
pub type ExtreadopcodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - STIG Opcode Extension"]
    #[inline(always)]
    pub fn extstigopcode(&self) -> ExtstigopcodeR {
        ExtstigopcodeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Polling Opcode Extension"]
    #[inline(always)]
    pub fn extpollopcode(&self) -> ExtpollopcodeR {
        ExtpollopcodeR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Write Opcode Extension"]
    #[inline(always)]
    pub fn extwriteopcode(&self) -> ExtwriteopcodeR {
        ExtwriteopcodeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Read Opcode Extension"]
    #[inline(always)]
    pub fn extreadopcode(&self) -> ExtreadopcodeR {
        ExtreadopcodeR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - STIG Opcode Extension"]
    #[inline(always)]
    pub fn extstigopcode(&mut self) -> ExtstigopcodeW<'_, OpcodeextlowerSpec> {
        ExtstigopcodeW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Polling Opcode Extension"]
    #[inline(always)]
    pub fn extpollopcode(&mut self) -> ExtpollopcodeW<'_, OpcodeextlowerSpec> {
        ExtpollopcodeW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Write Opcode Extension"]
    #[inline(always)]
    pub fn extwriteopcode(&mut self) -> ExtwriteopcodeW<'_, OpcodeextlowerSpec> {
        ExtwriteopcodeW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Read Opcode Extension"]
    #[inline(always)]
    pub fn extreadopcode(&mut self) -> ExtreadopcodeW<'_, OpcodeextlowerSpec> {
        ExtreadopcodeW::new(self, 24)
    }
}
#[doc = "Opcode Extension Register (Lower)\n\nYou can [`read`](crate::Reg::read) this register and get [`opcodeextlower::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opcodeextlower::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OpcodeextlowerSpec;
impl crate::RegisterSpec for OpcodeextlowerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opcodeextlower::R`](R) reader structure"]
impl crate::Readable for OpcodeextlowerSpec {}
#[doc = "`write(|w| ..)` method takes [`opcodeextlower::W`](W) writer structure"]
impl crate::Writable for OpcodeextlowerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OPCODEEXTLOWER to value 0x13ed_fa00"]
impl crate::Resettable for OpcodeextlowerSpec {
    const RESET_VALUE: u32 = 0x13ed_fa00;
}
