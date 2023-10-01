#[doc = "Register `SEQ1` reader"]
pub type R = crate::R<SEQ1_SPEC>;
#[doc = "Register `SEQ1` writer"]
pub type W = crate::W<SEQ1_SPEC>;
#[doc = "Field `INSTR4` reader - Sequence Instruction 4"]
pub type INSTR4_R = crate::FieldReader;
#[doc = "Field `INSTR4` writer - Sequence Instruction 4"]
pub type INSTR4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INSTR5` reader - Sequence Instruction 5"]
pub type INSTR5_R = crate::FieldReader;
#[doc = "Field `INSTR5` writer - Sequence Instruction 5"]
pub type INSTR5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INSTR6` reader - Sequence Instruction 6"]
pub type INSTR6_R = crate::FieldReader;
#[doc = "Field `INSTR6` writer - Sequence Instruction 6"]
pub type INSTR6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INSTR7` reader - Sequence Instruction 7"]
pub type INSTR7_R = crate::FieldReader;
#[doc = "Field `INSTR7` writer - Sequence Instruction 7"]
pub type INSTR7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Sequence Instruction 4"]
    #[inline(always)]
    pub fn instr4(&self) -> INSTR4_R {
        INSTR4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Sequence Instruction 5"]
    #[inline(always)]
    pub fn instr5(&self) -> INSTR5_R {
        INSTR5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Sequence Instruction 6"]
    #[inline(always)]
    pub fn instr6(&self) -> INSTR6_R {
        INSTR6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Sequence Instruction 7"]
    #[inline(always)]
    pub fn instr7(&self) -> INSTR7_R {
        INSTR7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sequence Instruction 4"]
    #[inline(always)]
    #[must_use]
    pub fn instr4(&mut self) -> INSTR4_W<SEQ1_SPEC, 0> {
        INSTR4_W::new(self)
    }
    #[doc = "Bits 8:15 - Sequence Instruction 5"]
    #[inline(always)]
    #[must_use]
    pub fn instr5(&mut self) -> INSTR5_W<SEQ1_SPEC, 8> {
        INSTR5_W::new(self)
    }
    #[doc = "Bits 16:23 - Sequence Instruction 6"]
    #[inline(always)]
    #[must_use]
    pub fn instr6(&mut self) -> INSTR6_W<SEQ1_SPEC, 16> {
        INSTR6_W::new(self)
    }
    #[doc = "Bits 24:31 - Sequence Instruction 7"]
    #[inline(always)]
    #[must_use]
    pub fn instr7(&mut self) -> INSTR7_W<SEQ1_SPEC, 24> {
        INSTR7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Sequence Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEQ1_SPEC;
impl crate::RegisterSpec for SEQ1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seq1::R`](R) reader structure"]
impl crate::Readable for SEQ1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`seq1::W`](W) writer structure"]
impl crate::Writable for SEQ1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEQ1 to value 0"]
impl crate::Resettable for SEQ1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
