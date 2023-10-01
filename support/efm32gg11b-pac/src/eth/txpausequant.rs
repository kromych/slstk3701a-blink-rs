#[doc = "Register `TXPAUSEQUANT` reader"]
pub type R = crate::R<TXPAUSEQUANT_SPEC>;
#[doc = "Register `TXPAUSEQUANT` writer"]
pub type W = crate::W<TXPAUSEQUANT_SPEC>;
#[doc = "Field `QUANT` reader - Transmit pause quantum"]
pub type QUANT_R = crate::FieldReader<u16>;
#[doc = "Field `QUANT` writer - Transmit pause quantum"]
pub type QUANT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `QUANTP1` reader - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 1."]
pub type QUANTP1_R = crate::FieldReader<u16>;
#[doc = "Field `QUANTP1` writer - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 1."]
pub type QUANTP1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Transmit pause quantum"]
    #[inline(always)]
    pub fn quant(&self) -> QUANT_R {
        QUANT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 1."]
    #[inline(always)]
    pub fn quantp1(&self) -> QUANTP1_R {
        QUANTP1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit pause quantum"]
    #[inline(always)]
    #[must_use]
    pub fn quant(&mut self) -> QUANT_W<TXPAUSEQUANT_SPEC, 0> {
        QUANT_W::new(self)
    }
    #[doc = "Bits 16:31 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 1."]
    #[inline(always)]
    #[must_use]
    pub fn quantp1(&mut self) -> QUANTP1_W<TXPAUSEQUANT_SPEC, 16> {
        QUANTP1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transmit Pause Quantum Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txpausequant::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txpausequant::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXPAUSEQUANT_SPEC;
impl crate::RegisterSpec for TXPAUSEQUANT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txpausequant::R`](R) reader structure"]
impl crate::Readable for TXPAUSEQUANT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txpausequant::W`](W) writer structure"]
impl crate::Writable for TXPAUSEQUANT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXPAUSEQUANT to value 0xffff_ffff"]
impl crate::Resettable for TXPAUSEQUANT_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
