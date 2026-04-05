#[doc = "Register `INITWAITVAL` reader"]
pub type R = crate::R<InitwaitvalSpec>;
#[doc = "Register `INITWAITVAL` writer"]
pub type W = crate::W<InitwaitvalSpec>;
#[doc = "Field `VALUE` reader - Wait counter value"]
pub type ValueR = crate::FieldReader;
#[doc = "Field `VALUE` writer - Wait counter value"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Wait counter value"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Wait counter value"]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<'_, InitwaitvalSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Initial Wait Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`initwaitval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`initwaitval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InitwaitvalSpec;
impl crate::RegisterSpec for InitwaitvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`initwaitval::R`](R) reader structure"]
impl crate::Readable for InitwaitvalSpec {}
#[doc = "`write(|w| ..)` method takes [`initwaitval::W`](W) writer structure"]
impl crate::Writable for InitwaitvalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INITWAITVAL to value 0xff"]
impl crate::Resettable for InitwaitvalSpec {
    const RESET_VALUE: u32 = 0xff;
}
