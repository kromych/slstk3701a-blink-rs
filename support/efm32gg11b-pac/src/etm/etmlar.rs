#[doc = "Register `ETMLAR` reader"]
pub type R = crate::R<ETMLAR_SPEC>;
#[doc = "Register `ETMLAR` writer"]
pub type W = crate::W<ETMLAR_SPEC>;
#[doc = "Field `KEY` reader - Key Value"]
pub type KEY_R = crate::BitReader;
#[doc = "Field `KEY` writer - Key Value"]
pub type KEY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Key Value"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Key Value"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<ETMLAR_SPEC, 0> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ETM Lock Access Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmlar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etmlar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETMLAR_SPEC;
impl crate::RegisterSpec for ETMLAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmlar::R`](R) reader structure"]
impl crate::Readable for ETMLAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etmlar::W`](W) writer structure"]
impl crate::Writable for ETMLAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETMLAR to value 0"]
impl crate::Resettable for ETMLAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
