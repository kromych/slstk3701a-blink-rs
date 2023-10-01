#[doc = "Register `OPCODEEXTLOWER` reader"]
pub type R = crate::R<OPCODEEXTLOWER_SPEC>;
#[doc = "Register `OPCODEEXTLOWER` writer"]
pub type W = crate::W<OPCODEEXTLOWER_SPEC>;
#[doc = "Field `EXTSTIGOPCODE` reader - STIG Opcode Extension"]
pub type EXTSTIGOPCODE_R = crate::FieldReader;
#[doc = "Field `EXTSTIGOPCODE` writer - STIG Opcode Extension"]
pub type EXTSTIGOPCODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `EXTPOLLOPCODE` reader - Polling Opcode Extension"]
pub type EXTPOLLOPCODE_R = crate::FieldReader;
#[doc = "Field `EXTPOLLOPCODE` writer - Polling Opcode Extension"]
pub type EXTPOLLOPCODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `EXTWRITEOPCODE` reader - Write Opcode Extension"]
pub type EXTWRITEOPCODE_R = crate::FieldReader;
#[doc = "Field `EXTWRITEOPCODE` writer - Write Opcode Extension"]
pub type EXTWRITEOPCODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `EXTREADOPCODE` reader - Read Opcode Extension"]
pub type EXTREADOPCODE_R = crate::FieldReader;
#[doc = "Field `EXTREADOPCODE` writer - Read Opcode Extension"]
pub type EXTREADOPCODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - STIG Opcode Extension"]
    #[inline(always)]
    pub fn extstigopcode(&self) -> EXTSTIGOPCODE_R {
        EXTSTIGOPCODE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Polling Opcode Extension"]
    #[inline(always)]
    pub fn extpollopcode(&self) -> EXTPOLLOPCODE_R {
        EXTPOLLOPCODE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Write Opcode Extension"]
    #[inline(always)]
    pub fn extwriteopcode(&self) -> EXTWRITEOPCODE_R {
        EXTWRITEOPCODE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Read Opcode Extension"]
    #[inline(always)]
    pub fn extreadopcode(&self) -> EXTREADOPCODE_R {
        EXTREADOPCODE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - STIG Opcode Extension"]
    #[inline(always)]
    #[must_use]
    pub fn extstigopcode(&mut self) -> EXTSTIGOPCODE_W<OPCODEEXTLOWER_SPEC, 0> {
        EXTSTIGOPCODE_W::new(self)
    }
    #[doc = "Bits 8:15 - Polling Opcode Extension"]
    #[inline(always)]
    #[must_use]
    pub fn extpollopcode(&mut self) -> EXTPOLLOPCODE_W<OPCODEEXTLOWER_SPEC, 8> {
        EXTPOLLOPCODE_W::new(self)
    }
    #[doc = "Bits 16:23 - Write Opcode Extension"]
    #[inline(always)]
    #[must_use]
    pub fn extwriteopcode(&mut self) -> EXTWRITEOPCODE_W<OPCODEEXTLOWER_SPEC, 16> {
        EXTWRITEOPCODE_W::new(self)
    }
    #[doc = "Bits 24:31 - Read Opcode Extension"]
    #[inline(always)]
    #[must_use]
    pub fn extreadopcode(&mut self) -> EXTREADOPCODE_W<OPCODEEXTLOWER_SPEC, 24> {
        EXTREADOPCODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Opcode Extension Register (Lower)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opcodeextlower::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opcodeextlower::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPCODEEXTLOWER_SPEC;
impl crate::RegisterSpec for OPCODEEXTLOWER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opcodeextlower::R`](R) reader structure"]
impl crate::Readable for OPCODEEXTLOWER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`opcodeextlower::W`](W) writer structure"]
impl crate::Writable for OPCODEEXTLOWER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPCODEEXTLOWER to value 0x13ed_fa00"]
impl crate::Resettable for OPCODEEXTLOWER_SPEC {
    const RESET_VALUE: Self::Ux = 0x13ed_fa00;
}
