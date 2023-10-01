#[doc = "Register `OPCODEEXTUPPER` reader"]
pub type R = crate::R<OPCODEEXTUPPER_SPEC>;
#[doc = "Register `OPCODEEXTUPPER` writer"]
pub type W = crate::W<OPCODEEXTUPPER_SPEC>;
#[doc = "Field `EXTWELOPCODE` reader - WEL Opcode Extension"]
pub type EXTWELOPCODE_R = crate::FieldReader;
#[doc = "Field `EXTWELOPCODE` writer - WEL Opcode Extension"]
pub type EXTWELOPCODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `WELOPCODE` reader - WEL Opcode"]
pub type WELOPCODE_R = crate::FieldReader;
#[doc = "Field `WELOPCODE` writer - WEL Opcode"]
pub type WELOPCODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 16:23 - WEL Opcode Extension"]
    #[inline(always)]
    pub fn extwelopcode(&self) -> EXTWELOPCODE_R {
        EXTWELOPCODE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - WEL Opcode"]
    #[inline(always)]
    pub fn welopcode(&self) -> WELOPCODE_R {
        WELOPCODE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - WEL Opcode Extension"]
    #[inline(always)]
    #[must_use]
    pub fn extwelopcode(&mut self) -> EXTWELOPCODE_W<OPCODEEXTUPPER_SPEC, 16> {
        EXTWELOPCODE_W::new(self)
    }
    #[doc = "Bits 24:31 - WEL Opcode"]
    #[inline(always)]
    #[must_use]
    pub fn welopcode(&mut self) -> WELOPCODE_W<OPCODEEXTUPPER_SPEC, 24> {
        WELOPCODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Opcode Extension Register (Upper)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opcodeextupper::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opcodeextupper::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPCODEEXTUPPER_SPEC;
impl crate::RegisterSpec for OPCODEEXTUPPER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opcodeextupper::R`](R) reader structure"]
impl crate::Readable for OPCODEEXTUPPER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`opcodeextupper::W`](W) writer structure"]
impl crate::Writable for OPCODEEXTUPPER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPCODEEXTUPPER to value 0x06f9_0000"]
impl crate::Resettable for OPCODEEXTUPPER_SPEC {
    const RESET_VALUE: Self::Ux = 0x06f9_0000;
}
