#[doc = "Register `CMD` reader"]
pub type R = crate::R<CMD_SPEC>;
#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMD_SPEC>;
#[doc = "Field `INSTR` reader - Execute Instruction"]
pub type INSTR_R = crate::FieldReader;
#[doc = "Field `INSTR` writer - Execute Instruction"]
pub type INSTR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SEQSTART` writer - Encryption/Decryption SEQUENCE Start"]
pub type SEQSTART_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SEQSTOP` writer - Sequence Stop"]
pub type SEQSTOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SEQSTEP` writer - Sequence Step"]
pub type SEQSTEP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:7 - Execute Instruction"]
    #[inline(always)]
    pub fn instr(&self) -> INSTR_R {
        INSTR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Execute Instruction"]
    #[inline(always)]
    #[must_use]
    pub fn instr(&mut self) -> INSTR_W<CMD_SPEC, 0> {
        INSTR_W::new(self)
    }
    #[doc = "Bit 9 - Encryption/Decryption SEQUENCE Start"]
    #[inline(always)]
    #[must_use]
    pub fn seqstart(&mut self) -> SEQSTART_W<CMD_SPEC, 9> {
        SEQSTART_W::new(self)
    }
    #[doc = "Bit 10 - Sequence Stop"]
    #[inline(always)]
    #[must_use]
    pub fn seqstop(&mut self) -> SEQSTOP_W<CMD_SPEC, 10> {
        SEQSTOP_W::new(self)
    }
    #[doc = "Bit 11 - Sequence Step"]
    #[inline(always)]
    #[must_use]
    pub fn seqstep(&mut self) -> SEQSTEP_W<CMD_SPEC, 11> {
        SEQSTEP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd::R`](R) reader structure"]
impl crate::Readable for CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
