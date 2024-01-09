#[doc = "Register `SEQ4` reader"]
pub type R = crate::R<SEQ4_SPEC>;
#[doc = "Register `SEQ4` writer"]
pub type W = crate::W<SEQ4_SPEC>;
#[doc = "Field `INSTR16` reader - Sequence Instruction 16"]
pub type INSTR16_R = crate::FieldReader;
#[doc = "Field `INSTR16` writer - Sequence Instruction 16"]
pub type INSTR16_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INSTR17` reader - Sequence Instruction 17"]
pub type INSTR17_R = crate::FieldReader;
#[doc = "Field `INSTR17` writer - Sequence Instruction 17"]
pub type INSTR17_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INSTR18` reader - Sequence Instruction 18"]
pub type INSTR18_R = crate::FieldReader;
#[doc = "Field `INSTR18` writer - Sequence Instruction 18"]
pub type INSTR18_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INSTR19` reader - Sequence Instruction 19"]
pub type INSTR19_R = crate::FieldReader;
#[doc = "Field `INSTR19` writer - Sequence Instruction 19"]
pub type INSTR19_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Sequence Instruction 16"]
    #[inline(always)]
    pub fn instr16(&self) -> INSTR16_R {
        INSTR16_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Sequence Instruction 17"]
    #[inline(always)]
    pub fn instr17(&self) -> INSTR17_R {
        INSTR17_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Sequence Instruction 18"]
    #[inline(always)]
    pub fn instr18(&self) -> INSTR18_R {
        INSTR18_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Sequence Instruction 19"]
    #[inline(always)]
    pub fn instr19(&self) -> INSTR19_R {
        INSTR19_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sequence Instruction 16"]
    #[inline(always)]
    #[must_use]
    pub fn instr16(&mut self) -> INSTR16_W<SEQ4_SPEC> {
        INSTR16_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Sequence Instruction 17"]
    #[inline(always)]
    #[must_use]
    pub fn instr17(&mut self) -> INSTR17_W<SEQ4_SPEC> {
        INSTR17_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Sequence Instruction 18"]
    #[inline(always)]
    #[must_use]
    pub fn instr18(&mut self) -> INSTR18_W<SEQ4_SPEC> {
        INSTR18_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Sequence Instruction 19"]
    #[inline(always)]
    #[must_use]
    pub fn instr19(&mut self) -> INSTR19_W<SEQ4_SPEC> {
        INSTR19_W::new(self, 24)
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
#[doc = "Sequence Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEQ4_SPEC;
impl crate::RegisterSpec for SEQ4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seq4::R`](R) reader structure"]
impl crate::Readable for SEQ4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`seq4::W`](W) writer structure"]
impl crate::Writable for SEQ4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEQ4 to value 0"]
impl crate::Resettable for SEQ4_SPEC {
    const RESET_VALUE: u32 = 0;
}
