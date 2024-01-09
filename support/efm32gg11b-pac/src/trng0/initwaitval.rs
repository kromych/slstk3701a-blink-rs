#[doc = "Register `INITWAITVAL` reader"]
pub type R = crate::R<INITWAITVAL_SPEC>;
#[doc = "Register `INITWAITVAL` writer"]
pub type W = crate::W<INITWAITVAL_SPEC>;
#[doc = "Field `VALUE` reader - Wait counter value"]
pub type VALUE_R = crate::FieldReader;
#[doc = "Field `VALUE` writer - Wait counter value"]
pub type VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Wait counter value"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Wait counter value"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<INITWAITVAL_SPEC> {
        VALUE_W::new(self, 0)
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
#[doc = "Initial Wait Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`initwaitval::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`initwaitval::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INITWAITVAL_SPEC;
impl crate::RegisterSpec for INITWAITVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`initwaitval::R`](R) reader structure"]
impl crate::Readable for INITWAITVAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`initwaitval::W`](W) writer structure"]
impl crate::Writable for INITWAITVAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INITWAITVAL to value 0xff"]
impl crate::Resettable for INITWAITVAL_SPEC {
    const RESET_VALUE: u32 = 0xff;
}
