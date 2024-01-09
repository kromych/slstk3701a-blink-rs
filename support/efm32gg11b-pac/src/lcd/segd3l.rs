#[doc = "Register `SEGD3L` reader"]
pub type R = crate::R<SEGD3L_SPEC>;
#[doc = "Register `SEGD3L` writer"]
pub type W = crate::W<SEGD3L_SPEC>;
#[doc = "Field `SEGD3L` reader - COM3 Segment Data Low"]
pub type SEGD3L_R = crate::FieldReader<u32>;
#[doc = "Field `SEGD3L` writer - COM3 Segment Data Low"]
pub type SEGD3L_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - COM3 Segment Data Low"]
    #[inline(always)]
    pub fn segd3l(&self) -> SEGD3L_R {
        SEGD3L_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - COM3 Segment Data Low"]
    #[inline(always)]
    #[must_use]
    pub fn segd3l(&mut self) -> SEGD3L_W<SEGD3L_SPEC> {
        SEGD3L_W::new(self, 0)
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
#[doc = "Segment Data Low Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`segd3l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`segd3l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEGD3L_SPEC;
impl crate::RegisterSpec for SEGD3L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`segd3l::R`](R) reader structure"]
impl crate::Readable for SEGD3L_SPEC {}
#[doc = "`write(|w| ..)` method takes [`segd3l::W`](W) writer structure"]
impl crate::Writable for SEGD3L_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEGD3L to value 0"]
impl crate::Resettable for SEGD3L_SPEC {
    const RESET_VALUE: u32 = 0;
}
