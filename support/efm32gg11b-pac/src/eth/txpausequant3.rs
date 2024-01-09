#[doc = "Register `TXPAUSEQUANT3` reader"]
pub type R = crate::R<TXPAUSEQUANT3_SPEC>;
#[doc = "Register `TXPAUSEQUANT3` writer"]
pub type W = crate::W<TXPAUSEQUANT3_SPEC>;
#[doc = "Field `QUANTP6` reader - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 6."]
pub type QUANTP6_R = crate::FieldReader<u16>;
#[doc = "Field `QUANTP6` writer - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 6."]
pub type QUANTP6_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `QUANTP7` reader - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 7."]
pub type QUANTP7_R = crate::FieldReader<u16>;
#[doc = "Field `QUANTP7` writer - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 7."]
pub type QUANTP7_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 6."]
    #[inline(always)]
    pub fn quantp6(&self) -> QUANTP6_R {
        QUANTP6_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 7."]
    #[inline(always)]
    pub fn quantp7(&self) -> QUANTP7_R {
        QUANTP7_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 6."]
    #[inline(always)]
    #[must_use]
    pub fn quantp6(&mut self) -> QUANTP6_W<TXPAUSEQUANT3_SPEC> {
        QUANTP6_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 7."]
    #[inline(always)]
    #[must_use]
    pub fn quantp7(&mut self) -> QUANTP7_W<TXPAUSEQUANT3_SPEC> {
        QUANTP7_W::new(self, 16)
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
#[doc = "Transmit Pause Quantum Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txpausequant3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txpausequant3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXPAUSEQUANT3_SPEC;
impl crate::RegisterSpec for TXPAUSEQUANT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txpausequant3::R`](R) reader structure"]
impl crate::Readable for TXPAUSEQUANT3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txpausequant3::W`](W) writer structure"]
impl crate::Writable for TXPAUSEQUANT3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXPAUSEQUANT3 to value 0xffff_ffff"]
impl crate::Resettable for TXPAUSEQUANT3_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
