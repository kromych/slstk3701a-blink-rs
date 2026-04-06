#[doc = "Register `SEQ3` reader"]
pub type R = crate::R<Seq3Spec>;
#[doc = "Register `SEQ3` writer"]
pub type W = crate::W<Seq3Spec>;
#[doc = "Field `INSTR12` reader - Sequence Instruction 12"]
pub type Instr12R = crate::FieldReader;
#[doc = "Field `INSTR12` writer - Sequence Instruction 12"]
pub type Instr12W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INSTR13` reader - Sequence Instruction 13"]
pub type Instr13R = crate::FieldReader;
#[doc = "Field `INSTR13` writer - Sequence Instruction 13"]
pub type Instr13W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INSTR14` reader - Sequence Instruction 14"]
pub type Instr14R = crate::FieldReader;
#[doc = "Field `INSTR14` writer - Sequence Instruction 14"]
pub type Instr14W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INSTR15` reader - Sequence Instruction 15"]
pub type Instr15R = crate::FieldReader;
#[doc = "Field `INSTR15` writer - Sequence Instruction 15"]
pub type Instr15W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Sequence Instruction 12"]
    #[inline(always)]
    pub fn instr12(&self) -> Instr12R {
        Instr12R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Sequence Instruction 13"]
    #[inline(always)]
    pub fn instr13(&self) -> Instr13R {
        Instr13R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Sequence Instruction 14"]
    #[inline(always)]
    pub fn instr14(&self) -> Instr14R {
        Instr14R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Sequence Instruction 15"]
    #[inline(always)]
    pub fn instr15(&self) -> Instr15R {
        Instr15R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sequence Instruction 12"]
    #[inline(always)]
    pub fn instr12(&mut self) -> Instr12W<'_, Seq3Spec> {
        Instr12W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Sequence Instruction 13"]
    #[inline(always)]
    pub fn instr13(&mut self) -> Instr13W<'_, Seq3Spec> {
        Instr13W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Sequence Instruction 14"]
    #[inline(always)]
    pub fn instr14(&mut self) -> Instr14W<'_, Seq3Spec> {
        Instr14W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Sequence Instruction 15"]
    #[inline(always)]
    pub fn instr15(&mut self) -> Instr15W<'_, Seq3Spec> {
        Instr15W::new(self, 24)
    }
}
#[doc = "Sequence Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`seq3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Seq3Spec;
impl crate::RegisterSpec for Seq3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seq3::R`](R) reader structure"]
impl crate::Readable for Seq3Spec {}
#[doc = "`write(|w| ..)` method takes [`seq3::W`](W) writer structure"]
impl crate::Writable for Seq3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEQ3 to value 0"]
impl crate::Resettable for Seq3Spec {}
