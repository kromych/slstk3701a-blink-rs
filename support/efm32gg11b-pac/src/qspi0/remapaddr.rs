#[doc = "Register `REMAPADDR` reader"]
pub type R = crate::R<REMAPADDR_SPEC>;
#[doc = "Register `REMAPADDR` writer"]
pub type W = crate::W<REMAPADDR_SPEC>;
#[doc = "Field `VALUE` reader - Remap Address Value"]
pub type VALUE_R = crate::FieldReader<u32>;
#[doc = "Field `VALUE` writer - Remap Address Value"]
pub type VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Remap Address Value"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Remap Address Value"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<REMAPADDR_SPEC> {
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
#[doc = "Remap Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remapaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remapaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REMAPADDR_SPEC;
impl crate::RegisterSpec for REMAPADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remapaddr::R`](R) reader structure"]
impl crate::Readable for REMAPADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`remapaddr::W`](W) writer structure"]
impl crate::Writable for REMAPADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REMAPADDR to value 0"]
impl crate::Resettable for REMAPADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
