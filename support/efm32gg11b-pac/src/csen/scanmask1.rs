#[doc = "Register `SCANMASK1` reader"]
pub type R = crate::R<SCANMASK1_SPEC>;
#[doc = "Register `SCANMASK1` writer"]
pub type W = crate::W<SCANMASK1_SPEC>;
#[doc = "Field `SCANINPUTEN` reader - Scan Channel Mask."]
pub type SCANINPUTEN_R = crate::FieldReader<u32>;
#[doc = "Field `SCANINPUTEN` writer - Scan Channel Mask."]
pub type SCANINPUTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Scan Channel Mask."]
    #[inline(always)]
    pub fn scaninputen(&self) -> SCANINPUTEN_R {
        SCANINPUTEN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Scan Channel Mask."]
    #[inline(always)]
    #[must_use]
    pub fn scaninputen(&mut self) -> SCANINPUTEN_W<SCANMASK1_SPEC> {
        SCANINPUTEN_W::new(self, 0)
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
#[doc = "Scan Channel Mask 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scanmask1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scanmask1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCANMASK1_SPEC;
impl crate::RegisterSpec for SCANMASK1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scanmask1::R`](R) reader structure"]
impl crate::Readable for SCANMASK1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scanmask1::W`](W) writer structure"]
impl crate::Writable for SCANMASK1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCANMASK1 to value 0"]
impl crate::Resettable for SCANMASK1_SPEC {
    const RESET_VALUE: u32 = 0;
}
