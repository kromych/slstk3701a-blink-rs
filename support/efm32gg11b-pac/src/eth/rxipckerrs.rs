#[doc = "Register `RXIPCKERRS` reader"]
pub type R = crate::R<RXIPCKERRS_SPEC>;
#[doc = "Register `RXIPCKERRS` writer"]
pub type W = crate::W<RXIPCKERRS_SPEC>;
#[doc = "Field `COUNT` reader - IP header checksum errors"]
pub type COUNT_R = crate::FieldReader;
#[doc = "Field `COUNT` writer - IP header checksum errors"]
pub type COUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - IP header checksum errors"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - IP header checksum errors"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<RXIPCKERRS_SPEC> {
        COUNT_W::new(self, 0)
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
#[doc = "IP Header Checksum Errors\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipckerrs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxipckerrs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXIPCKERRS_SPEC;
impl crate::RegisterSpec for RXIPCKERRS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxipckerrs::R`](R) reader structure"]
impl crate::Readable for RXIPCKERRS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxipckerrs::W`](W) writer structure"]
impl crate::Writable for RXIPCKERRS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXIPCKERRS to value 0"]
impl crate::Resettable for RXIPCKERRS_SPEC {
    const RESET_VALUE: u32 = 0;
}
