#[doc = "Register `ETMFFLR` reader"]
pub type R = crate::R<ETMFFLR_SPEC>;
#[doc = "Register `ETMFFLR` writer"]
pub type W = crate::W<ETMFFLR_SPEC>;
#[doc = "Field `BYTENUM` reader - Bytes left in FIFO"]
pub type BYTENUM_R = crate::FieldReader;
#[doc = "Field `BYTENUM` writer - Bytes left in FIFO"]
pub type BYTENUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Bytes left in FIFO"]
    #[inline(always)]
    pub fn bytenum(&self) -> BYTENUM_R {
        BYTENUM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bytes left in FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn bytenum(&mut self) -> BYTENUM_W<ETMFFLR_SPEC> {
        BYTENUM_W::new(self, 0)
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
#[doc = "ETM Fifo Full Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmfflr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etmfflr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETMFFLR_SPEC;
impl crate::RegisterSpec for ETMFFLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmfflr::R`](R) reader structure"]
impl crate::Readable for ETMFFLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etmfflr::W`](W) writer structure"]
impl crate::Writable for ETMFFLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETMFFLR to value 0"]
impl crate::Resettable for ETMFFLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
