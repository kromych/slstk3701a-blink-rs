#[doc = "Register `AREGA` reader"]
pub type R = crate::R<AREGA_SPEC>;
#[doc = "Register `AREGA` writer"]
pub type W = crate::W<AREGA_SPEC>;
#[doc = "Field `AREGA` reader - Animation Register a Data"]
pub type AREGA_R = crate::FieldReader;
#[doc = "Field `AREGA` writer - Animation Register a Data"]
pub type AREGA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Animation Register a Data"]
    #[inline(always)]
    pub fn arega(&self) -> AREGA_R {
        AREGA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Animation Register a Data"]
    #[inline(always)]
    #[must_use]
    pub fn arega(&mut self) -> AREGA_W<AREGA_SPEC> {
        AREGA_W::new(self, 0)
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
#[doc = "Animation Register a\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arega::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arega::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AREGA_SPEC;
impl crate::RegisterSpec for AREGA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arega::R`](R) reader structure"]
impl crate::Readable for AREGA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`arega::W`](W) writer structure"]
impl crate::Writable for AREGA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AREGA to value 0"]
impl crate::Resettable for AREGA_SPEC {
    const RESET_VALUE: u32 = 0;
}
