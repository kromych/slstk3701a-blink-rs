#[doc = "Register `HASHTOP` reader"]
pub type R = crate::R<HASHTOP_SPEC>;
#[doc = "Register `HASHTOP` writer"]
pub type W = crate::W<HASHTOP_SPEC>;
#[doc = "Field `ADDR` reader - The remaining 32 bits of the hash address register."]
pub type ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - The remaining 32 bits of the hash address register."]
pub type ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The remaining 32 bits of the hash address register."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The remaining 32 bits of the hash address register."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<HASHTOP_SPEC> {
        ADDR_W::new(self, 0)
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
#[doc = "Hash Register Top \\[63:32\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashtop::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashtop::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASHTOP_SPEC;
impl crate::RegisterSpec for HASHTOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hashtop::R`](R) reader structure"]
impl crate::Readable for HASHTOP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hashtop::W`](W) writer structure"]
impl crate::Writable for HASHTOP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASHTOP to value 0"]
impl crate::Resettable for HASHTOP_SPEC {
    const RESET_VALUE: u32 = 0;
}
