#[doc = "Register `SCANFIFOCLEAR` writer"]
pub type W = crate::W<SCANFIFOCLEAR_SPEC>;
#[doc = "Field `SCANFIFOCLEAR` writer - Clear Scan FIFO Content"]
pub type SCANFIFOCLEAR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear Scan FIFO Content"]
    #[inline(always)]
    #[must_use]
    pub fn scanfifoclear(&mut self) -> SCANFIFOCLEAR_W<SCANFIFOCLEAR_SPEC> {
        SCANFIFOCLEAR_W::new(self, 0)
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
#[doc = "Scan FIFO Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scanfifoclear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCANFIFOCLEAR_SPEC;
impl crate::RegisterSpec for SCANFIFOCLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`scanfifoclear::W`](W) writer structure"]
impl crate::Writable for SCANFIFOCLEAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCANFIFOCLEAR to value 0"]
impl crate::Resettable for SCANFIFOCLEAR_SPEC {
    const RESET_VALUE: u32 = 0;
}
