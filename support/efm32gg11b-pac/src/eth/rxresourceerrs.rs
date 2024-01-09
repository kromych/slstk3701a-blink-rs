#[doc = "Register `RXRESOURCEERRS` reader"]
pub type R = crate::R<RXRESOURCEERRS_SPEC>;
#[doc = "Register `RXRESOURCEERRS` writer"]
pub type W = crate::W<RXRESOURCEERRS_SPEC>;
#[doc = "Field `COUNT` reader - Receive resource errors"]
pub type COUNT_R = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - Receive resource errors"]
pub type COUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - Receive resource errors"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - Receive resource errors"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<RXRESOURCEERRS_SPEC> {
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
#[doc = "Receive Resource Errors\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxresourceerrs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxresourceerrs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXRESOURCEERRS_SPEC;
impl crate::RegisterSpec for RXRESOURCEERRS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxresourceerrs::R`](R) reader structure"]
impl crate::Readable for RXRESOURCEERRS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxresourceerrs::W`](W) writer structure"]
impl crate::Writable for RXRESOURCEERRS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXRESOURCEERRS to value 0"]
impl crate::Resettable for RXRESOURCEERRS_SPEC {
    const RESET_VALUE: u32 = 0;
}
