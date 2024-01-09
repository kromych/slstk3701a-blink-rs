#[doc = "Register `QDATA1BIG` reader"]
pub type R = crate::R<QDATA1BIG_SPEC>;
#[doc = "Register `QDATA1BIG` writer"]
pub type W = crate::W<QDATA1BIG_SPEC>;
#[doc = "Field `QDATA1BIG` reader - Quad Data 1 Big Endian Access"]
pub type QDATA1BIG_R = crate::FieldReader<u32>;
#[doc = "Field `QDATA1BIG` writer - Quad Data 1 Big Endian Access"]
pub type QDATA1BIG_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Quad Data 1 Big Endian Access"]
    #[inline(always)]
    pub fn qdata1big(&self) -> QDATA1BIG_R {
        QDATA1BIG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Quad Data 1 Big Endian Access"]
    #[inline(always)]
    #[must_use]
    pub fn qdata1big(&mut self) -> QDATA1BIG_W<QDATA1BIG_SPEC> {
        QDATA1BIG_W::new(self, 0)
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
#[doc = "QDATA1 Register Big Endian Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qdata1big::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qdata1big::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QDATA1BIG_SPEC;
impl crate::RegisterSpec for QDATA1BIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qdata1big::R`](R) reader structure"]
impl crate::Readable for QDATA1BIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`qdata1big::W`](W) writer structure"]
impl crate::Writable for QDATA1BIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QDATA1BIG to value 0"]
impl crate::Resettable for QDATA1BIG_SPEC {
    const RESET_VALUE: u32 = 0;
}
