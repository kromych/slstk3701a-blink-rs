#[doc = "Register `UPPERWRPROT` reader"]
pub type R = crate::R<UPPERWRPROT_SPEC>;
#[doc = "Register `UPPERWRPROT` writer"]
pub type W = crate::W<UPPERWRPROT_SPEC>;
#[doc = "Field `SUBSECTOR` reader - Upper Block Number"]
pub type SUBSECTOR_R = crate::FieldReader<u32>;
#[doc = "Field `SUBSECTOR` writer - Upper Block Number"]
pub type SUBSECTOR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Upper Block Number"]
    #[inline(always)]
    pub fn subsector(&self) -> SUBSECTOR_R {
        SUBSECTOR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Upper Block Number"]
    #[inline(always)]
    #[must_use]
    pub fn subsector(&mut self) -> SUBSECTOR_W<UPPERWRPROT_SPEC, 0> {
        SUBSECTOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Upper Write Protection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`upperwrprot::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`upperwrprot::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UPPERWRPROT_SPEC;
impl crate::RegisterSpec for UPPERWRPROT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`upperwrprot::R`](R) reader structure"]
impl crate::Readable for UPPERWRPROT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`upperwrprot::W`](W) writer structure"]
impl crate::Writable for UPPERWRPROT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UPPERWRPROT to value 0"]
impl crate::Resettable for UPPERWRPROT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
