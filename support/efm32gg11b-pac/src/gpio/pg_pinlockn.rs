#[doc = "Register `PG_PINLOCKN` reader"]
pub type R = crate::R<PG_PINLOCKN_SPEC>;
#[doc = "Register `PG_PINLOCKN` writer"]
pub type W = crate::W<PG_PINLOCKN_SPEC>;
#[doc = "Field `PINLOCKN` reader - Unlocked Pins"]
pub type PINLOCKN_R = crate::FieldReader<u16>;
#[doc = "Field `PINLOCKN` writer - Unlocked Pins"]
pub type PINLOCKN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Unlocked Pins"]
    #[inline(always)]
    pub fn pinlockn(&self) -> PINLOCKN_R {
        PINLOCKN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Unlocked Pins"]
    #[inline(always)]
    #[must_use]
    pub fn pinlockn(&mut self) -> PINLOCKN_W<PG_PINLOCKN_SPEC, 0> {
        PINLOCKN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port Unlocked Pins Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pg_pinlockn::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pg_pinlockn::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PG_PINLOCKN_SPEC;
impl crate::RegisterSpec for PG_PINLOCKN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pg_pinlockn::R`](R) reader structure"]
impl crate::Readable for PG_PINLOCKN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pg_pinlockn::W`](W) writer structure"]
impl crate::Writable for PG_PINLOCKN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PG_PINLOCKN to value 0xffff"]
impl crate::Resettable for PG_PINLOCKN_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
