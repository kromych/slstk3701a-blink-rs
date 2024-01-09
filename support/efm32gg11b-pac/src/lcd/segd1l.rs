#[doc = "Register `SEGD1L` reader"]
pub type R = crate::R<SEGD1L_SPEC>;
#[doc = "Register `SEGD1L` writer"]
pub type W = crate::W<SEGD1L_SPEC>;
#[doc = "Field `SEGD1L` reader - COM1 Segment Data Low"]
pub type SEGD1L_R = crate::FieldReader<u32>;
#[doc = "Field `SEGD1L` writer - COM1 Segment Data Low"]
pub type SEGD1L_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - COM1 Segment Data Low"]
    #[inline(always)]
    pub fn segd1l(&self) -> SEGD1L_R {
        SEGD1L_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - COM1 Segment Data Low"]
    #[inline(always)]
    #[must_use]
    pub fn segd1l(&mut self) -> SEGD1L_W<SEGD1L_SPEC> {
        SEGD1L_W::new(self, 0)
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
#[doc = "Segment Data Low Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`segd1l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`segd1l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEGD1L_SPEC;
impl crate::RegisterSpec for SEGD1L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`segd1l::R`](R) reader structure"]
impl crate::Readable for SEGD1L_SPEC {}
#[doc = "`write(|w| ..)` method takes [`segd1l::W`](W) writer structure"]
impl crate::Writable for SEGD1L_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEGD1L to value 0"]
impl crate::Resettable for SEGD1L_SPEC {
    const RESET_VALUE: u32 = 0;
}
