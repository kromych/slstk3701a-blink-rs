#[doc = "Register `KEY3` reader"]
pub type R = crate::R<KEY3_SPEC>;
#[doc = "Register `KEY3` writer"]
pub type W = crate::W<KEY3_SPEC>;
#[doc = "Field `VALUE` reader - Key 3"]
pub type VALUE_R = crate::FieldReader<u32>;
#[doc = "Field `VALUE` writer - Key 3"]
pub type VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Key 3"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key 3"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<KEY3_SPEC> {
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
#[doc = "Key Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY3_SPEC;
impl crate::RegisterSpec for KEY3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key3::R`](R) reader structure"]
impl crate::Readable for KEY3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`key3::W`](W) writer structure"]
impl crate::Writable for KEY3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY3 to value 0"]
impl crate::Resettable for KEY3_SPEC {
    const RESET_VALUE: u32 = 0;
}
