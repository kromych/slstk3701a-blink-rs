#[doc = "Register `ETMISCIN` reader"]
pub type R = crate::R<ETMISCIN_SPEC>;
#[doc = "Register `ETMISCIN` writer"]
pub type W = crate::W<ETMISCIN_SPEC>;
#[doc = "Field `EXTIN` reader - EXTIN Value"]
pub type EXTIN_R = crate::FieldReader;
#[doc = "Field `EXTIN` writer - EXTIN Value"]
pub type EXTIN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COREHALT` reader - Core Halt"]
pub type COREHALT_R = crate::BitReader;
#[doc = "Field `COREHALT` writer - Core Halt"]
pub type COREHALT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - EXTIN Value"]
    #[inline(always)]
    pub fn extin(&self) -> EXTIN_R {
        EXTIN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Core Halt"]
    #[inline(always)]
    pub fn corehalt(&self) -> COREHALT_R {
        COREHALT_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - EXTIN Value"]
    #[inline(always)]
    #[must_use]
    pub fn extin(&mut self) -> EXTIN_W<ETMISCIN_SPEC> {
        EXTIN_W::new(self, 0)
    }
    #[doc = "Bit 4 - Core Halt"]
    #[inline(always)]
    #[must_use]
    pub fn corehalt(&mut self) -> COREHALT_W<ETMISCIN_SPEC> {
        COREHALT_W::new(self, 4)
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
#[doc = "Integration Test Miscellaneous Inputs Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmiscin::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etmiscin::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETMISCIN_SPEC;
impl crate::RegisterSpec for ETMISCIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmiscin::R`](R) reader structure"]
impl crate::Readable for ETMISCIN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etmiscin::W`](W) writer structure"]
impl crate::Writable for ETMISCIN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETMISCIN to value 0"]
impl crate::Resettable for ETMISCIN_SPEC {
    const RESET_VALUE: u32 = 0;
}
