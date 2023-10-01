#[doc = "Register `SEQ0` reader"]
pub type R = crate::R<SEQ0_SPEC>;
#[doc = "Register `SEQ0` writer"]
pub type W = crate::W<SEQ0_SPEC>;
#[doc = "Field `INSTR0` reader - Sequence Instruction 0"]
pub type INSTR0_R = crate::FieldReader;
#[doc = "Field `INSTR0` writer - Sequence Instruction 0"]
pub type INSTR0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INSTR1` reader - Sequence Instruction 1"]
pub type INSTR1_R = crate::FieldReader;
#[doc = "Field `INSTR1` writer - Sequence Instruction 1"]
pub type INSTR1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INSTR2` reader - Sequence Instruction 2"]
pub type INSTR2_R = crate::FieldReader;
#[doc = "Field `INSTR2` writer - Sequence Instruction 2"]
pub type INSTR2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INSTR3` reader - Sequence Instruction 3"]
pub type INSTR3_R = crate::FieldReader;
#[doc = "Field `INSTR3` writer - Sequence Instruction 3"]
pub type INSTR3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Sequence Instruction 0"]
    #[inline(always)]
    pub fn instr0(&self) -> INSTR0_R {
        INSTR0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Sequence Instruction 1"]
    #[inline(always)]
    pub fn instr1(&self) -> INSTR1_R {
        INSTR1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Sequence Instruction 2"]
    #[inline(always)]
    pub fn instr2(&self) -> INSTR2_R {
        INSTR2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Sequence Instruction 3"]
    #[inline(always)]
    pub fn instr3(&self) -> INSTR3_R {
        INSTR3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sequence Instruction 0"]
    #[inline(always)]
    #[must_use]
    pub fn instr0(&mut self) -> INSTR0_W<SEQ0_SPEC, 0> {
        INSTR0_W::new(self)
    }
    #[doc = "Bits 8:15 - Sequence Instruction 1"]
    #[inline(always)]
    #[must_use]
    pub fn instr1(&mut self) -> INSTR1_W<SEQ0_SPEC, 8> {
        INSTR1_W::new(self)
    }
    #[doc = "Bits 16:23 - Sequence Instruction 2"]
    #[inline(always)]
    #[must_use]
    pub fn instr2(&mut self) -> INSTR2_W<SEQ0_SPEC, 16> {
        INSTR2_W::new(self)
    }
    #[doc = "Bits 24:31 - Sequence Instruction 3"]
    #[inline(always)]
    #[must_use]
    pub fn instr3(&mut self) -> INSTR3_W<SEQ0_SPEC, 24> {
        INSTR3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Sequence Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEQ0_SPEC;
impl crate::RegisterSpec for SEQ0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seq0::R`](R) reader structure"]
impl crate::Readable for SEQ0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`seq0::W`](W) writer structure"]
impl crate::Writable for SEQ0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEQ0 to value 0"]
impl crate::Resettable for SEQ0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
