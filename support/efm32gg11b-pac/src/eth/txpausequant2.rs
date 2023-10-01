#[doc = "Register `TXPAUSEQUANT2` reader"]
pub type R = crate::R<TXPAUSEQUANT2_SPEC>;
#[doc = "Register `TXPAUSEQUANT2` writer"]
pub type W = crate::W<TXPAUSEQUANT2_SPEC>;
#[doc = "Field `QUANTP4` reader - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 4."]
pub type QUANTP4_R = crate::FieldReader<u16>;
#[doc = "Field `QUANTP4` writer - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 4."]
pub type QUANTP4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `QUANTP5` reader - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 5."]
pub type QUANTP5_R = crate::FieldReader<u16>;
#[doc = "Field `QUANTP5` writer - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 5."]
pub type QUANTP5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 4."]
    #[inline(always)]
    pub fn quantp4(&self) -> QUANTP4_R {
        QUANTP4_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 5."]
    #[inline(always)]
    pub fn quantp5(&self) -> QUANTP5_R {
        QUANTP5_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 4."]
    #[inline(always)]
    #[must_use]
    pub fn quantp4(&mut self) -> QUANTP4_W<TXPAUSEQUANT2_SPEC, 0> {
        QUANTP4_W::new(self)
    }
    #[doc = "Bits 16:31 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 5."]
    #[inline(always)]
    #[must_use]
    pub fn quantp5(&mut self) -> QUANTP5_W<TXPAUSEQUANT2_SPEC, 16> {
        QUANTP5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transmit Pause Quantum Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txpausequant2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txpausequant2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXPAUSEQUANT2_SPEC;
impl crate::RegisterSpec for TXPAUSEQUANT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txpausequant2::R`](R) reader structure"]
impl crate::Readable for TXPAUSEQUANT2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txpausequant2::W`](W) writer structure"]
impl crate::Writable for TXPAUSEQUANT2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXPAUSEQUANT2 to value 0xffff_ffff"]
impl crate::Resettable for TXPAUSEQUANT2_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
