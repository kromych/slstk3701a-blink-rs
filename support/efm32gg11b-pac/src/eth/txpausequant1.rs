#[doc = "Register `TXPAUSEQUANT1` reader"]
pub type R = crate::R<TXPAUSEQUANT1_SPEC>;
#[doc = "Register `TXPAUSEQUANT1` writer"]
pub type W = crate::W<TXPAUSEQUANT1_SPEC>;
#[doc = "Field `QUANTP2` reader - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 2."]
pub type QUANTP2_R = crate::FieldReader<u16>;
#[doc = "Field `QUANTP2` writer - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 2."]
pub type QUANTP2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `QUANTP3` reader - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 3."]
pub type QUANTP3_R = crate::FieldReader<u16>;
#[doc = "Field `QUANTP3` writer - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 3."]
pub type QUANTP3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 2."]
    #[inline(always)]
    pub fn quantp2(&self) -> QUANTP2_R {
        QUANTP2_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 3."]
    #[inline(always)]
    pub fn quantp3(&self) -> QUANTP3_R {
        QUANTP3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 2."]
    #[inline(always)]
    #[must_use]
    pub fn quantp2(&mut self) -> QUANTP2_W<TXPAUSEQUANT1_SPEC, 0> {
        QUANTP2_W::new(self)
    }
    #[doc = "Bits 16:31 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 3."]
    #[inline(always)]
    #[must_use]
    pub fn quantp3(&mut self) -> QUANTP3_W<TXPAUSEQUANT1_SPEC, 16> {
        QUANTP3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transmit Pause Quantum Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txpausequant1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txpausequant1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXPAUSEQUANT1_SPEC;
impl crate::RegisterSpec for TXPAUSEQUANT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txpausequant1::R`](R) reader structure"]
impl crate::Readable for TXPAUSEQUANT1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txpausequant1::W`](W) writer structure"]
impl crate::Writable for TXPAUSEQUANT1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXPAUSEQUANT1 to value 0xffff_ffff"]
impl crate::Resettable for TXPAUSEQUANT1_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
