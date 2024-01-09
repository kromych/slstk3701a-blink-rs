#[doc = "Register `DDATA2` reader"]
pub type R = crate::R<DDATA2_SPEC>;
#[doc = "Register `DDATA2` writer"]
pub type W = crate::W<DDATA2_SPEC>;
#[doc = "Field `DDATA2` reader - Double Data 0 Access"]
pub type DDATA2_R = crate::FieldReader<u32>;
#[doc = "Field `DDATA2` writer - Double Data 0 Access"]
pub type DDATA2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Double Data 0 Access"]
    #[inline(always)]
    pub fn ddata2(&self) -> DDATA2_R {
        DDATA2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Double Data 0 Access"]
    #[inline(always)]
    #[must_use]
    pub fn ddata2(&mut self) -> DDATA2_W<DDATA2_SPEC> {
        DDATA2_W::new(self, 0)
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
#[doc = "DDATA2 Register Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddata2::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddata2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDATA2_SPEC;
impl crate::RegisterSpec for DDATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddata2::R`](R) reader structure"]
impl crate::Readable for DDATA2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ddata2::W`](W) writer structure"]
impl crate::Writable for DDATA2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDATA2 to value 0"]
impl crate::Resettable for DDATA2_SPEC {
    const RESET_VALUE: u32 = 0;
}
