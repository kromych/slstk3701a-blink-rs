#[doc = "Register `KEY2` reader"]
pub type R = crate::R<KEY2_SPEC>;
#[doc = "Register `KEY2` writer"]
pub type W = crate::W<KEY2_SPEC>;
#[doc = "Field `VALUE` reader - Key 2"]
pub type VALUE_R = crate::FieldReader<u32>;
#[doc = "Field `VALUE` writer - Key 2"]
pub type VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Key 2"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key 2"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<KEY2_SPEC> {
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
#[doc = "Key Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY2_SPEC;
impl crate::RegisterSpec for KEY2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key2::R`](R) reader structure"]
impl crate::Readable for KEY2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`key2::W`](W) writer structure"]
impl crate::Writable for KEY2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY2 to value 0"]
impl crate::Resettable for KEY2_SPEC {
    const RESET_VALUE: u32 = 0;
}
