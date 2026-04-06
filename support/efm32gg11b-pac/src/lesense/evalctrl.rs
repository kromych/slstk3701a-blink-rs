#[doc = "Register `EVALCTRL` reader"]
pub type R = crate::R<EvalctrlSpec>;
#[doc = "Register `EVALCTRL` writer"]
pub type W = crate::W<EvalctrlSpec>;
#[doc = "Field `WINSIZE` reader - Sliding Window and Step Detection Size"]
pub type WinsizeR = crate::FieldReader<u16>;
#[doc = "Field `WINSIZE` writer - Sliding Window and Step Detection Size"]
pub type WinsizeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Sliding Window and Step Detection Size"]
    #[inline(always)]
    pub fn winsize(&self) -> WinsizeR {
        WinsizeR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Sliding Window and Step Detection Size"]
    #[inline(always)]
    pub fn winsize(&mut self) -> WinsizeW<'_, EvalctrlSpec> {
        WinsizeW::new(self, 0)
    }
}
#[doc = "LESENSE Evaluation Control\n\nYou can [`read`](crate::Reg::read) this register and get [`evalctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evalctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvalctrlSpec;
impl crate::RegisterSpec for EvalctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evalctrl::R`](R) reader structure"]
impl crate::Readable for EvalctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`evalctrl::W`](W) writer structure"]
impl crate::Writable for EvalctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVALCTRL to value 0"]
impl crate::Resettable for EvalctrlSpec {}
