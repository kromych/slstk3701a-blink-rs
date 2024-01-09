#[doc = "Register `QDATA0` reader"]
pub type R = crate::R<QDATA0_SPEC>;
#[doc = "Register `QDATA0` writer"]
pub type W = crate::W<QDATA0_SPEC>;
#[doc = "Field `QDATA0` reader - Quad Data 0 Access"]
pub type QDATA0_R = crate::FieldReader<u32>;
#[doc = "Field `QDATA0` writer - Quad Data 0 Access"]
pub type QDATA0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Quad Data 0 Access"]
    #[inline(always)]
    pub fn qdata0(&self) -> QDATA0_R {
        QDATA0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Quad Data 0 Access"]
    #[inline(always)]
    #[must_use]
    pub fn qdata0(&mut self) -> QDATA0_W<QDATA0_SPEC> {
        QDATA0_W::new(self, 0)
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
#[doc = "QDATA0 Register Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qdata0::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qdata0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QDATA0_SPEC;
impl crate::RegisterSpec for QDATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qdata0::R`](R) reader structure"]
impl crate::Readable for QDATA0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`qdata0::W`](W) writer structure"]
impl crate::Writable for QDATA0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QDATA0 to value 0"]
impl crate::Resettable for QDATA0_SPEC {
    const RESET_VALUE: u32 = 0;
}
