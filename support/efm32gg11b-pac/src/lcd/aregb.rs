#[doc = "Register `AREGB` reader"]
pub type R = crate::R<AREGB_SPEC>;
#[doc = "Register `AREGB` writer"]
pub type W = crate::W<AREGB_SPEC>;
#[doc = "Field `AREGB` reader - Animation Register B Data"]
pub type AREGB_R = crate::FieldReader;
#[doc = "Field `AREGB` writer - Animation Register B Data"]
pub type AREGB_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Animation Register B Data"]
    #[inline(always)]
    pub fn aregb(&self) -> AREGB_R {
        AREGB_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Animation Register B Data"]
    #[inline(always)]
    #[must_use]
    pub fn aregb(&mut self) -> AREGB_W<AREGB_SPEC> {
        AREGB_W::new(self, 0)
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
#[doc = "Animation Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aregb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aregb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AREGB_SPEC;
impl crate::RegisterSpec for AREGB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aregb::R`](R) reader structure"]
impl crate::Readable for AREGB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aregb::W`](W) writer structure"]
impl crate::Writable for AREGB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AREGB to value 0"]
impl crate::Resettable for AREGB_SPEC {
    const RESET_VALUE: u32 = 0;
}
