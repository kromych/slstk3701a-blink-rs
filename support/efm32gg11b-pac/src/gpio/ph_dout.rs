#[doc = "Register `PH_DOUT` reader"]
pub type R = crate::R<PH_DOUT_SPEC>;
#[doc = "Register `PH_DOUT` writer"]
pub type W = crate::W<PH_DOUT_SPEC>;
#[doc = "Field `DOUT` reader - Data Out"]
pub type DOUT_R = crate::FieldReader<u16>;
#[doc = "Field `DOUT` writer - Data Out"]
pub type DOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Data Out"]
    #[inline(always)]
    pub fn dout(&self) -> DOUT_R {
        DOUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Out"]
    #[inline(always)]
    #[must_use]
    pub fn dout(&mut self) -> DOUT_W<PH_DOUT_SPEC, 0> {
        DOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port Data Out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ph_dout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ph_dout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PH_DOUT_SPEC;
impl crate::RegisterSpec for PH_DOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ph_dout::R`](R) reader structure"]
impl crate::Readable for PH_DOUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ph_dout::W`](W) writer structure"]
impl crate::Writable for PH_DOUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PH_DOUT to value 0"]
impl crate::Resettable for PH_DOUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
