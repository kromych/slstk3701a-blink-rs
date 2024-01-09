#[doc = "Register `MASKADD1TOP` reader"]
pub type R = crate::R<MASKADD1TOP_SPEC>;
#[doc = "Register `MASKADD1TOP` writer"]
pub type W = crate::W<MASKADD1TOP_SPEC>;
#[doc = "Field `ADDRMASK` reader - Specific Address Mask"]
pub type ADDRMASK_R = crate::FieldReader<u16>;
#[doc = "Field `ADDRMASK` writer - Specific Address Mask"]
pub type ADDRMASK_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Specific Address Mask"]
    #[inline(always)]
    pub fn addrmask(&self) -> ADDRMASK_R {
        ADDRMASK_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Specific Address Mask"]
    #[inline(always)]
    #[must_use]
    pub fn addrmask(&mut self) -> ADDRMASK_W<MASKADD1TOP_SPEC> {
        ADDRMASK_W::new(self, 0)
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
#[doc = "Specific Address Mask 1 Top 47:32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maskadd1top::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maskadd1top::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MASKADD1TOP_SPEC;
impl crate::RegisterSpec for MASKADD1TOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maskadd1top::R`](R) reader structure"]
impl crate::Readable for MASKADD1TOP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`maskadd1top::W`](W) writer structure"]
impl crate::Writable for MASKADD1TOP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MASKADD1TOP to value 0"]
impl crate::Resettable for MASKADD1TOP_SPEC {
    const RESET_VALUE: u32 = 0;
}
