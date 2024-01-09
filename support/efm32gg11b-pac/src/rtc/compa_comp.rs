#[doc = "Register `COMPA_COMP` reader"]
pub type R = crate::R<COMPA_COMP_SPEC>;
#[doc = "Register `COMPA_COMP` writer"]
pub type W = crate::W<COMPA_COMP_SPEC>;
#[doc = "Field `COMP` reader - Compare Value"]
pub type COMP_R = crate::FieldReader<u32>;
#[doc = "Field `COMP` writer - Compare Value"]
pub type COMP_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Compare Value"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Compare Value"]
    #[inline(always)]
    #[must_use]
    pub fn comp(&mut self) -> COMP_W<COMPA_COMP_SPEC> {
        COMP_W::new(self, 0)
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
#[doc = "Compare Value Register X\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`compa_comp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`compa_comp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMPA_COMP_SPEC;
impl crate::RegisterSpec for COMPA_COMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`compa_comp::R`](R) reader structure"]
impl crate::Readable for COMPA_COMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`compa_comp::W`](W) writer structure"]
impl crate::Writable for COMPA_COMP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMPA_COMP to value 0"]
impl crate::Resettable for COMPA_COMP_SPEC {
    const RESET_VALUE: u32 = 0;
}
