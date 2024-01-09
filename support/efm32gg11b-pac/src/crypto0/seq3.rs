#[doc = "Register `SEQ3` reader"]
pub type R = crate::R<SEQ3_SPEC>;
#[doc = "Register `SEQ3` writer"]
pub type W = crate::W<SEQ3_SPEC>;
#[doc = "Field `INSTR12` reader - Sequence Instruction 12"]
pub type INSTR12_R = crate::FieldReader;
#[doc = "Field `INSTR12` writer - Sequence Instruction 12"]
pub type INSTR12_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INSTR13` reader - Sequence Instruction 13"]
pub type INSTR13_R = crate::FieldReader;
#[doc = "Field `INSTR13` writer - Sequence Instruction 13"]
pub type INSTR13_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INSTR14` reader - Sequence Instruction 14"]
pub type INSTR14_R = crate::FieldReader;
#[doc = "Field `INSTR14` writer - Sequence Instruction 14"]
pub type INSTR14_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INSTR15` reader - Sequence Instruction 15"]
pub type INSTR15_R = crate::FieldReader;
#[doc = "Field `INSTR15` writer - Sequence Instruction 15"]
pub type INSTR15_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Sequence Instruction 12"]
    #[inline(always)]
    pub fn instr12(&self) -> INSTR12_R {
        INSTR12_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Sequence Instruction 13"]
    #[inline(always)]
    pub fn instr13(&self) -> INSTR13_R {
        INSTR13_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Sequence Instruction 14"]
    #[inline(always)]
    pub fn instr14(&self) -> INSTR14_R {
        INSTR14_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Sequence Instruction 15"]
    #[inline(always)]
    pub fn instr15(&self) -> INSTR15_R {
        INSTR15_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sequence Instruction 12"]
    #[inline(always)]
    #[must_use]
    pub fn instr12(&mut self) -> INSTR12_W<SEQ3_SPEC> {
        INSTR12_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Sequence Instruction 13"]
    #[inline(always)]
    #[must_use]
    pub fn instr13(&mut self) -> INSTR13_W<SEQ3_SPEC> {
        INSTR13_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Sequence Instruction 14"]
    #[inline(always)]
    #[must_use]
    pub fn instr14(&mut self) -> INSTR14_W<SEQ3_SPEC> {
        INSTR14_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Sequence Instruction 15"]
    #[inline(always)]
    #[must_use]
    pub fn instr15(&mut self) -> INSTR15_W<SEQ3_SPEC> {
        INSTR15_W::new(self, 24)
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
#[doc = "Sequence Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEQ3_SPEC;
impl crate::RegisterSpec for SEQ3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seq3::R`](R) reader structure"]
impl crate::Readable for SEQ3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`seq3::W`](W) writer structure"]
impl crate::Writable for SEQ3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEQ3 to value 0"]
impl crate::Resettable for SEQ3_SPEC {
    const RESET_VALUE: u32 = 0;
}
