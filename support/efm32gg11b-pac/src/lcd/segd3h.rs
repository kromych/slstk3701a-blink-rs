#[doc = "Register `SEGD3H` reader"]
pub type R = crate::R<SEGD3H_SPEC>;
#[doc = "Register `SEGD3H` writer"]
pub type W = crate::W<SEGD3H_SPEC>;
#[doc = "Field `SEGD3H` reader - COM3 Segment Data High"]
pub type SEGD3H_R = crate::FieldReader;
#[doc = "Field `SEGD3H` writer - COM3 Segment Data High"]
pub type SEGD3H_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - COM3 Segment Data High"]
    #[inline(always)]
    pub fn segd3h(&self) -> SEGD3H_R {
        SEGD3H_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - COM3 Segment Data High"]
    #[inline(always)]
    #[must_use]
    pub fn segd3h(&mut self) -> SEGD3H_W<SEGD3H_SPEC> {
        SEGD3H_W::new(self, 0)
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
#[doc = "Segment Data High Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`segd3h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`segd3h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEGD3H_SPEC;
impl crate::RegisterSpec for SEGD3H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`segd3h::R`](R) reader structure"]
impl crate::Readable for SEGD3H_SPEC {}
#[doc = "`write(|w| ..)` method takes [`segd3h::W`](W) writer structure"]
impl crate::Writable for SEGD3H_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEGD3H to value 0"]
impl crate::Resettable for SEGD3H_SPEC {
    const RESET_VALUE: u32 = 0;
}
