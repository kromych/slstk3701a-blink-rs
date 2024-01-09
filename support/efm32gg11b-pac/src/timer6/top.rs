#[doc = "Register `TOP` reader"]
pub type R = crate::R<TOP_SPEC>;
#[doc = "Register `TOP` writer"]
pub type W = crate::W<TOP_SPEC>;
#[doc = "Field `TOP` reader - Counter Top Value"]
pub type TOP_R = crate::FieldReader<u32>;
#[doc = "Field `TOP` writer - Counter Top Value"]
pub type TOP_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Counter Top Value"]
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counter Top Value"]
    #[inline(always)]
    #[must_use]
    pub fn top(&mut self) -> TOP_W<TOP_SPEC> {
        TOP_W::new(self, 0)
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
#[doc = "Counter Top Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`top::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`top::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOP_SPEC;
impl crate::RegisterSpec for TOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`top::R`](R) reader structure"]
impl crate::Readable for TOP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`top::W`](W) writer structure"]
impl crate::Writable for TOP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOP to value 0xffff"]
impl crate::Resettable for TOP_SPEC {
    const RESET_VALUE: u32 = 0xffff;
}
