#[doc = "Register `JUMBOMAXLEN` reader"]
pub type R = crate::R<JUMBOMAXLEN_SPEC>;
#[doc = "Register `JUMBOMAXLEN` writer"]
pub type W = crate::W<JUMBOMAXLEN_SPEC>;
#[doc = "Field `JUMBOMAXLEN` reader - Maximum Jumbo Frame Size - resets to the gem_jumbo_max_length define value."]
pub type JUMBOMAXLEN_R = crate::FieldReader<u16>;
#[doc = "Field `JUMBOMAXLEN` writer - Maximum Jumbo Frame Size - resets to the gem_jumbo_max_length define value."]
pub type JUMBOMAXLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Maximum Jumbo Frame Size - resets to the gem_jumbo_max_length define value."]
    #[inline(always)]
    pub fn jumbomaxlen(&self) -> JUMBOMAXLEN_R {
        JUMBOMAXLEN_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Maximum Jumbo Frame Size - resets to the gem_jumbo_max_length define value."]
    #[inline(always)]
    #[must_use]
    pub fn jumbomaxlen(&mut self) -> JUMBOMAXLEN_W<JUMBOMAXLEN_SPEC> {
        JUMBOMAXLEN_W::new(self, 0)
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
#[doc = "Maximum Jumbo Frame Size.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jumbomaxlen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jumbomaxlen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JUMBOMAXLEN_SPEC;
impl crate::RegisterSpec for JUMBOMAXLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jumbomaxlen::R`](R) reader structure"]
impl crate::Readable for JUMBOMAXLEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`jumbomaxlen::W`](W) writer structure"]
impl crate::Writable for JUMBOMAXLEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets JUMBOMAXLEN to value 0x2800"]
impl crate::Resettable for JUMBOMAXLEN_SPEC {
    const RESET_VALUE: u32 = 0x2800;
}
