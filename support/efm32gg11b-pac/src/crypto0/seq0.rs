#[doc = "Register `SEQ0` reader"]
pub type R = crate::R<Seq0Spec>;
#[doc = "Register `SEQ0` writer"]
pub type W = crate::W<Seq0Spec>;
#[doc = "Field `INSTR0` reader - Sequence Instruction 0"]
pub type Instr0R = crate::FieldReader;
#[doc = "Field `INSTR0` writer - Sequence Instruction 0"]
pub type Instr0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INSTR1` reader - Sequence Instruction 1"]
pub type Instr1R = crate::FieldReader;
#[doc = "Field `INSTR1` writer - Sequence Instruction 1"]
pub type Instr1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INSTR2` reader - Sequence Instruction 2"]
pub type Instr2R = crate::FieldReader;
#[doc = "Field `INSTR2` writer - Sequence Instruction 2"]
pub type Instr2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INSTR3` reader - Sequence Instruction 3"]
pub type Instr3R = crate::FieldReader;
#[doc = "Field `INSTR3` writer - Sequence Instruction 3"]
pub type Instr3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Sequence Instruction 0"]
    #[inline(always)]
    pub fn instr0(&self) -> Instr0R {
        Instr0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Sequence Instruction 1"]
    #[inline(always)]
    pub fn instr1(&self) -> Instr1R {
        Instr1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Sequence Instruction 2"]
    #[inline(always)]
    pub fn instr2(&self) -> Instr2R {
        Instr2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Sequence Instruction 3"]
    #[inline(always)]
    pub fn instr3(&self) -> Instr3R {
        Instr3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sequence Instruction 0"]
    #[inline(always)]
    pub fn instr0(&mut self) -> Instr0W<'_, Seq0Spec> {
        Instr0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Sequence Instruction 1"]
    #[inline(always)]
    pub fn instr1(&mut self) -> Instr1W<'_, Seq0Spec> {
        Instr1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Sequence Instruction 2"]
    #[inline(always)]
    pub fn instr2(&mut self) -> Instr2W<'_, Seq0Spec> {
        Instr2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Sequence Instruction 3"]
    #[inline(always)]
    pub fn instr3(&mut self) -> Instr3W<'_, Seq0Spec> {
        Instr3W::new(self, 24)
    }
}
#[doc = "Sequence Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`seq0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Seq0Spec;
impl crate::RegisterSpec for Seq0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seq0::R`](R) reader structure"]
impl crate::Readable for Seq0Spec {}
#[doc = "`write(|w| ..)` method takes [`seq0::W`](W) writer structure"]
impl crate::Writable for Seq0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEQ0 to value 0"]
impl crate::Resettable for Seq0Spec {}
