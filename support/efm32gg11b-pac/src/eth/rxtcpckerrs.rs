#[doc = "Register `RXTCPCKERRS` reader"]
pub type R = crate::R<RXTCPCKERRS_SPEC>;
#[doc = "Register `RXTCPCKERRS` writer"]
pub type W = crate::W<RXTCPCKERRS_SPEC>;
#[doc = "Field `COUNT` reader - TCP checksum errors"]
pub type COUNT_R = crate::FieldReader;
#[doc = "Field `COUNT` writer - TCP checksum errors"]
pub type COUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - TCP checksum errors"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TCP checksum errors"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<RXTCPCKERRS_SPEC> {
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
#[doc = "TCP Checksum Errors\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxtcpckerrs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxtcpckerrs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXTCPCKERRS_SPEC;
impl crate::RegisterSpec for RXTCPCKERRS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxtcpckerrs::R`](R) reader structure"]
impl crate::Readable for RXTCPCKERRS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxtcpckerrs::W`](W) writer structure"]
impl crate::Writable for RXTCPCKERRS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXTCPCKERRS to value 0"]
impl crate::Resettable for RXTCPCKERRS_SPEC {
    const RESET_VALUE: u32 = 0;
}
