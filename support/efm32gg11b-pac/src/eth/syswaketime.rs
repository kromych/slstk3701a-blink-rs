#[doc = "Register `SYSWAKETIME` reader"]
pub type R = crate::R<SYSWAKETIME_SPEC>;
#[doc = "Register `SYSWAKETIME` writer"]
pub type W = crate::W<SYSWAKETIME_SPEC>;
#[doc = "Field `SYSWAKETIME` reader - Count of 64ns, 320ns or 3200ns intervals before transmission starts after deassertion of tx_lpi_en"]
pub type SYSWAKETIME_R = crate::FieldReader<u16>;
#[doc = "Field `SYSWAKETIME` writer - Count of 64ns, 320ns or 3200ns intervals before transmission starts after deassertion of tx_lpi_en"]
pub type SYSWAKETIME_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Count of 64ns, 320ns or 3200ns intervals before transmission starts after deassertion of tx_lpi_en"]
    #[inline(always)]
    pub fn syswaketime(&self) -> SYSWAKETIME_R {
        SYSWAKETIME_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Count of 64ns, 320ns or 3200ns intervals before transmission starts after deassertion of tx_lpi_en"]
    #[inline(always)]
    #[must_use]
    pub fn syswaketime(&mut self) -> SYSWAKETIME_W<SYSWAKETIME_SPEC, 0> {
        SYSWAKETIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "System wake time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syswaketime::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syswaketime::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSWAKETIME_SPEC;
impl crate::RegisterSpec for SYSWAKETIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syswaketime::R`](R) reader structure"]
impl crate::Readable for SYSWAKETIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`syswaketime::W`](W) writer structure"]
impl crate::Writable for SYSWAKETIME_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSWAKETIME to value 0"]
impl crate::Resettable for SYSWAKETIME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
