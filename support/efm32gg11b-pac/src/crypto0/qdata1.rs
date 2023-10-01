#[doc = "Register `QDATA1` reader"]
pub type R = crate::R<QDATA1_SPEC>;
#[doc = "Register `QDATA1` writer"]
pub type W = crate::W<QDATA1_SPEC>;
#[doc = "Field `QDATA1` reader - Quad Data 1 Access"]
pub type QDATA1_R = crate::FieldReader<u32>;
#[doc = "Field `QDATA1` writer - Quad Data 1 Access"]
pub type QDATA1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Quad Data 1 Access"]
    #[inline(always)]
    pub fn qdata1(&self) -> QDATA1_R {
        QDATA1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Quad Data 1 Access"]
    #[inline(always)]
    #[must_use]
    pub fn qdata1(&mut self) -> QDATA1_W<QDATA1_SPEC, 0> {
        QDATA1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "QDATA1 Register Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qdata1::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qdata1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QDATA1_SPEC;
impl crate::RegisterSpec for QDATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qdata1::R`](R) reader structure"]
impl crate::Readable for QDATA1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`qdata1::W`](W) writer structure"]
impl crate::Writable for QDATA1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets QDATA1 to value 0"]
impl crate::Resettable for QDATA1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
