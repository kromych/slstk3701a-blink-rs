#[doc = "Register `PRECNT` reader"]
pub type R = crate::R<PRECNT_SPEC>;
#[doc = "Register `PRECNT` writer"]
pub type W = crate::W<PRECNT_SPEC>;
#[doc = "Field `PRECNT` reader - Pre-Counter Value"]
pub type PRECNT_R = crate::FieldReader<u16>;
#[doc = "Field `PRECNT` writer - Pre-Counter Value"]
pub type PRECNT_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - Pre-Counter Value"]
    #[inline(always)]
    pub fn precnt(&self) -> PRECNT_R {
        PRECNT_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Pre-Counter Value"]
    #[inline(always)]
    #[must_use]
    pub fn precnt(&mut self) -> PRECNT_W<PRECNT_SPEC> {
        PRECNT_W::new(self, 0)
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
#[doc = "Pre-Counter Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`precnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`precnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRECNT_SPEC;
impl crate::RegisterSpec for PRECNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`precnt::R`](R) reader structure"]
impl crate::Readable for PRECNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`precnt::W`](W) writer structure"]
impl crate::Writable for PRECNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRECNT to value 0"]
impl crate::Resettable for PRECNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
