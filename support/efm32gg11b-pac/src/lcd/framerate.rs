#[doc = "Register `FRAMERATE` reader"]
pub type R = crate::R<FRAMERATE_SPEC>;
#[doc = "Register `FRAMERATE` writer"]
pub type W = crate::W<FRAMERATE_SPEC>;
#[doc = "Field `FRDIV` reader - Frame Rate Divider"]
pub type FRDIV_R = crate::FieldReader<u16>;
#[doc = "Field `FRDIV` writer - Frame Rate Divider"]
pub type FRDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Frame Rate Divider"]
    #[inline(always)]
    pub fn frdiv(&self) -> FRDIV_R {
        FRDIV_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Frame Rate Divider"]
    #[inline(always)]
    #[must_use]
    pub fn frdiv(&mut self) -> FRDIV_W<FRAMERATE_SPEC> {
        FRDIV_W::new(self, 0)
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
#[doc = "Frame Rate\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`framerate::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`framerate::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRAMERATE_SPEC;
impl crate::RegisterSpec for FRAMERATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`framerate::R`](R) reader structure"]
impl crate::Readable for FRAMERATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`framerate::W`](W) writer structure"]
impl crate::Writable for FRAMERATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FRAMERATE to value 0"]
impl crate::Resettable for FRAMERATE_SPEC {
    const RESET_VALUE: u32 = 0;
}
