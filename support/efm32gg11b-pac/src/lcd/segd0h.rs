#[doc = "Register `SEGD0H` reader"]
pub type R = crate::R<SEGD0H_SPEC>;
#[doc = "Register `SEGD0H` writer"]
pub type W = crate::W<SEGD0H_SPEC>;
#[doc = "Field `SEGD0H` reader - COM0 Segment Data High"]
pub type SEGD0H_R = crate::FieldReader;
#[doc = "Field `SEGD0H` writer - COM0 Segment Data High"]
pub type SEGD0H_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - COM0 Segment Data High"]
    #[inline(always)]
    pub fn segd0h(&self) -> SEGD0H_R {
        SEGD0H_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - COM0 Segment Data High"]
    #[inline(always)]
    #[must_use]
    pub fn segd0h(&mut self) -> SEGD0H_W<SEGD0H_SPEC> {
        SEGD0H_W::new(self, 0)
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
#[doc = "Segment Data High Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`segd0h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`segd0h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEGD0H_SPEC;
impl crate::RegisterSpec for SEGD0H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`segd0h::R`](R) reader structure"]
impl crate::Readable for SEGD0H_SPEC {}
#[doc = "`write(|w| ..)` method takes [`segd0h::W`](W) writer structure"]
impl crate::Writable for SEGD0H_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEGD0H to value 0"]
impl crate::Resettable for SEGD0H_SPEC {
    const RESET_VALUE: u32 = 0;
}
